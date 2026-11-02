#!/bin/bash
# Differential chain fuzzer: gen.py -> UB-free C -> compile+run via the derived chain AND gcc; compare
# exit codes (mod 256). Reports divergences / chain compile-fails / chain crashes as repros in $WORK.
# Found R97 (deref pointer-stride) and, with hand-reduction, seeded R98/R99. Usage: bash run.sh [N] [START]
set -u
HERE="$(cd "$(dirname "$0")" && pwd)"
CC="$HERE/.."
EMIT="$CC/../L0/emitter"
M0="$CC/../L0/m0/m0.bin"
WORK="${WORK:-$(mktemp -d)}"; mkdir -p "$WORK"
N=${1:-200}; START=${2:-0}
chmod +x "$M0"; "$M0" < "$EMIT/rzir-to-elf-cf.m0" > "$WORK/cf.bin"; chmod +x "$WORK/cf.bin"
chain() {
  "$CC/ccc.bin"          < "$1"          > "$WORK/_c.rzir" 2>/dev/null || return 10
  "$CC/link/rzir-ld.bin" < "$WORK/_c.rzir" > "$WORK/_c.ld" 2>/dev/null || return 11
  "$CC/opt/rzir-opt.bin" < "$WORK/_c.ld"   > "$WORK/_c.opt" 2>/dev/null || return 12
  "$CC/ra/rzir-ra.bin"   < "$WORK/_c.opt"  > "$WORK/_c.ra"  2>/dev/null || return 13
  "$WORK/cf.bin"         < "$WORK/_c.ra"   > "$2" 2>/dev/null || return 14
  chmod +x "$2"
}
div=0; tested=0; cfail=0; ccrash=0
for ((s=START; s<START+N; s++)); do
  python3 "$HERE/gen.py" "$s" > "$WORK/p.c" 2>/dev/null || continue
  gcc -w -O0 "$WORK/p.c" -o "$WORK/p.gcc" 2>/dev/null || continue
  "$WORK/p.gcc"; gexit=$?
  chain "$WORK/p.c" "$WORK/p.elf"; cr=$?
  if [ $cr -ge 10 ]; then cfail=$((cfail+1)); echo "SEED $s: CHAIN-COMPILE-FAIL (stage $cr)"; cp "$WORK/p.c" "$WORK/fail_$s.c"; continue; fi
  "$WORK/p.elf"; cexit=$?
  if [ $cexit -ge 128 ] && [ $cexit -ne $gexit ]; then ccrash=$((ccrash+1)); echo "SEED $s: CHAIN-CRASH exit=$cexit (gcc=$gexit)"; cp "$WORK/p.c" "$WORK/crash_$s.c"; continue; fi
  tested=$((tested+1))
  if [ "$cexit" != "$gexit" ]; then div=$((div+1)); echo "SEED $s: DIVERGE chain=$cexit gcc=$gexit"; cp "$WORK/p.c" "$WORK/div_$s.c"; fi
done
echo "==== $N seeds from $START: tested=$tested diverge=$div compile-fail=$cfail crash=$ccrash (repros in $WORK) ===="
