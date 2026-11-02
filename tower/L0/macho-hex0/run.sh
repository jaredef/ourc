#!/usr/bin/env sh
# Mach-O hex0 verification. The seed is an arm64 Mach-O binary that expands
# commented hex from stdin to stdout. No clang/as/ld/codesign signing step is
# used by this runner; codesign is used only as an inspector.
set -eu

cd "$(dirname "$0")"
rm -rf build
mkdir -p build
chmod +x hex0.bin

./hex0.bin < hex0.hand.hex > build/from-source.bin
if ! cmp -s hex0.bin build/from-source.bin; then
  echo "Mach-O hex0 does not rebuild itself from hex0.hand.hex" >&2
  exit 1
fi

./hex0.bin < fixtures/hex0.hex > build/from-hex.bin
if ! cmp -s hex0.bin build/from-hex.bin; then
  echo "Mach-O hex0 is not self-reproducing from fixtures/hex0.hex" >&2
  exit 1
fi

printf '\112' > build/expect-4a.bin
printf '4a' | ./hex0.bin > build/lowercase.bin
if ! cmp -s build/expect-4a.bin build/lowercase.bin; then
  echo "lowercase hex pair did not decode to 0x4a" >&2
  exit 1
fi

printf '4A' | ./hex0.bin > build/uppercase.bin
if [ "$(wc -c < build/uppercase.bin | tr -d ' ')" != "0" ]; then
  echo "uppercase hex unexpectedly decoded" >&2
  exit 1
fi

printf '4' | ./hex0.bin > build/odd-nibble.bin
if [ "$(wc -c < build/odd-nibble.bin | tr -d ' ')" != "0" ]; then
  echo "odd trailing nibble unexpectedly emitted a byte" >&2
  exit 1
fi

printf '4# ignored through newline\na' | ./hex0.bin > build/comment-boundary.bin
if ! cmp -s build/expect-4a.bin build/comment-boundary.bin; then
  echo "comment boundary did not preserve nibble state across newline" >&2
  exit 1
fi

perl -e 'print "aa" x 32768' > build/dense-32768.hex
./hex0.bin < build/dense-32768.hex > build/dense-32768.bin
if [ "$(wc -c < build/dense-32768.bin | tr -d ' ')" != "32768" ]; then
  echo "dense 65536-byte source did not decode to 32768 bytes" >&2
  exit 1
fi

perl -e 'print "aa" x 32769' > build/dense-overread.hex
./hex0.bin < build/dense-overread.hex > build/dense-overread.bin
if [ "$(wc -c < build/dense-overread.bin | tr -d ' ')" != "32768" ]; then
  echo "oversized dense source was not capped by the single read window" >&2
  exit 1
fi

./hex0.bin < fixtures/mini-macho.hex > build/mini-macho
chmod +x build/mini-macho
case "$(file -b build/mini-macho)" in
  *Mach-O*arm64*) : ;;
  *) echo "hex expansion did not yield an arm64 Mach-O executable" >&2; exit 1 ;;
esac
codesign --verify --verbose=4 build/mini-macho >/dev/null 2>&1

cp build/mini-macho build/bad-mini-macho
printf '\000' | dd of=build/bad-mini-macho bs=1 count=1 conv=notrunc >/dev/null 2>&1
if codesign --verify --verbose=4 build/bad-mini-macho >/dev/null 2>&1; then
  echo "corrupted Mach-O fixture unexpectedly passed codesign verification" >&2
  exit 1
fi

set +e
./build/mini-macho
status="$?"
set -e
if [ "$status" != "42" ]; then
  echo "hex-expanded Mach-O exited $status, expected 42" >&2
  exit 1
fi

echo "Mach-O hex0 verified: self-hosting arm64 seed + signed mini-Mach-O exit 42; lowercase/comment/size probes green"
