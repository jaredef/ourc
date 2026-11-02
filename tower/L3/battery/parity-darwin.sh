#!/usr/bin/env bash
# Darwin-lane parity gate. For each *.rs given (default: this battery): compile+run with
#   (a) rzrs --target=darwin -> rzir -> rzir-link-darwin | rzir-opt-darwin | rzir-ra-darwin
#       -> cf-darwin -> v3 wrapper => signed native arm64 Mach-O (rzcc's all-native pipeline)
#   HISTORY: this gate opened on the python front because the then-committed
#   rzir-opt-darwin.bin (the O2 seam build) hoisted `loop; vN=lt mut,mut; brz` conditions out
#   of the loop -- an O2 LICM/LCSE operand-mutability bug shared with the oracle, fixed in both
#   (rzir-opt.py + rzir-opt-darwin.c, 2026-07-15) and the bin re-derived; native front restored.
#   (The cf-darwin i64-compare/div/mod/mul gap noted at this gate's introduction is DISCHARGED:
#   W-i64cmp x-form twins landed in cf-darwin-pic.symbolic.m0; both batteries 90/90.)
#   (b) host rustc -O
# and diff stdout+exit (rho*=1). *.skip reported never silent; *.reject = BOTH must reject.
# cf/wrapper come from the rzcc toolchain cache (ROOT/.rzcc-toolchain, built from the symbolic
# source of record on first rzcc run -- NOT the frozen r6 rung, whose cf predates the tcc-darwin
# op alphabet and truncates SILENTLY; see 2026-07-13 session).  Requires W/build (sh run-v3.sh).
#   RZRS=<rzrs binary> bash battery/parity-darwin.sh [tests...]
set -u
HERE="$(cd "$(dirname "$0")" && pwd)"
A="$HERE/../.."
ROOT="$A/../.."
D="$A/2026-07-09-darwin-chain-self-host"
TC="$ROOT/.rzcc-toolchain"
[ -x "$TC/cf.macho" ] || { echo "run rzcc once to build $TC" >&2; exit 2; }
RZRS="${RZRS:?set RZRS to the rzrs binary}"
OUT="${OUT:-/tmp/rzrs-darwin-build}"; mkdir -p "$OUT"
[ $# -gt 0 ] && tests=("$@") || tests=("$HERE"/*.rs)
pass=0; fail=0; skip=0; rej=0; total=0
for t in "${tests[@]}"; do
  [ -e "$t" ] || continue
  name="$(basename "$t" .rs)"; total=$((total+1))
  if [ -e "${t%.rs}.skip" ]; then echo "  SKIP $name ($(cat "${t%.rs}.skip"))"; skip=$((skip+1)); continue; fi
  if [ -e "${t%.rs}.reject" ]; then
    "$RZRS" --target=darwin < "$t" > /dev/null 2>&1; a=$?
    rustc -O --edition 2021 -o "$OUT/$name.rustc" "$t" > /dev/null 2>&1; b=$?
    if [ $a -ne 0 ] && [ $b -ne 0 ]; then echo "  PASS $name (both reject)"; pass=$((pass+1)); rej=$((rej+1));
    else echo "  FAIL $name (reject: rzrs=$a rustc=$b)"; fail=$((fail+1)); fi
    continue
  fi
  "$RZRS" --target=darwin < "$t" > "$OUT/$name.rzir" 2>/dev/null || { echo "  FAIL $name (rzrs)"; fail=$((fail+1)); continue; }
  cat "$OUT/$name.rzir" | "$D/linker-inversion/rzir-link-darwin.bin" | "$D/rzir-opt-darwin/rzir-opt-darwin.bin" \
    | "$D/rzir-ra-darwin/rzir-ra-darwin.bin" > "$OUT/$name.postra" || { echo "  FAIL $name (front)"; fail=$((fail+1)); continue; }
  "$TC/cf.macho" < "$OUT/$name.postra" > "$OUT/$name.body" || { echo "  FAIL $name (cf)"; fail=$((fail+1)); continue; }
  rm -f "$OUT/$name.macho"   # T18-vnode: macOS caches signatures per vnode
  cat "$TC/tpl-v3.bin" "$OUT/$name.body" | "$TC/we-v3.macho" > "$OUT/$name.macho"; chmod +x "$OUT/$name.macho"
  dout="$(perl -e 'alarm 20; exec @ARGV' "$OUT/$name.macho" 2>/dev/null)"; drc=$?   # alarm: a miscompiled loop must FAIL, not hang the gate
  rustc -O --edition 2021 -o "$OUT/$name.rustc" "$t" 2>/dev/null || { echo "  FAIL $name (rustc rejects, rzrs accepted)"; fail=$((fail+1)); continue; }
  rout="$("$OUT/$name.rustc" 2>/dev/null)"; rrc=$?
  if [ "$dout" = "$rout" ] && [ $drc -eq $rrc ]; then pass=$((pass+1));
  else echo "  FAIL $name (darwin rc=$drc out='${dout:0:40}' | rustc rc=$rrc out='${rout:0:40}')"; fail=$((fail+1)); fi
done
echo "== $pass/$((total-skip)) pass ($rej reject-pairs), $skip skipped, $fail fail"
