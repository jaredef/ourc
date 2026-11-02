#!/usr/bin/env sh
# Generate the optional clang-oracle baseline body source.
# This is an audit aid only; it is not the final trust path.
set -eu

cd "$(dirname "$0")/.."
BUILD="build/derived-source"
rm -rf "$BUILD"
mkdir -p "$BUILD"

clang -arch arm64 -c src/hex2-oracle.s -o "$BUILD/hex2-oracle.o"
otool -tX "$BUILD/hex2-oracle.o" | node ../macho-hex1/tools/extract-oracle-code.js > "$BUILD/hex2-oracle.code"
node ../macho-hex1/tools/write-labeled-hex.js "$BUILD/hex2-oracle.code" "Mach-O hex2 arm64 clang audit baseline" \
  > derived-macho-hex2.labeled.hex

echo "wrote derived-macho-hex2.labeled.hex"

