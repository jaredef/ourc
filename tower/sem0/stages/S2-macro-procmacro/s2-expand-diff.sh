#!/usr/bin/env bash
# S2 macro-EXPANSION differential harness (MAC-0 instrument).
# For each witnesses/*.rs: the EXPECTED post-expansion AST is s1_parser's dump of `rustc -Zunpretty=expanded`
# (rustc does the macro expansion; s1_parser serializes the already-expanded source). The ACTUAL is s1_parser's
# dump of the RAW witness -- once the S2 engine expands macros inside s1_parser, ACTUAL == EXPECTED.
# Until the engine lands, mismatches are the live MAC board (RED = expansion still owed). The s1_parser build
# is the same dual (rustc oracle + rzrs stage0) as s1-ast-diff.sh. Requires SP (L6.rzrs + cf.hoist) + CC.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; S1="$HERE/../S1-lex-parse-ast"
SP="${SP:?set SP}"; CC="${CC:?set CC}"
P="$S1/s1_parser.rs"
. "$(dirname "$0")/../s1-build.sh"   # builds $SP/s1.rc + $SP/s1.rz once per input set, then reuses them
# Structural compare: strip @lo..hi spans. The rustc-EXPANDED source and the RAW witness carry different byte
# positions (id!() is longer than its expansion), so an expanded AST can only match structurally, not on spans.
# NodeIds/spans are the position-dependent class the MAC board ignores (hygiene numbering is handled per-rung).
strip(){ sed 's/@[0-9]*\.\.[0-9]*//g'; }
fail=0
for w in "$HERE"/witnesses/*.rs; do
  [ -e "$w" ] || continue
  n="$(basename "$w" .rs)"
  # EXPECTED: rustc-expanded source -> strip the injected prelude cruft -> s1_parser AST dump (unexpanded mode)
  rustc +nightly -Zunpretty=expanded --edition 2021 "$w" 2>/dev/null \
    | grep -vE '^#!\[feature|^extern crate std;|^#\[prelude_import\]|^use std::prelude' > "$SP/$n.exp.rs"
  "$SP/s1.rc" < "$SP/$n.exp.rs" 2>/dev/null | strip > "$SP/$n.expect"
  # ACTUAL: raw witness with the `//@S2EXPAND` marker prepended -> in-parser expansion, dumped by BOTH builds.
  { echo '//@S2EXPAND'; cat "$w"; } > "$SP/$n.in.rs"
  "$SP/s1.rc" < "$SP/$n.in.rs" 2>/dev/null | strip > "$SP/$n.rc"
  "$SP/s1.rz" < "$SP/$n.in.rs" 2>/dev/null | strip > "$SP/$n.rz"
  if ! cmp -s "$SP/$n.rc" "$SP/$n.rz"; then echo "  FAIL BOOTSTRAP ($n): rz != rc"; fail=1; continue; fi
  if cmp -s "$SP/$n.rc" "$SP/$n.expect"; then echo "  ok   EXPANSION ($n): s1_parser expansion == rustc -Zunpretty=expanded"
  else echo "  RED  EXPANSION ($n): not yet expanded (engine owed)"; diff "$SP/$n.expect" "$SP/$n.rc" | head -6; fi
done
[ "$fail" = 0 ] && echo "== s2-expand: BOOTSTRAP green (expansion RED until the engine lands) ==" || echo "== s2-expand: BOOTSTRAP broken =="
exit $fail
