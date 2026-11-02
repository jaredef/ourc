#!/usr/bin/env sh
# Materialize the current body source as a signed Mach-O executable and write its
# full hex source.
set -eu

cd "$(dirname "$0")/.."
BUILD="build/materialize-$$"
rm -rf "$BUILD"
mkdir -p "$BUILD"

../macho-hex0/hex0.bin < derived-macho-hex2.labeled.hex > "$BUILD/body.bin"
node ../macho-hex0/tools/make-macho-hex0.js \
  --wrap-code "$BUILD/body.bin" macho-hex2.bin \
  "rusteze-macho-hex2"
chmod +x macho-hex2.bin
node tools/write-labeled-hex.js macho-hex2.bin "Mach-O arm64 hex2 executable" \
  > macho-hex2.hand.hex
node tools/measure-artifact.js "$BUILD/body.bin" macho-hex2.bin macho-hex2.hand.hex \
  > measurement.md

echo "wrote macho-hex2.bin, macho-hex2.hand.hex, and measurement.md"

