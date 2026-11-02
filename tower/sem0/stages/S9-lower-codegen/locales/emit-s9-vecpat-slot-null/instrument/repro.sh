#!/usr/bin/env bash
# Rebuild the MINUS/PLUS pair via the SEM0 self-host chain and report the green->crash flip + crash rip.
# The ONLY source delta is one UNUSED free fn iterating a destructured Vec<Pat> (vecpat_probe) — its mere
# COMPILATION flips the whole binary from exit-0 to a SIGSEGV null-store, at full binary size (not truncated):
# the emit-scale value-numbering/slot-assignment collision (C-SE class). CANARY: PLUS exits 0 == MINUS.
set -uo pipefail
SP="${SP:?set SP}"; CC="${CC:?set CC}"; HERE="$(cd "$(dirname "$0")" && pwd)"; FX="$HERE/../fixtures"
back(){ "$CC/link/rzir-ld.bin" 2>/dev/null | "$CC/opt/rzir-opt.bin" 2>/dev/null | "$CC/ra/rzir-ra.bin" 2>/dev/null | "$SP/cf.hoist"; }
for tag in MINUS PLUS; do
  "$SP/sem0.cc" < "$FX/repro_$tag.rs" 2>/dev/null | back > "$SP/repro_$tag.rz" 2>/dev/null; chmod +x "$SP/repro_$tag.rz"
  "$SP/repro_$tag.rz" < /dev/null > /dev/null 2>&1; e=$?
  echo "  repro_$tag: exit=$e size=$(wc -c < "$SP/repro_$tag.rz")"
  [ $tag = PLUS ] && [ $e -ne 0 ] && python3 "$HERE/segcatch.py" "$SP/repro_$tag.rz" | sed 's/^/    /'
done
