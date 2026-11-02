#!/usr/bin/env sh
# Multi-op (constant-folding) backend authored in M0. Handles N op lines --
# including a computed let's two ops -- which the single-op template emitter
# (rzir-to-elf.m0) could not. It evaluates the all-constant IR at compile time
# and emits exit(final). This NECESSARILY diverges from the 1-op cc reference
# (the compiler outgrowing its bootstrap oracle, L041), so it is verified by
# EXIT CODE, not byte-identity. Chain: hex2 -> M0 -> mop-emitter. No cc.
set -eu
cd "$(dirname "$0")"
mkdir -p build
M0=../m0/m0.bin
chmod +x "$M0"
S3=../../../locales/stage3-ir-backend/fixtures
S2=../../../locales/stage2-core/fixtures

"$M0" < rzir-to-elf-mop.m0          > build/mop.bin
"$M0" < ../frontend/rz2-to-rzir.m0  > build/res.bin
chmod +x build/mop.bin build/res.bin

# (a) the folding backend on rzir fixtures, by exit code
rzir() {
  ir="$1"; want="$2"
  ./build/mop.bin < "$ir" > build/o.elf
  chmod +x build/o.elf
  set +e; ./build/o.elf; s="$?"; set -e
  [ "$s" = "$want" ] || { echo "mop $ir: exit $s, expected $want" >&2; exit 1; }
}
rzir "$S3/add.rzir" 11
rzir "$S3/sub.rzir" 38
rzir "$S3/renamed-temps.rzir" 12
rzir "$S3/threeconst.rzir" 15
rzir "$S2/expected-ir/compute.rzir" 20   # the computed let (2 ops)

# (b) the FULL compiler-free chain rz2 -> resolver -> mop-emitter -> ELF
chain() {
  n="$1"; want="$2"
  ./build/res.bin < "$S2/$n.rz2" | ./build/mop.bin > "build/$n.elf"
  chmod +x "build/$n.elf"
  set +e; "./build/$n.elf"; s="$?"; set -e
  [ "$s" = "$want" ] || { echo "chain $n: exit $s, expected $want" >&2; exit 1; }
}
chain add 42
chain sub 38
chain swap 218
chain names 218
chain threelet 15
chain compute 20      # let d = add a b ; return sub d a -> 30 - 10 = 20

rm -rf build
echo "multi-op backend verified: folds N ops; rz2 -> ELF end-to-end incl the computed let (no cc)"
