#!/bin/bash
# stdio-demo/run.sh -- the derived, host-free chain compiles a REAL #include-using stdio
# program and it runs BYTE-IDENTICAL to gcc. Demonstrates that tcc-0.9.27's modern-glibc
# incompatibility (__gnuc_va_list / _Float32) is a HEADER-PROVENANCE issue, not a chain gap:
# given era-appropriate headers, the derived tcc compiles stdio C correctly.
#
#   derived rzcpp (preprocess, era headers)  ->  derived tcc (compile -> .o)  ->  ld (link) -> run
#
# Needs: TCC=<derived tcc.elf>  RZCPP=<derived rzcpp.bin>  (both built by the derived chain).
set -e
HERE=$(cd "$(dirname "$0")" && pwd)
: "${TCC:?set TCC to the derived tcc.elf}" "${RZCPP:?set RZCPP to the derived rzcpp.bin}"
W=$(mktemp -d)
RUN="$HERE/../link/safe-run.sh"
"$RZCPP" -I"$HERE/include" < "$HERE/demo.c" > "$W/demo.i"
if grep -q '__gnuc_va_list\|_Float32' "$W/demo.i"; then echo "FAIL: modern-glibc idiom leaked"; exit 1; fi
"$RUN" "$TCC" -c "$W/demo.i" -o "$W/demo.o"
chmod +r "$W/demo.o"
cc "$W/demo.o" -o "$W/demo.derived"          # final link via system ld (per the founding add(3,4) milestone)
cc -w "$HERE/demo.c" -o "$W/demo.gcc"
set +e
"$W/demo.derived" 3 4 5 6 > "$W/out.derived"; dr=$?
"$W/demo.gcc"     3 4 5 6 > "$W/out.gcc";     gr=$?
set -e
echo "--- derived chain (exit $dr) ---"; cat "$W/out.derived"
if diff -q "$W/out.derived" "$W/out.gcc" >/dev/null && [ "$dr" = "$gr" ]; then
  echo "PASS: derived == gcc (output byte-identical, exit $dr)"
else
  echo "FAIL: divergence (derived exit $dr, gcc exit $gr)"; diff "$W/out.derived" "$W/out.gcc" || true; rm -rf "$W"; exit 1
fi
rm -rf "$W"
