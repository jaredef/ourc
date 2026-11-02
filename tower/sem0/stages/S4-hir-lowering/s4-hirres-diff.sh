#!/usr/bin/env bash
# S4 HIR-6 (P-HIR-RES-DEFID) differential -- the S3->S4 Res/DefId bridge.
# EXPECTED = rustc hir-tree's fn-call resolutions: each `res: Def(Fn, DefId(.. :: NAME))` -> `Fn NAME` (crate-normalized,
# sorted-unique). ACTUAL = s1_parser's `//@S4HIRRES` mode (`HIRRES Fn NAME` per fn-call path resolved against the crate's
# top-level defs). Proves HIR paths carry the fn Res == rustc. BOOTSTRAP = rz==rc. Const/Static/method Res = later slices.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; S1="$HERE/../S1-lex-parse-ast"
SP="${SP:?set SP}"; CC="${CC:?set CC}"
P="$S1/s1_parser.rs"
. "$(dirname "$0")/../s1-build.sh"   # builds $SP/s1.rc + $SP/s1.rz once per input set, then reuses them
orc(){ rustc +nightly -Zunpretty=hir-tree --edition 2021 --crate-type=lib "$1" 2>/dev/null | tr -s ' \n\t' ' ' \
  | sed -E 's/Def\( (Fn|Const|Static|Struct|Enum|Union|TyAlias)( \{[^}]*\})?, DefId\( ?0:[^)]*:: ?([a-zA-Z0-9_]+) ?\)/\nKV \1 \3\n/g' \
  | grep -oE '^KV (Fn|Const|Static|Struct|Enum|Union|TyAlias) [a-zA-Z0-9_]+' | sed 's/^KV //' | sort -u
  rustc +nightly -Zunpretty=hir-tree --edition 2021 --crate-type=lib "$1" 2>/dev/null | tr -s ' \n\t' ' ' \
    | grep -oE 'res: Def\( Ctor\([^)]*\), DefId\( ?0:[^)]*::[A-Za-z0-9_]+::[A-Za-z0-9_]+::\{constructor' \
    | sed -E 's/.*::([A-Za-z0-9_]+::[A-Za-z0-9_]+)::\{constructor/Variant \1/' | sort -u
  rustc +nightly -Zunpretty=hir-tree --edition 2021 --crate-type=lib "$1" 2>/dev/null | tr -s ' \n\t' ' ' \
    | grep -oE 'res: Def\( (Fn|Trait)( \{[^}]*\})?, DefId\([1-9][0-9]*:[0-9]+ ~ [a-z_]+\[[0-9a-f]+\]::[a-zA-Z0-9_:]+ ?\)' \
    | sed -E 's/res: Def\( ([A-Za-z]+)[^,]*, DefId\([0-9]+:[0-9]+ ~ ([a-z_]+)\[[0-9a-f]+\]::([a-zA-Z0-9_:]+) ?\)/\1 \2::\3/' | sort -u
  rustc +nightly -Zunpretty=hir-tree --edition 2021 --crate-type=lib "$1" 2>/dev/null | tr -s ' \n\t' ' ' \
    | grep -oE 'res: Def\( (Struct|Enum)( \{[^}]*\})?, DefId\([1-9][0-9]*:[0-9]+ ~ [a-z_]+\[[0-9a-f]+\]::[a-zA-Z0-9_:]+ ?\)' \
    | sed -E 's/res: Def\( ([A-Za-z]+)[^,]*, DefId\([0-9]+:[0-9]+ ~ ([a-z_]+)\[[0-9a-f]+\]::([a-zA-Z0-9_:]+) ?\)/\1 \2::\3/' \
    | grep -E 'Struct alloc::vec::Vec|Struct alloc::string::String|Struct alloc::boxed::Box|Enum core::option::Option|Enum core::result::Result|Enum core::cmp::Ordering' | sort -u; }
fail=0
for w in "$HERE"/witnesses-hirres/*.rs; do
  [ -e "$w" ] || continue
  n="$(basename "$w" .rs)"
  orc "$w" > "$SP/$n.hr.expect"
  { echo '//@S4HIRRES'; cat "$w"; } > "$SP/$n.hr.in"
  "$SP/s1.rc" < "$SP/$n.hr.in" 2>/dev/null | grep -oE '^HIRRES .*' | sed 's/^HIRRES //' | sort -u > "$SP/$n.hr.rc"
  "$SP/s1.rz" < "$SP/$n.hr.in" 2>/dev/null | grep -oE '^HIRRES .*' | sed 's/^HIRRES //' | sort -u > "$SP/$n.hr.rz"
  if ! cmp -s "$SP/$n.hr.rc" "$SP/$n.hr.rz"; then echo "  FAIL BOOTSTRAP ($n): rz != rc"; fail=1; continue; fi
  if cmp -s "$SP/$n.hr.rc" "$SP/$n.hr.expect"; then echo "  ok   HIRRES ($n): fn-call Res == rustc hir-tree ($(wc -l < "$SP/$n.hr.expect"))"
  else echo "  RED  HIRRES ($n): != rustc"; diff "$SP/$n.hr.expect" "$SP/$n.hr.rc" | head; fail=1; fi
done
[ "$fail" = 0 ] && echo "== s4-hirres: HIR-6 fn-call Res bridge == rustc ==" || echo "== s4-hirres: BROKEN =="
exit $fail
