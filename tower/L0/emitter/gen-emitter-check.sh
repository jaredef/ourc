#!/usr/bin/env sh
# General codegen backend (rzir-to-elf-gen.m0): ONE backend for the whole
# language. A value-location model -- every vN gets a runtime slot at
# 0x410000 + N*4 -- and each assignment generates real load/op/store
# instructions (output code is variable-length; p_filesz is computed). Handles
# arbitrary SSA DAGs: constants, runtime values (argc), a value used more than
# once (2*argc), computed temps. Supersedes the folding (mop) and one-op (cg)
# backends. Chain: hex2 -> M0 -> {resolver, gen}. No cc. Verified by exit code.
set -eu
cd "$(dirname "$0")"
mkdir -p build
M0=../m0/m0.bin
chmod +x "$M0"
S3=../../../locales/stage3-ir-backend/fixtures
F=../../../locales/stage2-core/fixtures

"$M0" < rzir-to-elf-gen.m0          > build/gen.bin
"$M0" < ../frontend/rz2-to-rzir.m0  > build/res.bin
chmod +x build/gen.bin build/res.bin

# (a) the general backend on rzir fixtures (all-constant), by exit code
rz() { ./build/gen.bin < "$1" > build/o; chmod +x build/o; set +e; ./build/o; s="$?"; set -e;
       [ "$s" = "$2" ] || { echo "gen $1: exit $s, expected $2" >&2; exit 1; }; }
rz "$S3/add.rzir" 11
rz "$S3/sub.rzir" 38
rz "$S3/renamed-temps.rzir" 12
rz "$S3/threeconst.rzir" 15

# (b) the FULL chain rz2 -> resolver -> gen -> ELF, ONE backend for everything
ch() { ./build/res.bin < "$F/$1.rz2" | ./build/gen.bin > "build/$1.elf"; chmod +x "build/$1.elf";
       set +e; "./build/$1.elf"; s="$?"; set -e;
       [ "$s" = "$2" ] || { echo "chain $1: exit $s, expected $2" >&2; exit 1; }; }
ch add 42
ch sub 38
ch swap 218
ch names 218
ch threelet 15
ch compute 20

# (c) runtime-dependent: the SAME binary varies with argc
rt() { # rz2, extra-args, expected  (save args -- `set --` below clobbers $1..$3)
  f="$1"; n="$2"; want="$3"
  ./build/res.bin < "$F/$f.rz2" | ./build/gen.bin > build/r.elf; chmod +x build/r.elf
  set --; i=0; while [ "$i" -lt "$n" ]; do set -- "$@" x; i=$((i+1)); done
  set +e; ./build/r.elf "$@"; s="$?"; set -e
  [ "$s" = "$want" ] || { echo "runtime $f argc=$((n+1)): exit $s, expected $want" >&2; exit 1; }
}
rt argc   0 41   ; rt argc   2 43     # n + 40
rt twoarg 0 2    ; rt twoarg 3 8      # n + n = 2*argc
rt sel    0 3    ; rt sel    1 7      # (argc-1) ? 7 : 3  -- a runtime BRANCH

# (d) control flow on the hand-written rzir directly (the SAME binary branches)
./build/gen.bin < sel.rzir > build/cf.elf; chmod +x build/cf.elf
x=$(set +e; ./build/cf.elf; echo $?); y=$(set +e; ./build/cf.elf z; echo $?)
[ "$x" = 3 ] && [ "$y" = 7 ] || { echo "sel.rzir branch wrong: $x/$y (want 3/7)" >&2; exit 1; }

rm -rf build
echo "general codegen verified: const + runtime + value-reuse + CONTROL FLOW (sel branch); rz2 -> ELF (no cc)"
