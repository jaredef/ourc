#!/bin/bash
# vendor.sh -- lay ourc/src out as this crate's src/ (one module per ourc crate), the same assembly as
# ../self-module.sh: each module imports every other one, and main.rs holds odrv (the driver). Run before packaging.
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; S="$HERE/../src"; D="$HERE/src"
C="ospan oerrors oindex oarena oabi olex oast oresolve ohir olower omir omirtrans ocodegen osession oparse otypeck omono oborrowck"
rm -rf "$D"; mkdir -p "$D"
for c in $C; do { for d in $C; do [ "$d" = "$c" ] || { echo "use crate::$d;"; echo "use crate::$d::*;"; }; done; cat "$S/$c.rs"; } > "$D/$c.rs"; done
# ourc's source is written for its own compilers too, so rustc's lints are not its style guide
# odrv's host hooks are stubs under rustc (ourc swaps them for libc when it compiles itself): drop them, add hooks.rs
{ echo '#![allow(warnings)]'; for c in $C; do echo "mod $c;"; done; for c in $C; do echo "use $c::*;"; done
  awk '/^fn __ourc_(system|writefile|budget)\(/{skip=1} !skip{print} skip && /^}$/{skip=0}' "$S/odrv.rs"; cat "$HERE/hooks.rs"; } > "$D/main.rs"
cp "$HERE/../../LICENSE-MIT" "$HERE/../../LICENSE-APACHE" "$HERE/"
echo "vendor: $(ls "$D" | wc -l) files from ourc/src -> $D"
