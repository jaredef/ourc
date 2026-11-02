#!/usr/bin/env sh
# Build and exercise the arm64 assembly oracle for macho-M0 slice 2.
#
# clang is used only as an authoring calculator here.
set -eu

cd "$(dirname "$0")/.."
BUILD="build/label-ref-oracle-$$"
rm -rf "$BUILD"
mkdir -p "$BUILD"

clang -arch arm64 -c src/m0-label-ref-oracle.s -o "$BUILD/m0-label-ref-oracle.o"
otool -tX "$BUILD/m0-label-ref-oracle.o" | node ../macho-hex1/tools/extract-oracle-code.js > "$BUILD/m0-label-ref-oracle.code"

node ../macho-hex0/tools/make-macho-hex0.js \
  --wrap-code "$BUILD/m0-label-ref-oracle.code" "$BUILD/m0-label-ref-oracle.macho" \
  "rusteze-macho-m0-label-ref-oracle-$$"
chmod +x "$BUILD/m0-label-ref-oracle.macho"
codesign --verify --verbose=4 "$BUILD/m0-label-ref-oracle.macho" >/dev/null 2>&1

check_fixture() {
  fixture="$1"
  name="$(basename "$fixture" .m0)"
  node tools/expand-macho-m0.js < "$fixture" > "$BUILD/$name.expected"
  "$BUILD/m0-label-ref-oracle.macho" < "$fixture" > "$BUILD/$name.actual"
  cmp -s "$BUILD/$name.expected" "$BUILD/$name.actual" || {
    echo "label-ref oracle mismatch for $fixture" >&2
    exit 1
  }
}

check_fixture fixtures/test-token-hex.m0
check_fixture fixtures/test-branch.m0
check_fixture fixtures/test-call.m0
check_fixture fixtures/test-layout.m0

run_fixture() {
  fixture="$1"
  name="$(basename "$fixture" .m0)"
  "$BUILD/m0-label-ref-oracle.macho" < "$fixture" > "$BUILD/$name.code"
  node ../macho-hex0/tools/make-macho-hex0.js \
    --wrap-code "$BUILD/$name.code" "$BUILD/$name.macho" \
    "rusteze-macho-m0-label-ref-$name-$$"
  chmod +x "$BUILD/$name.macho"
  codesign --verify --verbose=4 "$BUILD/$name.macho" >/dev/null 2>&1
  set +e
  "$BUILD/$name.macho"
  status="$?"
  set -e
  [ "$status" = "42" ] || {
    echo "$fixture wrapped output exited $status, expected 42" >&2
    exit 1
  }
}

run_fixture fixtures/test-branch.m0
run_fixture fixtures/test-call.m0

expect_status() {
  source="$1"
  want="$2"
  printf '%s\n' "$source" > "$BUILD/status.m0"
  set +e
  "$BUILD/m0-label-ref-oracle.macho" < "$BUILD/status.m0" >/dev/null 2>&1
  status="$?"
  set -e
  [ "$status" = "$want" ] || {
    echo "label-ref oracle '$source' exited $status, expected $want" >&2
    exit 1
  }
}

expect_status 'b.ne' 3
expect_status '40058' 3
expect_status '4005' 0

echo "macho-M0 label-ref asm oracle verified"

