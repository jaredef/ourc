#!/bin/bash
# Build `ourc` ONLY -- the semc path, no test suite. Extracted from cli.sh so a derivation rung can get a
# compiler to probe with WITHOUT paying for 165 programs or a ui-gauge sweep. One rule, two entry points:
# cli.sh still builds the same way and then runs its suite.
set -u
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SEM0="${SEM0:?}"; R2L="${R2L:?}"; LLC="${LLC:?}"
LINK_RS="$ROOT/ourc/rt/link.rs"
RT="$ROOT/ourc/rt/runtime.rzir"
S="$(cd "$(dirname "$0")" && pwd)/src"
CRATES="${CRATES:-ospan oerrors oindex oarena oabi olex oast oresolve ohir olower omir omirtrans ocodegen osession oparse otypeck omono oborrowck}"
W="$(mktemp -d)"; trap 'rm -rf "$W"' EXIT
# STACK-M2R (ourc surface arc): r2l emits one alloca per virtual register and llc does not promote them, so a big recursive fn
# (olower::lower: 29,798 allocas = ~238 KB/frame) overflowed the 8 MB stack after ~30 nested `let`s. mem2reg promotes them;
# falls back to the plain llc path when `opt` is absent.
OPT="${OPT:-$(command -v opt || true)}"
semcbk(){ cat "$1" "$RT" | "$R2L" > "$W/x.ll" 2>/dev/null; if [ -n "$OPT" ] && "$OPT" -passes=mem2reg "$W/x.ll" -o "$W/x.bc" 2>/dev/null; then "$LLC" -O2 -filetype=obj "$W/x.bc" -o "$W/x.o" 2>/dev/null; else "$LLC" -O2 -filetype=obj "$W/x.ll" -o "$W/x.o" 2>/dev/null; fi; cc -no-pie "$W/x.o" -o "$2" 2>/dev/null; chmod +x "$2"; }
{ echo '//@S9'; cat "$LINK_RS"; } | "$SEM0" > "$W/link.rz" 2>/dev/null; semcbk "$W/link.rz" "$W/link.semc"
: > "$W/mh.txt"
for c in $CRATES; do { echo "//@METADATA crate=$c"; cat "$W/mh.txt"; cat "$S/$c.rs"; } | "$SEM0" 2>/dev/null | grep '^META ' | sed 's/^META /\/\/META /' >> "$W/mh.txt"; done
# a crate SEM0 REJECTS (exit != 0, e.g. E0560) fails the build with SEM0's own message -- it was discarded, and the link
# went on to produce a broken compiler
for c in $CRATES; do { echo "//@SUPERSET crate=$c"; cat "$W/mh.txt"; cat "$S/$c.rs"; } | "$SEM0" > "$W/$c.rz" 2>"$W/$c.sem0err" || { echo "FAIL build-ourc: SEM0 rejected crate $c: $(tail -1 "$W/$c.sem0err")" >&2; exit 1; }; done
{ echo '//@SUPERSET crate=odrv'; cat "$W/mh.txt"; cat "$S/odrv.rs"; } | "$SEM0" > "$W/odrv.rz" 2>"$W/odrv.sem0err" || { echo "FAIL build-ourc: SEM0 rejected crate odrv: $(tail -1 "$W/odrv.sem0err")" >&2; exit 1; }
{ cat "$W/odrv.rz"; for c in $CRATES; do echo '@@@CRATE@@@'; cat "$W/$c.rz"; done; } | timeout 300 "$W/link.semc" > "$W/linked.rz" 2>/dev/null
semcbk "$W/linked.rz" "$W/ourc"
# a crate SEM0 rejects (its stderr is discarded above) leaves a STUB that still links: refuse an undersized image and name
# the crates that failed, instead of "building" a compiler that rejects every program
sz=$(stat -c%s "$W/ourc" 2>/dev/null || echo 0)
if [ "$sz" -lt 500000 ]; then   # (STACK-M2R: the promoted image is ~1.07 MB; a stub is far smaller) echo "FAIL build-ourc: image is $sz bytes (stub) -- SEM0 rejected:" >&2
  for c in $CRATES odrv; do { echo "//@SUPERSET crate=$c"; cat "$S/$c.rs"; } | "$SEM0" >/dev/null 2>"$W/$c.err" || echo "  $c: $(tail -1 "$W/$c.err")" >&2; done; exit 1; fi
# SMOKE: an image can link and still be broken (a // comment appended mid-line silently dropped code twice): it must
# compile a trivial program before it is installed
printf 'fn main() { let x = 40; println!("{}", x + 2); }\n' > "$W/smoke.rs"
if ! "$W/ourc" "$W/smoke.rs" --emit-ir 2>&1 | grep -q '^define'; then echo "FAIL build-ourc: the built ourc does not compile a trivial program (crash or reject)" >&2; exit 1; fi
OUT="${OURC_OUT:-/tmp/ourc}"; cp "$W/ourc" "$OUT"; echo "built $OUT ($(stat -c%s "$OUT") bytes) -- our compiler, by semc, from our crates"
