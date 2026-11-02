#!/usr/bin/env bash
# S5 typeck + trait-solver differential (TYCK-0 instrument).
# EXPECTED = rustc's TYPED HIR (`-Zunpretty=hir,typed`): the HIR pretty-print with EVERY expr annotated by its inferred
# type as `(EXPR as TYPE)` -- literal inference (`1 as i32`), let-propagation, method-res, trait selection
# (`call::<A>`). That IS the typeck target: same inferred types + same method/trait selection as rustc.
# ACTUAL = s1_parser's `//@S5TYPED`-mode typed-HIR dump (lines prefixed `TYPED ` -> the same canonical stream). Until
# the inference engine lands the marker is inert, so ACTUAL != EXPECTED = RED (typeck owed). BOOTSTRAP = rz==rc. Needs SP+CC.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; S1="$HERE/../S1-lex-parse-ast"
SP="${SP:?set SP}"; CC="${CC:?set CC}"
P="$S1/s1_parser.rs"
. "$(dirname "$0")/../s1-build.sh"   # builds $SP/s1.rc + $SP/s1.rz once per input set, then reuses them
# rustc typed-HIR oracle, normalized to a canonical single-line token stream
tynorm(){ rustc +nightly -Zunpretty=hir,typed --edition 2021 "$1" 2>/dev/null \
  | grep -vE '^#!\[|^extern crate |^#\[prelude_import\]|^use ::|^use std::prelude' \
  | sed -E 's/#\[allow\([^]]*\)\]//g; s/#\[attr = [^]]*\]//g' | tr -s ' \n\t' ' ' | sed -E 's/^ +//; s/ +$//'; }
fail=0
for w in "$HERE"/witnesses/*.rs; do
  [ -e "$w" ] || continue
  n="$(basename "$w" .rs)"
  tynorm "$w" > "$SP/$n.ty.expect"
  { echo '//@S5TYPED'; cat "$w"; } > "$SP/$n.ty.in"
  "$SP/s1.rc" < "$SP/$n.ty.in" 2>/dev/null > "$SP/$n.ty.rc"
  "$SP/s1.rz" < "$SP/$n.ty.in" 2>/dev/null > "$SP/$n.ty.rz"
  if ! cmp -s "$SP/$n.ty.rc" "$SP/$n.ty.rz"; then echo "  FAIL BOOTSTRAP ($n): rz != rc"; fail=1; continue; fi
  grep -oE '^TYPED .*' "$SP/$n.ty.rc" | sed 's/^TYPED //' | tr -s ' \n\t' ' ' | sed -E 's/^ +//; s/ +$//' > "$SP/$n.ty.actual"
  if cmp -s "$SP/$n.ty.actual" "$SP/$n.ty.expect"; then echo "  ok   TYPED ($n): s1_parser typeck == rustc -Zunpretty=hir,typed"
  else echo "  RED  TYPED ($n): typeck owed (expected $(wc -c < "$SP/$n.ty.expect") chars, got $(wc -c < "$SP/$n.ty.actual"))"; fi
done
[ "$fail" = 0 ] && echo "== s5-typeck: BOOTSTRAP green (TYPED RED until the inference engine lands) ==" || echo "== s5-typeck: BOOTSTRAP broken =="
exit $fail
