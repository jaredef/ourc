#!/usr/bin/env bash
# census-gate.sh -- the T0 REGRESSION gate, wired into `build-tower --gates` (F2: baseline-committed,
# regression-detecting, able to FAIL). Runs the crate census (PARSE outcome, crate-census.sh) with the
# tower-built SEM0 rung and FAILS if any crate regressed below baseline.txt (T0 -> non-T0) or a new CRASH
# appeared.
#
#   F1  measures the OUTCOME (every module parses), never a proxy -- see crate-census.sh probe().
#   F2  baseline is committed (baseline.txt); this gate returns non-zero on any regression.
#   F3  names its RUNG (SEM0 = $SP/wf.gen1, size-asserted) and its ORACLE (none -- T0 needs none).
#   F4  crate-census.sh asserts its own retirement (retires at T3, not here).
#
# ORACLE-class: the pinned rustc source is MEASUREMENT-ONLY and never enters the produced chain. If it is not
# cached this gate SKIPS cleanly (exit 0) -- it does NOT fetch inside the gate (the tower gate stays host-free
# where the oracle source is absent, e.g. CI). It is the heaviest gate (~10 min, 1847 files) and only runs
# where the source is already cached.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
CENSUS="$HERE/crate-census.sh"; BASE="$HERE/baseline.txt"; BOARDC="$HERE/board.txt"
SP="${SP:?set SP}"

# --- RUNG (F3): the tower-built SEM0, size-asserted (a stub exits 0 on everything) -----------------------
FE="$SP/SEM0.rzrs"; [ -x "$FE" ] || FE="$SP/wf.gen1"
[ -x "$FE" ] || { echo "  ok   census-gate SKIP (no SEM0 rung in \$SP yet)"; exit 0; }
sz=$(stat -c%s "$FE"); [ "$sz" -ge 1000000 ] || { echo "  FAIL census-gate: rung $FE is $sz B (stub)"; exit 1; }

# --- ORACLE source (measurement-only): skip if absent ---------------------------------------------------
SRC="${SRC:-}"   # first cached copy that exists (a box whose /tmp is tmpfs keeps it under ~/Developer/rustc-src)
[ -n "$SRC" ] || for s in /tmp/rustc-src/rust-1.95.0 "$HOME/Developer/rustc-src/rustc-1.95.0-src"; do [ -d "$s/compiler" ] && { SRC="$s"; break; }; done
SRC="${SRC:-/tmp/rustc-src/rust-1.95.0}"
[ -d "$SRC/compiler" ] || { echo "  ok   census-gate SKIP (oracle source not cached at $SRC)"; exit 0; }
[ -f "$BASE" ] || { echo "  FAIL census-gate: no baseline.txt"; exit 1; }

# --- run the census into a TEMP board (never dirty the committed board.txt) -----------------------------
W="$(mktemp -d)"; trap 'rm -rf "$W"' EXIT
SP="$SP" SRC="$SRC" BOARD="$W/board.txt" bash "$CENSUS" > "$W/run.log" 2>&1 || {
  echo "  FAIL census-gate: crate-census.sh errored"; tail -3 "$W/run.log"; exit 1; }
[ -f "$W/board.txt" ] || { echo "  FAIL census-gate: census produced no board"; exit 1; }

# --- regression check: a crate T0 in baseline but not in the new board, or any CRASH --------------------
join -a1 -a2 -e MISSING -o '0,1.2,2.2' <(sort "$BASE") <(sort "$W/board.txt") > "$W/j"
reg=$(awk '$2=="T0" && $3!="T0" {print $1"("$2"->"$3")"}' "$W/j")
crash=$(awk '$3=="CRASH" {print $1}' "$W/j")
if [ -n "$reg" ] || [ -n "$crash" ]; then
  echo "  FAIL census-gate: T0 regressions [${reg:-none}] new/other CRASH [${crash:-none}]"; exit 1
fi
t0=$(awk '$2=="T0"' "$W/board.txt" | wc -l); tot=$(wc -l < "$W/board.txt")
echo "  ok   crate census (T0 regression): $t0/$tot T0, 0 CRASH, no regression vs baseline"
exit 0
