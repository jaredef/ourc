#!/usr/bin/env bash
# S1 AST differential harness. Compiles the S1 parser with rustc (oracle build) AND rzrs (L5 bootstrap),
# runs both, and asserts, for BOTH the embedded-witness differential (empty stdin) AND the AST-10 whole-file
# crate differential (a real .rs fixture piped on stdin):
#   (1) BOOTSTRAP -- rzrs-built output == rustc-built output (self-host on the parser);
#   (2) FIDELITY  -- the canonical AST == the golden transcribed from rustc's -Zunpretty=ast-tree.
# The parser reads stdin: non-empty -> whole-file crate mode; empty (/dev/null) -> embedded witnesses.
# Requires SP (scratch with L6.rzrs + cf.hoist) + CC (cc-in-c backend). Exits non-zero on any mismatch.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
SP="${SP:?set SP}"; CC="${CC:?set CC (2026-06-30-cc-in-c)}"
P="$HERE/s1_parser.rs"; G="$HERE/goldens/s1-ast.txt"
. "$(dirname "$0")/../s1-build.sh"   # builds $SP/s1.rc + $SP/s1.rz once per input set, then reuses them
chmod +x "$SP/s1.rz"
fail=0

# --- Embedded-witness differential (empty stdin) ---
"$SP/s1.rc" < /dev/null > "$SP/s1.out.rc" 2>/dev/null
"$SP/s1.rz" < /dev/null > "$SP/s1.out.rz" 2>/dev/null
if cmp -s "$SP/s1.out.rc" "$SP/s1.out.rz"; then echo "  ok   BOOTSTRAP (witnesses): rzrs-built == rustc-built"; else echo "  FAIL BOOTSTRAP (witnesses): rzrs != rustc"; diff "$SP/s1.out.rc" "$SP/s1.out.rz" | head; fail=1; fi
if cmp -s "$SP/s1.out.rc" "$G"; then echo "  ok   FIDELITY (witnesses): canonical AST == golden"; else echo "  FAIL FIDELITY (witnesses): != golden"; diff "$G" "$SP/s1.out.rc" | head; fail=1; fi

# --- AST-10 whole-file crate differential (real .rs fixtures piped on stdin) ---
for fx in "$HERE"/fixtures/*.rs; do
  [ -e "$fx" ] || continue
  n="$(basename "$fx" .rs)"; wg="$HERE/goldens/whole-$n.txt"
  "$SP/s1.rc" < "$fx" > "$SP/w.$n.rc" 2>/dev/null
  "$SP/s1.rz" < "$fx" > "$SP/w.$n.rz" 2>/dev/null
  if cmp -s "$SP/w.$n.rc" "$SP/w.$n.rz"; then echo "  ok   BOOTSTRAP (whole-file $n): rzrs-built == rustc-built"; else echo "  FAIL BOOTSTRAP (whole-file $n): rzrs != rustc"; diff "$SP/w.$n.rc" "$SP/w.$n.rz" | head; fail=1; fi
  if [ -f "$wg" ]; then
    if cmp -s "$SP/w.$n.rc" "$wg"; then echo "  ok   FIDELITY (whole-file $n): crate AST == golden"; else echo "  FAIL FIDELITY (whole-file $n): != golden"; diff "$wg" "$SP/w.$n.rc" | head; fail=1; fi
  fi
done

[ "$fail" = 0 ] && echo "== S1 AST differential GREEN =="
exit $fail
