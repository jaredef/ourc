#!/usr/bin/env sh
# Build and exercise the arm64 assembly oracle for macho-hex1 semantics.
#
# This uses clang as an authoring oracle only. It is deliberately separate from
# run.sh and from the chain proof; the trusted seed still has to be hand-encoded
# and self-hosted later.
set -eu

cd "$(dirname "$0")/.."
BUILD="build/asm-oracle-$$"
rm -rf "$BUILD"
mkdir -p "$BUILD"

clang -arch arm64 -c src/hex1-oracle.s -o "$BUILD/hex1-oracle.o"
otool -tX "$BUILD/hex1-oracle.o" | node tools/extract-oracle-code.js > "$BUILD/hex1-oracle.code"

node ../macho-hex0/tools/make-macho-hex0.js \
  --wrap-code "$BUILD/hex1-oracle.code" "$BUILD/hex1-oracle.macho" \
  "rusteze-macho-hex1-asm-oracle-$$"
chmod +x "$BUILD/hex1-oracle.macho"
codesign --verify --verbose=4 "$BUILD/hex1-oracle.macho" >/dev/null 2>&1

check_fixture() {
  fixture="$1"
  name="$(basename "$fixture" .hex1)"
  node tools/expand-macho-hex1.js < "$fixture" > "$BUILD/$name.expected"
  "$BUILD/hex1-oracle.macho" < "$fixture" > "$BUILD/$name.actual"
  if ! cmp -s "$BUILD/$name.expected" "$BUILD/$name.actual"; then
    echo "asm oracle mismatch for $fixture" >&2
    exit 1
  fi
}

check_fixture fixtures/plain.hex1
check_fixture fixtures/test-b.hex1
check_fixture fixtures/test-bl.hex1

echo "macho-hex1 asm oracle verified against semantic oracle"
