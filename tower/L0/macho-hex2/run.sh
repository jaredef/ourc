#!/usr/bin/env sh
# Mach-O hex2 semantic proof.
#
# This verifies the target contract before promotion into a hand-audited arm64
# body seed.
set -eu

cd "$(dirname "$0")"
BUILD_DIR="build/run-$$"
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"

for fixture in fixtures/*.hex2; do
  case "$(basename "$fixture")" in
    bad-*) continue ;;
  esac
  node tools/check-fixture-integrity.js "$fixture"
done

for fixture in fixtures/bad-*.hex2; do
  if node tools/check-fixture-integrity.js "$fixture" >/dev/null 2>&1; then
    echo "$fixture passed integrity check but should fail" >&2
    exit 1
  fi
done

chmod +x ../macho-hex0/hex0.bin
../macho-hex0/hex0.bin < fixtures/plain.hex2 > "$BUILD_DIR/plain.hex0.bin"
node tools/expand-macho-hex2.js < fixtures/plain.hex2 > "$BUILD_DIR/plain.hex2.bin"
cmp -s "$BUILD_DIR/plain.hex0.bin" "$BUILD_DIR/plain.hex2.bin" || {
  echo "macho-hex2 plain hex output differs from macho-hex0" >&2
  exit 1
}

node tools/expand-macho-hex2.js < ../macho-hex1/fixtures/test-b.hex1 > "$BUILD_DIR/test-b.hex2.bin"
node ../macho-hex1/tools/expand-macho-hex1.js < ../macho-hex1/fixtures/test-b.hex1 > "$BUILD_DIR/test-b.hex1.bin"
cmp -s "$BUILD_DIR/test-b.hex1.bin" "$BUILD_DIR/test-b.hex2.bin" || {
  echo "macho-hex2 @ output differs from macho-hex1" >&2
  exit 1
}

node tools/expand-macho-hex2.js < ../macho-hex1/fixtures/test-bl.hex1 > "$BUILD_DIR/test-bl.hex2.bin"
node ../macho-hex1/tools/expand-macho-hex1.js < ../macho-hex1/fixtures/test-bl.hex1 > "$BUILD_DIR/test-bl.hex1.bin"
cmp -s "$BUILD_DIR/test-bl.hex1.bin" "$BUILD_DIR/test-bl.hex2.bin" || {
  echo "macho-hex2 ! output differs from macho-hex1" >&2
  exit 1
}

node tools/expand-macho-hex2.js < ../macho-hex1/fixtures/test-b-back.hex1 > "$BUILD_DIR/test-b-back.hex2.bin"
node ../macho-hex1/tools/expand-macho-hex1.js < ../macho-hex1/fixtures/test-b-back.hex1 > "$BUILD_DIR/test-b-back.hex1.bin"
cmp -s "$BUILD_DIR/test-b-back.hex1.bin" "$BUILD_DIR/test-b-back.hex2.bin" || {
  echo "macho-hex2 backward @ output differs from macho-hex1" >&2
  exit 1
}

node tools/expand-macho-hex2.js < ../macho-hex1/fixtures/test-bl-back.hex1 > "$BUILD_DIR/test-bl-back.hex2.bin"
node ../macho-hex1/tools/expand-macho-hex1.js < ../macho-hex1/fixtures/test-bl-back.hex1 > "$BUILD_DIR/test-bl-back.hex1.bin"
cmp -s "$BUILD_DIR/test-bl-back.hex1.bin" "$BUILD_DIR/test-bl-back.hex2.bin" || {
  echo "macho-hex2 backward ! output differs from macho-hex1" >&2
  exit 1
}

node tools/expand-macho-hex2.js < fixtures/test-offset.hex2 > "$BUILD_DIR/test-offset.bin"
printf '\x00\xaa\x01\x00\x00\x00\x06\x00\x00\x00' > "$BUILD_DIR/test-offset.expected"
cmp -s "$BUILD_DIR/test-offset.expected" "$BUILD_DIR/test-offset.bin" || {
  echo "macho-hex2 & fixture output mismatch" >&2
  exit 1
}

node tools/expand-macho-hex2.js < fixtures/test-offset-forward.hex2 > "$BUILD_DIR/test-offset-forward.bin"
printf '\x05\x00\x00\x00\x00' > "$BUILD_DIR/test-offset-forward.expected"
cmp -s "$BUILD_DIR/test-offset-forward.expected" "$BUILD_DIR/test-offset-forward.bin" || {
  echo "macho-hex2 forward & fixture output mismatch" >&2
  exit 1
}

node tools/expand-macho-hex2.js < fixtures/test-address.hex2 > "$BUILD_DIR/test-address.bin"
printf '\xb0\x02\x00\x00\x01\x00\x00\x00' > "$BUILD_DIR/test-address.expected"
cmp -s "$BUILD_DIR/test-address.expected" "$BUILD_DIR/test-address.bin" || {
  echo "macho-hex2 = fixture output mismatch" >&2
  exit 1
}

node tools/expand-macho-hex2.js < fixtures/test-address-forward.hex2 > "$BUILD_DIR/test-address-forward.bin"
printf '\xb9\x02\x00\x00\x01\x00\x00\x00\x00' > "$BUILD_DIR/test-address-forward.expected"
cmp -s "$BUILD_DIR/test-address-forward.expected" "$BUILD_DIR/test-address-forward.bin" || {
  echo "macho-hex2 forward = fixture output mismatch" >&2
  exit 1
}

if [ -f derived-macho-hex2.labeled.hex ]; then
  ../macho-hex0/hex0.bin < derived-macho-hex2.labeled.hex > "$BUILD_DIR/derived-body.bin"
  node ../macho-hex0/tools/make-macho-hex0.js \
    --wrap-code "$BUILD_DIR/derived-body.bin" "$BUILD_DIR/derived-body.macho" \
    "rusteze-macho-hex2-derived-body-$$"
  chmod +x "$BUILD_DIR/derived-body.macho"
  codesign --verify --verbose=4 "$BUILD_DIR/derived-body.macho" >/dev/null 2>&1

  check_derived_body() {
    fixture="$1"
    name="$(basename "$fixture")"
    node tools/expand-macho-hex2.js < "$fixture" > "$BUILD_DIR/$name.derived.expected"
    "$BUILD_DIR/derived-body.macho" < "$fixture" > "$BUILD_DIR/$name.derived.actual"
    cmp -s "$BUILD_DIR/$name.derived.expected" "$BUILD_DIR/$name.derived.actual" || {
      echo "derived body mismatch for $fixture" >&2
      exit 1
    }
  }

  check_derived_body fixtures/plain.hex2
  check_derived_body ../macho-hex1/fixtures/test-b.hex1
  check_derived_body ../macho-hex1/fixtures/test-bl.hex1
  check_derived_body ../macho-hex1/fixtures/test-b-back.hex1
  check_derived_body ../macho-hex1/fixtures/test-bl-back.hex1
  check_derived_body fixtures/test-offset.hex2
  check_derived_body fixtures/test-offset-forward.hex2
  check_derived_body fixtures/test-address.hex2
  check_derived_body fixtures/test-address-forward.hex2
  body_status="derived body verified against semantic oracle"
else
  body_status="derived body not materialized"
fi

if [ -x macho-hex2.bin ] && [ -f macho-hex2.hand.hex ]; then
  ./macho-hex2.bin < macho-hex2.hand.hex > "$BUILD_DIR/self.bin"
  cmp -s macho-hex2.bin "$BUILD_DIR/self.bin" || {
    echo "macho-hex2.bin does not rebuild itself from macho-hex2.hand.hex" >&2
    exit 1
  }

  check_self_fixture() {
    fixture="$1"
    name="$(basename "$fixture")"
    node tools/expand-macho-hex2.js < "$fixture" > "$BUILD_DIR/$name.self.expected"
    ./macho-hex2.bin < "$fixture" > "$BUILD_DIR/$name.self.actual"
    cmp -s "$BUILD_DIR/$name.self.expected" "$BUILD_DIR/$name.self.actual" || {
      echo "self-hosted macho-hex2 mismatch for $fixture" >&2
      exit 1
    }
  }

  check_self_fixture fixtures/plain.hex2
  check_self_fixture ../macho-hex1/fixtures/test-b.hex1
  check_self_fixture ../macho-hex1/fixtures/test-bl.hex1
  check_self_fixture ../macho-hex1/fixtures/test-b-back.hex1
  check_self_fixture ../macho-hex1/fixtures/test-bl-back.hex1
  check_self_fixture fixtures/test-offset.hex2
  check_self_fixture fixtures/test-offset-forward.hex2
  check_self_fixture fixtures/test-address.hex2
  check_self_fixture fixtures/test-address-forward.hex2
  self_status="self-hosting seed verified"
else
  self_status="self-hosting seed not materialized"
fi

echo "macho-hex2 semantic contract verified: hex1-compatible, resolves & offsets and = preferred addresses"
echo "macho-hex2 $body_status"
echo "macho-hex2 $self_status"
