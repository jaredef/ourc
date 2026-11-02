#!/bin/bash
# cx.sh construct.rs... -- ourc construct-generation probe: each construct (a self-contained, rustc-subsystem-shaped
# program with fn main) is compiled by rustc (golden) and by semc on ourc's own path (//@SUPERSET -> link.semc ->
# r2l -> llc -> cc). Verdict per construct: AGREE / MISCOMPILE (output or exit differs) / REFUSE (UNLOWERED or no
# binary). Discovery mode: the first divergence is the located substrate pipeline (see derivation-mode.md).
set -u
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SEM0="${SEM0:?}"; R2L="${R2L:?}"; LLC="${LLC:?}"
LINK_RS="$ROOT/ourc/rt/link.rs"
RT="$ROOT/ourc/rt/runtime.rzir"
W="${CXW:-$(mktemp -d)}"; mkdir -p "$W"
semcbk(){ rm -f "$W/x.ll" "$W/x.o" "$2"; cat "$1" "$RT" | "$R2L" > "$W/x.ll" 2>/dev/null; "$LLC" -O0 -filetype=obj "$W/x.ll" -o "$W/x.o" 2>/dev/null; cc -no-pie "$W/x.o" -o "$2" -lm 2>/dev/null; chmod +x "$2" 2>/dev/null; }
[ -x "$W/link.semc" ] || { { echo '//@S9'; cat "$LINK_RS"; } | "$SEM0" > "$W/link.rz" 2>/dev/null; semcbk "$W/link.rz" "$W/link.semc"; }
for f in "$@"; do
  n=$(basename "$f" .rs); rm -f "$W/$n.g" "$W/$n.s"
  rustc -O --edition 2021 -A warnings "$f" -o "$W/$n.g" 2>"$W/$n.ge" || { echo "$n GOLDEN-FAIL $(grep -m1 '^error' "$W/$n.ge")"; continue; }
  timeout 60 "$W/$n.g" > "$W/$n.gout" 2>/dev/null; gr=$?
  if [ "${MODE:-S9}" = S9 ]; then { echo '//@S9'; cat "$f"; } | timeout 300 "$SEM0" > "$W/$n.rz" 2>/dev/null
  else { echo "//@SUPERSET crate=$n"; cat "$f"; } | timeout 300 "$SEM0" > "$W/$n.rz" 2>/dev/null; fi
  if grep -q UNLOWERED "$W/$n.rz"; then echo "$n REFUSE $(grep -m1 -o 'UNLOWERED.*' "$W/$n.rz" | cut -c1-120)"; continue; fi
  if [ "${MODE:-S9}" = S9 ]; then cp "$W/$n.rz" "$W/$n.lk.rz"; else timeout 300 "$W/link.semc" < "$W/$n.rz" > "$W/$n.lk.rz" 2>/dev/null; fi; semcbk "$W/$n.lk.rz" "$W/$n.s"
  [ -x "$W/$n.s" ] || { echo "$n REFUSE (no binary: link/r2l/llc)"; continue; }
  timeout 60 "$W/$n.s" > "$W/$n.sout" 2>/dev/null; sr=$?
  if [ $gr = $sr ] && cmp -s "$W/$n.gout" "$W/$n.sout"; then echo "$n AGREE ($(wc -l < "$W/$n.gout")L exit=$gr)"
  else echo "$n MISCOMPILE exit golden=$gr semc=$sr; first diff:"; diff "$W/$n.gout" "$W/$n.sout" | head -4 | sed 's/^/    /'; fi
done
