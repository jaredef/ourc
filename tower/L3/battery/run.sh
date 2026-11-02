#!/usr/bin/env bash
# rust-to-rzir battery -- the arc's measuring stick. For each battery/*.rs: compile+run with
#   (a) the DERIVED toolchain: rzrs -> rzir -> rzir-ld|rzir-opt|rzir-ra -> cf -> ELF  (host-free but ld/rustc-free entirely; cf writes the ELF directly)
#   (b) host rustc
# and diff stdout+exit (rho*=1). A `*.rs` with a sibling `*.skip` is reported SKIP, never silent.
#   RZRS=<rzrs binary> SP=<scratchpad with cf.bin> bash battery/run.sh
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
CC="$HERE/../../L2"
SP="${SP:?set SP (scratch dir; may be empty -- sp_seed mints the roots)}"
# cold start + capacity: see apparatus/lib/scratch.sh. These batteries lower whatever the frontend under
# test emits, so they route through $(sp_cf) rather than bare cf.bin -- M0 cf overflows SILENTLY into a
# 259-byte stub, which reads downstream as "the compiler miscompiled it" rather than "the image was too big".
. "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/../../lib/scratch.sh"
sp_seed || exit 1
RZRS="${RZRS:?set RZRS to the rzrs binary}"
RUN="${RUN:-cat}"                                  # optional wrapper (e.g. safe-run.sh) for a derived-tcc-built rzrs
OUT="${OUT:-/tmp/rzrs-build}"; mkdir -p "$OUT"

chain() {  # stdin=rzir -> $1=elf
  "$CC/link/rzir-ld.bin" | "$CC/opt/rzir-opt.bin" | "$CC/ra/rzir-ra.bin" | "$(sp_cf)" > "$1"
  chmod +x "$1"
}

pass=0; fail=0; skip=0; total=0
for t in "$HERE"/*.rs; do
  [ -e "$t" ] || continue
  name="$(basename "$t" .rs)"; total=$((total+1))
  if [ -e "${t%.rs}.skip" ]; then echo "  SKIP $name  ($(cat "${t%.rs}.skip"))"; skip=$((skip+1)); continue; fi
  if [ -e "${t%.rs}.reject" ]; then                         # ownership/borrow error: BOTH compilers must reject
    $RZRS < "$t" >/dev/null 2>"$OUT/$name.err"; rzrc=$?
    rustc -O "$t" -o "$OUT/$name.rustc" 2>/dev/null; rurc=$?
    if [ "$rzrc" != 0 ] && [ "$rurc" != 0 ]; then echo "  PASS $name  (both reject: $(head -1 "$OUT/$name.err" | sed 's/rzrs: //'))"; pass=$((pass+1))
    else echo "  FAIL $name  (rzrs rc=$rzrc, rustc rc=$rurc -- expected both to reject)"; fail=$((fail+1)); fi
    continue
  fi
  # (a) derived toolchain
  if ! $RZRS < "$t" > "$OUT/$name.rzir" 2>"$OUT/$name.err"; then
    echo "  FAIL $name  (rzrs: $(head -1 "$OUT/$name.err"))"; fail=$((fail+1)); continue
  fi
  chain "$OUT/$name.elf" < "$OUT/$name.rzir" 2>/dev/null
  dout="$("$OUT/$name.elf" 2>/dev/null)"; drc=$?
  # (b) rustc
  rustc -O "$t" -o "$OUT/$name.rustc" 2>/dev/null
  hout="$("$OUT/$name.rustc" 2>/dev/null)"; hrc=$?
  if [ "$dout" = "$hout" ] && [ "$drc" = "$hrc" ]; then
    echo "  PASS $name  (exit $drc)"; pass=$((pass+1))
  else
    echo "  FAIL $name  derived(rc=$drc)='$dout'  rustc(rc=$hrc)='$hout'"; fail=$((fail+1))
  fi
done
graded=$((total-skip)); pct=0; [ "$graded" -gt 0 ] && pct=$((100*pass/graded))
echo "== rust-to-rzir battery: $pass/$graded rho*=1  (${pct}%)  |  $skip skipped, $fail failed =="
[ "$fail" = 0 ]
