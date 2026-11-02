#!/bin/bash
set -u
ROOT="$(cd "$(dirname "$0")/../../../.." && pwd)"
SEM0="${SEM0:?}"; R2L="${R2L:?}"; LLC="${LLC:?}"
LINK_RS="$ROOT/ourc/rt/link.rs"
RT="$ROOT/ourc/rt/runtime.rzir"
D="$(cd "$(dirname "$0")" && pwd)"; W="$(mktemp -d)"; trap 'rm -rf "$W"' EXIT
semcbk(){ cat "$1" "$RT" | "$R2L" > "$W/x.ll" 2>/dev/null; "$LLC" -O2 -filetype=obj "$W/x.ll" -o "$W/x.o" 2>/dev/null; cc -no-pie "$W/x.o" -o "$2" 2>/dev/null; chmod +x "$2"; }
{ echo 'mod pa {'; cat "$D/pa.rs"; echo '}'; echo 'use pa::N;'; cat "$D/pb.rs"; } > "$W/g.rs"
rustc -O --edition 2021 "$W/g.rs" -o "$W/g" 2>/dev/null && echo "golden: $("$W/g" | tr '\n' ' ')"
{ echo '//@S9'; cat "$LINK_RS"; } | "$SEM0" > "$W/l.rz" 2>/dev/null; semcbk "$W/l.rz" "$W/link.semc"
{ echo '//@METADATA crate=pa'; cat "$D/pa.rs"; } | "$SEM0" 2>/dev/null | grep '^META ' | sed 's/^META /\/\/META /' > "$W/mh.txt"
{ echo '//@SUPERSET crate=pa'; cat "$D/pa.rs"; } | "$SEM0" > "$W/pa.rz" 2>/dev/null
{ echo '//@SUPERSET crate=pb'; cat "$W/mh.txt"; cat "$D/pb.rs"; } | "$SEM0" > "$W/pb.rz" 2>/dev/null
{ cat "$W/pb.rz"; echo '@@@CRATE@@@'; cat "$W/pa.rz"; } | timeout 120 "$W/link.semc" > "$W/lk.rz" 2>/dev/null
semcbk "$W/lk.rz" "$W/p"; echo "semc:   $("$W/p" 2>/dev/null | tr '\n' ' ')"
