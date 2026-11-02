#!/usr/bin/env bash
# D0 OVER-ACCEPTANCE BOARD -- the only instrument in this arc that can see the finite-stratum defect class.
#
# Every other corpus here (r2, the witness board, tree-syntax-board's 1,847 files) holds only VALID programs, so
# an over-accepting front end is invisible to all of them: tree-syntax-board reported `0 OVER-ACCEPTED` for
# months while the lexer accepted every byte >= 128 as an identifier character. A conformance claim needs inputs
# the front end must REJECT, or it is unfalsifiable.
#
# `accept-*` fixtures must be accepted by BOTH; `reject-*` must be rejected by both. rustc is consulted live
# rather than trusted from the filename, so a fixture that stops being invalid is reported as a bad fixture
# instead of silently passing.
#
#   SP=<scratch with sem0.cc + cf.hoist> bash overaccept-board.sh
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../../.." && pwd)"
CC="${TOWER_L2:-$ROOT/tower/L2}"
: "${SP:?set SP}"
[ -x "$SP/sem0.cc" ] || { echo "  FAIL: no \$SP/sem0.cc"; exit 1; }
[ -s "$SP/cf.hoist" ] || { echo "  FAIL: no \$SP/cf.hoist"; exit 1; }
for b in link/rzir-ld.bin opt/rzir-opt.bin ra/rzir-ra.bin; do
  [ -x "$CC/$b" ] || { echo "  FAIL: missing backend $CC/$b -- every row would agree and mean nothing"; exit 1; }
done
command -v rustc >/dev/null || { echo "  FAIL: rustc absent; this board is a DIFFERENTIAL"; exit 1; }

W="$(mktemp -d)"; trap 'rm -rf "$W"' EXIT

# THE TIER UNDER TEST IS THE DEV TARGET. $SP/sem0.cc is the FROZEN tier and lags it by a whole rung; testing it
# reports the pre-fix answer on every row. That mistake has been made twice in this arc, so mint explicitly.
P="$ROOT/tower/sem0/stages/S1-lex-parse-ast/s1_parser.rs"
back(){ "$SP/ld.bin" 2>/dev/null | "$CC/opt/rzir-opt.bin" 2>/dev/null | "$CC/ra/rzir-ra.bin" 2>/dev/null | "$SP/cf.hoist"; }
timeout 900 "$SP/sem0.cc" < "$P" 2>/dev/null | back > "$W/dev" 2>/dev/null; chmod +x "$W/dev"
[ "$(stat -c%s "$W/dev")" -ge 100000 ] || { echo "  FAIL: dev target did not mint ($(stat -c%s "$W/dev") bytes)"; exit 1; }

KO="$HERE/known-open.txt"
known(){ [ -f "$KO" ] && grep -qxF "$1" "$KO"; }
fail=0; n=0; diverge=0; kdiv=0
for f in "$HERE"/fixtures/*.rs; do
  b="$(basename "$f" .rs)"; n=$((n+1))
  if rustc --edition 2021 -O -o "$W/x" "$f" >/dev/null 2>&1; then want=ACCEPT; else want=REJECT; fi
  case "$b" in
    accept-*) [ "$want" = ACCEPT ] || { printf '   %-34s BAD FIXTURE: rustc rejects an accept-* case\n' "$b"; fail=1; continue; };;
    reject-*) [ "$want" = REJECT ] || { printf '   %-34s BAD FIXTURE: rustc accepts a reject-* case\n' "$b"; fail=1; continue; };;
  esac
  { echo '//@S9'; cat "$f"; } > "$W/in.rs"
  timeout 120 "$W/dev" < "$W/in.rs" > "$W/r" 2>/dev/null
  if [ -s "$W/r" ] && ! grep -q UNLOWERED "$W/r"; then got=ACCEPT; else got=REJECT; fi
  if [ "$want" = "$got" ]; then
    printf '   %-34s %s (agrees with rustc)\n' "$b" "$got"
    # A defect that is CLOSED must be removed from known-open.txt. Leaving it there keeps the ratchet slack:
    # the entry would silently absorb a future regression of the very defect it claims is still open.
    if known "$b"; then printf '   %-34s CLOSED but still on known-open.txt -- delete that line\n' "$b"; fail=1; fi
  elif known "$b"; then printf '   %-34s want=%-6s got=%-6s  OVER-ACCEPTS (known-open)\n' "$b" "$want" "$got"; kdiv=$((kdiv+1))
  else printf '   %-34s want=%-6s got=%-6s  OVER-ACCEPTS  ** NEW **\n' "$b" "$want" "$got"; diverge=$((diverge+1)); fi
done
echo
echo "  $n fixtures; $kdiv known-open, $diverge NEW divergence(s)"
# Also fail if known-open lists something that now AGREES -- a closed defect must be removed from the list, or
# the ratchet silently loses its grip and a later regression reads as expected.
if [ -f "$KO" ]; then
  while read -r k; do
    case "$k" in ''|\#*) continue;; esac
    [ -f "$HERE/fixtures/$k.rs" ] || { echo "   stale known-open entry (no such fixture): $k"; fail=1; }
  done < "$KO"
fi
if [ "$diverge" -eq 0 ] && [ "$fail" -eq 0 ]; then
  echo "== D0 board GREEN: $kdiv known-open, no new over-acceptance =="; exit 0
fi
echo "== D0 board RED: a divergence not on known-open.txt -- this is the class no other instrument here sees =="
exit 1
