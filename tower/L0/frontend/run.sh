#!/usr/bin/env sh
# rz2 -> rzir front-end resolver (with NAME RESOLUTION) authored in M0, and the
# end-to-end compiler-free chain. hex2 builds M0; M0 builds the resolver and the
# emitter; then rz2 --(resolver)--> rzir --(emitter)--> ELF --> runs. No cc.
set -eu
cd "$(dirname "$0")"
mkdir -p build
M0=../m0/m0.bin
chmod +x "$M0"
F=../../../locales/stage2-core/fixtures

"$M0" < rz2-to-rzir.m0             > build/resolver.bin
"$M0" < ../emitter/rzir-to-elf.m0  > build/emitter.bin
chmod +x build/resolver.bin build/emitter.bin

# 1. The resolver's rzir is byte-identical to Stage2's expected-ir -- INCLUDING
#    swap, where `return sub y x` resolves operands by name to `sub v1 v0`
#    (not the positional `sub v0 v1`). This is the name-resolution proof.
# `compute` is a COMPUTED let (let d = add a b) -> a 2-op rzir, proving the
# shared op-expression lowering (emit_rhs). It needs a multi-op backend to run
# end-to-end, so it is checked at the rzir boundary only.
# `names` uses MULTI-CHAR names (foo/bar) with reversed operands -- proving
# full-name resolution, not just first-char.
for n in add sub swap names threelet compute sum count; do
  ./build/resolver.bin < "$F/$n.rz2" > "build/$n.rzir"
  cmp -s "build/$n.rzir" "$F/expected-ir/$n.rzir" \
    || { echo "$n.rzir != Stage2 expected-ir" >&2; exit 1; }
done

# 2. End-to-end: rz2 -> ELF -> correct exit. The emitter now resolves op operands
#    by their vN (matching each against the first const's vN), so swap's
#    `sub v1 v0` runs too. add 40+2=42, sub 40-2=38, swap 2-40=-38=218.
chain() {
  n="$1"; want="$2"
  ./build/resolver.bin < "$F/$n.rz2" | ./build/emitter.bin > "build/$n.elf"
  chmod +x "build/$n.elf"
  set +e; "./build/$n.elf"; s="$?"; set -e
  [ "$s" = "$want" ] || { echo "$n: rz2->ELF exit $s, expected $want" >&2; exit 1; }
}
chain add 42
chain sub 38
chain swap 218
chain names 218
chain threelet 15   # 3 consts: add a c = 10 + 5, with the emitter's vN->value table

mkdir -p build

# the loop fixture compiles through the unified cf backend (sum 1..argc)
"$M0" < ../emitter/rzir-to-elf-cf.m0 > build/cf.bin; chmod +x build/cf.bin
loopchk() { n="$1"; want="$2"
  ./build/resolver.bin < "$F/sum.rz2" | ./build/cf.bin > build/sum.elf; chmod +x build/sum.elf
  set --; i=0; while [ "$i" -lt "$n" ]; do set -- "$@" x; i=$((i+1)); done
  set +e; ./build/sum.elf "$@"; s="$?"; set -e
  [ "$s" = "$want" ] || { echo "sum loop argc=$((n+1)): $s != $want" >&2; exit 1; }; }
loopchk 0 1 ; loopchk 2 6 ; loopchk 4 15
cntchk() { c="$1"; w="$2"; ./build/resolver.bin < "$F/count.rz2" | ./build/cf.bin > build/c.elf; chmod +x build/c.elf
  set --; i=0; while [ "$i" -lt "$c" ]; do set -- "$@" x; i=$((i+1)); done
  set +e; ./build/c.elf "$@"; r="$?"; set -e; [ "$r" = "$w" ] || { echo "count argc=$((c+1)): $r != $w" >&2; exit 1; }; }
cntchk 0 1 ; cntchk 4 15

# op-WIDTH-agnostic emit_rhs: 3-char (mul -> argc^2) and 2-char (lt -> 1<argc) ops
sqchk() { c="$1"; w="$2"; ./build/resolver.bin < "$F/square.rz2" | ./build/cf.bin > build/sq.elf; chmod +x build/sq.elf
  set --; i=0; while [ "$i" -lt "$c" ]; do set -- "$@" x; i=$((i+1)); done
  set +e; ./build/sq.elf "$@"; r="$?"; set -e; [ "$r" = "$w" ] || { echo "square argc=$((c+1)): $r != $w" >&2; exit 1; }; }
sqchk 0 1 ; sqchk 4 25
ltchk() { c="$1"; w="$2"; ./build/resolver.bin < "$F/cmp2.rz2" | ./build/cf.bin > build/lt.elf; chmod +x build/lt.elf
  set --; i=0; while [ "$i" -lt "$c" ]; do set -- "$@" x; i=$((i+1)); done
  set +e; ./build/lt.elf "$@"; r="$?"; set -e; [ "$r" = "$w" ] || { echo "cmp2 argc=$((c+1)): $r != $w" >&2; exit 1; }; }
ltchk 0 0 ; ltchk 1 1

# MULTI-FUNCTION: rz2 with N functions and (nested) calls, end-to-end through cf
fnchk() { c="$1"; w="$2"; rz="$3"; ./build/resolver.bin < "$F/$rz" | ./build/cf.bin > build/fn.elf; chmod +x build/fn.elf
  set --; i=0; while [ "$i" -lt "$c" ]; do set -- "$@" x; i=$((i+1)); done
  set +e; ./build/fn.elf "$@"; r="$?"; set -e; [ "$r" = "$w" ] || { echo "$rz argc=$((c+1)): $r != $w" >&2; exit 1; }; }
fnchk 0 2 fn.rz2 ; fnchk 4 10 fn.rz2          # main->dbl, 2*argc
fnchk 0 2 chain.rz2 ; fnchk 3 5 chain.rz2     # main->dbl->inc, argc+1

# IF construct + RECURSION from rz2 source
ifchk() { c="$1"; w="$2"; ./build/resolver.bin < "$F/iff.rz2" | ./build/cf.bin > build/if.elf; chmod +x build/if.elf
  set --; i=0; while [ "$i" -lt "$c" ]; do set -- "$@" x; i=$((i+1)); done
  set +e; ./build/if.elf "$@"; r="$?"; set -e; [ "$r" = "$w" ] || { echo "iff argc=$((c+1)): $r != $w" >&2; exit 1; }; }
ifchk 0 100 ; ifchk 1 105                      # if (argc-1): 100 / 105
reckchk() { c="$1"; w="$2"; ./build/resolver.bin < "$F/recsum.rz2" | ./build/cf.bin > build/rs.elf; chmod +x build/rs.elf
  set --; i=0; while [ "$i" -lt "$c" ]; do set -- "$@" x; i=$((i+1)); done
  set +e; ./build/rs.elf "$@"; r="$?"; set -e; [ "$r" = "$w" ] || { echo "recsum argc=$((c+1)): $r != $w" >&2; exit 1; }; }
reckchk 0 1 ; reckchk 2 6 ; reckchk 4 15       # RECURSION: sum(argc)=argc(argc+1)/2

# the chain compiles real programs of every shape, from source
factchk() { c="$1"; w="$2"; ./build/resolver.bin < "$F/fact.rz2" | ./build/cf.bin > build/f.elf; chmod +x build/f.elf
  set --; i=0; while [ "$i" -lt "$c" ]; do set -- "$@" x; i=$((i+1)); done
  set +e; ./build/f.elf "$@"; r="$?"; set -e; [ "$r" = "$w" ] || { echo "fact argc=$((c+1)): $r != $w" >&2; exit 1; }; }
factchk 0 1 ; factchk 2 6 ; factchk 4 120      # factorial: recursion + multiply
stochk() { c="$1"; w="$2"; ./build/resolver.bin < "$F/sumto.rz2" | ./build/cf.bin > build/s.elf; chmod +x build/s.elf
  set --; i=0; while [ "$i" -lt "$c" ]; do set -- "$@" x; i=$((i+1)); done
  set +e; ./build/s.elf "$@"; r="$?"; set -e; [ "$r" = "$w" ] || { echo "sumto argc=$((c+1)): $r != $w" >&2; exit 1; }; }
stochk 0 1 ; stochk 4 15                        # a function CONTAINING a loop

a2chk() { c="$1"; w="$2"; ./build/resolver.bin < "$F/add2.rz2" | ./build/cf.bin > build/a2.elf; chmod +x build/a2.elf
  set --; i=0; while [ "$i" -lt "$c" ]; do set -- "$@" x; i=$((i+1)); done
  set +e; ./build/a2.elf "$@"; r="$?"; set -e; [ "$r" = "$w" ] || { echo "add2 argc=$((c+1)): $r != $w" >&2; exit 1; }; }
a2chk 0 11 ; a2chk 3 14                         # MULTI-ARG: add2(argc,10) = argc+10
a3chk() { c="$1"; w="$2"; ./build/resolver.bin < "$F/add3.rz2" | ./build/cf.bin > build/a3.elf; chmod +x build/a3.elf
  set --; i=0; while [ "$i" -lt "$c" ]; do set -- "$@" x; i=$((i+1)); done
  set +e; ./build/a3.elf "$@"; r="$?"; set -e; [ "$r" = "$w" ] || { echo "add3 argc=$((c+1)): $r != $w" >&2; exit 1; }; }
a3chk 0 31 ; a3chk 3 34                         # 3-ARG: add3(argc,10,20)=argc+30

ptrchk() { c="$1"; w="$2"; ./build/resolver.bin < "$F/ptr.rz2" | ./build/cf.bin > build/p.elf; chmod +x build/p.elf
  set --; i=0; while [ "$i" -lt "$c" ]; do set -- "$@" x; i=$((i+1)); done
  set +e; ./build/p.elf "$@"; r="$?"; set -e; [ "$r" = "$w" ] || { echo "ptr argc=$((c+1)): $r != $w" >&2; exit 1; }; }
ptrchk 0 8 ; ptrchk 3 11                        # POINTERS: (*&a)+(*&b) = argc+7

memchk() { fx="$1"; c="$2"; w="$3"; ./build/resolver.bin < "$F/$fx" | ./build/cf.bin > build/m.elf; chmod +x build/m.elf
  set --; i=0; while [ "$i" -lt "$c" ]; do set -- "$@" x; i=$((i+1)); done
  set +e; ./build/m.elf "$@"; r="$?"; set -e; [ "$r" = "$w" ] || { echo "$fx argc=$((c+1)): $r != $w" >&2; exit 1; }; }
memchk store.rz2 0 41 ; memchk store.rz2 3 44   # POINTER WRITE: store *p, read back
memchk arr.rz2 0 41 ; memchk arr.rz2 3 44       # ARRAY WRITE: put a[2], read back via get

blchk() { c="$1"; w="$2"; ./build/resolver.bin < "$F/recsum-bl.rz2" | ./build/cf.bin > build/bl.elf; chmod +x build/bl.elf
  set --; i=0; while [ "$i" -lt "$c" ]; do set -- "$@" x; i=$((i+1)); done
  set +e; ./build/bl.elf "$@"; r="$?"; set -e; [ "$r" = "$w" ] || { echo "recsum-bl argc=$((c+1)): $r != $w" >&2; exit 1; }; }
blchk 0 1 ; blchk 3 10 ; blchk 4 15             # BLOCK-LOCAL LETS: m,r declared inside the if

mlchk() { c="$1"; w="$2"; ./build/resolver.bin < "$F/manyloc.rz2" | ./build/cf.bin > build/ml.elf; chmod +x build/ml.elf
  set --; i=0; while [ "$i" -lt "$c" ]; do set -- "$@" x; i=$((i+1)); done
  set +e; ./build/ml.elf "$@"; r="$?"; set -e; [ "$r" = "$w" ] || { echo "manyloc argc=$((c+1)): $r != $w" >&2; exit 1; }; }
mlchk 0 11 ; mlchk 3 14                          # MULTI-DIGIT vN: 11 locals, add a k (v0,v10) = argc+10










rm -rf build
echo "front-end verified: rz2 -> rzir N-let name-resolves (add/sub/swap/names/threelet); rz2 -> ELF runs end-to-end (no cc)"
