#!/usr/bin/env bash
# climb.sh -- reconstruct today's ccc from the HAND-HEX seed, cc/ld-free, via the PRESERVED GIT LADDER.
#
#   hex0/hex1 -> hex2 -> m0 -> cf         (chain-proof.sh; hex-derived assembler + backend)
#   m0 < c-to-rzir.m0    -> M0cc          (the hex-derived M0 C compiler; compiles narrow C)
#   M0cc | cf < ccc@self-host             -> ccc_0   (the 285-line self-hosting ccc; M0cc CAN compile it)
#   ccc_0 is FIXED-ADDRESS, so ccc_0 | cf < rzir-ld.c -> ld   (escapes the "building ld needs ld" chicken-egg)
#   greedy climb: the current ccc compiles the HIGHEST git rung it can into a working ccc (ccc|cf, or
#   ccc|ld|cf once the compiler emits gdef/symbolic globals), jump, repeat, until it reaches today's ccc.c.
#
# The rungs are the ccc.c commits from the self-hosting fixpoint (3e6e4a6) to HEAD -- extracted from git on
# demand, so nothing is duplicated. Asserts the reconstructed ccc == the committed ccc.bin on ccc.c.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../.." && pwd)"
MC="$ROOT/tower/L0"
CP="$ROOT/tower/L1/c-frontend"
CCPATH="apparatus/arcs/L2/ccc.c"
LDC="$ROOT/tower/L2/link/rzir-ld.c"
SELFHOST=3e6e4a6                                   # the first self-hosting ccc.c commit (climb starts here)
W="${SP:-/tmp}/ccc-climb"; rm -rf "$W"; mkdir -p "$W"
cd "$ROOT"

# --- hex floor: M0 assembler -> cf + M0cc ---
M0="$MC/m0/m0.bin"
"$M0" < "$MC/emitter/rzir-to-elf-cf.m0" > "$W/cf" 2>/dev/null; chmod +x "$W/cf"; CF="$W/cf"
( cd "$CP" && "$M0" < c-to-rzir.m0 > "$W/m0cc" 2>/dev/null ); chmod +x "$W/m0cc"
echo "hex floor: cf $(wc -c <"$CF") B, M0cc $(wc -c <"$W/m0cc") B"

# --- the rung ladder: ccc.c commits from self-host to HEAD, then the working-tree ccc.c as the final rung ---
mapfile -t RUNG < <(git log --reverse --format='%h' -- "$CCPATH" 2>/dev/null | sed -n "/$SELFHOST/,\$p")
N=${#RUNG[@]}                                       # committed rungs; the working tree is rung N (final)
echo "ladder rungs: $((N+1)) (self-host ${RUNG[0]} .. working-tree ccc.c)"
rung_src() { # $1 = rung index -> writes the source to $W/rung.c
  if [ "$1" -lt "$N" ]; then git show "${RUNG[$1]}:$CCPATH" > "$W/rung.c" 2>/dev/null
  else cp "$ROOT/$CCPATH" "$W/rung.c"; fi
}

LD=""
compile() { # $1 = compiler, $2 = source .c, $3 = out ; 0 iff out is a WORKING ccc
  "$1" < "$2" > "$W/o.rzir" 2>/dev/null; [ -s "$W/o.rzir" ] || return 1
  if grep -q '^gdef' "$W/o.rzir"; then [ -n "$LD" ] || return 3; "$LD" < "$W/o.rzir" 2>/dev/null | "$CF" > "$3" 2>/dev/null
  else "$CF" < "$W/o.rzir" > "$3" 2>/dev/null; fi
  chmod +x "$3" 2>/dev/null
  printf 'int main(){return 5;}' | timeout 8 "$3" > "$W/probe.rzir" 2>/dev/null; [ -s "$W/probe.rzir" ]
}

# seed ccc_0 from the hand-hex, and build ld with the fixed-address ccc_0 (unlocks the gdef regime)
rung_src 0; compile "$W/m0cc" "$W/rung.c" "$W/ccc" || { echo "SEED FAILED"; exit 1; }
echo "ccc_0 built from hand-hex (M0cc | cf)"
if "$W/ccc" < "$LDC" 2>/dev/null | "$CF" > "$W/ld" 2>/dev/null && chmod +x "$W/ld" && [ -s "$W/ld" ]; then
  LD="$W/ld"; echo "ld built from rzir-ld.c via fixed-address ccc_0 ($(wc -c <"$W/ld") B) -- gdef regime unlocked"
fi

# greedy climb
cur=0; jumps=0
while [ "$cur" -lt "$N" ]; do
  hi=$cur
  for k in $(seq $((cur+1)) "$N"); do rung_src "$k"; if compile "$W/ccc" "$W/rung.c" "$W/ccc.try"; then hi=$k; else break; fi; done
  [ "$hi" -eq "$cur" ] && { echo "  stuck at rung $cur"; break; }
  rung_src "$hi"; compile "$W/ccc" "$W/rung.c" "$W/ccc.next" && mv "$W/ccc.next" "$W/ccc"
  jumps=$((jumps+1)); echo "  jump $jumps: rung $cur -> rung $hi ($(wc -l <"$W/rung.c")L)"
  cur=$hi
done

echo
if [ "$cur" -eq "$N" ]; then
  "$W/ccc" < "$ROOT/$CCPATH" > "$W/recon.rzir" 2>/dev/null
  "$ROOT/tower/L2/ccc.bin" < "$ROOT/$CCPATH" > "$W/committed.rzir" 2>/dev/null
  if cmp -s "$W/recon.rzir" "$W/committed.rzir"; then
    echo "CCC RECONSTRUCTED FROM HAND-HEX in $jumps jumps, cc/ld-free."
    echo "FIXPOINT: reconstructed ccc == committed ccc.bin on ccc.c ($(wc -l <"$W/recon.rzir") rzir lines, byte-identical)."
    cp "$W/ccc" "$W/ccc.hex"; echo "reconstructed ccc -> $W/ccc.hex"
    exit 0
  fi
  echo "reached HEAD but reconstructed ccc DIVERGES from committed ccc.bin"; exit 1
fi
echo "CLIMB PARTIAL: reached rung $cur / $N in $jumps jumps."; exit 1