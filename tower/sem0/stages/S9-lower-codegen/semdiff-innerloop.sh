#!/bin/bash
# Inner loop for reference-model work: build gen1 from $1, then run the S9 sem witnesses against rustc.
# This is the gate that caught the reverted C-S9-MUTREF-SCALAR fix while a 10-probe hand differential could not.
set -uo pipefail
SRC="$1"; SP=$(cat /tmp/sp49); CC="${TOWER_L2:-$(cd "$(dirname "$0")/../../.." && pwd)/L2}"
WIT="$(cd "$(dirname "$0")" && pwd)/witnesses"
T=$(mktemp -d)
back(){ "$CC/link/rzir-ld.bin" 2>/dev/null | "$CC/opt/rzir-opt.bin" 2>/dev/null | "$CC/ra/rzir-ra.bin" 2>/dev/null | "$SP/cf.hoist"; }
setarch -R timeout 300 "$SP/L6.rzrs" < "$SRC" 2>/dev/null | back > "$T/gen1"; chmod +x "$T/gen1"
sz=$(stat -c%s "$T/gen1"); if [ "$sz" -lt 500000 ]; then echo "GEN1-STUB $sz"; exit 2; fi
fail=0; n=0
for w in "$WIT"/*.rs; do
  b=$(basename "$w" .rs); n=$((n+1))
  exp="$(rustc --edition 2021 "$w" -o "$T/$b.rc" 2>/dev/null && setarch -R "$T/$b.rc" 2>/dev/null; printf 'X%d' "$?")"
  { echo '//@S9'; cat "$w"; } > "$T/$b.in"
  setarch -R timeout 60 "$T/gen1" < "$T/$b.in" 2>/dev/null | back > "$T/$b.bin" 2>/dev/null; chmod +x "$T/$b.bin" 2>/dev/null
  act="$(setarch -R timeout 10 "$T/$b.bin" 2>/dev/null; printf 'X%d' "$?")"
  [ "$exp" = "$act" ] || { echo "  RED $b: want [$exp] got [$act]"; fail=$((fail+1)); }
done
echo "sem witnesses: $((n-fail))/$n ok"; rm -rf "$T"; exit $fail
