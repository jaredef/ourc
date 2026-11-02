#!/bin/bash
# Our-compiler composition gate: N crates WE authored, linked by OUR semc-built linker, built by OUR backend.
# Oracle: the semc-built composed compiler == the rustc-built golden (byte-identical output).
set -u
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SEM0="${SEM0:?}"; R2L="${R2L:?}"; LLC="${LLC:?}"
LINK_RS="$ROOT/ourc/rt/link.rs"
RT="$ROOT/ourc/rt/runtime.rzir"
S="$(cd "$(dirname "$0")" && pwd)/src"
CRATES="${CRATES:-ospan oerrors oindex oarena oabi olex oast oresolve ohir olower omir omirtrans ocodegen osession oparse otypeck omono oborrowck}"   # deps in order; odrv is the entry
W="$(mktemp -d)"; trap 'rm -rf "$W"' EXIT
semcbk(){ cat "$1" "$RT" | "$R2L" > "$W/x.ll" 2>/dev/null; "$LLC" -O2 -filetype=obj "$W/x.ll" -o "$W/x.o" 2>/dev/null; cc -no-pie "$W/x.o" -o "$2" -lm 2>/dev/null; chmod +x "$2"; }
# ---- rustc golden: crates as modules + flat `use` so the same source compiles both ways
SEDP=""; for d in $CRATES; do SEDP="$SEDP s/\\b$d::/crate::$d::/g;"; done
{ for c in $CRATES; do echo "mod $c {"; for d in $CRATES; do [ "$d" = "$c" ] || echo "    #[allow(unused_imports)] use crate::$d::*;"; done; sed "$SEDP" "$S/$c.rs"; echo "}"; done
  for c in $CRATES; do echo "#[allow(unused_imports)] use $c::*;"; done
  sed "$SEDP" "$S/odrv.rs"
} > "$W/golden.rs"
rustc -O --edition 2021 "$W/golden.rs" -o "$W/golden" 2>"$W/ge" || { echo "FAIL rustc golden:"; grep -m6 '^error' "$W/ge"; exit 1; }
"$W/golden" > "$W/golden.out"
# ---- semc: linker, then each crate //@METADATA + //@SUPERSET, then combine
{ echo '//@S9'; cat "$LINK_RS"; } | "$SEM0" > "$W/link.rz" 2>/dev/null; semcbk "$W/link.rz" "$W/link.semc"
: > "$W/mh.txt"
for c in $CRATES; do { echo "//@METADATA crate=$c"; cat "$W/mh.txt"; cat "$S/$c.rs"; } | "$SEM0" 2>/dev/null | grep '^META ' | sed 's/^META /\/\/META /' >> "$W/mh.txt"; done
for c in $CRATES; do { echo "//@SUPERSET crate=$c"; cat "$W/mh.txt"; cat "$S/$c.rs"; } | "$SEM0" > "$W/$c.rz" 2>/dev/null; done
{ echo '//@SUPERSET crate=odrv'; cat "$W/mh.txt"; cat "$S/odrv.rs"; } | "$SEM0" > "$W/odrv.rz" 2>/dev/null
for c in $CRATES odrv; do grep -q UNLOWERED "$W/$c.rz" && { echo "FAIL: $c UNLOWERED"; grep UNLOWERED "$W/$c.rz"|sort -u|head -3; exit 1; }; done
{ cat "$W/odrv.rz"; for c in $CRATES; do echo '@@@CRATE@@@'; cat "$W/$c.rz"; done; } | timeout 300 "$W/link.semc" > "$W/linked.rz" 2>/dev/null
[ -s "$W/linked.rz" ] || { echo "FAIL: linker empty"; exit 1; }
for c in $CRATES; do if grep -q "call $c::" "$W/linked.rz"; then echo "FAIL: dangling $c:: calls"; grep -m3 "call $c::" "$W/linked.rz"; exit 1; fi; done
semcbk "$W/linked.rz" "$W/ourc"
"$W/ourc" > "$W/semc.out" 2>/dev/null
echo "--- golden ---"; cat "$W/golden.out"
echo "--- ourc (semc-composed) ---"; cat "$W/semc.out"
[ -s "$W/golden.out" ] || { echo "FAIL: golden produced no output (our code bug)"; exit 1; }
[ -s "$W/semc.out" ] || { echo "FAIL: ourc produced no output"; exit 1; }
cmp -s "$W/golden.out" "$W/semc.out" || { echo "FAIL: differs"; diff "$W/golden.out" "$W/semc.out" | head -8; exit 1; }
# END-TO-END: take the IR OUR compiler emitted and build+run it -- exit code must equal the computed value
sed -n '/^--- IR for prog0 ---$/,$p' "$W/semc.out" | tail -n +2 > "$W/prog0.ll"
if [ -s "$W/prog0.ll" ]; then
  "$LLC" -O2 -filetype=obj "$W/prog0.ll" -o "$W/prog0.o" 2>/dev/null && cc -no-pie "$W/prog0.o" -o "$W/prog0" 2>/dev/null
  if [ -x "$W/prog0" ]; then "$W/prog0"; rc=$?; want=$(grep -m1 '^\[0\] ok' "$W/semc.out" | sed 's/.*= \([0-9-]*\) .*/\1/')
    [ "$rc" = "$want" ] && echo "  ok   END-TO-END: ourc-emitted IR builds+RUNS, exit=$rc == computed $want" || { echo "FAIL: ran exit=$rc want=$want"; exit 1; }
  else echo "FAIL: ourc IR did not build"; exit 1; fi
fi
echo "  ok   composition: $(echo $CRATES odrv | wc -w) crates, semc-built == rustc golden (byte-identical)"
