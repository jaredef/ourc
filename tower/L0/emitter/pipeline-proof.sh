#!/usr/bin/env sh
# End-to-end COMPILER-FREE chain proof.
#
#   rz2 --[Stage2]--> rzir --[derived emitter]--> ELF --> exit code
#                          \__ the compiler-free seam __/
#
# Uses ONLY: the committed hex1 (constraint-derived) + the labeled emitter source.
# No cc/ld/as. hex1 builds the emitter; the emitter lowers Stage2's ACTUAL emitted
# IR (locales/stage2-core/fixtures/expected-ir, consumed byte-unchanged) to
# runnable ELF executables. Asserts every artifact's exit code.
set -eu

cd "$(dirname "$0")"
mkdir -p build
HEX1=../hex1/hex1.bin
chmod +x "$HEX1"
S2=../../../locales/stage2-core/fixtures/expected-ir

# Build the lowering compiler-free.
"$HEX1" < rzir-to-elf-folding.labeled.hex > build/emitter.bin
chmod +x build/emitter.bin

run() {
  ir="$1"; want="$2"
  ./build/emitter.bin < "$S2/$ir.rzir" > "build/$ir.elf"
  chmod +x "build/$ir.elf"
  set +e; "./build/$ir.elf"; s="$?"; set -e
  if [ "$s" != "$want" ]; then
    echo "$ir: ELF exited $s, expected $want" >&2; exit 1
  fi
}

# Every Stage2-emitted IR artifact -> compiler-free ELF, exit code asserted.
run add                      42
run sub                      38
run add-small                12
run add-three                11
run sub-three                 6
run add-renamed              17
run add-swapped              17
run block-outer-read         38
run block-shadow-add         11
run multi-name-add           11
run outer-return-after-block 16
run prefix-name-sub          38
run sub-swapped             218

echo "chain verified: rz2 -> rzir -> ELF for all 13 Stage2 IR artifacts, no cc/ld/as"
