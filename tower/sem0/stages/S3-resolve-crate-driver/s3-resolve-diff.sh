#!/usr/bin/env bash
# S3 name-resolution differential harness (RES-0 instrument).
# EXPECTED = the normalized DefId-PATH set rustc assigns (via `-Zunpretty=hir-tree`): each crate-owned def's
# resolution path, crate-hash stripped, std/prelude/{use#0} filtered, sorted-unique. That IS the resolution
# target -- every item + use + (later) path binds to the same Res/DefId path as rustc.
# ACTUAL = s1_parser's `//@S3RESOLVE`-mode def-path dump. Until the resolver lands, s1_parser does not emit the
# map (the marker is an inert `//` comment), so ACTUAL != EXPECTED = RED (resolver owed). BOOTSTRAP = rz==rc on
# the witness (the same dual as s1/s2). Requires SP (L6.rzrs + cf.hoist) + CC.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; S1="$HERE/../S1-lex-parse-ast"
SP="${SP:?set SP}"; CC="${CC:?set CC}"
P="$S1/s1_parser.rs"
. "$(dirname "$0")/../s1-build.sh"   # builds $SP/s1.rc + $SP/s1.rz once per input set, then reuses them
# rustc def-path oracle
defpaths(){ rustc +nightly -Zunpretty=hir-tree --edition 2021 "$1" 2>/dev/null \
  | grep -oE 'owner_id: DefId\(0:[0-9]+ ~ [a-z_]+\[[0-9a-f]+\]::[^),]+\)' \
  | sed -E 's/.*\[[0-9a-f]+\]:://; s/\)$//' | grep -vE '^std$|prelude|^\{use#' | sort -u; }
# rustc E0603 private-use oracle: the set of names rustc reports as private (`NAME` is private) via a use.
privnames(){ rustc --edition 2021 --emit=metadata -o "$SP/.oracle.rmeta" "$1" 2>&1 | grep -oE '`[a-z_A-Z0-9]+` is private' | sed -E 's/`//g; s/ is private//' | sort -u; }
# rustc E0432 unresolved-import oracle: the set of names rustc reports as missing (`no `NAME` in ...`).
nonames(){ rustc --edition 2021 --emit=metadata -o "$SP/.oracle.rmeta" "$1" 2>&1 | grep -oE 'no `[a-z_A-Z0-9]+` in' | sed -E 's/no `//; s/` in//' | sort -u; }
# rustc E0252 duplicate-import oracle: the set of names defined multiple times by imports.
dupnames(){ rustc --edition 2021 --emit=metadata -o "$SP/.oracle.rmeta" "$1" 2>&1 | grep -oE 'the name `[a-z_A-Z0-9]+` is defined multiple times' | sed -E 's/the name `//; s/` is defined multiple times//' | sort -u; }
fail=0
for w in "$HERE"/witnesses/*.rs; do
  [ -e "$w" ] || continue
  n="$(basename "$w" .rs)"
  defpaths "$w" > "$SP/$n.res.expect"
  privnames "$w" > "$SP/$n.priv.expect"
  nonames "$w" > "$SP/$n.no.expect"
  dupnames "$w" > "$SP/$n.dup.expect"
  { echo '//@S3RESOLVE'; cat "$w"; } > "$SP/$n.res.in"
  "$SP/s1.rc" < "$SP/$n.res.in" 2>/dev/null > "$SP/$n.res.rc"
  "$SP/s1.rz" < "$SP/$n.res.in" 2>/dev/null > "$SP/$n.res.rz"
  if ! cmp -s "$SP/$n.res.rc" "$SP/$n.res.rz"; then echo "  FAIL BOOTSTRAP ($n): rz != rc"; fail=1; continue; fi
  # RESPATH def-path set -- compared only when rustc produced hir-tree (a privacy-error witness aborts -> empty expected)
  grep -oE '^RESPATH .*' "$SP/$n.res.rc" | sed 's/^RESPATH //' | sort -u > "$SP/$n.res.actual"
  if [ -s "$SP/$n.res.expect" ]; then
    if cmp -s "$SP/$n.res.actual" "$SP/$n.res.expect"; then echo "  ok   RESOLVE ($n): s1_parser def-paths == rustc hir-tree"
    else echo "  RED  RESOLVE ($n): def-paths != rustc (expected $(wc -l < "$SP/$n.res.expect"), got $(wc -l < "$SP/$n.res.actual"))"; fail=1; fi
  fi
  # PRIVUSE set -- always compared (== rustc E0603 private-name set)
  grep -oE '^PRIVUSE .*' "$SP/$n.res.rc" | sed 's/^PRIVUSE //' | sort -u > "$SP/$n.priv.actual"
  if cmp -s "$SP/$n.priv.actual" "$SP/$n.priv.expect"; then echo "  ok   PRIVUSE ($n): private-use set == rustc E0603 ($(wc -l < "$SP/$n.priv.expect"))"
  else echo "  RED  PRIVUSE ($n): != rustc E0603"; diff "$SP/$n.priv.expect" "$SP/$n.priv.actual" | head -4; fail=1; fi
  # NOUSE set -- unresolved imports == rustc E0432 (no `NAME` in ...)
  grep -oE '^NOUSE .*' "$SP/$n.res.rc" | sed 's/^NOUSE //' | sort -u > "$SP/$n.no.actual"
  if cmp -s "$SP/$n.no.actual" "$SP/$n.no.expect"; then echo "  ok   NOUSE ($n): unresolved-import set == rustc E0432 ($(wc -l < "$SP/$n.no.expect"))"
  else echo "  RED  NOUSE ($n): != rustc E0432"; diff "$SP/$n.no.expect" "$SP/$n.no.actual" | head -4; fail=1; fi
  # DUPUSE set -- duplicate imports == rustc E0252
  grep -oE '^DUPUSE .*' "$SP/$n.res.rc" | sed 's/^DUPUSE //' | sort -u > "$SP/$n.dup.actual"
  if cmp -s "$SP/$n.dup.actual" "$SP/$n.dup.expect"; then echo "  ok   DUPUSE ($n): duplicate-import set == rustc E0252 ($(wc -l < "$SP/$n.dup.expect"))"
  else echo "  RED  DUPUSE ($n): != rustc E0252"; diff "$SP/$n.dup.expect" "$SP/$n.dup.actual" | head -4; fail=1; fi
done
[ "$fail" = 0 ] && echo "== s3-resolve: BOOTSTRAP green (RESOLVE RED until the resolver lands) ==" || echo "== s3-resolve: BOOTSTRAP broken =="
exit $fail
