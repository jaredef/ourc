#!/usr/bin/env bash
# t3run-board.sh -- the T3 RUN board (chapter opened 2026-09-06, keeper-authorized after the T1' resolve frontier
# closed). T3's telos observable is "the compiled crate BEHAVES == rustc's build" (oracle = the crate's tests),
# the meta-arc endpoint. That is reached through three ascending SUB-TIERS; this instrument measures the first and
# scaffolds the rest so the board is honest about how far execution actually reaches (never overclaiming RUN):
#
#   T3-LOWER  SEM0 //@S9 emits rzir for the whole spliced crate with rc=0, non-empty, and ZERO `UNLOWERED`
#             markers. NECESSARY, not sufficient -- it is emission, not execution.
#   T3-ASM    that rzir assembles via the backend spine (ld|opt|ra|cf) to an ELF.            [future sub-tier]
#   T3-RUN    a driver over the crate's test assertions executes and behaves == rustc's build. [future; ⛔ vendoring]
#
# This rung measures T3-LOWER across all compiler/* crates. F1: from a RUN of the rung, not a pattern count.
# F3: RUNG = $SP/wf.gen1 (size-asserted), ORACLE = none at T3-LOWER (emission is self-contained; the oracle enters
# at T3-RUN). Feeds the SPLICED whole crate (H-SPLICE), same as the T1'/T2 boards.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
SRC="${SRC:-/tmp/rustc-src/rust-1.95.0}"
SP="${SP:?set SP}"; FE="$SP/SEM0.rzrs"; [ -x "$FE" ] || FE="$SP/wf.gen1"
CRATES="${CRATES:-$(ls -d "$SRC"/compiler/*/ 2>/dev/null | xargs -n1 basename | tr "\n" " ")}"
BOARD="${BOARD:-$HERE/t3run-board.txt}"
[ -x "$FE" ] || { echo "  ok   t3-board SKIP (no SEM0 rung)"; exit 0; }
sz=$(stat -c%s "$FE"); [ "$sz" -ge 1000000 ] || { echo "  FAIL t3: rung is $sz B (stub)"; exit 1; }
[ -d "$SRC/compiler" ] || { echo "  ok   t3-board SKIP (crate source not cached)"; exit 0; }
W="$(mktemp -d)"; trap 'rm -rf "$W"' EXIT
: > "$BOARD"
for cr in $CRATES; do
  D="$SRC/compiler/$cr/src"; [ -f "$D/lib.rs" ] || { echo "$cr NOLIB" >> "$BOARD"; continue; }
  python3 "$HERE/splice-crate.py" "$D/lib.rs" > "$W/spliced.rs" 2>/dev/null
  { echo '//@S9'; cat "$W/spliced.rs"; } > "$W/in.rs"
  timeout 180 "$FE" < "$W/in.rs" > "$W/out" 2>/dev/null; rc=$?
  if   [ "$rc" -ne 0 ]; then v=S9-FAIL
  elif grep -q UNLOWERED "$W/out"; then v=UNLOWERED
  elif [ -s "$W/out" ]; then v=LOWERED
  else v=EMPTY; fi
  echo "$cr $v" >> "$BOARD"
done
sort -o "$BOARD" "$BOARD"
# regression check vs committed baseline: a crate that was LOWERED must not drop (NOLIB never fails the gate).
BASE="${BASE:-$HERE/t3run-baseline.txt}"
if [ -f "$BASE" ]; then
  reg=$(join <(sort "$BASE") <(sort "$BOARD") | awk '$2=="LOWERED" && $3!="LOWERED" && $3!="NOLIB" {print $1"("$2"->"$3")"}')
  if [ -n "$reg" ]; then echo "  FAIL t3-board: T3-LOWER regressions [$reg]"; exit 1; fi
fi
l=$(awk '$2=="LOWERED"' "$BOARD" | wc -l); f=$(awk '$2=="S9-FAIL"' "$BOARD" | wc -l); t=$(wc -l < "$BOARD")
echo "  ok   T3-LOWER board: $l LOWERED, $f S9-FAIL, of $t crates (LOWER is necessary-not-sufficient; ASM/RUN future)"
exit 0
