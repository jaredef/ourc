#!/usr/bin/env bash
# S6 monomorphization differential (MONO-0 instrument).
# EXPECTED = rustc's collected mono-item SET (`-Zprint-mono-items=yes`), filtered to USER-CRATE items and normalized:
# each `MONO_ITEM fn NAME @@ ..` -> NAME (a bare fn or a `name::<T, ..>` instantiation), std/core/alloc + trait-shim +
# closure noise dropped, sorted-unique. That IS the monomorphization target: the same set of instantiations rustc emits.
# ACTUAL = s1_parser's `//@S6MONO`-mode mono-item dump (lines prefixed `MONO `). Until the collector lands the marker
# is inert, so ACTUAL != EXPECTED = RED (collector owed). BOOTSTRAP = rz==rc. Needs SP+CC.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; S1="$HERE/../S1-lex-parse-ast"
SP="${SP:?set SP}"; CC="${CC:?set CC}"
P="$S1/s1_parser.rs"
. "$(dirname "$0")/../s1-build.sh"   # builds $SP/s1.rc + $SP/s1.rz once per input set, then reuses them
# rustc mono-collection oracle, filtered to the user crate + normalized (sorted-unique instantiation names)
monoset(){ rustc +nightly -Zprint-mono-items=yes --edition 2021 "$1" -o "$SP/m.bin" 2>&1 \
  | grep -E '^MONO_ITEM fn ' | sed -E 's/^MONO_ITEM fn //; s/ @@.*//' \
  | grep -vE 'std::|core::|alloc::| as |\{closure|- shim' | sort -u; }
fail=0
for w in "$HERE"/witnesses/*.rs; do
  [ -e "$w" ] || continue
  n="$(basename "$w" .rs)"
  monoset "$w" > "$SP/$n.mono.expect"
  { echo '//@S6MONO'; cat "$w"; } > "$SP/$n.mono.in"
  "$SP/s1.rc" < "$SP/$n.mono.in" 2>/dev/null > "$SP/$n.mono.rc"
  "$SP/s1.rz" < "$SP/$n.mono.in" 2>/dev/null > "$SP/$n.mono.rz"
  if ! cmp -s "$SP/$n.mono.rc" "$SP/$n.mono.rz"; then echo "  FAIL BOOTSTRAP ($n): rz != rc"; fail=1; continue; fi
  grep -oE '^MONO .*' "$SP/$n.mono.rc" | sed 's/^MONO //' | sort -u > "$SP/$n.mono.actual"
  if cmp -s "$SP/$n.mono.actual" "$SP/$n.mono.expect"; then echo "  ok   MONO ($n): s1_parser mono-set == rustc -Zprint-mono-items"
  else echo "  RED  MONO ($n): collector owed (expected $(wc -l < "$SP/$n.mono.expect") items, got $(wc -l < "$SP/$n.mono.actual"))"; fi
done
[ "$fail" = 0 ] && echo "== s6-mono: BOOTSTRAP green (MONO RED until the collector lands) ==" || echo "== s6-mono: BOOTSTRAP broken =="
exit $fail
