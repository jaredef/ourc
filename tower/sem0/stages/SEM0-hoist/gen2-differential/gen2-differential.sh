#!/usr/bin/env bash
# SEM0 gen2 DIFFERENTIAL -- does the SEM0-BUILT SEM0 agree with the L6-built one on real Rust?
#
# WHY THIS EXISTS. Everything that tests gen2 today tests it on material SEM0 was built against:
#   * `g2 == g3` compares gen2 against itself on ONE file, s1_parser.rs.
#   * sem0-hoist-readiness.sh runs the 8 semantic gates on SEM0's OWN S-tier fixtures.
#   * the 904-probe r2-corpus board is scored with gen1, and was MEASURED identical under gen2.
# None of them run gen2 over large real Rust. Measured 2026-08-26 on a 192-file sample of rustc 1.83.0:
# gen1 and gen2 diverge on 9 of 192 files -- 7 where gen2 SIGSEGVs and gen1 succeeds, 2 with differing output.
# That is the northstar-rung failure one tier in: the board measures the tier a hoist would RETIRE, while the
# artifact a hoist would INSTALL goes unmeasured on anything but its own fixtures.
#
#   SP=<scratch with L6.rzrs + the backend> bash gen2-differential.sh
#
# Corpus = the repo's own large Rust sources + the fixtures/ directory. The repo sources are a CONTROL, not
# coverage: all seven of them agree under gen1 and gen2 (measured, including a 956 KB file), which is itself the
# finding that the tower's own source is not representative of real Rust. The fixtures carry the divergences
# minimized out of real rustc source.
#
# Baseline discipline, as elsewhere in this repo: a divergence listed in baseline.txt is KNOWN and does not fail
# the lane; anything else does.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; A="$(cd "$HERE/../../../.." && pwd)"   # -> tower
SP="${SP:?set SP}"; CC="${CC:-$A/L2}"
P="$A/sem0/stages/S1-lex-parse-ast/s1_parser.rs"
W="$(mktemp -d)"; trap 'rm -rf "$W"' EXIT
back(){ "$CC/link/rzir-ld.bin" 2>/dev/null | "$CC/opt/rzir-opt.bin" 2>/dev/null | "$CC/ra/rzir-ra.bin" 2>/dev/null | "$SP/cf.hoist"; }

echo "== SEM0 gen2 differential =="
[ -x "$SP/L6.rzrs" ] || { echo "  FAIL: no \$SP/L6.rzrs"; exit 1; }
timeout 300 "$SP/L6.rzrs" < "$P" 2>/dev/null | back > "$W/gen1"; chmod +x "$W/gen1"
[ "$(stat -c%s "$W/gen1")" -ge 100000 ] || { echo "  FAIL: gen1 is a stub"; exit 1; }
{ echo '//@S9'; cat "$P"; } > "$W/s9.rs"
timeout 900 "$W/gen1" < "$W/s9.rs" > "$W/g2.rzir" 2>/dev/null
[ -s "$W/g2.rzir" ] || { echo "  FAIL: gen1 self-emit is empty"; exit 1; }
back < "$W/g2.rzir" > "$W/gen2"; chmod +x "$W/gen2"
[ "$(stat -c%s "$W/gen2")" -ge 100000 ] || { echo "  FAIL: gen2 is a stub"; exit 1; }
echo "  gen1 $(stat -c%s "$W/gen1") B   gen2 $(stat -c%s "$W/gen2") B"

CORPUS=$(ls "$HERE"/fixtures/*.rs 2>/dev/null)
for f in "$A/L6/L6.rzrs.rs" "$A/L5/L5.rzrs.rs" \
         "$A/L4/L4.rzrs.rs" "$A/backend-cf/cf.rs" \
         "$A/backend-toolchain/rzir-ld.rs" "$A/backend-toolchain/rzir-opt.rs" \
         "$A/backend-toolchain/rzir-ra.rs"; do
  [ -f "$f" ] && CORPUS="$CORPUS $f"
done

: > "$W/now.txt"; n=0; same=0
for f in $CORPUS; do
  n=$((n+1)); bn=$(basename "$f")
  timeout 300 "$W/gen1" < "$f" > "$W/a.out" 2>/dev/null; ra=$?
  timeout 300 "$W/gen2" < "$f" > "$W/b.out" 2>/dev/null; rb=$?
  if [ "$ra" = "$rb" ] && cmp -s "$W/a.out" "$W/b.out"; then same=$((same+1))
  else echo "$bn gen1=$ra gen2=$rb" >> "$W/now.txt"; fi
done
sort -o "$W/now.txt" "$W/now.txt"
BASE="$HERE/baseline.txt"; [ -f "$BASE" ] || : > "$BASE"
grep -vE '^\s*#|^\s*$' "$BASE" | sort > "$W/base.txt"
NEW=$(comm -13 "$W/base.txt" "$W/now.txt")
FIXED=$(comm -23 "$W/base.txt" "$W/now.txt")
echo "  corpus $n files: $same agree, $(wc -l < "$W/now.txt") diverge ($(wc -l < "$W/base.txt") known-open)"
[ -n "$FIXED" ] && { echo "  GREEN -- these no longer diverge, rebaseline:"; echo "$FIXED" | sed 's/^/      /'; }
if [ -n "$NEW" ]; then
  echo "  RED  NEW DIVERGENCE -- gen2 disagrees with gen1 where it did not before:"
  echo "$NEW" | sed 's/^/      /'
  echo "  A hoist installs gen2. A divergence here is a defect in the artifact that would ship."
  exit 1
fi
echo "  ok  no new gen1/gen2 divergence"
