#!/usr/bin/env sh
# Build and exercise the arm64 assembly oracle for macho-hex2 semantics.
#
# clang is used only as an authoring calculator here. The trusted path still
# requires hand-audited bytes and self-hosting promotion.
set -eu

cd "$(dirname "$0")/.."
BUILD="build/asm-oracle-$$"
rm -rf "$BUILD"
mkdir -p "$BUILD"

clang -arch arm64 -c src/hex2-oracle.s -o "$BUILD/hex2-oracle.o"
otool -tX "$BUILD/hex2-oracle.o" | node ../macho-hex1/tools/extract-oracle-code.js > "$BUILD/hex2-oracle.code"

node ../macho-hex0/tools/make-macho-hex0.js \
  --wrap-code "$BUILD/hex2-oracle.code" "$BUILD/hex2-oracle.macho" \
  "rusteze-macho-hex2-asm-oracle-$$"
chmod +x "$BUILD/hex2-oracle.macho"
codesign --verify --verbose=4 "$BUILD/hex2-oracle.macho" >/dev/null 2>&1

check_fixture() {
  fixture="$1"
  name="$(basename "$fixture")"
  node tools/expand-macho-hex2.js < "$fixture" > "$BUILD/$name.expected"
  "$BUILD/hex2-oracle.macho" < "$fixture" > "$BUILD/$name.actual"
  if ! cmp -s "$BUILD/$name.expected" "$BUILD/$name.actual"; then
    echo "asm oracle mismatch for $fixture" >&2
    exit 1
  fi
}

check_fixture fixtures/plain.hex2
check_fixture ../macho-hex1/fixtures/test-b.hex1
check_fixture ../macho-hex1/fixtures/test-bl.hex1
check_fixture ../macho-hex1/fixtures/test-b-back.hex1
check_fixture fixtures/test-offset.hex2
check_fixture fixtures/test-address.hex2
check_fixture fixtures/test-address-forward.hex2

echo "macho-hex2 asm oracle verified against semantic oracle"

