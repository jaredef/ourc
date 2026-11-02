#!/usr/bin/env sh
# COMPILER-FREE CHAIN PROOF — target-aware dispatcher.
#
# The existing full ladder is x86_64 Linux ELF. Running it on Apple Silicon
# macOS executes no proof: the ELF seed binaries cannot run here. The dispatcher
# keeps that boundary explicit and routes this host to the local Mach-O proof.
set -eu

os="$(uname -s)"
arch="$(uname -m)"

case "$os:$arch" in
  Darwin:arm64)
    cd "$(dirname "$0")"
    echo "[1/5] macho-hex0 — self-hosting signed arm64 Mach-O seed"
    sh macho-hex0/run.sh
    echo "[2/5] macho-hex1 — self-hosting arm64 symbolic branch seed"
    sh macho-hex1/run.sh
    echo "[3/5] macho-hex2 — self-hosting arm64/Mach-O symbolic layout seed"
    sh macho-hex2/run.sh
    echo "[4/5] macho-M0 — self-hosting arm64/Mach-O token assembler seed"
    sh macho-m0/run.sh
    echo "[5/5] rzir -> Mach-O — semantic flat lowering connector"
    sh macho-emitter/run.sh
    echo
    echo "CHAIN PARTIAL: arm64-macos Mach-O hex0 seed proof is green;"
    echo "arm64 hex1 self-hosting seed is green;"
    echo "arm64 hex2 self-hosting seed is green;"
    echo "arm64 M0 self-hosting seed is green;"
    echo "arm64 rzir -> Mach-O semantic lowering is green."
    echo "The higher Linux ELF ladder is target-specific and is skipped on this host."
    exit 0
    ;;
  Linux:x86_64)
    ;;
  *)
    echo "unsupported proof host: $os $arch" >&2
    echo "supported: Linux x86_64 for full ELF ladder, Darwin arm64 for Mach-O seed proof" >&2
    exit 1
    ;;
esac

# COMPILER-FREE CHAIN PROOF — the whole Linux ELF ladder from a clone, with cc
# nowhere.
#
# Trust base: two small, audited, self-hosting committed binaries.
#   hex0.bin  (hand-derived)          — self-hosts from hex0.hand.hex
#   hex1.bin  (constraint-derived)    — self-hosts from derived-hex1.labeled.hex
# Everything above is built FROM them: the emitter is expanded by hex1; the
# emitter lowers Stage2's rzir to runnable ELF. No cc / ld / as is invoked here
# or by any step below.
cd "$(dirname "$0")"

# Guard: put failing stubs for every host compiler/assembler/linker on PATH, so
# if ANY step actually invokes one, it fails loudly. This proves cc-freedom by
# execution, not by reading the scripts.
mkdir -p build/nobin
for tool in cc gcc clang ld as g++ clang++; do
  printf '#!/bin/sh\necho "FORBIDDEN: %s invoked" >&2\nexit 127\n' "$tool" > "build/nobin/$tool"
  chmod +x "build/nobin/$tool"
done
PATH="$PWD/build/nobin:$PATH"
export PATH

# run one rung's check; its output is indented, and ANY failure stops the proof (a `sh x | sed` pipe used to
# discard the exit status, so missing fixtures and a segfault still ended in "CHAIN PROVEN").
step() {
  sh "$1" > build/step.log 2>&1; rc=$?
  sed 's/^/      /' build/step.log
  [ "$rc" = 0 ] || { echo "CHAIN FAILED: $1 exited $rc"; exit 1; }
  if grep -qE 'No such file|Segmentation fault|core dumped' build/step.log; then
    echo "CHAIN FAILED: $1 reported a missing file or crash"; exit 1
  fi
}

echo "[1/4] hex0  — hand-derived seed, self-hosts + authors an ELF"
step hex0/run.sh
echo "[2/4] hex1  — constraint-derived, self-hosts + resolves labels"
step hex1/run.sh
echo "[2.5/4] hex2 — hex1 + absolute/offset address refs (removes the off-by-4 seam)"
step hex2/run.sh
echo "[2.6/4] M0   — token assembler: multi-char labels + %-macros (encode-once), derived against hex2"
step m0/run.sh
echo "[2.7/4] front-end — rz2 -> rzir (M0 resolver) + end-to-end rz2 -> ELF (no cc)"
step frontend/run.sh
echo "[2.8/4] multi-op — folding backend: rz2 -> ELF incl the computed let (no cc)"
step emitter/mop-emitter-check.sh
echo "[2.9/4] codegen — non-constant value (argc): same binary varies at runtime (no cc)"
step emitter/cg-emitter-check.sh
echo "[2.95/4] general codegen — ONE slot-based backend: const+runtime+reuse, rz2->ELF (no cc)"
step emitter/gen-emitter-check.sh
echo "[2.97/4] loops — UNIFIED backend: straight-line + sel + nested loops, one codegen (no cc)"
step emitter/cf-emitter-check.sh
echo "[3/4] emitter — derived rzir->ELF, all 4 flat fixtures"
step emitter/run.sh
echo "[4/4] chain — rz2 -> rzir -> ELF on all 13 Stage2 IR artifacts"
step emitter/pipeline-proof.sh

echo
echo "CHAIN PROVEN: hex0 -> hex1 -> emitter -> ELF, built from two audited seeds, no cc/ld/as."
