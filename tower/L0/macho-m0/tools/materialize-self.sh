#!/usr/bin/env sh
# Materialize the current body source as a signed Mach-O executable and write its
# full hex source.
set -eu

cd "$(dirname "$0")/.."
BUILD="build/materialize-$$"
rm -rf "$BUILD"
mkdir -p "$BUILD"

../macho-hex0/hex0.bin < derived-macho-m0.labeled.hex > "$BUILD/body.bin"
node ../macho-hex0/tools/make-macho-hex0.js \
  --wrap-code "$BUILD/body.bin" macho-m0.bin \
  "rusteze-macho-m0"
chmod +x macho-m0.bin
node tools/write-labeled-hex.js macho-m0.bin "Mach-O arm64 M0 executable" \
  > macho-m0.hand.hex
node tools/measure-artifact.js "$BUILD/body.bin" macho-m0.bin macho-m0.hand.hex \
  > measurement.md

echo "wrote macho-m0.bin, macho-m0.hand.hex, and measurement.md"

