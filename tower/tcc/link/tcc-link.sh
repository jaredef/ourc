#!/bin/bash
# tcc-link.sh -- link tcc into a runnable ELF via the derived chain.
# CORRECTED: tcc.h defines ONE_SOURCE=1 by default, so tcc.c is a FULL amalgamation
# (main + libtcc + tccpp/tccgen/tccelf/tccasm/x86_64-gen/i386-asm/tccrun). Compiling all 9
# units separately produced DUPLICATE definitions of every function -> the linker picked
# arbitrary copies. The correct program is the single tcc.c amalgamation + rzlibc + bridge.
set -e
# Regenerate $SP/tcc.fresh.pp.c host-free first with: TCCSRC=<tcc-0.9.27> bash ../gen-fixture.sh $SP/tcc.fresh.pp.c
SP=${SP:?set SP to the scratchpad dir holding cf.bin + tcc.fresh.pp.c}
CC="${TOWER_L2:-$(cd "$(dirname "$0")/../.." && pwd)/L2}"
RZL=$CC/../2026-07-02-rzlibc
LNK=$(cd "$(dirname "$0")" && pwd)

# R1 (c99-closure): rzcpp-preprocess the vendored soft-float (macros/#include -> ccc-ready) so the
# bridge's real strtod can build the double bit pattern in pure integer arithmetic. rzcpp.bin is built
# by the derived chain (host-free), exactly as gen-fixture.sh does.
"$CC/ccc.bin" < "$LNK/../rzcpp.c" | "$CC/link/rzir-ld.bin" | "$CC/opt/rzir-opt.bin" | "$CC/ra/rzir-ra.bin" | "$SP/cf.bin" > "$SP/rzcpp.bin"; chmod +x "$SP/rzcpp.bin"
"$SP/rzcpp.bin" -I"$LNK" < "$LNK/softfloat.c" > "$SP/softfloat.pp.c"

> $SP/amalg.rzir
"$CC/ccc.bin" < "$SP/tcc.fresh.pp.c" >> $SP/amalg.rzir                        # the whole program (ONE_SOURCE)
for u in rzstring rzctype rzmalloc rzstdio rzprintf rzproc; do "$CC/ccc.bin" < "$RZL/$u.c" >> $SP/amalg.rzir; done
"$CC/ccc.bin" < "$SP/softfloat.pp.c" >> $SP/amalg.rzir                        # R1: integer soft-float (strtod/float-const)
"$CC/ccc.bin" < "$LNK/tcc-bridge.c" >> $SP/amalg.rzir
cat "$LNK/bogus-if.rzir" >> $SP/amalg.rzir

"$CC/link/rzir-ld.bin"  < $SP/amalg.rzir    > $SP/amalg.linked   # derived C linker (was rzir-ld.py) -- host-free
"$CC/opt/rzir-opt.bin"  < $SP/amalg.linked  > $SP/amalg.opt      # derived C optimizer (was rzir-opt.py) -- host-free
"$CC/ra/rzir-ra.bin"    < $SP/amalg.opt     > $SP/amalg.ra       # derived C register allocator (was rzir-ra.py) -- host-free
"$SP/cf.bin"                  < $SP/amalg.ra      > $SP/tcc.elf
chmod +x $SP/tcc.elf
echo "built $SP/tcc.elf ($(stat -c%s $SP/tcc.elf) B)"
