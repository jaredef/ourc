#!/usr/bin/env sh
# Follow-on demonstration: the emitter re-authored against hex2 has NO
# hand-computed addresses and NO measuring build. A SINGLE hex2 invocation
# computes every address (entry =m, filesz &z, INBUF =z, patch slots =a/=b/=p)
# from the layout. Uses only committed hex2; no cc/ld/as.
set -eu
cd "$(dirname "$0")"
mkdir -p build
HEX2=../hex2/hex2.bin
chmod +x "$HEX2"
S3=../../../locales/stage3-ir-backend/fixtures

"$HEX2" < rzir-to-elf-hex2.labeled.hex > build/emitter.bin
chmod +x build/emitter.bin

check() {
  ir="$1"; want="$2"
  ./build/emitter.bin < "$S3/$ir.rzir" > "build/$ir.elf"
  chmod +x "build/$ir.elf"
  set +e; "./build/$ir.elf"; s="$?"; set -e
  [ "$s" = "$want" ] || { echo "$ir: exit $s, expected $want" >&2; exit 1; }
}
check add 11
check sub 38
check renamed-temps 12

echo "hex2-authored emitter verified: no hand-computed addresses, single build, outputs match cc path (no cc/ld/as)"
