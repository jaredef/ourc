#!/bin/bash
# three-way/compare.sh -- differential over gcc vs stock tcc-0.9.27 vs the derived tcc.
# For a self-contained C program (no #includes, or already preprocessed), each path is
# compile -> .o -> link (cc) -> run; the harness compares exit codes (and stdout if the
# program prints). A DIVERGENCE is a real bug in one of the three.
#
#   GCC=gcc  STOCK=<stock tcc-0.9.27 binary>  DTCC=<derived tcc.elf>  compare.sh <name> <src> [args]
#
# Build the reference stock tcc:  (cd tcc-0.9.27 && ./configure --cc=gcc && make tcc)
# Build the derived tcc:          SP=<scratchpad> bash ../link/tcc-link.sh
set -u
HERE=$(cd "$(dirname "$0")" && pwd)
: "${STOCK:?set STOCK to stock tcc-0.9.27}" "${DTCC:?set DTCC to derived tcc.elf}"
GCC=${GCC:-gcc}; RUN="$HERE/../link/safe-run.sh"; W=$(mktemp -d)
name="$1"; printf '%s' "$2" > "$W/t.c"; shift 2; args=("$@")
run(){ "$1" "${args[@]}" >"$2" 2>&1; echo $?; }
$GCC -w -O0 "$W/t.c" -o "$W/g" 2>/dev/null && G=$(run "$W/g" "$W/g.out") || G=CC
$STOCK -c "$W/t.c" -o "$W/s.o" 2>/dev/null && cc "$W/s.o" -o "$W/s" 2>/dev/null && S=$(run "$W/s" "$W/s.out") || S=CC
"$RUN" "$DTCC" -c "$W/t.c" -o "$W/d.o" 2>/dev/null; chmod +r "$W/d.o" 2>/dev/null
cc "$W/d.o" -o "$W/d" 2>/dev/null && D=$(run "$W/d" "$W/d.out") || D=CC
ok="ok"
[ "$G" = "$S" ] && [ "$S" = "$D" ] || ok="** EXIT DIVERGE"
if [ -f "$W/g.out" ] && [ -f "$W/d.out" ]; then diff -q "$W/g.out" "$W/d.out" >/dev/null 2>&1 || ok="** STDOUT DIVERGE"; fi
printf '%-22s gcc=%-4s stock=%-4s derived=%-4s %s\n' "$name" "$G" "$S" "$D" "$ok"
rm -rf "$W"
