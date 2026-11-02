#!/usr/bin/env bash
# Measure L5.rzrs.rs's parity with rzrs.c: build rzrs.self from L5.rzrs.rs, then run every program in the
# rust-to-rzir (44) + rust-closure (46) batteries through rzrs.self -> chain -> ELF vs rustc -O.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; A="$HERE/.."; CC="${TOWER_L2:-$A/L2}"; SP="${SP:?set SP}"
SELF="$A/L5/L5.rzrs.rs"
# Backend chain = the all-Rust twins ($SP/B.{ld,opt,ra}); populate via build-live-backend.sh first.
# rzrs.self is a frontend-scale image, past M0 cf.bin's ~5.7MB ceiling -- which overflows SILENTLY to a
# 259-byte stub, so every battery program then "miscompiled" and parity read 0/90. Lower via $CFCAP.
. "$A/lib/scratch.sh"
sp_seed || exit 1
chain(){ "$SP/B.ld" | "$SP/B.opt" | "$SP/B.ra" | "$(sp_cf)" > "$1" 2>/dev/null; chmod +x "$1" 2>/dev/null; }
# build rzrs.self via the staged bootstrap: the mainline is tier-1 (natural if-|| the C seed rejects), so it
# is built by tier-0 (minted once from the FROZEN seed by the C seed), not by the C seed directly.
SEED="$A/L4/L4.rzrs.rs"
"$SP/L3.rzrs" < "$SEED" > "$SP/parity.L4.rzrs.rzir" 2>/dev/null || { echo "C seed failed on frozen seed"; exit 1; }
chain "$SP/parity.L4.rzrs" < "$SP/parity.L4.rzrs.rzir"
timeout 90 "$SP/parity.L4.rzrs" < "$SELF" > "$SP/parity.L5.rzrs.rzir" 2>/dev/null || { echo "tier-0 failed on mainline L5.rzrs.rs"; exit 1; }
chain "$SP/parity.L5.rzrs" < "$SP/parity.L5.rzrs.rzir"
sp_require "$SP/parity.L5.rzrs" "parity.L5.rzrs (mainline frontend)" 100000 || exit 1
pass=0; total=0; fails=""
for t in "$A"/L3/battery/*.rs "$A"/rust-closure/battery/*.rs; do
  [ -e "$t" ] || continue
  case "$t" in *reject*|*skip*) continue;; esac
  total=$((total+1)); name="$(basename "$t")"
  if ! timeout 10 "$SP/parity.L5.rzrs" < "$t" > "$SP/p.rzir" 2>/dev/null; then
    # rzrs rejected the program -- a match iff rustc also rejects it (both reject invalid code = correct)
    if rustc -O "$t" -o "$SP/p.host" 2>/dev/null; then fails="$fails $name"; else pass=$((pass+1)); fi
    continue
  fi
  chain "$SP/p.elf" < "$SP/p.rzir"
  [ -x "$SP/p.elf" ] || { fails="$fails $name"; continue; }
  dout="$(timeout 10 "$SP/p.elf" 2>/dev/null)"; drc=$?
  rustc -O "$t" -o "$SP/p.host" 2>/dev/null; hout="$("$SP/p.host" 2>/dev/null)"; hrc=$?
  if [ "$dout" = "$hout" ] && [ "$drc" = "$hrc" ]; then pass=$((pass+1)); else fails="$fails $name"; fi
done
echo "== L5.rzrs.rs parity: $pass/$total match rustc =="
if [ -n "${1:-}" ]; then echo "FAILS:"; echo "$fails" | tr ' ' '\n' | grep -v '^$' | head -60; fi
# the gate's exit status is the RESULT, not whatever the last command happened to return. The old tail was a
# bare `[ -n "${1:-}" ] && {..}`, so an argument-less run exited 1 even at a full 90/90.
[ "$pass" = "$total" ]
