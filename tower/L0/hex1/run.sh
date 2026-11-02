#!/usr/bin/env sh
# hex1 verification — uses ONLY the committed hex0 + hex1 binaries. No cc/ld/as.
#
# hex1 = hex0 plus single-char labels (:X define, }X rel32 reference). It is now
# DERIVED from hex1-seed.md (the constraints E1-E9) per Doc 247: every
# instruction byte in derived-hex1.labeled.hex traces to a constraint, and the
# jump displacements are resolved by hex1 itself (E5), so cc is no longer in the
# provenance. The checks below are the seed's V1-V3.
set -eu

cd "$(dirname "$0")"
mkdir -p build
chmod +x hex1.bin
HEX0=../hex0/hex0.bin
chmod +x "$HEX0"

# V3 (E6, P2) self-hosting + compiler-free build: hex1 rebuilds itself from its
# own derived, labeled source.
./hex1.bin < derived-hex1.labeled.hex > build/self.bin
if ! cmp -s hex1.bin build/self.bin; then
  echo "hex1 does not self-host from derived-hex1.labeled.hex" >&2
  exit 1
fi

# V2 (E4-E6) label resolution: a jmp via }A skips an int3 trap and exits 42.
./hex1.bin < fixtures/test-jump.hex1 > build/j.elf
chmod +x build/j.elf
set +e; ./build/j.elf; s="$?"; set -e
if [ "$s" != "42" ]; then echo "hex1 label-jump test exited $s, expected 42" >&2; exit 1; fi

# V1 (E1-E3, E9) hex0 conservativity: plain hex expands exactly like hex0.
./hex1.bin < ../hex0/fixtures/mini-elf.hex > build/m.elf
"$HEX0"   < ../hex0/fixtures/mini-elf.hex > build/m0.elf
if ! cmp -s build/m.elf build/m0.elf; then
  echo "hex1 plain-hex output differs from hex0" >&2
  exit 1
fi
chmod +x build/m.elf
set +e; ./build/m.elf; s="$?"; set -e
if [ "$s" != "42" ]; then echo "hex1 plain-hex test exited $s, expected 42" >&2; exit 1; fi

echo "hex1 verified: DERIVED from constraints, self-hosting, resolves labels, hex0-compatible (no cc/ld/as)"
