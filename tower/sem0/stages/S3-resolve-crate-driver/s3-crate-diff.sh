#!/usr/bin/env bash
# S3 CROSS-FILE resolution differential (the crate-driver rearchitecture).
# EXPECTED = rustc's resolution on the REAL multi-file crate (root.rs; rustc reads the module tree natively):
#   def-paths (-Zunpretty=hir-tree) + E0603 privacy + E0432 unresolved + E0252 duplicate.
# ACTUAL   = s1_parser's //@S3RESOLVE output on the crate-driver-ASSEMBLED single source (crate-driver.py root.rs).
# This proves cross-FILE name resolution == rustc. BOOTSTRAP = rz==rc on the assembled source. Requires SP + CC.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; S1="$HERE/../S1-lex-parse-ast"
SP="${SP:?set SP}"; CC="${CC:?set CC}"
P="$S1/s1_parser.rs"
. "$(dirname "$0")/../s1-build.sh"   # builds $SP/s1.rc + $SP/s1.rz once per input set, then reuses them
defpaths(){ rustc +nightly -Zunpretty=hir-tree --edition 2021 "$1" 2>/dev/null \
  | grep -oE 'owner_id: DefId\(0:[0-9]+ ~ [a-z_]+\[[0-9a-f]+\]::[^),]+\)' \
  | sed -E 's/.*\[[0-9a-f]+\]:://; s/\)$//' | grep -vE '^std$|prelude|^\{use#' | sort -u; }
privnames(){ rustc --edition 2021 --emit=metadata -o "$SP/.oracle.rmeta" "$1" 2>&1 | grep -oE '`[a-z_A-Z0-9]+` is private' | sed -E 's/`//g; s/ is private//' | sort -u; }
nonames(){ rustc --edition 2021 --emit=metadata -o "$SP/.oracle.rmeta" "$1" 2>&1 | grep -oE 'no `[a-z_A-Z0-9]+` in' | sed -E 's/no `//; s/` in//' | sort -u; }
dupnames(){ rustc --edition 2021 --emit=metadata -o "$SP/.oracle.rmeta" "$1" 2>&1 | grep -oE 'the name `[a-z_A-Z0-9]+` is defined multiple times' | sed -E 's/the name `//; s/` is defined multiple times//' | sort -u; }
fail=0
for cd in "$HERE"/crates/*/; do
  [ -e "$cd/root.rs" ] || continue
  n="$(basename "$cd")"; root="$cd/root.rs"
  defpaths  "$root" > "$SP/c.$n.res.expect"
  privnames "$root" > "$SP/c.$n.priv.expect"
  nonames   "$root" > "$SP/c.$n.no.expect"
  dupnames  "$root" > "$SP/c.$n.dup.expect"
  { echo '//@S3RESOLVE'; python3 "$HERE/crate-driver.py" "$root"; } > "$SP/c.$n.in"
  "$SP/s1.rc" < "$SP/c.$n.in" 2>/dev/null > "$SP/c.$n.rc"
  "$SP/s1.rz" < "$SP/c.$n.in" 2>/dev/null > "$SP/c.$n.rz"
  if ! cmp -s "$SP/c.$n.rc" "$SP/c.$n.rz"; then echo "  FAIL BOOTSTRAP ($n): rz != rc"; fail=1; continue; fi
  for tag in "RESPATH res RESOLVE hir-tree" "PRIVUSE priv PRIVUSE E0603" "NOUSE no NOUSE E0432" "DUPUSE dup DUPUSE E0252"; do
    set -- $tag; pfx="$1"; ext="$2"; label="$3"; oracle="$4"
    grep -oE "^$pfx .*" "$SP/c.$n.rc" | sed "s/^$pfx //" | sort -u > "$SP/c.$n.$ext.actual"
    # def-paths gated only when rustc produced hir-tree (a resolution-error crate aborts)
    if [ "$pfx" = "RESPATH" ] && [ ! -s "$SP/c.$n.res.expect" ]; then continue; fi
    if cmp -s "$SP/c.$n.$ext.actual" "$SP/c.$n.$ext.expect"; then echo "  ok   $label ($n): cross-file == rustc $oracle ($(wc -l < "$SP/c.$n.$ext.expect"))"
    else echo "  RED  $label ($n): != rustc $oracle"; diff "$SP/c.$n.$ext.expect" "$SP/c.$n.$ext.actual" | head -4; fail=1; fi
  done
done
[ "$fail" = 0 ] && echo "== s3-crate: cross-file resolution == rustc ==" || echo "== s3-crate: BROKEN =="
exit $fail
