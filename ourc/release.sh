#!/bin/bash
# release.sh -- build the ourc release from this repository alone, rooted in the hand-hex seed (V3, keeper option c):
# no pinned binary is used. Every compiler on the way is minted here:
#   1. tower/build-tower.sh (from hex)      hex0 -> ... -> L6 -> SEM0 (cf-minted, SEM0.cf)
#   2. tower/backend-llvm/sem0-stage-llvm.sh  rzir2ll from source -> SEM0.llvm, admitted by C2
#                                              (its emit is byte-identical to the hex-rooted cf frozen's)
#   3. rzir2ll.rs through scripts/semc.sh     r2l.llvm with the mmap heap (ourc's linked rzir is ~19 MB)
#   4. ourc/build-ourc.sh                     ourc.g0, the SEM0-built ourc
#   5. that ourc compiles ourc                ourc.g1 (the release binary), then ourc.g2 from ourc.g1; the IR of both
#                                              self-compiles must be byte-identical and so must the two binaries
#   6. ourc-v0.1-x86_64-linux.tar.gz + SHA256SUMS
# Artifact names: apparatus/docs/tower-naming-and-trust-path.md, "Artifact names".
# Usage:  bash ourc/release.sh OUTDIR        (needs llc, opt, cc; about 20 min on 32 cores, over an hour on 4; needs ~24 GB RAM)
#         REUSE_SP=<dir> bash ourc/release.sh OUTDIR   (skip steps 1-2 when <dir> already holds a C2-admitted mint)
set -uo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"; OUT="${1:?usage: release.sh OUTDIR}"; mkdir -p "$OUT"; OUT="$(cd "$OUT" && pwd)"
VER=v0.1; NAME="ourc-$VER-x86_64-linux"; LLC="${LLC:-llc}"
die() { echo "release: FAIL -- $*" >&2; exit 1; }
for t in "$LLC" opt cc; do command -v "$t" >/dev/null 2>&1 || die "$t not found"; done
ulimit -s unlimited
SP="${REUSE_SP:-$OUT/sp}"; mkdir -p "$SP"
if [ -z "${REUSE_SP:-}" ]; then
  echo "== 1. tower from the hand-hex (SP=$SP)"
  SP="$SP" bash "$ROOT/tower/build-tower.sh" > "$OUT/tower.log" 2>&1 || die "build-tower (see $OUT/tower.log)"
  echo "== 2. SEM0.llvm (C2)"
  # the from-hex environment build-tower sets for its own stages: the L2 backend and M0 derived from the hand-hex
  export TOWER_L2="$SP/seeds/L2" TOWER_M0="$SP/seeds/L0/m0/m0.bin" CC="$SP/seeds/L2"
  SP="$SP" bash "$ROOT/tower/backend-llvm/sem0-stage-llvm.sh" > "$OUT/sem0-llvm.log" 2>&1 || die "sem0-stage-llvm (see $OUT/sem0-llvm.log)"
fi
[ -x "$SP/SEM0.llvm" ] && [ -x "$SP/r2l.llvm.mmap.elf" ] || die "no C2-admitted SEM0.llvm / r2l.llvm.mmap.elf in $SP"
grep -q 'OK' "$OUT/sem0-llvm.log" 2>/dev/null || [ -n "${REUSE_SP:-}" ] || die "sem0-stage-llvm did not report OK"
echo "== 3. r2l.llvm"
SEM0="$SP/SEM0.llvm" R2L="$SP/r2l.llvm.mmap.elf" LLC="$LLC" bash "$ROOT/scripts/semc.sh" "$ROOT/tower/backend-llvm/rzir2ll.rs" -o "$OUT/r2l.llvm" > "$OUT/r2l.log" 2>&1 || die "r2l (see $OUT/r2l.log)"
echo "== 4. ourc.g0, the SEM0-built ourc"
SEM0="$SP/SEM0.llvm" R2L="$OUT/r2l.llvm" LLC="$LLC" OURC_OUT="$OUT/ourc.g0" bash "$ROOT/ourc/build-ourc.sh" > "$OUT/build-ourc.log" 2>&1 || die "build-ourc (see $OUT/build-ourc.log)"
echo "== 5. ourc.g1, ourc.g2 and the fixed point"
W="$OUT/self"; mkdir -p "$W"
OURC="$OUT/ourc.g0" bash "$ROOT/ourc/self-module.sh" "$W" > "$OUT/self-module.log" 2>&1
[ -s "$W/main.rs" ] || die "self-module image not assembled (see $OUT/self-module.log)"
"$OUT/ourc.g0" "$W/main.rs" --emit-ir > "$W/ir1.ll" 2>"$W/ir1.err" || die "the SEM0-built ourc rejected ourc: $(head -c 200 "$W/ir1.err")"
mk() { "$LLC" -O2 -filetype=obj "$1" -o "$1.o" && cc -no-pie "$1.o" -o "$2" -lm; }
mk "$W/ir1.ll" "$W/ourc.g1" || die "link ourc.g1"
"$W/ourc.g1" --mem-limit 0 --time-limit 0 "$W/main.rs" --emit-ir > "$W/ir2.ll" 2>"$W/ir2.err" || die "ourc.g1 rejected ourc"
cmp -s "$W/ir1.ll" "$W/ir2.ll" || die "IR1 != IR2: ourc.g1 compiles ourc differently from ourc.g0"
mk "$W/ir2.ll" "$W/ourc.g2" || die "link ourc.g2"
"$W/ourc.g2" --mem-limit 0 --time-limit 0 "$W/main.rs" --emit-ir > "$W/ir3.ll" 2>/dev/null || die "ourc.g2 rejected ourc"
cmp -s "$W/ir2.ll" "$W/ir3.ll" || die "not a fixed point (IR2 != IR3)"
cmp -s "$W/ourc.g1" "$W/ourc.g2" || die "ourc.g1 and ourc.g2 differ as executables"
echo "== 6. package"
P="$OUT/$NAME"; mkdir -p "$P"
cp "$W/ourc.g1" "$P/ourc"; cp "$ROOT/ourc/README.md" "$ROOT/ourc/LIMITATIONS.md" "$ROOT/LICENSE-MIT" "$ROOT/LICENSE-APACHE" "$P/"
# a reproducible tarball: fixed order, owner, and mtime (the commit's), so two clean builds give the same bytes
MT="$(git -C "$ROOT" log -1 --format=%ct)"
tar --sort=name --owner=0 --group=0 --numeric-owner --mtime="@$MT" -C "$OUT" -cf - "$NAME" | gzip -n > "$OUT/$NAME.tar.gz"
( cd "$OUT" && sha256sum "$NAME/ourc" "$NAME.tar.gz" > SHA256SUMS )
{ echo "commit $(git -C "$ROOT" rev-parse HEAD)"
  # the host tools behind the `host` pins: another LLVM, cc, linker or libc gives a different (equally checked) hash
  echo "toolchain: $("$LLC" --version | grep -o "LLVM version [0-9.]*"); $(cc --version | head -1); $(ld --version | head -1); $(ldd --version | head -1)"
  for f in "$SP/SEM0.cf" "$SP/SEM0.llvm" "$OUT/r2l.llvm" "$OUT/ourc.g0"; do
    [ -f "$f" ] && echo "$(sha256sum "$f" | cut -c1-64)  $(basename "$f")"; done; } > "$OUT/PROVENANCE"
echo "release: OK"; cat "$OUT/SHA256SUMS"
