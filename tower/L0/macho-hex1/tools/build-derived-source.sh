#!/usr/bin/env sh
# Generate the optional clang-oracle baseline body source.
# This is an audit aid only; it does not overwrite the audited seed source.
set -eu

cd "$(dirname "$0")/.."
BUILD="build/derived-source"
rm -rf "$BUILD"
mkdir -p "$BUILD"

clang -arch arm64 -c src/hex1-oracle.s -o "$BUILD/hex1-oracle.o"
otool -tX "$BUILD/hex1-oracle.o" | node tools/extract-oracle-code.js > "$BUILD/hex1-oracle.code"
node tools/write-labeled-hex.js "$BUILD/hex1-oracle.code" "Mach-O hex1 arm64 clang audit baseline" \
  > "$BUILD/clang-baseline.labeled.hex"

echo "wrote $BUILD/clang-baseline.labeled.hex"
