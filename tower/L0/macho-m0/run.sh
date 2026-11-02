#!/usr/bin/env sh
# Mach-O M0 semantic proof.
set -eu

cd "$(dirname "$0")"
BUILD_DIR="build/run-$$"
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"

node tools/expand-macho-m0.js < fixtures/test-token-hex.m0 > "$BUILD_DIR/test-token-hex.bin"
printf '\xde\xad\xbe\xef' > "$BUILD_DIR/test-token-hex.expected"
cmp -s "$BUILD_DIR/test-token-hex.expected" "$BUILD_DIR/test-token-hex.bin" || {
  echo "macho-M0 token hex fixture mismatch" >&2
  exit 1
}

node tools/expand-macho-m0.js < fixtures/test-branch.m0 > "$BUILD_DIR/test-branch.bin"
printf '\x04\x00\x00\x14\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x40\x05\x80\xd2\x30\x00\x80\xd2\x01\x10\x00\xd4' > "$BUILD_DIR/test-branch.expected"
cmp -s "$BUILD_DIR/test-branch.expected" "$BUILD_DIR/test-branch.bin" || {
  echo "macho-M0 @ branch fixture mismatch" >&2
  exit 1
}

node tools/expand-macho-m0.js < fixtures/test-call.m0 > "$BUILD_DIR/test-call.bin"
printf '\x04\x00\x00\x94\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x40\x05\x80\xd2\x30\x00\x80\xd2\x01\x10\x00\xd4' > "$BUILD_DIR/test-call.expected"
cmp -s "$BUILD_DIR/test-call.expected" "$BUILD_DIR/test-call.bin" || {
  echo "macho-M0 ! call fixture mismatch" >&2
  exit 1
}

node tools/expand-macho-m0.js < fixtures/test-bcond.m0 > "$BUILD_DIR/test-bcond.bin"
printf '\x80\x00\x00\x54\x20\x00\x80\xd2\x30\x00\x80\xd2\x01\x10\x00\xd4\x40\x05\x80\xd2\x30\x00\x80\xd2\x01\x10\x00\xd4' > "$BUILD_DIR/test-bcond.expected"
cmp -s "$BUILD_DIR/test-bcond.expected" "$BUILD_DIR/test-bcond.bin" || {
  echo "macho-M0 ? b.cond fixture mismatch" >&2
  exit 1
}

node tools/expand-macho-m0.js < fixtures/test-cbz.m0 > "$BUILD_DIR/test-cbz.bin"
printf '\x80\x00\x00\x34\x20\x00\x80\xd2\x30\x00\x80\xd2\x01\x10\x00\xd4\x40\x05\x80\xd2\x30\x00\x80\xd2\x01\x10\x00\xd4' > "$BUILD_DIR/test-cbz.expected"
cmp -s "$BUILD_DIR/test-cbz.expected" "$BUILD_DIR/test-cbz.bin" || {
  echo "macho-M0 ? cbz fixture mismatch" >&2
  exit 1
}

node tools/expand-macho-m0.js < fixtures/test-bcond-back.m0 > "$BUILD_DIR/test-bcond-back.bin"
printf '\x40\x05\x80\xd2\x30\x00\x80\xd2\x01\x10\x00\xd4\x20\x00\x80\xd2\x30\x00\x80\xd2\x01\x10\x00\xd4\x40\xff\xff\x54' > "$BUILD_DIR/test-bcond-back.expected"
cmp -s "$BUILD_DIR/test-bcond-back.expected" "$BUILD_DIR/test-bcond-back.bin" || {
  echo "macho-M0 ? backward b.cond fixture mismatch" >&2
  exit 1
}

node tools/expand-macho-m0.js < fixtures/test-layout.m0 > "$BUILD_DIR/test-layout.bin"
printf '\x00\xaa\x01\x00\x00\x00\x06\x00\x00\x00\xb1\x02\x00\x00\x01\x00\x00\x00' > "$BUILD_DIR/test-layout.expected"
cmp -s "$BUILD_DIR/test-layout.expected" "$BUILD_DIR/test-layout.bin" || {
  echo "macho-M0 layout fixture mismatch" >&2
  exit 1
}

node tools/expand-macho-m0.js < fixtures/test-macro.m0 > "$BUILD_DIR/test-macro.bin"
node tools/expand-macho-m0.js < fixtures/test-macro-raw.m0 > "$BUILD_DIR/test-macro-raw.bin"
cmp -s "$BUILD_DIR/test-macro-raw.bin" "$BUILD_DIR/test-macro.bin" || {
  echo "macho-M0 macro output differs from raw hex" >&2
  exit 1
}

node ../macho-hex0/tools/make-macho-hex0.js \
  --wrap-code "$BUILD_DIR/test-branch.bin" "$BUILD_DIR/test-branch.macho" \
  "rusteze-macho-m0-branch-$$"
node ../macho-hex0/tools/make-macho-hex0.js \
  --wrap-code "$BUILD_DIR/test-call.bin" "$BUILD_DIR/test-call.macho" \
  "rusteze-macho-m0-call-$$"
node ../macho-hex0/tools/make-macho-hex0.js \
  --wrap-code "$BUILD_DIR/test-macro.bin" "$BUILD_DIR/test-macro.macho" \
  "rusteze-macho-m0-macro-$$"
chmod +x "$BUILD_DIR/test-branch.macho" "$BUILD_DIR/test-call.macho" "$BUILD_DIR/test-macro.macho"
codesign --verify --verbose=4 "$BUILD_DIR/test-branch.macho" >/dev/null 2>&1
codesign --verify --verbose=4 "$BUILD_DIR/test-call.macho" >/dev/null 2>&1
codesign --verify --verbose=4 "$BUILD_DIR/test-macro.macho" >/dev/null 2>&1

for exe in "$BUILD_DIR/test-branch.macho" "$BUILD_DIR/test-call.macho" "$BUILD_DIR/test-macro.macho"; do
  set +e
  "$exe"
  status="$?"
  set -e
  if [ "$status" != "42" ]; then
    echo "$exe exited $status, expected 42" >&2
    exit 1
  fi
done

err() {
  printf '%s\n' "$1" > "$BUILD_DIR/t.m0"
  set +e
  node tools/expand-macho-m0.js < "$BUILD_DIR/t.m0" >/dev/null 2>&1
  status="$?"
  set -e
  [ "$status" = "$2" ] || {
    echo "M0 reject-unknown: '$1' exited $status, expected $2" >&2
    exit 1
  }
}

err 'b.ne' 3
err '40058' 3
err '4005' 0
set +e
node tools/expand-macho-m0.js < fixtures/test-bad-macro-body.m0 >/dev/null 2>&1
status="$?"
set -e
[ "$status" = "1" ] || {
  echo "invalid macro body exited $status, expected 1" >&2
  exit 1
}

if [ -f derived-macho-m0.labeled.hex ]; then
  ../macho-hex0/hex0.bin < derived-macho-m0.labeled.hex > "$BUILD_DIR/derived-body.bin"
  node ../macho-hex0/tools/make-macho-hex0.js \
    --wrap-code "$BUILD_DIR/derived-body.bin" "$BUILD_DIR/derived-body.macho" \
    "rusteze-macho-m0-derived-body-$$"
  chmod +x "$BUILD_DIR/derived-body.macho"
  codesign --verify --verbose=4 "$BUILD_DIR/derived-body.macho" >/dev/null 2>&1

  check_derived_body() {
    fixture="$1"
    name="$(basename "$fixture" .m0)"
    node tools/expand-macho-m0.js < "$fixture" > "$BUILD_DIR/$name.derived.expected"
    "$BUILD_DIR/derived-body.macho" < "$fixture" > "$BUILD_DIR/$name.derived.actual"
    cmp -s "$BUILD_DIR/$name.derived.expected" "$BUILD_DIR/$name.derived.actual" || {
      echo "derived M0 body mismatch for $fixture" >&2
      exit 1
    }
  }

  check_derived_body fixtures/test-token-hex.m0
  check_derived_body fixtures/test-branch.m0
  check_derived_body fixtures/test-call.m0
  check_derived_body fixtures/test-bcond.m0
  check_derived_body fixtures/test-cbz.m0
  check_derived_body fixtures/test-bcond-back.m0
  check_derived_body fixtures/test-layout.m0
  check_derived_body fixtures/test-macro.m0
  check_derived_body fixtures/test-macro-raw.m0

  "$BUILD_DIR/derived-body.macho" < fixtures/test-macro.m0 > "$BUILD_DIR/derived-macro.code"
  node ../macho-hex0/tools/make-macho-hex0.js \
    --wrap-code "$BUILD_DIR/derived-macro.code" "$BUILD_DIR/derived-macro.macho" \
    "rusteze-macho-m0-derived-macro-$$"
  chmod +x "$BUILD_DIR/derived-macro.macho"
  codesign --verify --verbose=4 "$BUILD_DIR/derived-macro.macho" >/dev/null 2>&1
  set +e
  "$BUILD_DIR/derived-macro.macho"
  status="$?"
  set -e
  [ "$status" = "42" ] || {
    echo "derived M0 macro fixture exited $status, expected 42" >&2
    exit 1
  }

  body_status="derived body verified against semantic oracle"
else
  body_status="derived body not materialized"
fi

if [ -x macho-m0.bin ] && [ -f macho-m0.hand.hex ]; then
  ./macho-m0.bin < macho-m0.hand.hex > "$BUILD_DIR/self.bin"
  cmp -s macho-m0.bin "$BUILD_DIR/self.bin" || {
    echo "macho-m0.bin does not rebuild itself from macho-m0.hand.hex" >&2
    exit 1
  }

  check_self_fixture() {
    fixture="$1"
    name="$(basename "$fixture" .m0)"
    node tools/expand-macho-m0.js < "$fixture" > "$BUILD_DIR/$name.self.expected"
    ./macho-m0.bin < "$fixture" > "$BUILD_DIR/$name.self.actual"
    cmp -s "$BUILD_DIR/$name.self.expected" "$BUILD_DIR/$name.self.actual" || {
      echo "self-hosted M0 mismatch for $fixture" >&2
      exit 1
    }
  }

  check_self_fixture fixtures/test-token-hex.m0
  check_self_fixture fixtures/test-branch.m0
  check_self_fixture fixtures/test-call.m0
  check_self_fixture fixtures/test-bcond.m0
  check_self_fixture fixtures/test-cbz.m0
  check_self_fixture fixtures/test-bcond-back.m0
  check_self_fixture fixtures/test-layout.m0
  check_self_fixture fixtures/test-macro.m0
  check_self_fixture fixtures/test-macro-raw.m0

  ./macho-m0.bin < fixtures/test-macro.m0 > "$BUILD_DIR/self-macro.code"
  node ../macho-hex0/tools/make-macho-hex0.js \
    --wrap-code "$BUILD_DIR/self-macro.code" "$BUILD_DIR/self-macro.macho" \
    "rusteze-macho-m0-self-macro-$$"
  chmod +x "$BUILD_DIR/self-macro.macho"
  codesign --verify --verbose=4 "$BUILD_DIR/self-macro.macho" >/dev/null 2>&1
  set +e
  "$BUILD_DIR/self-macro.macho"
  status="$?"
  set -e
  [ "$status" = "42" ] || {
    echo "self-hosted M0 macro fixture exited $status, expected 42" >&2
    exit 1
  }

  self_status="self-hosting seed verified"
else
  self_status="self-hosting seed not materialized"
fi

echo "macho-M0 semantic contract verified: multi-char labels, @/!/&/= refs, macros, reject-unknown"
echo "macho-M0 $body_status"
echo "macho-M0 $self_status"
