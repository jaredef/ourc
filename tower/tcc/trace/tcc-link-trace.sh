#!/bin/bash
# tcc-link-trace.sh -- build a TRACED derived tcc: the tcc unit is piped through rzir-trace
# (shadow-stack hooks + fid map), tcc_error calls __tbacktrace() before longjmp, and the trace
# runtime is linked in. Needs SP with cf.bin + tcc.fresh.pp.c (as tcc-link.sh). Writes:
#   $SP/tcc.trace.elf   the traced tcc (function names are embedded; the backtrace is self-decoding)
set -e
SP=${SP:?set SP to the scratchpad (cf.bin + tcc.fresh.pp.c)}
HERE=$(cd "$(dirname "$0")" && pwd)
CC="${TOWER_L2:-$(cd "$(dirname "$0")/../.." && pwd)/L2}"
RZL=$CC/../2026-07-02-rzlibc
LNK=$HERE/../link
TRACE=${TRACE:?set TRACE to rzir-trace.bin}
# 1. inject __tbacktrace() into tcc_error + shadow-stack reset around the setjmp
python3 - "$SP/tcc.fresh.pp.c" "$SP/tcc.trace.pp.c" <<'PY'
import sys
src=open(sys.argv[1]).read()
src=src.replace("        longjmp(s1->error_jmp_buf, 1);",
                "        __tbacktrace(); longjmp(s1->error_jmp_buf, 1);",1)
src=src.replace("    if (setjmp(s1->error_jmp_buf) == 0) {",
                "    { int __td=__tdepth(); if (setjmp(s1->error_jmp_buf) != 0) __treset(__td); }\n    if (setjmp(s1->error_jmp_buf) == 0) {",1)
# `--trace`: scan argv, enable the verbose execution log, and strip the flag so tcc's own
# parser never sees it (dropped, not passed on). Anchored on main's unique `FILE *ppfp = 1;`
# + its own `redo:` (there are several redo: labels; this targets main's). Uses direct
# argv0[i][j] (the double-subscript ccc bug this scan first surfaced is now fixed, R116).
anchor="    FILE *ppfp = 1;\n\nredo:\n    argc = argc0, argv = argv0;"
scan=("    FILE *ppfp = 1;\n\n"
      "    { int __ti; int __tj; __tj=1;\n"
      "      for (__ti=1; __ti<argc0; __ti=__ti+1) {\n"
      "        if (argv0[__ti][0]==45 && argv0[__ti][1]==45 && argv0[__ti][2]==116 && argv0[__ti][3]==114 && argv0[__ti][4]==97 && argv0[__ti][5]==99 && argv0[__ti][6]==101 && argv0[__ti][7]==0) { __tverbose(1); }\n"
      "        else { argv0[__tj]=argv0[__ti]; __tj=__tj+1; } }\n"
      "      argc0=__tj; }\n"
      "redo:\n    argc = argc0, argv = argv0;")
assert anchor in src, "main redo: anchor not found"
src=src.replace(anchor, scan, 1)
open(sys.argv[2],"w").write("int __tbacktrace();int __tdepth();int __treset(int);int __tverbose(int);\n"+src)
PY
# 2. assemble: tcc unit THROUGH rzir-trace; runtime + rzlibc + bridge with NORMAL ccc
> "$SP/amalg.trace.rzir"
"$CC/ccc.bin" < "$SP/tcc.trace.pp.c" | "$TRACE" >> "$SP/amalg.trace.rzir"
"$CC/ccc.bin" < "$HERE/trace-runtime.c" >> "$SP/amalg.trace.rzir"
for u in rzstring rzctype rzmalloc rzstdio rzprintf rzproc; do "$CC/ccc.bin" < "$RZL/$u.c" >> "$SP/amalg.trace.rzir"; done
"$CC/ccc.bin" < "$LNK/tcc-bridge.c" >> "$SP/amalg.trace.rzir"
cat "$LNK/bogus-if.rzir" >> "$SP/amalg.trace.rzir"
"$CC/link/rzir-ld.bin" < "$SP/amalg.trace.rzir" > "$SP/amalg.trace.ld"
"$CC/opt/rzir-opt.bin" < "$SP/amalg.trace.ld"  > "$SP/amalg.trace.opt"
"$CC/ra/rzir-ra.bin"   < "$SP/amalg.trace.opt" > "$SP/amalg.trace.ra"
"$SP/cf.bin"           < "$SP/amalg.trace.ra"  > "$SP/tcc.trace.elf"
chmod +x "$SP/tcc.trace.elf"
echo "built $SP/tcc.trace.elf ($(stat -c%s "$SP/tcc.trace.elf") B) -- names embedded, self-decoding"
