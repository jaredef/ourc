#!/usr/bin/env sh
# Build and exercise the arm64 assembly oracle for macho-M0 slice 3.
#
# clang is used only as an authoring calculator here.
set -eu

cd "$(dirname "$0")/.."
BUILD="build/macro-oracle-$$"
rm -rf "$BUILD"
mkdir -p "$BUILD"

clang -arch arm64 -c src/m0-macro-oracle.s -o "$BUILD/m0-macro-oracle.o"
otool -tX "$BUILD/m0-macro-oracle.o" | node ../macho-hex1/tools/extract-oracle-code.js > "$BUILD/m0-macro-oracle.code"

node ../macho-hex0/tools/make-macho-hex0.js \
  --wrap-code "$BUILD/m0-macro-oracle.code" "$BUILD/m0-macro-oracle.macho" \
  "rusteze-macho-m0-macro-oracle-$$"
chmod +x "$BUILD/m0-macro-oracle.macho"
codesign --verify --verbose=4 "$BUILD/m0-macro-oracle.macho" >/dev/null 2>&1

check_fixture() {
  fixture="$1"
  name="$(basename "$fixture" .m0)"
  node tools/expand-macho-m0.js < "$fixture" > "$BUILD/$name.expected"
  "$BUILD/m0-macro-oracle.macho" < "$fixture" > "$BUILD/$name.actual"
  cmp -s "$BUILD/$name.expected" "$BUILD/$name.actual" || {
    echo "macro oracle mismatch for $fixture" >&2
    exit 1
  }
}

check_fixture fixtures/test-token-hex.m0
check_fixture fixtures/test-branch.m0
check_fixture fixtures/test-call.m0
check_fixture fixtures/test-bcond.m0
check_fixture fixtures/test-cbz.m0
check_fixture fixtures/test-bcond-back.m0
check_fixture fixtures/test-layout.m0
check_fixture fixtures/test-macro.m0
check_fixture fixtures/test-macro-raw.m0

"$BUILD/m0-macro-oracle.macho" < fixtures/test-macro.m0 > "$BUILD/test-macro.code"
node ../macho-hex0/tools/make-macho-hex0.js \
  --wrap-code "$BUILD/test-macro.code" "$BUILD/test-macro.macho" \
  "rusteze-macho-m0-macro-test-$$"
chmod +x "$BUILD/test-macro.macho"
codesign --verify --verbose=4 "$BUILD/test-macro.macho" >/dev/null 2>&1
set +e
"$BUILD/test-macro.macho"
status="$?"
set -e
[ "$status" = "42" ] || {
  echo "macro fixture wrapped output exited $status, expected 42" >&2
  exit 1
}

expect_status() {
  source="$1"
  want="$2"
  printf '%s\n' "$source" > "$BUILD/status.m0"
  set +e
  "$BUILD/m0-macro-oracle.macho" < "$BUILD/status.m0" >/dev/null 2>&1
  status="$?"
  set -e
  [ "$status" = "$want" ] || {
    echo "macro oracle '$source' exited $status, expected $want" >&2
    exit 1
  }
}

expect_status 'b.ne' 3
expect_status '40058' 3
expect_status '4005' 0

set +e
"$BUILD/m0-macro-oracle.macho" < fixtures/test-bad-macro-body.m0 >/dev/null 2>&1
status="$?"
set -e
[ "$status" = "3" ] || {
  echo "invalid macro body exited $status, expected 3" >&2
  exit 1
}

echo "macho-M0 macro asm oracle verified"
