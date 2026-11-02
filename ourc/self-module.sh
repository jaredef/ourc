#!/bin/bash
# self-module.sh -- P4 oracle harness: ourc compiles its OWN source as one program of `mod X;` files.
#   OURC=<ourc> bash self-module.sh [WORKDIR]     -> prints the error count + the grouped residual
# Assembly mirrors ourc/run.sh's rustc golden: each module file gets `use crate::<dep>;` + `use crate::<dep>::*;` for every other crate (the name import is what makes `dep::f()` resolve in Rust 2018+ -- P6 SIB-PATH / E0433 -- and the
# crates call each other bare, relying on SEM0's flat superset namespace), the root globs every crate, then odrv's body.
# FILE-splice form (mod X; -> X.rs), not inline `mod X { .. }`: measured 2026-10-29, the inline assembly of the same text
# walls at ~2200 errors on a parse error while the file-splice form does not (open question, recorded in the trajectory).
set -u
ROOT="$(cd "$(dirname "$(readlink -f "$0")")/.." && pwd)"; S="$ROOT/ourc/src"; OURC="${OURC:?}"
W="${1:-$(mktemp -d)}"; mkdir -p "$W"
C="ospan oerrors oindex oarena oabi olex oast oresolve ohir olower omir omirtrans ocodegen osession oparse otypeck omono oborrowck"
for c in $C; do { for d in $C; do [ "$d" = "$c" ] || { echo "use crate::$d;"; echo "use crate::$d::*;"; }; done; cat "$S/$c.rs"; } > "$W/$c.rs"; done
{ for c in $C; do echo "mod $c;"; done; for c in $C; do echo "use $c::*;"; done; cat "$S/odrv.rs"; } > "$W/main.rs"
ulimit -s unlimited
timeout 1500 "$OURC" "$W/main.rs" --emit-ir > "$W/out.ll" 2> "$W/err.txt"
# only ourc's STATUS LINE and diagnostics, on stderr (V1.1 CLEAN-EMIT-IR: stdout is IR only), carry diagnostics -- once ourc compiles itself, the IR holds ourc's
# own error-message STRINGS as constants (`c"error[0..0]: ..."`), which a whole-file grep miscounted as errors
st="$(cat "$W/err.txt")"
n=$(printf '%s\n' "$st" | tr ';' '\n' | grep -c 'error\[')
echo "self-module: $n errors, $(grep -c '^define' "$W/out.ll") defines (work: $W)"
printf '%s\n' "$st" | head -1 | cut -c1-200
printf '%s\n' "$st" | tr ';' '\n' | grep -o 'error\[[^]]*\]: .*' | sed -E 's/error\[[0-9.]+\]: //' | sort | uniq -c | sort -rn | head -40
# --link: also build ourc' from the self-emitted IR (llc -O2 + cc) and smoke-test it on a trivial program
if [ "${2:-}" = "--link" ]; then
  LLC="${LLC:-$(ls ~/.rustup/toolchains/*/lib/rustlib/*/bin/llc 2>/dev/null | head -1)}"
  sed -n '/^\(@\|declare\|define\)/,$p' "$W/out.ll" > "$W/prime.ll"
  "$LLC" -O2 -filetype=obj "$W/prime.ll" -o "$W/prime.o" && cc -no-pie "$W/prime.o" -o "$W/ourc.prime" -lm && echo "ourc' linked: $W/ourc.prime ($(stat -c%s "$W/ourc.prime") B)"
  printf 'fn main() { let x = 40; println!("{}", x + 2); }\n' > "$W/smoke.rs"
  timeout 60 "$W/ourc.prime" "$W/smoke.rs" --emit-ir > "$W/smoke.out" 2>&1; echo "ourc' smoke rc=$? ($(grep -c '^define' "$W/smoke.out") defines)"
fi
