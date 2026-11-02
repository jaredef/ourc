#!/usr/bin/env bash
# llvm-parity.sh -- the rzir -> LLVM IR bridge board (Route B, apparatus/arcs/2026-09-27-rzir-llvm-bridge).
# Builds the SELF-HOSTED translator (tower/backend-llvm/rzir2ll.rs, compiled by SEM0 via the spine), then runs the
# parity battery through:  .rs -> SEM0.FROZEN(//@S9) -> rzir -> rzir2ll -> llc -> gcc -> native ELF, comparing
# stdout+exit to rustc -O. Zero-regression vs baseline.txt. SKIPS cleanly (exit 0) when llc/gcc are absent, so CI
# stays host-free -- this is a capability board, not a hex-rooted trust gate (LLVM + the system linker are external).
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; A="$HERE/.."; SP="${SP:?set SP}"
. "$A/lib/scratch.sh"; sp_seed || exit 1
FE="$SP/sem0.frozen"
[ -x "$FE" ] || { echo "llvm-parity: SKIP (no $SP/sem0.frozen -- run sem0-stage first)"; exit 0; }
# locate llc + a C driver; skip cleanly if unavailable
LLC="$(command -v llc 2>/dev/null || ls ~/.rustup/toolchains/*/lib/rustlib/*/bin/llc 2>/dev/null | head -1)"
CCB="$(command -v cc 2>/dev/null || command -v gcc 2>/dev/null)"
if [ -z "$LLC" ] || [ -z "$CCB" ]; then echo "llvm-parity: SKIP (llc/cc not found; capability board is host-gated)"; exit 0; fi
CF="$(sp_cf)"
chain(){ "$SP/ld.bin" | "$SP/opt.bin" | "$SP/ra.bin" | "$CF" > "$1" 2>/dev/null; chmod +x "$1" 2>/dev/null; }
# 1. build the self-hosted translator
{ echo '//@S9'; cat "$HERE/rzir2ll.rs"; } | timeout 120 "$FE" > "$SP/r2l.rzir" 2>/dev/null || { echo "llvm-parity: FAIL (SEM0 could not lower rzir2ll.rs)"; exit 1; }
if grep -q UNLOWERED "$SP/r2l.rzir"; then echo "llvm-parity: FAIL (rzir2ll.rs has UNLOWERED constructs)"; exit 1; fi
chain "$SP/r2l.elf" < "$SP/r2l.rzir"
[ -x "$SP/r2l.elf" ] || { echo "llvm-parity: FAIL (translator did not link)"; exit 1; }
# 2. run the battery through the bridge vs rustc -O
pass=0; total=0; fails=""
for t in "$A"/L3/battery/*.rs "$A"/rust-closure/battery/*.rs; do
  [ -e "$t" ] || continue
  case "$t" in *reject*|*skip*) continue;; esac
  total=$((total+1)); name="$(basename "$t")"
  { echo '//@S9'; cat "$t"; } | timeout 20 "$FE" > "$SP/lp.rzir" 2>/dev/null || { fails="$fails $name(frozen)"; continue; }
  timeout 20 "$SP/r2l.elf" < "$SP/lp.rzir" > "$SP/lp.ll" 2>/dev/null || { fails="$fails $name(xlate)"; continue; }
  "$LLC" -O2 -filetype=obj "$SP/lp.ll" -o "$SP/lp.o" 2>/dev/null || { fails="$fails $name(llc)"; continue; }
  "$CCB" -no-pie "$SP/lp.o" -o "$SP/lp.elf" 2>/dev/null || { fails="$fails $name(link)"; continue; }
  dout="$(timeout 20 "$SP/lp.elf" 2>/dev/null)"; drc=$?
  rustc -O "$t" -o "$SP/lp.host" 2>/dev/null; hout="$("$SP/lp.host" 2>/dev/null)"; hrc=$?
  if [ "$dout" = "$hout" ] && [ "$drc" = "$hrc" ]; then pass=$((pass+1)); else fails="$fails $name"; fi
done
echo "== rzir->LLVM bridge parity: $pass/$total match rustc =="
BL="$HERE/baseline.txt"
if [ "${LLVM_PARITY_BASELINE:-0}" = 1 ]; then echo "$pass" > "$BL"; echo "llvm-parity: baseline written ($pass/$total)"; exit 0; fi
base=0; [ -f "$BL" ] && base="$(cat "$BL")"
if [ -n "${1:-}" ] || [ "$pass" -lt "$base" ]; then echo "FAILS:"; echo "$fails" | tr ' ' '\n' | grep -v '^$' | head -60; fi
[ "$pass" -ge "$base" ]
