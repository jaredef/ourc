#!/usr/bin/env sh
# Generate the optional clang-oracle baseline body source for the current
# macho-M0 macro slice.
set -eu

cd "$(dirname "$0")/.."
BUILD="build/derived-source"
rm -rf "$BUILD"
mkdir -p "$BUILD"

clang -arch arm64 -c src/m0-macro-oracle.s -o "$BUILD/m0-macro-oracle.o"
otool -tX "$BUILD/m0-macro-oracle.o" | node ../macho-hex1/tools/extract-oracle-code.js > "$BUILD/m0-macro-oracle.code"
node ../macho-hex1/tools/write-labeled-hex.js "$BUILD/m0-macro-oracle.code" "Mach-O M0 arm64 macro-slice audit baseline" \
  > derived-macho-m0.labeled.hex

echo "wrote derived-macho-m0.labeled.hex"

