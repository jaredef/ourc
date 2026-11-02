#!/usr/bin/env bash
# t3asm-board.sh -- the T3-ASM sub-tier board: does each LOWERED crate's //@S9 rzir ASSEMBLE via the backend
# spine (ld|opt|ra|cf) to a valid ELF? (t3-run-pipeline-oracle.md sub-tier 2.) Verdicts:
#   ASSEMBLED   valid ELF (7f454c46 magic) >= 5000 bytes from the whole-spliced-crate rzir
#   ASM-FAIL    LOWERS but the backend produces a stub / empty / non-ELF (a cf/backend capacity or codegen meet)
#   NOT-LOWERED S9 did not emit clean rzir (a T3-LOWER meet, tracked by t3run-board.sh; not an ASM verdict)
#   NOLIB       no lib.rs (scope-out)
# NECESSARY, not sufficient: an ELF without a synthesized entry/harness does not RUN (that is T3-RUN, ⛔ vendoring).
# F1: verdict from a RUN of the backend, not a pattern count. F3: RUNG = $SP/wf.gen1 (size-asserted) + the
# produced backend spine ($SP/{ld,opt,ra}.bin + $SP/cf.hoist / cf.bin); ORACLE = none. H-SPLICE whole-crate feed.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
SRC="${SRC:-/tmp/rustc-src/rust-1.95.0}"
SP="${SP:?set SP}"; FE="$SP/wf.gen1"; [ -x "$FE" ] || FE="$SP/SEM0.rzrs"
CC="${TOWER_L2:-$HERE/../L2}"
CFH="$SP/cf.hoist"; [ -x "$CFH" ] || CFH="$SP/cf.bin"; [ -x "$CFH" ] || CFH="$HERE/../L5/cf.hoist.seed"
LD="$SP/ld.bin"; [ -x "$LD" ] || LD="$CC/link/rzir-ld.bin"
OPT="$SP/opt.bin"; [ -x "$OPT" ] || OPT="$CC/opt/rzir-opt.bin"
RA="$SP/ra.bin"; [ -x "$RA" ] || RA="$CC/ra/rzir-ra.bin"
CRATES="${CRATES:-$(ls -d "$SRC"/compiler/*/ 2>/dev/null | xargs -n1 basename | tr "\n" " ")}"
BOARD="${BOARD:-$HERE/t3asm-board.txt}"
[ -x "$FE" ] || { echo "  ok   t3asm-board SKIP (no SEM0 rung)"; exit 0; }
sz=$(stat -c%s "$FE"); [ "$sz" -ge 1000000 ] || { echo "  FAIL t3asm: rung is $sz B (stub)"; exit 1; }
W="$(mktemp -d)"; trap 'rm -rf "$W"' EXIT
: > "$BOARD"
asm(){ "$LD" | "$OPT" | "$RA" | "$CFH"; }
for cr in $CRATES; do
  D="$SRC/compiler/$cr/src"; [ -f "$D/lib.rs" ] || { echo "$cr NOLIB" >>"$BOARD"; continue; }
  python3 "$HERE/splice-crate.py" "$D/lib.rs" > "$W/s.rs" 2>/dev/null
  { echo '//@S9'; cat "$W/s.rs"; } | timeout 120 "$FE" > "$W/rz" 2>/dev/null; rc=$?
  if [ "$rc" -ne 0 ] || grep -q UNLOWERED "$W/rz" || [ ! -s "$W/rz" ]; then echo "$cr NOT-LOWERED" >>"$BOARD"; continue; fi
  timeout 180 bash -c 'asm(){ "'"$LD"'" | "'"$OPT"'" | "'"$RA"'" | "'"$CFH"'"; }; asm' < "$W/rz" > "$W/elf" 2>/dev/null
  esz=$(wc -c <"$W/elf" 2>/dev/null || echo 0); mg=$(head -c4 "$W/elf" 2>/dev/null | od -An -tx1 2>/dev/null | tr -d ' \n')
  if [ "$mg" = "7f454c46" ] && [ "$esz" -ge 5000 ]; then echo "$cr ASSEMBLED" >>"$BOARD"; else echo "$cr ASM-FAIL" >>"$BOARD"; fi
done
sort -o "$BOARD" "$BOARD"
BASE="${BASE:-$HERE/t3asm-baseline.txt}"
if [ -f "$BASE" ]; then
  reg=$(join <(sort "$BASE") <(sort "$BOARD") | awk '$2=="ASSEMBLED" && $3!="ASSEMBLED" && $3!="NOLIB" {print $1"("$2"->"$3")"}')
  [ -n "$reg" ] && { echo "  FAIL t3asm-board: regressions [$reg]"; exit 1; }
fi
a=$(awk '$2=="ASSEMBLED"' "$BOARD"|wc -l); f=$(awk '$2=="ASM-FAIL"' "$BOARD"|wc -l); t=$(wc -l <"$BOARD")
echo "  ok   T3-ASM board: $a ASSEMBLED, $f ASM-FAIL, of $t crates (ASM is necessary-not-sufficient; RUN future)"
exit 0
