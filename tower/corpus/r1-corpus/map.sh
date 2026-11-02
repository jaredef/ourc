#!/usr/bin/env bash
# R1 corpus differential map: run each probes/*.rs through the derived toolchain AND rustc (oracle), categorize.
#   SP=<scratch with rzrs.self + cf.full> bash map.sh
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; A="$(cd "$HERE/../.." && pwd)"; CC="${TOWER_L2:-$A/L2}"
SP="${SP:?set SP (scratch with rzrs.self + cf.full)}"
# C-CORPUS-ASLR-NONDETERMINISM (see r2-corpus/map.sh for the measurement): a miscompiled probe dereferences a
# wild pointer whose fate depends on the address-space layout, so the SAME binary classifies differently across
# runs. `setarch -R` makes it single-valued. Measured on r2's symbol_interner_hashmap: 40 runs gave 26x139 +
# 14x132 with randomisation on, and 40x132 with it off.
# NOTE, not fixed here: the derived-binary run below is UNBOUNDED. r2-corpus/map.sh records that an unbounded run
# stalled its whole sweep once a compiler started emitting non-terminating code, and added `timeout 5`. This
# harness still has that exposure; left alone because changing it would alter this board's classification
# semantics, which is not what this rung is about.
NORAND=""
if command -v setarch >/dev/null 2>&1; then if setarch -R true >/dev/null 2>&1; then NORAND="setarch -R"; fi; fi
WORK="$(mktemp -d)"
chain(){ "$CC/link/rzir-ld.bin" | "$CC/opt/rzir-opt.bin" | "$CC/ra/rzir-ra.bin" | "$SP/cf.full"; }
for f in "$HERE"/probes/*.rs; do
  n=$(basename "$f" .rs)
  if rustc -O --edition 2021 -o "$WORK/$n.o" "$f" 2>/dev/null; then ostat="ok:$("$WORK/$n.o" 2>/dev/null):$?"; else ostat="REJECT"; fi
  timeout 10 "$SP/rzrs.self" < "$f" > "$WORK/$n.rzir" 2>"$WORK/$n.err"; fc=$?
  if [ "$fc" = 124 ]; then dstat="HANG"
  elif [ "$fc" != 0 ]; then dstat="REJECT:$(head -c60 "$WORK/$n.err")"
  else chain < "$WORK/$n.rzir" > "$WORK/$n.elf" 2>/dev/null; chmod +x "$WORK/$n.elf" 2>/dev/null
    if [ -s "$WORK/$n.elf" ]; then dstat="ok:$($NORAND "$WORK/$n.elf" 2>/dev/null):$?"; else dstat="NOELF"; fi; fi
  if [ "$ostat" = "$dstat" ]; then cat="OK"
  elif [ "$ostat" = "REJECT" ] && echo "$dstat" | grep -q REJECT; then cat="REJECT-OK"
  elif [ "$ostat" = "REJECT" ]; then cat="UNSOUND"
  elif echo "$dstat" | grep -q REJECT; then cat="GAP"
  elif [ "$dstat" = "HANG" ]; then cat="HANG"
  else cat="MISCOMP"; fi
  printf '%-22s %-9s oracle=%-16s derived=%s\n' "$n" "$cat" "$ostat" "$dstat"
done | sort -k2
rm -rf "$WORK"
