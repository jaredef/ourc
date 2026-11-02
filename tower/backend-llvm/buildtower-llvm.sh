#!/bin/bash
# Tier-2 libLLVM backend build for tower-rustc (ONEBIN driver-integration D2, 2026-10-10).
# PROVEN: links + runs with real libLLVM, advancing tower-rustc PAST the null-codegen-backend SIGSEGV
# (Linker::link/join_codegen) to a clean exit 101 (rustc's "compilation failed"; next frontier = make the
# DiagCtxt error visible). Builds a SCRATCH $J/tower-rustc-llvm -- it does NOT replace the canonical
# $J/tower-rustc, and does NOT change the produced trust chain until RB-4 (tower-rustc-llvm == stock rustc-1.95)
# passes and the keeper adopts it (the Tier-2 RB-4 rule).
# Prereqs (all present on this box 2026-10-10): rust-1.95 ci-llvm (headers+llvm-config+libLLVM) + the rustc
# llvm-wrapper source + c++. Run AFTER drvgauge produced $J/linked_drv.rz.
set -e
J=${J:?set J to the driver scratch dir}
LLVMROOT=/tmp/rustc-src/rustc-1.95.0-src/build/x86_64-unknown-linux-gnu/ci-llvm
CFG=$LLVMROOT/bin/llvm-config
WRAP=/tmp/rustc-src/rustc-1.95.0-src/compiler/rustc_llvm/llvm-wrapper
export PATH=~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin:$PATH

# 1. compile rustc's C++ llvm-wrapper (provides the 138 LLVMRust* FFI symbols) against ci-llvm
mkdir -p $J/llvmwrap
for f in $WRAP/*.cpp; do c++ $($CFG --cxxflags) -I$WRAP -c $f -o $J/llvmwrap/$(basename $f .cpp).o; done

# 2. genstubs that SKIP LLVM*/llvm_* danglers (so they stay external, resolved by the wrapper+libLLVM)
sed 's/^for c in dang:/for c in dang:\n    if c.startswith("LLVM"): continue/' $J/genstubs.py > $J/genstubs_llvm.py
python3 $J/genstubs_llvm.py $J/linked_drv.rz $J/drv.dang $J/stubs_llvm.rz
cat $J/linked_drv.rz $J/stubs_llvm.rz > $J/drv_full_llvm.rz

# 2b. INTERIM: bare-alias the one Rust->C++ callback the wrapper needs (LLVMRustStringWriteImpl), which SEM0
# crate-mangles to rustc_llvm$... and link.elf DCE's (no Rust caller). PROPER FIX PENDING = emitter emits
# #[no_mangle]/extern "C" fns with their bare name + link-retains them (s9_defname rung).
{ echo "//@SUPERSET crate=rustc_llvm"; cat $J/Lingest/all.meta; cat $J/Lingest/rustc_llvm.rs; } | $J/sem0.head 2>/dev/null \
  | awk '/^fn rustc_llvm\$LLVMRustStringWriteImpl/{f=1} f{print} f&&/^  ret /{exit}' \
  | sed '1s/^fn rustc_llvm\$LLVMRustStringWriteImpl/fn LLVMRustStringWriteImpl/' >> $J/drv_full_llvm.rz

# 3. rzir -> LLVM IR -> object -> link with wrapper + libLLVM
$J/towersp/r2l.elf < $J/drv_full_llvm.rz > $J/drv_llvm.ll 2>/dev/null
llc -O2 -filetype=obj $J/drv_llvm.ll -o $J/drv_llvm.o
cc -no-pie $J/drv_llvm.o $J/llvmwrap/*.o -L$($CFG --libdir) $($CFG --libs) -lstdc++ -lm -lz \
   -Wl,-rpath,$($CFG --libdir) -o $J/tower-rustc-llvm
echo "tower-rustc-llvm: $(ls -l $J/tower-rustc-llvm | awk '{print $5}') bytes"
