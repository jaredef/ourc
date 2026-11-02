#!/usr/bin/env bash
# S3 CROSS-CRATE resolution differential (f). A crate dir has root.rs (`extern crate b; ...`) + b.rs (the dep crate).
# ORACLE = rustc on the REAL 2-crate build: compile b as a lib, then root.rs with --extern b -> E0603/E0432 + a's
# def-paths (b's defs are NOT in a's crate). ACTUAL = s1_parser (CWD=crate dir) reading the dep crate's root itself.
# Proves cross-CRATE resolution == rustc. BOOTSTRAP = rz==rc. Requires SP + CC.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; S1="$HERE/../S1-lex-parse-ast"
SP="${SP:?set SP}"; CC="${CC:?set CC}"; export TMPDIR="$SP"
P="$S1/s1_parser.rs"
. "$(dirname "$0")/../s1-build.sh"   # builds $SP/s1.rc + $SP/s1.rz once per input set, then reuses them
fail=0
for cd in "$HERE"/crates-xc/*/; do
  [ -e "$cd/root.rs" ] || continue
  n="$(basename "$cd")"
  # rustc oracle: compile the dep crate b as a lib, then root.rs with --extern b
  mkdir -p "$SP/xcr.$n"; ( cd "$cd" && rustc --edition 2021 --crate-type=lib --crate-name=b b.rs -o "$SP/xcr.$n/libb.rlib" 2>/dev/null )
  ce=$( cd "$cd" && rustc --edition 2021 --extern b="$SP/xcr.$n/libb.rlib" root.rs -o "$SP/xc.$n.out" 2>&1 )
  echo "$ce" | grep -oE '`[a-z_A-Z0-9]+` is private' | sed -E 's/`//g; s/ is private//' | sort -u > "$SP/xc.$n.priv.expect"
  echo "$ce" | grep -oE 'no `[a-z_A-Z0-9]+` in' | sed -E 's/no `//; s/` in//' | sort -u > "$SP/xc.$n.no.expect"
  # def-paths of crate a (via hir-tree, with --extern); b's defs are excluded (separate crate)
  ( cd "$cd" && rustc +nightly -Zunpretty=hir-tree --edition 2021 --extern b="$SP/xcr.$n/libb.rlib" root.rs 2>/dev/null ) \
    | grep -oE 'owner_id: DefId\(0:[0-9]+ ~ [a-z_]+\[[0-9a-f]+\]::[^),]+\)' | sed -E 's/.*\[[0-9a-f]+\]:://; s/\)$//' \
    | grep -vE '^std$|prelude|^\{use#|^b$' | sort -u > "$SP/xc.$n.res.expect"
  { echo '//@S3RESOLVE'; cat "$cd/root.rs"; } > "$SP/xc.$n.in"
  ( cd "$cd" && "$SP/s1.rc" ) < "$SP/xc.$n.in" 2>/dev/null > "$SP/xc.$n.rc"
  ( cd "$cd" && "$SP/s1.rz" ) < "$SP/xc.$n.in" 2>/dev/null > "$SP/xc.$n.rz"
  if ! cmp -s "$SP/xc.$n.rc" "$SP/xc.$n.rz"; then echo "  FAIL BOOTSTRAP ($n): rz != rc"; fail=1; continue; fi
  grep -oE '^RESPATH .*' "$SP/xc.$n.rc" | sed 's/^RESPATH //' | sort -u > "$SP/xc.$n.res.actual"
  grep -oE '^PRIVUSE .*' "$SP/xc.$n.rc" | sed 's/^PRIVUSE //' | sort -u > "$SP/xc.$n.priv.actual"
  grep -oE '^NOUSE .*'   "$SP/xc.$n.rc" | sed 's/^NOUSE //'   | sort -u > "$SP/xc.$n.no.actual"
  if [ -s "$SP/xc.$n.res.expect" ]; then
    cmp -s "$SP/xc.$n.res.actual" "$SP/xc.$n.res.expect" && echo "  ok   RESOLVE ($n): a's def-paths == rustc ($(wc -l < "$SP/xc.$n.res.expect"))" || { echo "  RED  RESOLVE ($n)"; diff "$SP/xc.$n.res.expect" "$SP/xc.$n.res.actual"|head; fail=1; }
  fi
  cmp -s "$SP/xc.$n.priv.actual" "$SP/xc.$n.priv.expect" && echo "  ok   PRIVUSE ($n): cross-crate == rustc E0603 ($(wc -l < "$SP/xc.$n.priv.expect"))" || { echo "  RED  PRIVUSE ($n)"; diff "$SP/xc.$n.priv.expect" "$SP/xc.$n.priv.actual"|head; fail=1; }
  cmp -s "$SP/xc.$n.no.actual" "$SP/xc.$n.no.expect" && echo "  ok   NOUSE ($n): cross-crate == rustc E0432 ($(wc -l < "$SP/xc.$n.no.expect"))" || { echo "  RED  NOUSE ($n)"; diff "$SP/xc.$n.no.expect" "$SP/xc.$n.no.actual"|head; fail=1; }
done
[ "$fail" = 0 ] && echo "== s3-xcrate: cross-crate resolution == rustc ==" || echo "== s3-xcrate: BROKEN =="
exit $fail
