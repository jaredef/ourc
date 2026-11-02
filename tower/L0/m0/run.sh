#!/usr/bin/env sh
# M0 slice 1 verification — uses ONLY committed hex2 (+ hex0/hex1 below it). No cc.
#
# M0 = a token assembler with MULTI-CHAR labels and =/&/} refs, two-pass, derived
# from m0-seed.md and authored against hex2 (so M0 itself has no hand-computed
# addresses). Slice 1 is the tokenizer + label symbol table; macros are slice 2.
set -eu
cd "$(dirname "$0")"
mkdir -p build
HEX2=../hex2/hex2.bin
chmod +x "$HEX2"

# Build M0 compiler-free.
"$HEX2" < m0.labeled.hex > m0.bin
chmod +x m0.bin
case "$(file -b m0.bin)" in *ELF*executable*x86-64*) : ;; *) echo "m0 not an ELF" >&2; exit 1;; esac

run() {
  f="$1"; want="$2"
  ./m0.bin < "fixtures/$f" > "build/${f}.elf"
  chmod +x "build/${f}.elf"
  set +e; "./build/${f}.elf"; s="$?"; set -e
  [ "$s" = "$want" ] || { echo "$f: exit $s, expected $want" >&2; exit 1; }
}

# V1  multi-char labels via = (absolute entry) and & (offset filesz).
run test-label.m0 42
# V1b multi-char label via } (rel32 jump over an int3 trap).
run test-jump.m0  42
# V2  %-macros: define instruction encodings once, reuse by name -> exit 42.
run test-macro.m0 42
# V2  the macro output is byte-identical to the equivalent raw-hex program.
./m0.bin < fixtures/test-macro.m0     > build/macro.elf
./m0.bin < fixtures/test-macro-raw.m0 > build/raw.elf
cmp -s build/macro.elf build/raw.elf || { echo "macro output != raw-hex output" >&2; exit 1; }

# V3 (M8) reject the unknown: a bare token that is neither a macro nor valid
# even-length hex exits 3, not a silent garbage decode.
err() {
  printf '%s\n' "$1" > build/t.m0
  set +e; ./m0.bin < build/t.m0 >/dev/null 2>&1; s="$?"; set -e
  [ "$s" = "$2" ] || { echo "M8: '$1' exited $s, expected $2" >&2; exit 1; }
}
err 'jne' 3      # undefined macro (non-hex chars)
err '48c' 3      # odd-length hex
err '4889' 0     # valid hex passes

echo "M0 verified: multi-char labels + =/&/} refs + %-macros + M8 reject-unknown; no cc/ld/as"
