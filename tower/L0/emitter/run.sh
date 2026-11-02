#!/usr/bin/env sh
# rzir -> ELF emitter (flat 2-const-1-op) — uses ONLY committed hex1 + the
# derived source. No cc/ld/as anywhere.
#
# The emitter is DERIVED from emitter-seed.md (constraints EM1-EM7) and authored
# against hex1's labels. hex1 (itself constraint-derived) expands it to a binary;
# that binary reads rzir and writes a runnable ELF directly. Conformance (EM7):
# the emitted ELF's exit status equals the cc-built emitter's.
set -eu

cd "$(dirname "$0")"
mkdir -p build
HEX1=../hex1/hex1.bin
chmod +x "$HEX1"
S3=../../../locales/stage3-ir-backend/fixtures

# Build the emitter compiler-free (the folding emitter covers the full flat slice;
# rzir-to-elf.labeled.hex was the first 2-const-1-op sub-slice).
"$HEX1" < rzir-to-elf-folding.labeled.hex > build/emitter.bin
chmod +x build/emitter.bin
case "$(file -b build/emitter.bin)" in
  *ELF*executable*x86-64*) : ;;
  *) echo "emitter is not an x86-64 ELF executable" >&2; exit 1 ;;
esac

emit() {
  ir="$1"; want="$2"
  ./build/emitter.bin < "$S3/$ir.rzir" > "build/$ir.elf"
  chmod +x "build/$ir.elf"
  set +e; "./build/$ir.elf"; s="$?"; set -e
  if [ "$s" != "$want" ]; then
    echo "$ir: emitted ELF exited $s, expected $want (cc-path)" >&2; exit 1
  fi
}

emit add           11
emit sub           38
emit renamed-temps 12
emit chained-op     7

echo "rzir -> ELF emitter verified: derived from constraints, expanded by hex1, all 4 flat fixtures match cc path (no cc/ld/as)"
