#!/usr/bin/env bash
# rustc CRATE CENSUS -- T0 (PARSE) over all 72 compiler crates.
#
# WHAT THIS MEASURES (F3: every reading names its RUNG and its ORACLE).
#   RUNG   = SEM0 -- $SP/SEM0.rzrs, else $SP/wf.gen1 (the L6-built s1_parser). The rung that self-hosts.
#            NOT L6.rzrs: that is the retiring bootstrap, and measuring it is how the north star spent three
#            weeks reporting 12% for a component being retired (see
#            ../sem0/locales/northstar-rung/, C-REACH-WRONG-RUNG).
#   ORACLE = NONE. T0 asks "does the crate parse, whole, with its module tree resolved" -- a property of our
#            compiler alone. Tiers that need an oracle (T1' resolve / T2 accept / T3 run) are NOT measured here
#            because stable rustc cannot even build these crates un-vendored (rustc_lexer: 7 errors). See seed.md.
#
# T0 IS NOT CORRECTNESS. A crate that parses may still build the wrong AST. Do not quote a T0 count as progress
# toward `== rustc`; it is the floor that makes the oracle tiers worth attempting.
#
#   SP=<scratch with SEM0> bash crate-census.sh            # census; writes board.txt
#   SP=... BASELINE=1 bash crate-census.sh                 # (re)write baseline.txt from this run
#   SP=... SRC=/path/to/rust-1.95.0 bash crate-census.sh   # use an existing extracted tree
#
# The source is PINNED (TAG below) and cached under $CACHE, fetched once as a 26MB tarball. Network is
# ORACLE-class here: rustc source is fetched for MEASUREMENT and never enters the produced chain.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
TAG="${TAG:-1.95.0}"
CACHE="${CACHE:-/tmp/rustc-src}"
SRC="${SRC:-$CACHE/rust-$TAG}"
SP="${SP:?set SP (scratch holding SEM0.rzrs or wf.gen1)}"

FE="$SP/SEM0.rzrs"; [ -x "$FE" ] || FE="$SP/wf.gen1"
[ -x "$FE" ] || { echo "FAIL: no SEM0 binary in \$SP (expected SEM0.rzrs or wf.gen1 -- run s9-wholefile-diff.sh)"; exit 1; }
# §11: a stub binary exits 0 on everything. Refuse to measure with one.
sz=$(stat -c%s "$FE"); [ "$sz" -ge 1000000 ] || { echo "FAIL: SEM0 binary is $sz bytes (stub) -- cannot census"; exit 1; }

# --- stage the pinned source ------------------------------------------------
if [ ! -d "$SRC/compiler" ]; then
  mkdir -p "$CACHE"; cd "$CACHE"
  [ -f "rust-$TAG.tar.gz" ] || { echo "fetching rust-$TAG source (~26MB, once)..."
    timeout 900 curl -sL "https://codeload.github.com/rust-lang/rust/tar.gz/refs/tags/$TAG" -o "rust-$TAG.tar.gz" \
      || { echo "FAIL: could not fetch rust-$TAG"; exit 1; }; }
  tar xzf "rust-$TAG.tar.gz" --wildcards "rust-$TAG/compiler/*" 2>/dev/null
fi
[ -d "$SRC/compiler" ] || { echo "FAIL: no $SRC/compiler"; exit 1; }

W="$(mktemp -d)"; trap 'rm -rf "$W"' EXIT
BOARD="${BOARD:-$HERE/board.txt}"; BASE="$HERE/baseline.txt"

# --- the T0 probe for one crate ---------------------------------------------
# Returns: "<grade> <lines> <mods> <detail>"
#   grade T0        parses whole, module tree spliced, every declared+present mod appears in the output
#   grade SPLICE    ran clean but a declared module that EXISTS ON DISK never appeared -> the read did not happen.
#                   This is the trap recorded in trajectory R0: in the DEFAULT (AST-dump) mode the mod splice does
#                   not fire at all and the output is byte-identical with the submodules deleted, exit 0. Grading
#                   such a run T0 would report a whole crate "parsing" while one file of N was read.
#   grade CRASH     the compiler DIED (signal) on EVERY run. Outranks a rejection: it did not reject, it crashed.
#   grade FLAKY     the runs DISAGREED -- some crashed, some did not. Neither outcome is established.
#   grade FAIL      a diagnostic fired; detail = the first one
#   grade TIMEOUT / EMPTY   the run did not produce an admissible observation
probe() {
  # T0 PARSE, measured as an OUTCOME (F1), not through a resolve proxy. Every `.rs` module under the crate's
  # `src/` is fed to the parser STANDALONE (default parse mode, no marker); T0 requires ALL of them to parse
  # (rc 0). Feeding every file makes the R0 splice-trap structural -- you cannot "read one file of N" when you
  # feed all N -- so no CWD/`mod` splice and no name resolution is needed here. Name/path resolution is T1', a
  # DISTINCT tier the oracle adjudicates (a crate that parses whole but REFUSES in //@S3RESOLVE is T0-passing
  # and T1'-blocked, exactly rustc_middle 2026-09-05). Grades: T0 = every module parses; PARSEFAIL = a module
  # is cleanly refused (rc 101 -- a real parse gap, named); CRASH = a module DIED (signal) -- outranks, and is
  # re-confirmed 3x before it is believed (R2: resolve-mode crashes were nondeterministic; parse deaths are
  # re-checked the same way).
  local dir="$1" root="$2" lines rc f tot=0 ok=0 fails="" crash=""
  lines=$(find "$dir/src" -name '*.rs' -exec cat {} + 2>/dev/null | wc -l)
  # `include!("X.rs")` targets are FRAGMENTS, not standalone modules (e.g. the generated archs.rs = a bare
  # `match` arm body pulled into llvm.rs). rustc does not parse them standalone either, so feeding one to the
  # parser is a false PARSEFAIL. Exclude include! targets by basename, never excluding a real module root.
  local inc; inc=$(grep -rhoE 'include!\(\s*"[^"]+\.rs"' "$dir/src" 2>/dev/null | grep -oE '"[^"]+\.rs"' | tr -d '"' | xargs -n1 basename 2>/dev/null | grep -vxE 'mod\.rs|lib\.rs|main\.rs' | sort -u)
  while IFS= read -r f; do
    if [ -n "$inc" ] && echo "$inc" | grep -qxF "$(basename "$f")"; then continue; fi
    tot=$((tot+1))
    ( timeout 180 "$FE" < "$f" >/dev/null 2>&1 ); rc=$?
    if [ "$rc" -ge 128 ]; then
      # a signal: re-confirm 3x (deterministic crash) before grading CRASH
      local j sig=0
      for j in 1 2 3; do ( timeout 180 "$FE" < "$f" >/dev/null 2>&1 ); [ "$?" -ge 128 ] && sig=$((sig+1)); done
      if [ "$sig" -ge 2 ]; then crash="$crash ${f#$dir/src/}"; else fails="$fails ${f#$dir/src/}?flaky"; fi
    elif [ "$rc" -ne 0 ]; then fails="$fails ${f#$dir/src/}"
    else ok=$((ok+1)); fi
  done < <(find "$dir/src" -name '*.rs' 2>/dev/null | sort)
  if [ "$tot" -eq 0 ]; then echo "NOROOT $lines 0 (no .rs under src)"; return; fi
  if [ -n "$crash" ]; then echo "CRASH $lines $tot (module died:${crash:0:48})"; return; fi
  if [ -n "$fails" ]; then echo "PARSEFAIL $lines $tot ($((tot-ok))/$tot refuse:${fails:0:44})"; return; fi
  echo "T0 $lines $tot (all $tot modules parse)"
}
# --- run ---------------------------------------------------------------------
echo "== rustc crate census -- T0 PARSE =="
echo "   RUNG   : SEM0  ($FE, $sz bytes)"
echo "   ORACLE : none (T0 needs none; T1'/T2/T3 need vendoring -- see seed.md)"
echo "   SOURCE : rust-$TAG  ($(ls "$SRC/compiler" | wc -l) crates, $(find "$SRC/compiler" -name '*.rs' | wc -l) files)"
echo
printf '%-32s %-8s %8s %6s  %s\n' CRATE GRADE LINES MODS DETAIL
: > "$W/board"
for cd in "$SRC"/compiler/*/; do
  c=$(basename "$cd")
  root=""
  for cand in "$cd/src/lib.rs" "$cd/src/main.rs"; do [ -f "$cand" ] && { root="$cand"; break; }; done
  [ -n "$root" ] || { printf '%-32s %-8s %8s %6s  %s\n' "$c" "NOROOT" "-" "-" "no src/lib.rs or src/main.rs"; continue; }
  read -r g l m d <<<"$(probe "$cd" "$root")"
  printf '%-32s %-8s %8s %6s  %s\n' "$c" "$g" "$l" "$m" "$d"
  echo "$c $g $l" >> "$W/board"
done | tee "$W/full"

# --- distribution + the ranked blocker board ---------------------------------
echo
echo "== T0 distribution =="
awk '{print $2}' "$W/board" | sort | uniq -c | sort -rn | sed 's/^/   /'
tot=$(wc -l < "$W/board"); t0=$(awk '$2=="T0"' "$W/board" | wc -l)
lt=$(awk '{s+=$3} END{print s+0}' "$W/board"); l0=$(awk '$2=="T0"{s+=$3} END{print s+0}' "$W/board")
echo "   ---- $t0/$tot crates parse whole (T0);  $l0 / $lt lines ($(( l0*100/lt ))%) in T0 crates"
echo
echo "== ranked blockers (first diagnostic, by crate count) =="
grep -E " (FAIL|SPLICE|EMPTY|TIMEOUT|CRASH|FLAKY|NOROOT) " "$W/full" | sed -E 's/^[a-z_0-9]+ +[A-Z]+ +[0-9-]+ +[0-9-]+ +//' \
  | sed -E "s/[0-9]+/N/g; s/\`[^\`]*\`/\`X\`/g" | sort | uniq -c | sort -rn | head -20 | sed 's/^/   /'

awk '{print $1, $2}' "$W/board" | sort > "$BOARD"
if [ "${BASELINE:-0}" = 1 ]; then cp "$BOARD" "$BASE"; echo; echo "   BASELINED $BASE ($tot crates)"; fi

# --- F4: the instrument asserts its own retirement condition -----------------
echo
if [ "$t0" -eq "$tot" ]; then
  echo "   T0 COMPLETE: every crate parses. RETIREMENT is at T3 (all crates RUN == rustc), not here --"
  echo "   T0 is the floor, not correctness. Next: vendor the leaf tier and build crate-gate.sh."
else
  echo "   RETIREMENT CONDITION NOT MET (retires at T3 = all $tot crates RUN == rustc; T0 is $t0/$tot)."
fi
