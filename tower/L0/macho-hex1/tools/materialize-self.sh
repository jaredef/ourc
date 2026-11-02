#!/usr/bin/env sh
# Materialize the current body source as a signed Mach-O executable and write its
# full hex source.
set -eu

cd "$(dirname "$0")/.."
BUILD="build/materialize-$$"
rm -rf "$BUILD"
mkdir -p "$BUILD"

../macho-hex0/hex0.bin < derived-macho-hex1.labeled.hex > "$BUILD/body.bin"
node ../macho-hex0/tools/make-macho-hex0.js \
  --wrap-code "$BUILD/body.bin" macho-hex1.bin \
  "rusteze-macho-hex1"
chmod +x macho-hex1.bin
node tools/write-labeled-hex.js macho-hex1.bin "Mach-O arm64 hex1 executable" \
  > macho-hex1.hand.hex
node tools/measure-artifact.js "$BUILD/body.bin" macho-hex1.bin macho-hex1.hand.hex \
  > measurement.md

echo "wrote macho-hex1.bin, macho-hex1.hand.hex, and measurement.md"
