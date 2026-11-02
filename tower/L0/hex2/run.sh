#!/usr/bin/env sh
# hex2 verification — uses ONLY committed hex0 + hex1 + hex2 binaries. No cc/ld/as.
#
# hex2 = hex1 + absolute (=X -> BASE+offset) and offset (&X -> offset) label refs,
# DERIVED from hex2-seed.md against hex1's labels. It removes hand-computed
# absolute addresses (the off-by-4 seam) and the measuring build.
set -eu

cd "$(dirname "$0")"
mkdir -p build
chmod +x hex2.bin
HEX1=../hex1/hex1.bin
chmod +x "$HEX1"
HEX0=../hex0/hex0.bin
chmod +x "$HEX0"

# V3 self-host + compiler-free build: hex2 rebuilds itself from its derived source.
./hex2.bin < derived-hex2.labeled.hex > build/self.bin
cmp -s hex2.bin build/self.bin || { echo "hex2 does not self-host" >&2; exit 1; }

# V2 it is still a hex1: resolve a rel32 label jump (int3-skip -> exit 42).
./hex2.bin < ../hex1/fixtures/test-jump.hex1 > build/j.elf
chmod +x build/j.elf
set +e; ./build/j.elf; s="$?"; set -e
[ "$s" = "42" ] || { echo "hex2 label-jump exited $s, expected 42" >&2; exit 1; }

# V1 hex0-compat: plain hex expands exactly like hex0.
./hex2.bin < ../hex0/fixtures/mini-elf.hex > build/m.elf
"$HEX0"   < ../hex0/fixtures/mini-elf.hex > build/m0.elf
cmp -s build/m.elf build/m0.elf || { echo "hex2 plain-hex differs from hex0" >&2; exit 1; }

# V4 absolute/offset refs: an ELF with =entry and &filesz, no hand-computed addr.
./hex2.bin < fixtures/test-abs.hex2 > build/abs.elf
chmod +x build/abs.elf
case "$(file -b build/abs.elf)" in
  *ELF*executable*x86-64*) : ;;
  *) echo "hex2 =/& test did not yield a valid ELF" >&2; exit 1 ;;
esac
set +e; ./build/abs.elf; s="$?"; set -e
[ "$s" = "42" ] || { echo "hex2 =/& test exited $s, expected 42" >&2; exit 1; }

echo "hex2 verified: derived, self-hosting, hex1-compatible, resolves = (abs) and & (offset) refs (no cc/ld/as)"
