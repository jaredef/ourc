#!/usr/bin/env sh
# rzir -> Mach-O emitter semantic proof.
set -eu

cd "$(dirname "$0")"
BUILD_DIR="build/run-$$"
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"

FIXTURES="../../../locales/stage3-ir-backend/fixtures"

node tools/check-flat-status.js

RESOLVER_BODY="$BUILD_DIR/status-resolver-flat.body"
RESOLVER_ORACLE_O="$BUILD_DIR/status-resolver-flat.o"
RESOLVER_ORACLE_BODY="$BUILD_DIR/status-resolver-flat.oracle-body"
RESOLVER_MACHO="$BUILD_DIR/status-resolver-flat.macho"
PLAN_EMITTER_ORACLE_O="$BUILD_DIR/compute-plan-flat.o"
PLAN_EMITTER_ORACLE_BODY="$BUILD_DIR/compute-plan-flat.oracle-body"
PLAN_EMITTER_BODY="$BUILD_DIR/compute-plan-flat.body"
PLAN_EMITTER_MACHO="$BUILD_DIR/compute-plan-flat.macho"
clang -arch arm64 -c src/status-resolver-flat.s -o "$RESOLVER_ORACLE_O"
node tools/text-from-object.js "$RESOLVER_ORACLE_O" > "$RESOLVER_ORACLE_BODY"
../macho-m0/macho-m0.bin < status-resolver/status-resolver-flat.m0 > "$RESOLVER_BODY"
cmp -s "$RESOLVER_ORACLE_BODY" "$RESOLVER_BODY" || {
  echo "M0 flat status resolver body differs from assembly oracle" >&2
  exit 1
}
node ../macho-hex0/tools/make-macho-hex0.js \
  --wrap-code "$RESOLVER_BODY" "$RESOLVER_MACHO" "rusteze-status-resolver-flat-$$"
clang -arch arm64 -c src/compute-plan-flat.s -o "$PLAN_EMITTER_ORACLE_O"
node tools/text-from-object.js "$PLAN_EMITTER_ORACLE_O" > "$PLAN_EMITTER_ORACLE_BODY"
../macho-m0/macho-m0.bin < compute-plan-emitter/compute-plan-flat.m0 > "$PLAN_EMITTER_BODY"
cmp -s "$PLAN_EMITTER_ORACLE_BODY" "$PLAN_EMITTER_BODY" || {
  echo "M0 flat compute token plan emitter body differs from assembly oracle" >&2
  exit 1
}
node ../macho-hex0/tools/make-macho-hex0.js \
  --wrap-code "$PLAN_EMITTER_BODY" "$PLAN_EMITTER_MACHO" "rusteze-compute-plan-flat-$$"

../macho-m0/macho-m0.bin < ../macho-hex0/fixtures/mini-macho.hex > "$BUILD_DIR/fixed-wrapper-exit42.macho"
chmod +x "$BUILD_DIR/fixed-wrapper-exit42.macho"
case "$(file -b "$BUILD_DIR/fixed-wrapper-exit42.macho")" in
  *Mach-O*arm64*) : ;;
  *) echo "fixed wrapper fixture did not emit an arm64 Mach-O executable" >&2; exit 1 ;;
esac
codesign --verify --verbose=4 "$BUILD_DIR/fixed-wrapper-exit42.macho" >/dev/null 2>&1
set +e
"$BUILD_DIR/fixed-wrapper-exit42.macho"
fixed_status="$?"
set -e
[ "$fixed_status" = "42" ] || {
  echo "fixed wrapper fixture exited $fixed_status, expected 42" >&2
  exit 1
}

emit() {
  name="$1"
  want="$2"
  node tools/emit-flat-status.js < "$FIXTURES/$name.rzir" > "$BUILD_DIR/$name.status"
  if [ "$(wc -c < "$BUILD_DIR/$name.status" | tr -d ' ')" != "1" ]; then
    echo "$name status is not 1 byte" >&2
    exit 1
  fi
  ../macho-m0/macho-m0.bin < "status-templates/$name.m0" > "$BUILD_DIR/$name.m0-status"
  cmp -s "$BUILD_DIR/$name.status" "$BUILD_DIR/$name.m0-status" || {
    echo "$name M0-authored status differs from semantic status" >&2
    exit 1
  }
  "$RESOLVER_MACHO" < "$FIXTURES/$name.rzir" > "$BUILD_DIR/$name.resolver-status"
  cmp -s "$BUILD_DIR/$name.status" "$BUILD_DIR/$name.resolver-status" || {
    echo "$name M0-authored flat status resolver differs from semantic status" >&2
    exit 1
  }
  node tools/emit-flat-body.js < "$FIXTURES/$name.rzir" > "$BUILD_DIR/$name.body"
  if [ "$(wc -c < "$BUILD_DIR/$name.body" | tr -d ' ')" != "12" ]; then
    echo "$name body is not 12 bytes" >&2
    exit 1
  fi
  node tools/emit-flat-compute-body.js < "$FIXTURES/$name.rzir" > "$BUILD_DIR/$name.compute-body"
  node tools/emit-flat-compute-plan.js < "$FIXTURES/$name.rzir" > "$BUILD_DIR/$name.compute.tokens"
  cmp -s "compute-plans/$name.tokens" "$BUILD_DIR/$name.compute.tokens" || {
    echo "$name compute token plan differs from checked plan" >&2
    exit 1
  }
  "$PLAN_EMITTER_MACHO" < "$FIXTURES/$name.rzir" > "$BUILD_DIR/$name.flat-emitter.tokens"
  cmp -s "compute-plans/$name.tokens" "$BUILD_DIR/$name.flat-emitter.tokens" || {
    echo "$name M0-authored flat compute token plan emitter differs from checked plan" >&2
    exit 1
  }
  node tools/emit-flat-compute-body-via-m0.js --emit-source < "$FIXTURES/$name.rzir" > "$BUILD_DIR/$name.compute.m0"
  node tools/check-compute-m0-source.js < "$BUILD_DIR/$name.compute.m0" >/dev/null
  node tools/emit-flat-compute-body-via-m0.js < "$FIXTURES/$name.rzir" > "$BUILD_DIR/$name.compute-via-m0-body"
  cmp -s "$BUILD_DIR/$name.compute-body" "$BUILD_DIR/$name.compute-via-m0-body" || {
    echo "$name rzir->M0->compute-body output differs from compute body oracle" >&2
    exit 1
  }
  node ../macho-hex0/tools/make-macho-hex0.js \
    --wrap-code "$BUILD_DIR/$name.compute-via-m0-body" "$BUILD_DIR/$name.compute.macho" \
    "rusteze-rzir-compute-$name-$$"
  node tools/check-macho-body-slot.js "$BUILD_DIR/$name.compute.macho" "$BUILD_DIR/$name.compute-via-m0-body" \
    > "$BUILD_DIR/$name.compute.wrapper-slot.json"
  node tools/inspect-macho-wrapper-fields.js "$BUILD_DIR/$name.compute.macho" "$BUILD_DIR/$name.compute-via-m0-body" \
    > "$BUILD_DIR/$name.compute.wrapper-fields.json"
  node tools/check-derived-wrapper-layout.js "$BUILD_DIR/$name.compute-via-m0-body" \
    "rusteze-rzir-compute-$name-$$" "$BUILD_DIR/$name.compute.wrapper-fields.json" \
    > "$BUILD_DIR/$name.compute.derived-wrapper-layout.json"
  node tools/emit-derived-wrapper-manifest.js "$BUILD_DIR/$name.compute-via-m0-body" \
    "rusteze-rzir-compute-$name-$$" > "$BUILD_DIR/$name.compute.wrapper-manifest.json"
  node tools/check-derived-wrapper-manifest.js "$BUILD_DIR/$name.compute.wrapper-manifest.json" \
    "$BUILD_DIR/$name.compute.wrapper-fields.json" "$BUILD_DIR/$name.compute.derived-wrapper-layout.json" >/dev/null
  node tools/split-macho-wrapper-template.js \
    "$BUILD_DIR/$name.compute.macho" "$BUILD_DIR/$name.compute-via-m0-body" \
    "$BUILD_DIR/$name.compute.wrapper-template"
  node tools/emit-derived-wrapper-prefix.js "$BUILD_DIR/$name.compute-via-m0-body" \
    "rusteze-rzir-compute-$name-$$" > "$BUILD_DIR/$name.compute.derived-prefix.bin"
  node tools/write-derived-wrapper-prefix-m0.js "$BUILD_DIR/$name.compute-via-m0-body" \
    "rusteze-rzir-compute-$name-$$" > "$BUILD_DIR/$name.compute.derived-prefix.m0"
  ../macho-m0/macho-m0.bin < "$BUILD_DIR/$name.compute.derived-prefix.m0" > "$BUILD_DIR/$name.compute.derived-prefix.via-m0.bin"
  cmp -s "$BUILD_DIR/$name.compute.derived-prefix.bin" "$BUILD_DIR/$name.compute.derived-prefix.via-m0.bin" || {
    echo "$name M0-constructed compute wrapper prefix differs from derived prefix" >&2
    exit 1
  }
  cmp -s "$BUILD_DIR/$name.compute.derived-prefix.bin" "$BUILD_DIR/$name.compute.wrapper-template/prefix.bin" || {
    echo "$name derived compute wrapper prefix differs from oracle prefix" >&2
    exit 1
  }
  node tools/emit-derived-wrapper-linkedit-prefix.js "$BUILD_DIR/$name.compute-via-m0-body" \
    "rusteze-rzir-compute-$name-$$" > "$BUILD_DIR/$name.compute.derived-linkedit-prefix.bin"
  node tools/write-derived-wrapper-linkedit-prefix-m0.js "$BUILD_DIR/$name.compute-via-m0-body" \
    "rusteze-rzir-compute-$name-$$" > "$BUILD_DIR/$name.compute.derived-linkedit-prefix.m0"
  ../macho-m0/macho-m0.bin < "$BUILD_DIR/$name.compute.derived-linkedit-prefix.m0" > "$BUILD_DIR/$name.compute.derived-linkedit-prefix.via-m0.bin"
  cmp -s "$BUILD_DIR/$name.compute.derived-linkedit-prefix.bin" "$BUILD_DIR/$name.compute.derived-linkedit-prefix.via-m0.bin" || {
    echo "$name M0-constructed compute link-edit prefix differs from derived link-edit prefix" >&2
    exit 1
  }
  node tools/check-byte-prefix.js "$BUILD_DIR/$name.compute.derived-linkedit-prefix.bin" \
    "$BUILD_DIR/$name.compute.wrapper-template/suffix.bin"
  node tools/emit-derived-wrapper-code-signature.js "$BUILD_DIR/$name.compute-via-m0-body" \
    "rusteze-rzir-compute-$name-$$" > "$BUILD_DIR/$name.compute.derived-code-signature.bin"
  node tools/write-derived-wrapper-code-signature-m0.js "$BUILD_DIR/$name.compute-via-m0-body" \
    "rusteze-rzir-compute-$name-$$" > "$BUILD_DIR/$name.compute.derived-code-signature.m0"
  ../macho-m0/macho-m0.bin < "$BUILD_DIR/$name.compute.derived-code-signature.m0" > "$BUILD_DIR/$name.compute.derived-code-signature.via-m0.bin"
  cmp -s "$BUILD_DIR/$name.compute.derived-code-signature.bin" "$BUILD_DIR/$name.compute.derived-code-signature.via-m0.bin" || {
    echo "$name M0-constructed compute CodeDirectory differs from derived CodeDirectory" >&2
    exit 1
  }
  node tools/check-byte-suffix-after-prefix.js "$BUILD_DIR/$name.compute.derived-linkedit-prefix.bin" \
    "$BUILD_DIR/$name.compute.derived-code-signature.bin" "$BUILD_DIR/$name.compute.wrapper-template/suffix.bin"
  node tools/emit-derived-wrapper-macho.js "$BUILD_DIR/$name.compute-via-m0-body" \
    "rusteze-rzir-compute-$name-$$" > "$BUILD_DIR/$name.compute.derived.macho"
  cmp -s "$BUILD_DIR/$name.compute.derived.macho" "$BUILD_DIR/$name.compute.macho" || {
    echo "$name derived compute Mach-O differs from oracle Mach-O" >&2
    exit 1
  }
  cmp -s "$BUILD_DIR/$name.compute.macho" "$BUILD_DIR/$name.compute.wrapper-template/recomposed.macho" || {
    echo "$name compute wrapper template did not recompose the Mach-O image" >&2
    exit 1
  }
  node tools/write-derived-wrapper-m0.js "$BUILD_DIR/$name.compute-via-m0-body" \
    "rusteze-rzir-compute-$name-$$" "Structured M0 source for derived wrapped compute Mach-O: $name" \
    > "$BUILD_DIR/$name.compute.macho.m0"
  ../macho-m0/macho-m0.bin < "$BUILD_DIR/$name.compute.macho.m0" > "$BUILD_DIR/$name.compute.via-m0.macho"
  cmp -s "$BUILD_DIR/$name.compute.derived.macho" "$BUILD_DIR/$name.compute.via-m0.macho" || {
    echo "$name derived wrapped compute Mach-O differs after M0 expansion" >&2
    exit 1
  }
  chmod +x "$BUILD_DIR/$name.compute.macho"
  chmod +x "$BUILD_DIR/$name.compute.via-m0.macho"
  chmod +x "$BUILD_DIR/$name.compute.derived.macho"
  set +e
  "$BUILD_DIR/$name.compute.via-m0.macho"
  compute_status="$?"
  set -e
  [ "$compute_status" = "$want" ] || {
    echo "$name M0-expanded computed Mach-O exited $compute_status, expected $want" >&2
    exit 1
  }
  codesign --verify --verbose=4 "$BUILD_DIR/$name.compute.derived.macho" >/dev/null 2>&1
  set +e
  "$BUILD_DIR/$name.compute.derived.macho"
  derived_compute_status="$?"
  set -e
  [ "$derived_compute_status" = "$want" ] || {
    echo "$name derived computed Mach-O exited $derived_compute_status, expected $want" >&2
    exit 1
  }
  node tools/emit-flat-body-via-m0.js < "$FIXTURES/$name.rzir" > "$BUILD_DIR/$name.via-m0-body"
  cmp -s "$BUILD_DIR/$name.body" "$BUILD_DIR/$name.via-m0-body" || {
    echo "$name rzir->M0->body output differs from semantic body" >&2
    exit 1
  }
  cat body-templates/lib.m0 "body-templates/$name.m0" > "$BUILD_DIR/$name.template.m0"
  ../macho-m0/macho-m0.bin < "$BUILD_DIR/$name.template.m0" > "$BUILD_DIR/$name.m0-body"
  cmp -s "$BUILD_DIR/$name.body" "$BUILD_DIR/$name.m0-body" || {
    echo "$name M0-authored body differs from semantic body" >&2
    exit 1
  }
  node tools/emit-flat-macho.js < "$FIXTURES/$name.rzir" > "$BUILD_DIR/$name.macho"
  node tools/check-macho-body-slot.js "$BUILD_DIR/$name.macho" "$BUILD_DIR/$name.body" \
    > "$BUILD_DIR/$name.wrapper-slot.json"
  node tools/inspect-macho-wrapper-fields.js "$BUILD_DIR/$name.macho" "$BUILD_DIR/$name.body" \
    > "$BUILD_DIR/$name.wrapper-fields.json"
  node tools/check-derived-wrapper-layout.js "$BUILD_DIR/$name.body" \
    "rusteze-rzir-macho-flat" "$BUILD_DIR/$name.wrapper-fields.json" \
    > "$BUILD_DIR/$name.derived-wrapper-layout.json"
  node tools/emit-derived-wrapper-manifest.js "$BUILD_DIR/$name.body" \
    "rusteze-rzir-macho-flat" > "$BUILD_DIR/$name.wrapper-manifest.json"
  node tools/check-derived-wrapper-manifest.js "$BUILD_DIR/$name.wrapper-manifest.json" \
    "$BUILD_DIR/$name.wrapper-fields.json" "$BUILD_DIR/$name.derived-wrapper-layout.json" >/dev/null
  node tools/split-macho-wrapper-template.js \
    "$BUILD_DIR/$name.macho" "$BUILD_DIR/$name.body" \
    "$BUILD_DIR/$name.wrapper-template"
  node tools/emit-derived-wrapper-prefix.js "$BUILD_DIR/$name.body" \
    "rusteze-rzir-macho-flat" > "$BUILD_DIR/$name.derived-prefix.bin"
  node tools/write-derived-wrapper-prefix-m0.js "$BUILD_DIR/$name.body" \
    "rusteze-rzir-macho-flat" > "$BUILD_DIR/$name.derived-prefix.m0"
  ../macho-m0/macho-m0.bin < "$BUILD_DIR/$name.derived-prefix.m0" > "$BUILD_DIR/$name.derived-prefix.via-m0.bin"
  cmp -s "$BUILD_DIR/$name.derived-prefix.bin" "$BUILD_DIR/$name.derived-prefix.via-m0.bin" || {
    echo "$name M0-constructed status wrapper prefix differs from derived prefix" >&2
    exit 1
  }
  cmp -s "$BUILD_DIR/$name.derived-prefix.bin" "$BUILD_DIR/$name.wrapper-template/prefix.bin" || {
    echo "$name derived status wrapper prefix differs from oracle prefix" >&2
    exit 1
  }
  node tools/emit-derived-wrapper-linkedit-prefix.js "$BUILD_DIR/$name.body" \
    "rusteze-rzir-macho-flat" > "$BUILD_DIR/$name.derived-linkedit-prefix.bin"
  node tools/write-derived-wrapper-linkedit-prefix-m0.js "$BUILD_DIR/$name.body" \
    "rusteze-rzir-macho-flat" > "$BUILD_DIR/$name.derived-linkedit-prefix.m0"
  ../macho-m0/macho-m0.bin < "$BUILD_DIR/$name.derived-linkedit-prefix.m0" > "$BUILD_DIR/$name.derived-linkedit-prefix.via-m0.bin"
  cmp -s "$BUILD_DIR/$name.derived-linkedit-prefix.bin" "$BUILD_DIR/$name.derived-linkedit-prefix.via-m0.bin" || {
    echo "$name M0-constructed status link-edit prefix differs from derived link-edit prefix" >&2
    exit 1
  }
  node tools/check-byte-prefix.js "$BUILD_DIR/$name.derived-linkedit-prefix.bin" \
    "$BUILD_DIR/$name.wrapper-template/suffix.bin"
  node tools/emit-derived-wrapper-code-signature.js "$BUILD_DIR/$name.body" \
    "rusteze-rzir-macho-flat" > "$BUILD_DIR/$name.derived-code-signature.bin"
  node tools/write-derived-wrapper-code-signature-m0.js "$BUILD_DIR/$name.body" \
    "rusteze-rzir-macho-flat" > "$BUILD_DIR/$name.derived-code-signature.m0"
  ../macho-m0/macho-m0.bin < "$BUILD_DIR/$name.derived-code-signature.m0" > "$BUILD_DIR/$name.derived-code-signature.via-m0.bin"
  cmp -s "$BUILD_DIR/$name.derived-code-signature.bin" "$BUILD_DIR/$name.derived-code-signature.via-m0.bin" || {
    echo "$name M0-constructed status CodeDirectory differs from derived CodeDirectory" >&2
    exit 1
  }
  node tools/check-byte-suffix-after-prefix.js "$BUILD_DIR/$name.derived-linkedit-prefix.bin" \
    "$BUILD_DIR/$name.derived-code-signature.bin" "$BUILD_DIR/$name.wrapper-template/suffix.bin"
  node tools/emit-derived-wrapper-macho.js "$BUILD_DIR/$name.body" \
    "rusteze-rzir-macho-flat" > "$BUILD_DIR/$name.derived.macho"
  cmp -s "$BUILD_DIR/$name.derived.macho" "$BUILD_DIR/$name.macho" || {
    echo "$name derived status Mach-O differs from oracle Mach-O" >&2
    exit 1
  }
  cmp -s "$BUILD_DIR/$name.macho" "$BUILD_DIR/$name.wrapper-template/recomposed.macho" || {
    echo "$name status wrapper template did not recompose the Mach-O image" >&2
    exit 1
  }
  node tools/write-derived-wrapper-m0.js "$BUILD_DIR/$name.body" \
    "rusteze-rzir-macho-flat" "Structured M0 source for derived wrapped status Mach-O: $name" \
    > "$BUILD_DIR/$name.macho.m0"
  ../macho-m0/macho-m0.bin < "$BUILD_DIR/$name.macho.m0" > "$BUILD_DIR/$name.via-m0.macho"
  cmp -s "$BUILD_DIR/$name.derived.macho" "$BUILD_DIR/$name.via-m0.macho" || {
    echo "$name derived wrapped status Mach-O differs after M0 expansion" >&2
    exit 1
  }
  chmod +x "$BUILD_DIR/$name.macho"
  chmod +x "$BUILD_DIR/$name.via-m0.macho"
  chmod +x "$BUILD_DIR/$name.derived.macho"
  case "$(file -b "$BUILD_DIR/$name.macho")" in
    *Mach-O*arm64*) : ;;
    *) echo "$name did not emit an arm64 Mach-O executable" >&2; exit 1 ;;
  esac
  case "$(file -b "$BUILD_DIR/$name.via-m0.macho")" in
    *Mach-O*arm64*) : ;;
    *) echo "$name M0-expanded wrapped status output is not an arm64 Mach-O executable" >&2; exit 1 ;;
  esac
  codesign --verify --verbose=4 "$BUILD_DIR/$name.macho" >/dev/null 2>&1
  codesign --verify --verbose=4 "$BUILD_DIR/$name.via-m0.macho" >/dev/null 2>&1
  codesign --verify --verbose=4 "$BUILD_DIR/$name.derived.macho" >/dev/null 2>&1
  set +e
  "$BUILD_DIR/$name.via-m0.macho"
  status="$?"
  set -e
  [ "$status" = "$want" ] || {
    echo "$name M0-expanded status Mach-O exited $status, expected $want" >&2
    exit 1
  }
  set +e
  "$BUILD_DIR/$name.derived.macho"
  derived_status="$?"
  set -e
  [ "$derived_status" = "$want" ] || {
    echo "$name derived status Mach-O exited $derived_status, expected $want" >&2
    exit 1
  }
}

emit add 11
emit sub 38
emit renamed-temps 12
emit chained-op 7
emit threeconst 15
emit wide-temps 7

node tools/check-wrapper-slot-summary.js "$BUILD_DIR"/*.wrapper-slot.json "$BUILD_DIR"/*.compute.wrapper-slot.json >/dev/null
node tools/compare-wrapper-templates.js "$BUILD_DIR"/*.wrapper-template "$BUILD_DIR"/*.compute.wrapper-template \
  > "$BUILD_DIR/wrapper-template-comparison.json"

echo "rzir -> Mach-O emitter semantic slice verified: flat const/add/sub fixtures emit signed arm64 Mach-O; M0 flat status resolver agrees; M0 flat compute token-plan emitter agrees; M0 compute-body emission executes; fixed and fixture Mach-O wrappers expand through M0"
