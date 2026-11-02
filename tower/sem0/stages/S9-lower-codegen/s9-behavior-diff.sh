#!/usr/bin/env bash
# S9 BEHAVIOURAL differential -- what the emission-comparing gates structurally cannot see.
#
# WHY THIS EXISTS. On 2026-08-25 `std::process::exit(N)` was found to be a SILENT NO-OP in emit_s9: it compiles,
# emits, warns about nothing, and does nothing (exit 0 where rustc gives N). A refuse-cleanly fix therefore worked
# in gen1 (L6-built SEM0) and was behaviourally ABSENT in gen2 (SEM0-built SEM0) -- `delim_balance` was present in
# gen2's rzir, only the exit was dropped. All 16 standing gates were GREEN throughout.
#
# They were green for a structural reason, not an oversight:
#   SELF-EMIT   compares the rz-built and rustc-built EMISSIONS. Both run emit_s9, so a gap shared by both is
#               byte-identical on both sides. It can never catch an emit_s9 fidelity gap.
#   WITNESS     gen1 vs gen2 on EMPTY stdin -- exercises no exit path.
#   FIXED POINT gen2 == gen3: a fixed point of the EMISSION, not fidelity to the source.
#   WHOLE-FILE  35 fixtures, none malformed, none asserting an exit code.
#
# So every existing check compares emissions to each other. This one compares BEHAVIOUR to the source's meaning,
# via the only oracle that has it: rustc.
#
#   A. FIDELITY  emit_s9-built program == rustc-built program   (exit code AND stdout)   <- catches process::exit
#   B. AGREEMENT gen1 and gen2 behave identically on the same input (exit, stdout, stderr-empty-or-not)
#
# Known divergences live in behavior-baseline.txt and do NOT fail the gate -- they are recorded debt. Anything
# NEW fails. Same contract as r2-corpus/corpus-gate.sh.
#
#   SP=<scratch with wf.gen1 + wf.gen2 + cf.hoist> CC=<cc-in-c> bash s9-behavior-diff.sh
#   REBASELINE=1 SP=... bash s9-behavior-diff.sh     # adopt current divergences (use deliberately)
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
SP="${SP:?set SP}"; CC="${CC:?set CC (2026-06-30-cc-in-c)}"
BASE="$HERE/behavior-baseline.txt"
W="$(mktemp -d)"; trap 'rm -rf "$W"' EXIT

for b in wf.gen1 wf.gen2; do
  [ -x "$SP/$b" ] || { echo "  FAIL: no \$SP/$b -- run s9-wholefile-diff.sh first"; exit 1; }
  sz=$(stat -c%s "$SP/$b"); [ "$sz" -ge 100000 ] || { echo "  FAIL: $b is $sz bytes (stub)"; exit 1; }
done
command -v rustc >/dev/null || { echo "  skip: no rustc (this gate is an oracle differential)"; exit 0; }

# Part A (FIDELITY) codegens the TARGET program -> the meta-telos LLVM bridge (cf is self-host only). Part B
# (AGREEMENT) is emit-vs-emit (rzir text) and backend-independent. RZ_R2LCC=wf.gen1 lowers rzir2ll.rs for the gate.
RZ_R2LCC="${RZ_R2LCC:-$SP/wf.gen1}"
. "$HERE/../../../lib/llvm-backend.sh"
rz_backend_ready || { echo "  SKIP s9-behavior-diff: $RZ_BACKEND_MSG"; exit 0; }
echo "  [s9-behavior-diff Part-A backend: $RZ_BACKEND_EFFECTIVE]"
back(){ rz_back; }

: > "$W/live"
fail=0

# ---------- A. FIDELITY: emit_s9-built == rustc-built (exit code AND stdout) ----------
# The battery deliberately leads with exit-code contracts, the axis no existing gate observes.
fid() { # fid NAME SOURCE
  local n="$1" src="$2" a b ao bo
  printf '%s\n' "$src" > "$W/p.rs"
  { echo '//@S9'; cat "$W/p.rs"; } > "$W/p9.rs"
  timeout 60 "$SP/wf.gen1" < "$W/p9.rs" 2>/dev/null | back > "$W/p.elf" 2>/dev/null; chmod +x "$W/p.elf" 2>/dev/null
  ao=$(timeout 10 "$W/p.elf" 2>/dev/null); a=$?
  if rustc --edition 2021 -O -o "$W/p.rc" "$W/p.rs" 2>/dev/null; then
    bo=$(timeout 10 "$W/p.rc" 2>/dev/null); b=$?
  else echo "  skip $n (rustc cannot build the probe)"; return; fi
  if [ "$a" = "$b" ] && [ "$ao" = "$bo" ]; then echo "A/$n OK" >> "$W/live"
  else echo "A/$n DIVERGE(emit_s9=$a:'$ao' rustc=$b:'$bo')" >> "$W/live"; fi
}
fid exit1        'fn main() { std::process::exit(1); }'
fid exit3        'fn main() { std::process::exit(3); }'
fid exit_after_print 'fn main() { println!("x"); std::process::exit(1); }'
fid exit0_explicit   'fn main() { std::process::exit(0); }'
fid return_early 'fn main() { if true { return; } println!("no"); }'
fid plain_main   'fn main() { println!("ok"); }'
fid arith        'fn main() { let a = 6 * 7; println!("{}", a); }'

# ---------- B. AGREEMENT: gen1 and gen2 behave identically ----------
# Includes MALFORMED input, which no existing gate feeds either binary.
agr() { # agr NAME INPUT-STRING
  local n="$1" a b ao bo ae be
  printf '%s' "$2" > "$W/in.rs"
  ao=$(timeout 30 "$SP/wf.gen1" < "$W/in.rs" 2>"$W/e1"); a=$?
  bo=$(timeout 30 "$SP/wf.gen2" < "$W/in.rs" 2>"$W/e2"); b=$?
  ae=$([ -s "$W/e1" ] && echo msg || echo -); be=$([ -s "$W/e2" ] && echo msg || echo -)
  if [ "$a" = "$b" ] && [ "$ao" = "$bo" ] && [ "$ae" = "$be" ]; then echo "B/$n OK" >> "$W/live"
  else echo "B/$n DIVERGE(gen1=$a/$ae gen2=$b/$be)" >> "$W/live"; fi
}
agr unterminated_brace  'fn f() {'
agr unterminated_paren  'fn f( '
agr unterminated_brack  'fn f() { let a = ['
agr unterminated_str    'fn f() { let s = "abc'
agr stray_closer        '}'
agr extra_closer        'fn f() { }
}'
agr garbage             '@#$%^&'
agr empty               ''
agr valid_small         'pub const N: u32 = 5;'
agr valid_fn            'fn f(a: i32) -> i32 { a + 1 }'

# ---------- compare to baseline ----------
sort "$W/live" > "$W/live.s"
n=$(wc -l < "$W/live.s"); ok=$(grep -c ' OK$' "$W/live.s" || true)
[ "$n" -ge 10 ] || { echo "  FAIL: only $n probes ran"; exit 1; }
if [ "${REBASELINE:-0}" = 1 ]; then cp "$W/live.s" "$BASE"; echo "  REBASELINED $BASE ($n probes)"; fi
[ -f "$BASE" ] || { echo "  FAIL: missing $BASE (run once with REBASELINE=1)"; exit 1; }

echo "  board: $ok/$n behavioural probes agree"
# a probe that was OK and now diverges is a REGRESSION; a probe that was diverging and now agrees is progress.
regr=$(join <(sed 's/(.*//' "$BASE" | sort) <(sed 's/(.*//' "$W/live.s" | sort) -o 0,1.2,2.2 2>/dev/null | awk '$2=="OK" && $3!="OK"{print "    " $1}')
impr=$(join <(sed 's/(.*//' "$BASE" | sort) <(sed 's/(.*//' "$W/live.s" | sort) -o 0,1.2,2.2 2>/dev/null | awk '$2!="OK" && $3=="OK"{print "    " $1}')
[ -n "$impr" ] && { echo "  IMPROVED:"; echo "$impr"; }
if [ -n "$regr" ]; then echo "  RED  REGRESSION -- these agreed at baseline:"; echo "$regr"; fail=1; fi
echo "  known divergences (recorded debt, not failures):"
grep ' DIVERGE' "$W/live.s" | sed 's/^/    /' || echo "    (none)"

echo "== S9 behavioural differential: $([ "$fail" = 0 ] && echo GREEN || echo RED) =="
[ "$fail" = 0 ]
