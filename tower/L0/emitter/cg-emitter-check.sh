#!/usr/bin/env sh
# Codegen backend (rzir-to-elf-cg.m0): the first NON-folding backend. A non-
# constant value (argc) cannot be precomputed, so the backend GENERATES a program
# that loads argc from [rsp] at runtime and computes. Proof: the SAME binary
# yields different exit codes for different argc -- which folding cannot do.
# Chain: hex2 -> M0 -> {resolver, cg-backend}. No cc. Verified by exit code.
set -eu
cd "$(dirname "$0")"
mkdir -p build
M0=../m0/m0.bin
chmod +x "$M0"
F=../../../locales/stage2-core/fixtures

"$M0" < rzir-to-elf-cg.m0        > build/cg.bin
"$M0" < rzir-to-elf-mop.m0       > build/mop.bin
"$M0" < ../frontend/rz2-to-rzir.m0 > build/res.bin
chmod +x build/cg.bin build/mop.bin build/res.bin

run_argc() {  # binary, nargs(extra), expected
  bin="$1"; n="$2"; want="$3"
  set --; i=0; while [ "$i" -lt "$n" ]; do set -- "$@" x; i=$((i+1)); done
  set +e; "$bin" "$@"; s="$?"; set -e
  [ "$s" = "$want" ] || { echo "argc=$((n+1)): exit $s, expected $want" >&2; exit 1; }
}

# (a) cg backend on a hand-written argc.rzir: exit = argc + 40, so it MUST vary.
./build/cg.bin < argc.rzir > build/p
chmod +x build/p
run_argc ./build/p 0 41    # argc=1
run_argc ./build/p 2 43    # argc=3
run_argc ./build/p 5 46    # argc=6

# (b) folding on the SAME rzir is argc-blind (one fixed value) -- the contrast.
./build/mop.bin < argc.rzir > build/f
chmod +x build/f
a=$(set +e; ./build/f; echo $?); b=$(set +e; ./build/f x x x; echo $?)
[ "$a" = "$b" ] || { echo "folding unexpectedly varied with argc" >&2; exit 1; }

# (c) END-TO-END: argc.rz2 -> resolver -> cg-backend -> ELF, varying argc.
./build/res.bin < "$F/argc.rz2" | ./build/cg.bin > build/e
chmod +x build/e
run_argc ./build/e 0 41    # argc=1
run_argc ./build/e 3 44    # argc=4

rm -rf build
echo "codegen backend verified: argc is a RUNTIME value; same binary varies with argc; rz2 -> ELF end-to-end (no cc)"
