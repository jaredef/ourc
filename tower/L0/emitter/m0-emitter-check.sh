#!/usr/bin/env sh
# The emitter authored in M0 syntax (named encodings + multi-char labels).
# Chain: hex2 builds M0; M0 builds the emitter; the emitter lowers rzir to ELF.
# Uses only committed M0 (+ hex2 below it). No cc/ld/as. The encoding-error class
# is contained to the %-macro library in rzir-to-elf.m0.
set -eu
cd "$(dirname "$0")"
mkdir -p build
M0=../m0/m0.bin
chmod +x "$M0"
S3=../../../locales/stage3-ir-backend/fixtures

"$M0" < rzir-to-elf.m0 > build/emitter.bin
chmod +x build/emitter.bin
case "$(file -b build/emitter.bin)" in *ELF*executable*x86-64*) : ;; *) echo "not an ELF" >&2; exit 1;; esac

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
# >2 consts: the vN->value table (VALS[10]) resolves operands that skip a vN.
# threeconst: add v0 v2 = 10 + 5 = 15 (v1=20 is dead); the 2-const shortcut
# could not express this.
check threeconst 15

echo "M0-authored emitter verified: named encodings + vN->value table; outputs match cc path (no cc/ld/as)"
