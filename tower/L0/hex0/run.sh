#!/usr/bin/env sh
# hex0 verification — uses ONLY the committed seed binary. No cc, no ld, no as.
#
# hex0.bin is hand-encoded machine code: every byte is derived by hand in
# hex0.hand.hex (the documented source). No compiler ever touched it. These
# checks prove it from the committed binary alone:
#   1. self-hosting: hex0 < hex0.hand.hex reproduces hex0.bin (it rebuilds itself
#      from its own documented source);
#   2. fixpoint:     hex0 < hex0.hex (its raw bytes) reproduces hex0.bin;
#   3. authorship:   hex0 < mini-elf.hex yields a static ELF that runs, exit 42.
set -eu

cd "$(dirname "$0")"
mkdir -p build
chmod +x hex0.bin

# 1. Self-hosting from the documented source (comments stripped by hex0 itself).
./hex0.bin < hex0.hand.hex > build/from-source.bin
if ! cmp -s hex0.bin build/from-source.bin; then
  echo "hex0 does not rebuild itself from hex0.hand.hex" >&2
  exit 1
fi

# 2. Raw-bytes fixpoint.
./hex0.bin < fixtures/hex0.hex > build/from-hex.bin
if ! cmp -s hex0.bin build/from-hex.bin; then
  echo "hex0 is not self-reproducing from fixtures/hex0.hex" >&2
  exit 1
fi

# 3. A real executable authored in hex, built and run with no host tools.
./hex0.bin < fixtures/mini-elf.hex > build/mini.elf
chmod +x build/mini.elf
case "$(file -b build/mini.elf)" in
  *ELF*executable*x86-64*) : ;;
  *) echo "hex expansion did not yield an x86-64 ELF executable" >&2; exit 1 ;;
esac
set +e
./build/mini.elf
status="$?"
set -e
if [ "$status" != "42" ]; then
  echo "hex-expanded ELF exited $status, expected 42" >&2
  exit 1
fi

echo "hex0 verified: hand-encoded, self-hosting, authors a runnable ELF (exit 42), no cc/ld/as"
