#!/usr/bin/env bash
# S4 AST->HIR lowering differential (HIR-0 instrument).
# EXPECTED = rustc's HIR (`-Zunpretty=hir`), the desugared front-end, normalized to a canonical token stream: strip the
# injected prelude + #[allow(..)] attrs + collapse whitespace. That IS the lowering target -- the desugared control-flow
# shape (x? -> match branch{Break/Continue}; for -> loop{match next{None/Some}}; etc.).
# ACTUAL = s1_parser's `//@S4HIR`-mode HIR dump (lines prefixed `HIR ` -> the same canonical stream). Until the lowering
# lands the marker is an inert `//` comment, so ACTUAL != EXPECTED = RED (lowering owed). BOOTSTRAP = rz==rc. Needs SP+CC.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; S1="$HERE/../S1-lex-parse-ast"
SP="${SP:?set SP}"; CC="${CC:?set CC}"
P="$S1/s1_parser.rs"
. "$(dirname "$0")/../s1-build.sh"   # builds $SP/s1.rc + $SP/s1.rz once per input set, then reuses them
# rustc HIR oracle, normalized to a canonical single-line token stream
hirnorm(){ rustc +nightly -Zunpretty=hir --edition 2021 "$1" 2>/dev/null \
  | grep -vE '^#!\[|^extern crate |^#\[prelude_import\]|^use ::|^use std::prelude' \
  | sed -E 's/#\[allow\([^]]*\)\]//g; s/#\[attr = [^]]*\]//g' | tr -s ' \n\t' ' ' | sed -E 's/^ +//; s/ +$//'; }
fail=0
for w in "$HERE"/witnesses/*.rs; do
  [ -e "$w" ] || continue
  n="$(basename "$w" .rs)"
  hirnorm "$w" > "$SP/$n.hir.expect"
  { echo '//@S4HIR'; cat "$w"; } > "$SP/$n.hir.in"
  "$SP/s1.rc" < "$SP/$n.hir.in" 2>/dev/null > "$SP/$n.hir.rc"
  "$SP/s1.rz" < "$SP/$n.hir.in" 2>/dev/null > "$SP/$n.hir.rz"
  if ! cmp -s "$SP/$n.hir.rc" "$SP/$n.hir.rz"; then echo "  FAIL BOOTSTRAP ($n): rz != rc"; fail=1; continue; fi
  grep -oE '^HIR .*' "$SP/$n.hir.rc" | sed 's/^HIR //' | tr -s ' \n\t' ' ' | sed -E 's/^ +//; s/ +$//' > "$SP/$n.hir.actual"
  if cmp -s "$SP/$n.hir.actual" "$SP/$n.hir.expect"; then echo "  ok   HIR ($n): s1_parser lowering == rustc -Zunpretty=hir"
  else echo "  RED  HIR ($n): lowering owed (expected $(wc -c < "$SP/$n.hir.expect") chars, got $(wc -c < "$SP/$n.hir.actual"))"; fi
done
[ "$fail" = 0 ] && echo "== s4-hir: BOOTSTRAP green (HIR RED until the lowering lands) ==" || echo "== s4-hir: BOOTSTRAP broken =="
exit $fail
