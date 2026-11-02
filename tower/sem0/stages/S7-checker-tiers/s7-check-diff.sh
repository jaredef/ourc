#!/usr/bin/env bash
# S7 checker-tier differential (CHK-0 instrument). EXPECTED = rustc's ACCEPT/REJECT verdict on the program: `ACCEPT` if
# it compiles, else the first `E<nnnn>` error code (a REAL rustc reject -- per harness-bugs discipline, a witness is a
# genuine rustc reject, not a harness artifact). ACTUAL = s1_parser's `//@S7CHECK`-mode `VERDICT <v>` line. BOOTSTRAP =
# rz==rc. The accept/reject verdict is the gate; NEVER false-reject a valid program. Needs SP+CC.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; S1="$HERE/../S1-lex-parse-ast"
SP="${SP:?set SP}"; CC="${CC:?set CC}"
P="$S1/s1_parser.rs"
. "$(dirname "$0")/../s1-build.sh"   # builds $SP/s1.rc + $SP/s1.rz once per input set, then reuses them
rustc_verdict(){ rustc --edition 2021 "$1" -o "$SP/v.bin" 2>"$SP/v.err"; if [ $? -eq 0 ]; then echo "ACCEPT"; else grep -oE 'E[0-9]+' "$SP/v.err" | head -1; fi; }
fail=0
for w in "$HERE"/witnesses/*.rs; do
  [ -e "$w" ] || continue
  n="$(basename "$w" .rs)"
  rustc_verdict "$w" > "$SP/$n.chk.expect"
  { echo '//@S7CHECK'; cat "$w"; } > "$SP/$n.chk.in"
  "$SP/s1.rc" < "$SP/$n.chk.in" 2>/dev/null > "$SP/$n.chk.rc"
  "$SP/s1.rz" < "$SP/$n.chk.in" 2>/dev/null > "$SP/$n.chk.rz"
  if ! cmp -s "$SP/$n.chk.rc" "$SP/$n.chk.rz"; then echo "  FAIL BOOTSTRAP ($n): rz != rc"; fail=1; continue; fi
  grep -oE '^VERDICT .*' "$SP/$n.chk.rc" | sed 's/^VERDICT //' | head -1 > "$SP/$n.chk.actual"
  if cmp -s "$SP/$n.chk.actual" "$SP/$n.chk.expect"; then echo "  ok   CHECK ($n): verdict == rustc ($(cat "$SP/$n.chk.expect"))"
  else echo "  RED  CHECK ($n): verdict != rustc (want $(cat "$SP/$n.chk.expect"), got $(cat "$SP/$n.chk.actual"))"; fi
done
[ "$fail" = 0 ] && echo "== s7-check: BOOTSTRAP green (verdict RED until the checker lands) ==" || echo "== s7-check: BOOTSTRAP broken =="
exit $fail
