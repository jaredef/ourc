#!/usr/bin/env sh
# rzir-to-elf-cf.m0 -- the UNIFIED control-flow backend. One line-ordered,
# value-location codegen handles every construct: const / argc / binary-op
# assignments (with mutation = slot re-store), `sel` (a runtime conditional
# branch), and LOOPS (computed backward branch + backpatched forward exit -- the
# assembler ladder's label resolution inside the compiler). It supersedes the
# folding (mop), template (cg), and find-based (gen) backends. Chain:
# hex2 -> M0 -> cf. No cc. Verified by exit code.
set -eu
cd "$(dirname "$0")"
mkdir -p build
M0=../m0/m0.bin
chmod +x "$M0"
S3=../../../locales/stage3-ir-backend/fixtures
S2=../../../locales/stage2-core/fixtures/expected-ir

"$M0" < rzir-to-elf-cf.m0 > build/cf.bin
chmod +x build/cf.bin

rz() { ./build/cf.bin < "$1" > build/o; chmod +x build/o; set +e; ./build/o; s="$?"; set -e
       [ "$s" = "$2" ] || { echo "cf $1: exit $s, expected $2" >&2; exit 1; }; }
# straight-line (slots + ops)
rz "$S3/add.rzir" 11
rz "$S3/sub.rzir" 38
rz "$S3/renamed-temps.rzir" 12
rz "$S3/threeconst.rzir" 15
rz "$S2/compute.rzir" 20

# runtime-dependent: the SAME binary must vary with argc
vary() { # rzir, extra, expected
  ir="$1"; n="$2"; want="$3"
  ./build/cf.bin < "$ir" > build/v; chmod +x build/v
  set --; i=0; while [ "$i" -lt "$n" ]; do set -- "$@" x; i=$((i+1)); done
  set +e; ./build/v "$@"; s="$?"; set -e
  [ "$s" = "$want" ] || { echo "cf $ir argc=$((n+1)): exit $s, expected $want" >&2; exit 1; }
}
vary argc.rzir 0 41 ; vary argc.rzir 2 43       # runtime value
vary sel.rzir  0 3  ; vary sel.rzir  1 7        # conditional BRANCH
vary loop.rzir 0 1  ; vary loop.rzir 4 15       # LOOP (sum 1..argc): computed back-branch
vary loop.rzir 6 28
vary nested.rzir 1 4 ; vary nested.rzir 3 16    # NESTED loops (argc^2): label-table stack
vary nested.rzir 4 25
vary call.rzir 0 2 ; vary call.rzir 2 6         # FUNCTION CALL (dbl): arg in edi, result in eax
vary call.rzir 4 10
vary cmp.rzir 0 0 ; vary cmp.rzir 2 3           # COMPARISON (lt): while(i<n) -> sum(0..argc-1)
vary cmp.rzir 4 10
vary ptr.rzir 0 4 ; vary ptr.rzir 2 6           # POINTERS (ref/load/store): *(&v0)=argc+3 thru a ptr
vary ptr.rzir 4 8
vary arr.rzir 0 8 ; vary arr.rzir 1 9           # ARRAYS (get/put): a[argc]=argc+7 at a computed index
vary arr.rzir 2 10
vary mul.rzir 0 3 ; vary mul.rzir 2 9     # MUL (operands free via the scanner): argc*3
vary div.rzir 0 60 ; vary div.rzir 2 20  # DIV (idiv+cdq; mnemonic 'div' contains the 'v' marker): 60/argc
rz bigconst.rzir 145                      # CONST WIDTH: 200-55 (multi-digit pd; 2-digit pd gives 20-55)
vary mod.rzir 1 1 ; vary mod.rzir 2 2    # MOD (collides with 'mul' at 'm'; token dispatch is collision-free): 17%argc
rz noarg.rzir 7                           # NO-ARG call: a_call name-skip must stop at newline
vary recsum.rzir 0 1 ; vary recsum.rzir 4 15 # RECURSION: sum(n)=n+sum(n-1) -- each call's frame is separate
vary recsum.rzir 2 6
vary evenodd.rzir 0 0 ; vary evenodd.rzir 1 1 # MUTUAL RECURSION: even/odd via a function symbol table
vary evenodd.rzir 4 0
vary add2.rzir 0 11 ; vary add2.rzir 2 13 # MULTI-ARG: add2(argc,10)=argc+10 (arg1=edi, arg2=esi)
vary add3.rzir 0 31 ; vary add3.rzir 4 35 # 3-ARG: add3(argc,10,20) (reg table edi/esi/edx)
vary add6.rzir 0 51 ; vary add6.rzir 4 55 # 6-ARG: arg1(edi)+arg6(r9d) = argc+50
vary bytes.rzir 0 72 ; vary bytes.rzir 1 73 # CHAR/BYTE: byte string buf[0]=72 buf[1]=73, getb by index (scale 1)
vary eqset.rzir 0 0 ; vary eqset.rzir 1 1 # EQ true-case (argc==2): a_eq must not fall through to a_gt
vary cmpset.rzir 0 0 ; vary cmpset.rzir 2 1  # COMPARISONS: argc >= 3 (the full lt/eq/gt/le/ge/ne set)
vary bitand.rzir 0 1 ; vary bitand.rzir 1 0  # BITWISE: argc & 1 (parity) -- and/or/xor complete the operators
rz manyloc.rzir 21                       # MULTI-DIGIT vN: 12 locals, v11 = add v9 v10 = 21 (2-digit slot index)

rm -rf build
echo "unified cf backend verified: const/op/argc + sel (branch) + loop (computed back-branch), one backend (no cc)"
