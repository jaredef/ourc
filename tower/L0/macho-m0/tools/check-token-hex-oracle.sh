#!/usr/bin/env sh
# Build and exercise the arm64 assembly oracle for macho-M0 slice 1.
#
# clang is used only as an authoring calculator here.
set -eu

cd "$(dirname "$0")/.."
BUILD="build/token-hex-oracle-$$"
rm -rf "$BUILD"
mkdir -p "$BUILD"

clang -arch arm64 -c src/m0-token-hex-oracle.s -o "$BUILD/m0-token-hex-oracle.o"
otool -tX "$BUILD/m0-token-hex-oracle.o" | node ../macho-hex1/tools/extract-oracle-code.js > "$BUILD/m0-token-hex-oracle.code"

node ../macho-hex0/tools/make-macho-hex0.js \
  --wrap-code "$BUILD/m0-token-hex-oracle.code" "$BUILD/m0-token-hex-oracle.macho" \
  "rusteze-macho-m0-token-hex-oracle-$$"
chmod +x "$BUILD/m0-token-hex-oracle.macho"
codesign --verify --verbose=4 "$BUILD/m0-token-hex-oracle.macho" >/dev/null 2>&1

"$BUILD/m0-token-hex-oracle.macho" < fixtures/test-token-hex.m0 > "$BUILD/token-hex.actual"
printf '\xde\xad\xbe\xef' > "$BUILD/token-hex.expected"
cmp -s "$BUILD/token-hex.expected" "$BUILD/token-hex.actual" || {
  echo "token hex oracle output mismatch" >&2
  exit 1
}

expect_status() {
  source="$1"
  want="$2"
  printf '%s\n' "$source" > "$BUILD/status.m0"
  set +e
  "$BUILD/m0-token-hex-oracle.macho" < "$BUILD/status.m0" >/dev/null 2>&1
  status="$?"
  set -e
  [ "$status" = "$want" ] || {
    echo "token hex oracle '$source' exited $status, expected $want" >&2
    exit 1
  }
}

expect_status 'b.ne' 3
expect_status '40058' 3
expect_status '4005' 0

echo "macho-M0 token-hex asm oracle verified"

