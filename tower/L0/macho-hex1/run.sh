#!/usr/bin/env sh
# Mach-O hex1 verification.
#
# Verifies the Apple Silicon hex1 seed: hex0 conservativity, arm64 symbolic
# branch references, signed fixture execution, derived-body behavior, and
# self-hosting from macho-hex1.hand.hex.
set -eu

cd "$(dirname "$0")"
BUILD_DIR="build/run-$$"
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"

for fixture in fixtures/*.hex1; do
  case "$(basename "$fixture")" in
    bad-*) continue ;;
  esac
  node tools/check-fixture-integrity.js "$fixture"
done

for fixture in fixtures/bad-*.hex1; do
  if node tools/check-fixture-integrity.js "$fixture" >/dev/null 2>&1; then
    echo "$fixture passed integrity check but should fail" >&2
    exit 1
  fi
done

chmod +x ../macho-hex0/hex0.bin
../macho-hex0/hex0.bin < fixtures/plain.hex1 > "$BUILD_DIR/plain.hex0.bin"
node tools/expand-macho-hex1.js < fixtures/plain.hex1 > "$BUILD_DIR/plain.hex1.bin"
if ! cmp -s "$BUILD_DIR/plain.hex0.bin" "$BUILD_DIR/plain.hex1.bin"; then
  echo "macho-hex1 plain hex output differs from macho-hex0" >&2
  exit 1
fi

node tools/check-arm64-bref.js fixtures/test-b.hex1
node tools/check-arm64-bref.js fixtures/test-bl.hex1
node tools/check-arm64-bref.js fixtures/test-b-back.hex1
node tools/check-arm64-bref.js fixtures/test-bl-back.hex1
node tools/expand-macho-hex1.js < fixtures/test-b.hex1 > "$BUILD_DIR/test-b.oracle.bin"
node tools/expand-macho-hex1.js < fixtures/test-bl.hex1 > "$BUILD_DIR/test-bl.oracle.bin"
node tools/expand-macho-hex1.js < fixtures/test-b-back.hex1 > "$BUILD_DIR/test-b-back.oracle.bin"
node tools/expand-macho-hex1.js < fixtures/test-bl-back.hex1 > "$BUILD_DIR/test-bl-back.oracle.bin"
if ! cmp -s build/test-b.bin "$BUILD_DIR/test-b.oracle.bin"; then
  echo "test-b oracle output differs from reference checker" >&2
  exit 1
fi
if ! cmp -s build/test-bl.bin "$BUILD_DIR/test-bl.oracle.bin"; then
  echo "test-bl oracle output differs from reference checker" >&2
  exit 1
fi
if ! cmp -s build/test-b-back.bin "$BUILD_DIR/test-b-back.oracle.bin"; then
  echo "test-b-back oracle output differs from reference checker" >&2
  exit 1
fi
if ! cmp -s build/test-bl-back.bin "$BUILD_DIR/test-bl-back.oracle.bin"; then
  echo "test-bl-back oracle output differs from reference checker" >&2
  exit 1
fi

WRAP=../macho-hex0/tools/make-macho-hex0.js

run_code_fixture() {
  name="$1"
  node "$WRAP" --wrap-code "build/$name.bin" "$BUILD_DIR/$name.macho" "rusteze-macho-hex1-$name-$$"
  chmod +x "$BUILD_DIR/$name.macho"
  codesign --verify --verbose=4 "$BUILD_DIR/$name.macho" >/dev/null 2>&1
  set +e
  "$BUILD_DIR/$name.macho"
  status="$?"
  set -e
  if [ "$status" != "42" ]; then
    echo "$name Mach-O fixture exited $status, expected 42" >&2
    exit 1
  fi
}

run_code_fixture test-b
run_code_fixture test-bl
run_code_fixture test-b-back
run_code_fixture test-bl-back

../macho-hex0/hex0.bin < derived-macho-hex1.labeled.hex > "$BUILD_DIR/derived-body.bin"
node "$WRAP" --wrap-code "$BUILD_DIR/derived-body.bin" "$BUILD_DIR/derived-body.macho" \
  "rusteze-macho-hex1-derived-body-$$"
chmod +x "$BUILD_DIR/derived-body.macho"
codesign --verify --verbose=4 "$BUILD_DIR/derived-body.macho" >/dev/null 2>&1

check_derived_body() {
  fixture="$1"
  name="$(basename "$fixture" .hex1)"
  node tools/expand-macho-hex1.js < "$fixture" > "$BUILD_DIR/$name.derived.expected"
  "$BUILD_DIR/derived-body.macho" < "$fixture" > "$BUILD_DIR/$name.derived.actual"
  if ! cmp -s "$BUILD_DIR/$name.derived.expected" "$BUILD_DIR/$name.derived.actual"; then
    echo "derived body mismatch for $fixture" >&2
    exit 1
  fi
}

check_derived_body fixtures/plain.hex1
check_derived_body fixtures/test-b.hex1
check_derived_body fixtures/test-bl.hex1
check_derived_body fixtures/test-b-back.hex1
check_derived_body fixtures/test-bl-back.hex1

if [ -x macho-hex1.bin ] && [ -f macho-hex1.hand.hex ]; then
  ./macho-hex1.bin < macho-hex1.hand.hex > "$BUILD_DIR/self.bin"
  if ! cmp -s macho-hex1.bin "$BUILD_DIR/self.bin"; then
    echo "macho-hex1.bin does not rebuild itself from macho-hex1.hand.hex" >&2
    exit 1
  fi

  ./macho-hex1.bin < fixtures/plain.hex1 > "$BUILD_DIR/plain.self.bin"
  if ! cmp -s "$BUILD_DIR/plain.hex0.bin" "$BUILD_DIR/plain.self.bin"; then
    echo "macho-hex1.bin plain hex output differs from macho-hex0" >&2
    exit 1
  fi

  ./macho-hex1.bin < fixtures/test-b.hex1 > "$BUILD_DIR/test-b.self.bin"
  ./macho-hex1.bin < fixtures/test-bl.hex1 > "$BUILD_DIR/test-bl.self.bin"
  ./macho-hex1.bin < fixtures/test-b-back.hex1 > "$BUILD_DIR/test-b-back.self.bin"
  ./macho-hex1.bin < fixtures/test-bl-back.hex1 > "$BUILD_DIR/test-bl-back.self.bin"
  if ! cmp -s build/test-b.bin "$BUILD_DIR/test-b.self.bin"; then
    echo "macho-hex1.bin @ fixture output differs from reference" >&2
    exit 1
  fi
  if ! cmp -s build/test-bl.bin "$BUILD_DIR/test-bl.self.bin"; then
    echo "macho-hex1.bin ! fixture output differs from reference" >&2
    exit 1
  fi
  if ! cmp -s build/test-b-back.bin "$BUILD_DIR/test-b-back.self.bin"; then
    echo "macho-hex1.bin backward @ fixture output differs from reference" >&2
    exit 1
  fi
  if ! cmp -s build/test-bl-back.bin "$BUILD_DIR/test-bl-back.self.bin"; then
    echo "macho-hex1.bin backward ! fixture output differs from reference" >&2
    exit 1
  fi
  self_status="self-hosting seed verified"
else
  self_status="self-hosting seed not materialized"
fi

echo "macho-hex1 reference fixtures verified inside signed arm64 Mach-O"
echo "macho-hex1 derived body verified against semantic oracle"
echo "macho-hex1 $self_status"
