#!/bin/bash
# gen-fixture.sh -- HOST-FREE regeneration of the tcc preprocessed fixture (tcc.fresh.pp.c).
#
# The derived C preprocessor rzcpp (built by the derived chain, no host cpp) preprocesses tcc's
# ONE_SOURCE amalgamation into the .pp.c that ccc then compiles. This makes the tcc far gate
# reproducible from committed sources: rzcpp.c + the derived header set (include/) + ccc.bin + the
# rzir passes + the M0-assembled cf. The ONLY external input is the tcc-0.9.27 source tree.
#
# Usage:  TCCSRC=/path/to/tcc-0.9.27  bash gen-fixture.sh  [out.pp.c]
#   TCCSRC   the unpacked tcc-0.9.27 source (tcc.c is the ONE_SOURCE entry; needs tcc.h/elf.h/...).
#   out.pp.c defaults to $TCCSRC/../tcc.fresh.pp.c
#
# The config below matches how the committed fixture was made:
#   -DTCC_TARGET_X86_64   select the x86-64 target (PTR_SIZE==8 -> ELFW == ELF64_*). rzcpp does not
#                         predefine __x86_64__, so tcc.h's target auto-detect needs this explicitly.
#   -DONE_SOURCE          tcc.c pulls in libtcc.c/tcctools.c as one translation unit.
#   -DTCC_VERSION="0.9.27" tcc's version string (normally from config.h / the Makefile).
set -eu
HERE="$(cd "$(dirname "$0")" && pwd)"
CC=$HERE/../L2
EMIT=$HERE/../L0/emitter
M0=$HERE/../L0/m0/m0.bin
: "${TCCSRC:?set TCCSRC to the unpacked tcc-0.9.27 source tree}"
OUT=${1:-$TCCSRC/../tcc.fresh.pp.c}
WORK=$(mktemp -d)

# 1. M0-assemble cf (rzir -> ELF), and build rzcpp via the derived chain: rzcpp.bin = cf(ccc(rzcpp.c)).
chmod +x "$M0"
"$M0" < "$EMIT/rzir-to-elf-cf.m0" > "$WORK/cf.bin"; chmod +x "$WORK/cf.bin"
"$CC/ccc.bin"          < "$HERE/rzcpp.c"    > "$WORK/rzcpp.rzir"
"$CC/link/rzir-ld.bin" < "$WORK/rzcpp.rzir" > "$WORK/rzcpp.ld"
"$CC/opt/rzir-opt.bin" < "$WORK/rzcpp.ld"   > "$WORK/rzcpp.opt"
"$CC/ra/rzir-ra.bin"   < "$WORK/rzcpp.opt"  > "$WORK/rzcpp.ra"
"$WORK/cf.bin"         < "$WORK/rzcpp.ra"   > "$WORK/rzcpp.bin"; chmod +x "$WORK/rzcpp.bin"

# 2. Preprocess tcc's ONE_SOURCE amalgamation with the derived rzcpp (cf marshals argv, so -I/-D work).
"$WORK/rzcpp.bin" -DTCC_TARGET_X86_64 -DONE_SOURCE '-DTCC_VERSION="0.9.27"' \
    -I"$TCCSRC" -I"$HERE/include" < "$TCCSRC/tcc.c" > "$OUT"

echo "wrote $OUT ($(wc -l < "$OUT") lines; unexpanded ELFW: $(grep -c 'ELFW(' "$OUT" || true))"
rm -rf "$WORK"
