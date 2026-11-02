#!/usr/bin/env bash
# semc-gate.sh -- semc (scripts/semc.sh = SEM0 -> rzir -> r2l -> llc -> cc) built from the TOWER's own pieces,
# scored over the r2-corpus against rustc -O, with a zero-regression baseline.
#
# Why this gate exists: semc was only ever run by hand with a rustc-built SEM0 (sem0.head) from a scratch dir,
# so nothing showed that the compiler semc ships is the one the tower derives from the hand-hex. This runs semc
# with SEM0 = the tower's raw frozen ($SP/sem0.llvm.frozen, else $SP/sem0.frozen) and R2L = the tower-minted
# $SP/r2l.elf, over every r2-corpus probe, and compares stdout + exit code with the same probe built by rustc -O.
#
# Per-probe status:
#   MATCH          semc binary == rustc binary (stdout + exit)
#   DIFF           both built, behaviour differs -- incl. probes where rustc's binary panics (overflow, OOB) and
#                  semc's does not: SEM0 does no overflow/bounds checking (the UNSOUND-REJECT frontier)
#   SEM0-REJECT    semc failed in the SEM0 front-end
#   BACKEND-FAIL   semc failed in r2l / llc / link
#   ORACLE-REJECT  rustc rejects the probe (an unsound-reject probe); semc's result is reported, not gated
#   TIMEOUT        the semc-built binary did not finish in 10s
# The gate FAILS on any probe whose baseline is MATCH and is now anything else, and REPORTS improvements.
#
#   SP=<tower scratch> bash tower/semc/semc-gate.sh
#   REBASELINE=1 SP=... bash tower/semc/semc-gate.sh    # rewrite baseline.txt (only when improvements are intended)
#
# Host-gated (like the bridge board and ourc): SKIPS cleanly when llc / cc / rustc are absent. rustc is the
# oracle only -- it never builds anything semc runs.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; A="$HERE/.."; ROOT="$A/.."
SP="${SP:?set SP (tower scratch with sem0.frozen + r2l.elf)}"
BASE="$HERE/baseline.txt"; PROBES="$A/corpus/r2-corpus/probes"
FE="$SP/sem0.llvm.frozen"; [ -x "$FE" ] || FE="$SP/sem0.frozen"
[ -x "$FE" ] || { echo "  semc-gate: FAIL (no $SP/sem0{.llvm,}.frozen -- run sem0-stage first)"; exit 1; }
LLC="$(command -v llc 2>/dev/null || ls ~/.rustup/toolchains/*/lib/rustlib/*/bin/llc 2>/dev/null | head -1)"
for t in cc rustc; do command -v "$t" >/dev/null || { echo "  semc-gate: SKIP ($t not found; the semc path is host-gated)"; exit 0; }; done
[ -n "$LLC" ] || { echo "  semc-gate: SKIP (llc not found; the semc path is host-gated)"; exit 0; }
# the translator: reuse the one llvm-parity/ourc minted this run, else mint it the same way (SEM0 compiles rzir2ll.rs)
if [ ! -x "$SP/r2l.elf" ]; then
  . "$A/lib/scratch.sh"; sp_seed || exit 1
  { echo '//@S9'; cat "$A/backend-llvm/rzir2ll.rs"; } | timeout 120 "$FE" > "$SP/r2l.rzir" 2>/dev/null \
    || { echo "  semc-gate: FAIL (SEM0 could not lower rzir2ll.rs)"; exit 1; }
  "$SP/ld.bin" < "$SP/r2l.rzir" | "$SP/opt.bin" | "$SP/ra.bin" | "$(sp_cf)" > "$SP/r2l.elf" 2>/dev/null; chmod +x "$SP/r2l.elf"
  [ -s "$SP/r2l.elf" ] || { echo "  semc-gate: FAIL (translator did not link)"; exit 1; }
fi

W="$SP/semc-gate"; rm -rf "$W"; mkdir -p "$W/p"
export FE LLC W SEMC="$ROOT/scripts/semc.sh" R2L="$SP/r2l.elf"
probe() { # probe FILE -> prints "name STATUS"
  local f="$1" n; n="$(basename "$f" .rs)"; local d="$W/p/$n"; mkdir -p "$d"
  local ores=OK
  rustc -O --edition 2021 "$f" -o "$d/r" >/dev/null 2>&1 || ores=REJECT
  if ! SEM0="$FE" R2L="$R2L" LLC="$LLC" timeout 120 bash "$SEMC" "$f" -o "$d/s" > "$d/semc.log" 2>&1; then
    if [ "$ores" = REJECT ]; then echo "$n ORACLE-REJECT"; return; fi
    if grep -q 'SEM0 front-end' "$d/semc.log"; then echo "$n SEM0-REJECT"; else echo "$n BACKEND-FAIL"; fi; return
  fi
  [ "$ores" = REJECT ] && { echo "$n ORACLE-REJECT"; return; }
  local want got
  want="$(cd "$d" && timeout 10 ./r 2>/dev/null; echo "exit $?")"
  got="$(cd "$d" && timeout 10 ./s 2>/dev/null; echo "exit $?")"
  case "$got" in *"exit 124") echo "$n TIMEOUT"; return;; esac
  [ "$want" = "$got" ] && echo "$n MATCH" || echo "$n DIFF"
  rm -rf "$d"
}
export -f probe
ls "$PROBES"/*.rs | xargs -P "${JOBS:-$(nproc)}" -I{} bash -c 'probe "$@"' _ {} | sort > "$W/board.txt"

total=$(wc -l < "$W/board.txt")
summary() { awk '{c[$2]++} END {for (k in c) printf "%s=%d ", k, c[k]}' "$1"; }
echo "  semc board ($total probes, SEM0=$(basename "$FE")): $(summary "$W/board.txt")"
if [ "${REBASELINE:-0}" = 1 ] || [ ! -f "$BASE" ]; then
  cp "$W/board.txt" "$BASE"; echo "  semc-gate: baseline written ($BASE)"; exit 0
fi
reg=$(join "$BASE" "$W/board.txt" | awk '$2=="MATCH" && $3!="MATCH" {print "    " $1 ": MATCH -> " $3}')
imp=$(join "$BASE" "$W/board.txt" | awk '$2!="MATCH" && $3=="MATCH" {print "    " $1 ": " $2 " -> MATCH"}')
[ -n "$imp" ] && { echo "  improvements (REBASELINE=1 to record):"; echo "$imp"; }
if [ -n "$reg" ]; then echo "  semc-gate: FAIL -- regressions from MATCH:"; echo "$reg"; exit 1; fi
echo "  semc-gate: ok -- $(grep -c ' MATCH$' "$W/board.txt")/$total MATCH, zero regression vs baseline"
