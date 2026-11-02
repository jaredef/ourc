#!/usr/bin/env bash
# S9 LC-0 run-and-diff instrument. For each witness (a self-contained fn main):
#   EXPECTED = rustc <file> && ./a.out   (stdout + exit)
#   ACTUAL   = the SEMANTIC compiler lowers <file> via //@S9 -> rzir -> ld|opt|ra|cf -> run
# The gate is the RUN differential (dimension L), not a -Z dump. Until the //@S9 lowering lands, ACTUAL falls back to the
# L6 transpiler baseline (BASELINE=l6) to exercise the harness + backend; flip to BASELINE=sem once emit_s9 lowers a witness.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; S1="$HERE/../S1-lex-parse-ast"
SP="${SP:?set SP}"; CC="${CC:?set CC}"; BASELINE="${BASELINE:-l6}"
P="$S1/s1_parser.rs"
# TARGET codegen = the meta-telos LLVM bridge (cf is self-host only). rz_back reads rzir on stdin, writes an ELF
# to stdout (same shape as the old cf `back`). RZ_R2LCC = the SEM0 binary that lowers rzir2ll.rs for this gate.
RZ_R2LCC="${RZ_R2LCC:-$SP/s1.rz}"; [ -x "$RZ_R2LCC" ] || RZ_R2LCC="$SP/L6.rzrs"
. "$HERE/../../../lib/llvm-backend.sh"
rz_backend_ready || { echo "  SKIP s9-run-diff: $RZ_BACKEND_MSG"; exit 0; }
echo "  [s9-run-diff backend: $RZ_BACKEND_EFFECTIVE]"
back(){ rz_back; }
# Known-debt baseline (LLVM target backend surfaces rzir2ll gaps, e.g. bitwise). Like corpus-gate: a witness that
# was OK and now RED is a REGRESSION (fail); recorded RED debt does not fail. baseline file is per-backend.
BASE="$HERE/run-baseline.$RZ_BACKEND_EFFECTIVE.txt"
LIVE="$(mktemp)"; trap 'rm -f "$LIVE"' EXIT
fail=0
for w in "$HERE"/witnesses/*.rs; do
  [ -e "$w" ] || continue
  n="$(basename "$w" .rs)"
  exp="$(rustc --edition 2021 "$w" -o "$SP/$n.rc" 2>/dev/null && "$SP/$n.rc" 2>/dev/null; printf 'X%d' "$?")"
  if [ "$BASELINE" = "sem" ]; then
    # s1.rz (rz-compiled) truncates PIPE stdin -- feed the //@S9 input via a FILE (like the dump gates do).
    { echo '//@S9'; cat "$w"; } > "$SP/$n.s9in"
    "$SP/s1.rz" < "$SP/$n.s9in" 2>/dev/null | back > "$SP/$n.s9.bin" 2>/dev/null
  else
    "$SP/L6.rzrs" < "$w" 2>/dev/null | back > "$SP/$n.s9.bin" 2>/dev/null
  fi
  chmod +x "$SP/$n.s9.bin" 2>/dev/null
  act="$("$SP/$n.s9.bin" 2>/dev/null; printf 'X%d' "$?")"
  if [ "$exp" = "$act" ]; then echo "  ok   RUN ($n): == rustc [$exp]"; echo "$n OK" >> "$LIVE"
  else echo "  WANT ($n): want [$exp] got [$act]"; echo "$n RED" >> "$LIVE"; fi
done
sort -o "$LIVE" "$LIVE"
nok=$(grep -c ' OK$' "$LIVE" || true); ntot=$(wc -l < "$LIVE")
if [ "${REBASELINE:-0}" = 1 ]; then cp "$LIVE" "$BASE"; echo "  REBASELINED $BASE ($ntot witnesses, $nok OK)"; fi
echo "  board: $nok/$ntot witnesses == rustc  [backend: $RZ_BACKEND_EFFECTIVE]"
if [ -f "$BASE" ]; then
  regr=$(join "$BASE" "$LIVE" -o 0,1.2,2.2 2>/dev/null | awk '$2=="OK" && $3!="OK"{print "    " $1}')
  impr=$(join "$BASE" "$LIVE" -o 0,1.2,2.2 2>/dev/null | awk '$2!="OK" && $3=="OK"{print "    " $1}')
  [ -n "$impr" ] && { echo "  IMPROVED:"; echo "$impr"; }
  dbt=$(grep ' RED$' "$BASE" 2>/dev/null | awk '{print "    " $1}')
  [ -n "$dbt" ] && { echo "  known bridge-gap debt (recorded, not failures):"; echo "$dbt"; }
  if [ -n "$regr" ]; then echo "  RED  REGRESSION -- these were OK at baseline:"; echo "$regr"; fail=1; fi
else
  echo "  (no $BASE baseline yet -- run once with REBASELINE=1; strict mode: any not-OK fails)"
  [ "$nok" = "$ntot" ] || fail=1
fi
echo "== s9-run-diff (BASELINE=$BASELINE): the RUN differential is the H-h gate =="
exit $fail
