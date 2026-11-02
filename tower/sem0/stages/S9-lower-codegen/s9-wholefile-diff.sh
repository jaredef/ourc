#!/usr/bin/env bash
# S9 whole-file SELF-COMPILATION differential -- the gate that actually runs gen2.
#
# WHY THIS EXISTS. Every other standing gate exercises gen1 (the L6-built s1_parser) or the 52 sem witnesses.
# NOTHING ran gen2 -- emit_s9's SELF-COMPILED parser -- over a real crate. On 2026-08-24 a one-line lowering change
# (s9const(lit_value(*n)), refuted: see locales/s9-aggregate-word C-S1-LITERAL-PACK-AMBIGUOUS) took this board from
# 35/35 to 11 ok / 24 crash and passed ALL 14 standing gates; it was caught only by running this loop by hand.
# A self-compilation regression is invisible to a gate set that never self-compiles.
#
#   gen1 = L6.rzrs(s1_parser.rs)        -- the trusted side; s1-ast-diff.sh pins it against the rustc goldens
#   gen2 = gen1(//@S9 s1_parser.rs)     -- emit_s9 compiling itself, lowered through the same backend
#
# Three assertions, cheapest first:
#   SELF-EMIT   gen1's //@S9 self-emit == the rustc-built parser's  (byte-for-byte rzir; catches lowering drift)
#   WITNESS     gen2 < /dev/null == gen1 < /dev/null                (the 956-line embedded witness)
#   FIXED POINT gen2 < //@S9 s1_parser == gen1's self-emit   (gen2 == gen3; the SEM0 telos)
#   WHOLE-FILE  gen2 < fixture   == gen1 < fixture, for every S1 fixture
#
# Requires SP (L6.rzrs + cf.hoist, as the other S-tier gates) + CC (cc-in-c backend). Exits non-zero on any mismatch.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; S1="$HERE/../S1-lex-parse-ast"
SP="${SP:?set SP}"; CC="${CC:?set CC (2026-06-30-cc-in-c)}"
P="$S1/s1_parser.rs"
# >2GB ARENA (rung 284): the self-emit now declares __heap 2200MB (s1_parser), so wf.g2.rzir carries a
# heaptop >2^31 that the FROZEN C ld ($CC) saturates to INT32_MAX (i32) -> cf frame-collides -> SEGV. The Rust
# ld twin ($SP/ld.bin, i64 cur/heaptop) lowers it correctly and is DDC byte-identical to the C ld below 2GB
# (the whole provenance corpus), so this diverges from $CC only above 2GB, where only the Rust twin is correct.
back(){ "$SP/ld.bin" 2>/dev/null | "$CC/opt/rzir-opt.bin" 2>/dev/null | "$CC/ra/rzir-ra.bin" 2>/dev/null | "$SP/cf.hoist"; }
fail=0

# --- gen1: the trusted side --------------------------------------------------
# gen1 IS s1.rz (same SEM0_CC < $P | ld|opt|ra|cf.hoist pipeline): build-or-reuse it via the shared cache. It used to
# run under `timeout 300`, which the 1M-line self-compile outgrew (~11 min): the emit was cut, back linked a 4 MB stub
# that passed the size check and SEGV'd -> "EMPTY self-emit" (batch 7, and 2026-10-26 on a quiet machine).
. "$(dirname "$0")/../s1-build.sh"
cp "$SP/s1.rz" "$SP/wf.gen1"; chmod +x "$SP/wf.gen1"
sz=$(stat -c%s "$SP/wf.gen1" 2>/dev/null || echo 0)
[ "$sz" -ge 100000 ] || { echo "  FAIL build: gen1 is $sz bytes (segfault stub) -- cannot gate"; exit 1; }

# --- gen1 self-emit, and the rustc oracle for it -----------------------------
{ echo '//@S9'; cat "$P"; } > "$SP/wf.s9"
timeout 1800 "$SP/wf.gen1" < "$SP/wf.s9" > "$SP/wf.g2.rzir" 2>/dev/null
[ -s "$SP/wf.g2.rzir" ] || { echo "  FAIL build: gen1 produced an EMPTY self-emit -- cannot gate"; exit 1; }
if rustc --edition 2021 "$P" -o "$SP/wf.rc" 2>/dev/null; then
  timeout 1800 "$SP/wf.rc" < "$SP/wf.s9" > "$SP/wf.rc.rzir" 2>/dev/null
  if cmp -s "$SP/wf.g2.rzir" "$SP/wf.rc.rzir"; then
    echo "  ok   SELF-EMIT: rz-built == rustc-built ($(wc -l < "$SP/wf.g2.rzir") rzir lines, byte-identical)"
  else
    echo "  RED  SELF-EMIT: rz-built != rustc-built"; diff "$SP/wf.rc.rzir" "$SP/wf.g2.rzir" | head -4; fail=1
  fi
else
  echo "  skip SELF-EMIT: no rustc available (oracle-only check)"
fi

# --- gen2 --------------------------------------------------------------------
back > "$SP/wf.gen2" < "$SP/wf.g2.rzir" 2>/dev/null; chmod +x "$SP/wf.gen2"
sz=$(stat -c%s "$SP/wf.gen2" 2>/dev/null || echo 0)
[ "$sz" -ge 100000 ] || { echo "  FAIL build: gen2 is $sz bytes (segfault stub) -- cannot gate"; exit 1; }

# --- the embedded witness (empty stdin) --------------------------------------
"$SP/wf.gen1" < /dev/null > "$SP/wf.w1" 2>/dev/null
"$SP/wf.gen2" < /dev/null > "$SP/wf.w2" 2>/dev/null
if cmp -s "$SP/wf.w1" "$SP/wf.w2"; then echo "  ok   WITNESS: gen2 == gen1 ($(wc -l < "$SP/wf.w1") lines)"
else echo "  RED  WITNESS: gen2 != gen1"; diff "$SP/wf.w1" "$SP/wf.w2" | head -4; fail=1; fi

# --- the SEM0 FIXED POINT: gen2 compiles itself to gen2's own rzir ------------
# Reached 2026-08-24 and asserted here so it cannot regress silently. Nothing else in the gate set runs gen2 as a
# COMPILER over a real crate: the whole-file assertion below runs it as a DUMPER, and self-emit byte-identity above
# is gen1's. Costs ~1s.
timeout 1800 "$SP/wf.gen2" < "$SP/wf.s9" > "$SP/wf.g3.rzir" 2>/dev/null; rc=$?
if [ "$rc" -ne 0 ]; then
  echo "  RED  FIXED POINT: gen2 self-emit failed (exit $rc, $(wc -l < "$SP/wf.g3.rzir") lines)"; fail=1
elif cmp -s "$SP/wf.g2.rzir" "$SP/wf.g3.rzir"; then
  echo "  ok   FIXED POINT: gen2 == gen3 ($(wc -l < "$SP/wf.g3.rzir") rzir lines, byte-identical)"
else
  echo "  RED  FIXED POINT: gen2 != gen3"; diff "$SP/wf.g2.rzir" "$SP/wf.g3.rzir" | head -4; fail=1
fi

# --- CLOSED WORLD (locales/sem0-closed-world) ---------------------------------
# C-RZIR-CLOSED-WORLD: every call target in the self-emit must be DEFINED in it. This is the fail-closed
# predicate the substrate always satisfied and never asserted -- so nothing noticed when the dev image drifted
# to one undefined symbol (`std::fs::read_to_string`, closed 2026-08-28). An undefined target does not fail the
# build today: rzir-ld accepts it at rc=0 and the emitted call corrupts a real frame at runtime, which is the
# fail-open shape that makes 549 of the 571 mint-gap probes MISCOMP instead of refusals.
# No allowlist and no `__` convention is needed: the runtime prelude is emitted as ordinary `fn` definitions,
# so the backend-provided set is empty and the rule is plain set containment.
# Asserted on wf.g2.rzir -- the image the tier UNDER TEST produced. Asserting the frozen tier's output instead
# would measure the tier being retired, which is the `northstar-rung` failure this arc has already paid for.
cwc="$(mktemp)"; cwd="$(mktemp)"
grep -oE '= call [A-Za-z_][A-Za-z0-9_:]*' "$SP/wf.g2.rzir" | awk '{print $3}' | sort -u > "$cwc"
grep -oE '^fn [A-Za-z_][A-Za-z0-9_:]*'    "$SP/wf.g2.rzir" | awk '{print $2}' | sort -u > "$cwd"
cwu="$(comm -23 "$cwc" "$cwd")"; cwn="$(printf '%s' "$cwu" | grep -c . || true)"
if [ "$cwn" -eq 0 ]; then
  echo "  ok   CLOSED WORLD: every call target defined ($(wc -l < "$cwc") called, $(wc -l < "$cwd") defined)"
else
  echo "  RED  CLOSED WORLD: $cwn undefined call target(s) -- these compile to a silent miscompile, not an error"
  printf '%s\n' "$cwu" | sed 's/^/         /' | head -10
  fail=1
fi
rm -f "$cwc" "$cwd"

# --- SELF-IMAGE LINE HEADROOM (the twin ceiling) ------------------------------
# The C twins index lines with a 1000000-slot array (tower/L2/{link,opt,ra}/rzir-*.c); the Rust twins are at
# 6000000 and do not bind. (This comment said 500000 for all six, which was true before the 2026-09 bump.) `ld` is line-indexed, so
# a self-image past that overflows it -- and the Rust twin's failure mode is to emit 415 lines and EXIT 0, which
# surfaces downstream as a provenance DDC "C-path != Rust-path" that reads like the twins disagreeing when in
# fact one truncated. On 2026-08-25 a ~560-line emit_s9 change crossed it with 180 lines of headroom remaining
# and cost hours to diagnose. Assert the number here, where it is cheap and legible.
# See locales/sem0-refuse-cleanly/{CAPACITY-WALL,EMISSION-AUDIT}-2026-08-25.md.
# AMENDED 2026-10-16 (keeper-authorized): the two numbers here were conflated. TWINCAP is a WARNING line, not
# a ceiling -- the ceiling is TWINREAL, the smallest line-indexed array in the twins that actually bind
# (tower/L2/{link/rzir-ld,opt/rzir-opt,ra/rzir-ra}.c: gloff/gllen/loff/llen/cumcall, all [1000000]; the Rust
# twins are [6000000] and do not bind). Crossing the WARNING line is not an overflow and must not fail the
# tower: it says "start pricing the emission". Crossing TWINREAL is the silent truncation this clause exists
# to catch, and that still fails closed.
TWINCAP=1980000
TWINREAL=2000000
# RAISED 2026-10-26 (keeper-authorized): the L2 C twins line arrays 1000000 -> 2000000 and the opt/ra src/out byte
# buffers 24MB -> 48MB (ld 12MB -> 24MB, opt scr 2MB -> 4MB) -- the SEM0 self-image crossed 1,001,322 lines (batch 6).
img=$(wc -l < "$SP/wf.g2.rzir")
if [ "$img" -ge "$TWINREAL" ]; then
  echo "  RED  SELF-IMAGE: $img rzir lines -- AT OR OVER the $TWINREAL C-twin line ceiling. ld will truncate"
  echo "       SILENTLY (emits ~415 lines, exit 0) and provenance will fail as a DDC divergence. See EMISSION-AUDIT."; fail=1
elif [ "$img" -ge "$TWINCAP" ]; then
  echo "  warn SELF-IMAGE: $img rzir lines -- past the $TWINCAP warning line, $((TWINREAL-img)) lines under the"
  echo "       $TWINREAL C-twin ceiling. Not an overflow; price the emission before the next emit_s9 growth."
else
  echo "  ok   SELF-IMAGE: $img rzir lines of $TWINREAL C-twin ceiling -- $((TWINREAL-img)) lines of headroom"
fi

# --- ADDRESS-SPACE HEADROOM (C-EAS-BUDGET-SILENT) ----------------------------
# cf derives p_memsz as fbase/16MB + (FRAME_SZ/4096)*7/4 units and CLAMPS it to 255, silently. Recompute the
# UNCLAMPED value here from the same inputs and compare it to the byte cf actually wrote: a mismatch means the
# frame budget was truncated and the program is short of the depth cf claims to give it. This detects the clamp
# exactly rather than guessing from a threshold, and it needs no cf change.
# p_memsz is a 3-byte UNIT field since C-EAS rung 3 (was byte 3 alone, capped at 255 units = 4080MB); read all three.
ht=$("$SP/ld.bin" < "$SP/wf.g2.rzir" 2>/dev/null | "$CC/opt/rzir-opt.bin" 2>/dev/null | "$CC/ra/rzir-ra.bin" 2>/dev/null | grep -m1 '^heaptop' | awk '{print $2}')
flo=$("$SP/ld.bin" < "$SP/wf.g2.rzir" 2>/dev/null | "$CC/opt/rzir-opt.bin" 2>/dev/null | "$CC/ra/rzir-ra.bin" 2>/dev/null | grep -m1 '^framelo' | awk '{print $2}')
mv=$(grep -o '^  v[0-9]*' "$SP/wf.g2.rzir" | tr -d ' v' | sort -n | tail -1)
got=$(od -An -tu1 -j107 -N3 "$SP/wf.gen2" | awk '{print $1 + $2*256 + $3*65536}')
eval "$(python3 - "$ht" "$mv" "$got" "${flo:-0}" <<'PYEOF'
import sys
ht=int(sys.argv[1] or 0); mv=int(sys.argv[2] or 0); got=int(sys.argv[3]); flo=int(sys.argv[4] or 0)
U=16777216
frame=((mv+1)*8+4095)//4096*4096
if frame<131072: frame=131072
if flo>0:
    # B1 FRAME-BELOW: fbase is pinned at framelo; p_memsz spans [0, arena_top]+256MB (cf's mszfb).
    fbase=flo
    want=(ht+268435456)//U
else:
    fbase=((ht+U-1)//U*U + U) if ht>0 else 134217728
    want=fbase//U + (frame//4096)*7//4
if want<64: want=64
print(f'WANT={want}; GOT={got}; FRAME={frame}; FBASE={fbase}')
PYEOF
)"
if [ "$WANT" -gt 4095 ]; then
  echo "  RED  HEADROOM: frame budget CLAMPED -- cf wanted $WANT units, the encoding caps at 4095 (p_memsz $GOT)"
  echo "       FRAME_SZ=$FRAME fbase=$FBASE ; see locales/emitted-address-space (C-EAS-BUDGET-SILENT)"; fail=1
elif [ "$WANT" != "$GOT" ]; then
  echo "  RED  HEADROOM: p_memsz byte is $GOT, recomputed budget is $WANT -- cf and the gate disagree"; fail=1
else
  echo "  ok   HEADROOM: p_memsz $GOT units ($((GOT*16))MB) of 4095 -- $((4095-GOT)) units ($(((4095-GOT)*16/1024))GB) to the encoding ceiling"
fi

# --- the whole-file crate differential ---------------------------------------
ok=0; crash=0; diff_n=0; bad=""
for fx in "$S1"/fixtures/*.rs; do
  [ -e "$fx" ] || continue
  n="$(basename "$fx" .rs)"
  "$SP/wf.gen1" < "$fx" > "$SP/wf.a" 2>/dev/null
  "$SP/wf.gen2" < "$fx" > "$SP/wf.b" 2>/dev/null; rc=$?
  if cmp -s "$SP/wf.a" "$SP/wf.b"; then ok=$((ok+1))
  elif [ "$rc" -ge 128 ]; then crash=$((crash+1)); bad="$bad $n(SEGV)"
  else diff_n=$((diff_n+1)); bad="$bad $n(diff)"; fi
done
tot=$((ok+crash+diff_n))
if [ "$crash" = 0 ] && [ "$diff_n" = 0 ]; then
  echo "  ok   WHOLE-FILE: gen2 == gen1 on $ok/$tot fixtures"
else
  echo "  RED  WHOLE-FILE: $ok/$tot ok, $crash crash, $diff_n diff --$bad"; fail=1
fi

echo "== S9 whole-file self-compilation differential: $([ "$fail" = 0 ] && echo GREEN || echo RED) =="
[ "$fail" = 0 ]
