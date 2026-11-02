#!/usr/bin/env bash
# Phase-C REAL-rustc-crate differential (CD-9 instrument).
# For each real rustc crate spec (name + github file path[s]), fetch the file(s) (raw.githubusercontent 1.83.0, cached in
# $SP/rustc-measure/src), build a //@UNIT file= manifest, run the //@CRATE driver, and diff its CRATERES def-paths against
# rustc's own def-paths (-Zunpretty=hir-tree, user-crate normalized: crate-hash stripped, {use#}/prelude/std/core/alloc
# filtered, each prefixed <crate>|). BOOTSTRAP = rz==rc. Requires SP (L6.rzrs + cf.hoist + s1.rc/s1.rz) + CC + curl.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; S1="$HERE/../S1-lex-parse-ast"
SP="${SP:?set SP}"; CC="${CC:?set CC}"
P="$S1/s1_parser.rs"
WD="$SP/rustc-measure/src"; mkdir -p "$WD"
BASE="https://raw.githubusercontent.com/rust-lang/rust/1.83.0"
. "$(dirname "$0")/../s1-build.sh"   # builds $SP/s1.rc + $SP/s1.rz once per input set, then reuses them

fetch(){ # $1 = github path -> echoes the cached local file
  local gp="$1" bn; bn="$(echo "$gp" | tr '/' '_')"
  [ -s "$WD/$bn" ] || timeout 25 curl -s "$BASE/$gp" -o "$WD/$bn" 2>/dev/null
  echo "$WD/$bn"
}
oracle(){ # $1 = crate name  $2 = root file ; emits <crate>|<defpath> sorted-unique (std/core/alloc/{use#}/prelude filtered)
  rustc +nightly -Zunpretty=hir-tree --edition 2021 --crate-type=lib --crate-name "$1" "$2" 2>/dev/null \
    | grep -oE 'owner_id: DefId\(0:[0-9]+ ~ [a-z_]+\[[0-9a-f]+\]::[^),]+\)' \
    | sed -E 's/.*\[[0-9a-f]+\]:://; s/\)$//' | grep -vE '^std$|^core$|^alloc$|prelude|^\{use#' | sort -u | sed "s/^/$1|/"; }

# Crate specs: NAME|<root-github-path>[|<mod-github-path>...]. CD-9: rustc_index single-file lib.rs (submodule files = CD-12).
SPECS=(
  "rustc_index|compiler/rustc_index/src/lib.rs"
  "rustc_lexer|compiler/rustc_lexer/src/cursor.rs"
  "rustc_data_structures|compiler/rustc_data_structures/src/lib.rs"
  "core_convert|library/core/src/convert/mod.rs"
  "rustc_lexer_unescape|compiler/rustc_lexer/src/unescape.rs"
  "rustc_span_editdist|compiler/rustc_span/src/edit_distance.rs"
  "rustc_abi|compiler/rustc_abi/src/lib.rs"
  "rustc_parse_format|compiler/rustc_parse_format/src/lib.rs"
  "rustc_fs_util|compiler/rustc_fs_util/src/lib.rs"
)
fail=0
for spec in "${SPECS[@]}"; do
  IFS='|' read -r name root rest <<< "$spec"
  rootf="$(fetch "$root")"
  [ -s "$rootf" ] || { echo "  SKIP ($name): fetch failed (offline?)"; continue; }
  oracle "$name" "$rootf" > "$SP/rc_$name.expect"
  # build the file= manifest (root = lib.rs; extra mods appended as units in later rungs)
  { echo '//@CRATE'; echo "//@UNIT crate=$name path=lib.rs root=1 file=$rootf"; } > "$SP/rc_$name.in"
  "$SP/s1.rc" < "$SP/rc_$name.in" 2>/dev/null > "$SP/rc_$name.rc"
  "$SP/s1.rz" < "$SP/rc_$name.in" 2>/dev/null > "$SP/rc_$name.rz"
  if ! cmp -s "$SP/rc_$name.rc" "$SP/rc_$name.rz"; then echo "  FAIL BOOTSTRAP ($name): rz != rc"; fail=1; continue; fi
  grep -oE '^CRATERES .*' "$SP/rc_$name.rc" | sed 's/^CRATERES //' | sort -u > "$SP/rc_$name.actual"
  if cmp -s "$SP/rc_$name.actual" "$SP/rc_$name.expect"; then echo "  ok   REALCRATE ($name): def-paths == rustc ($(wc -l < "$SP/rc_$name.expect"))"
  else echo "  RED  REALCRATE ($name): != rustc (expected $(wc -l < "$SP/rc_$name.expect"), got $(wc -l < "$SP/rc_$name.actual"))"; diff "$SP/rc_$name.expect" "$SP/rc_$name.actual" | head -12 | sed 's/^/      /'; fi
done

# --- MULTI-FILE crate (CD-13): a real rustc crate's module tree supplied as file= UNITS. The oracle must read the siblings
# by their real relative names, so fetch into a real-layout dir (mod cursor; <- cursor.rs next to lib.rs). rustc_lexer is a
# dep-light leaf; its unicode_properties use is E0432-unresolved but rustc still emits the full hir-tree (incl submodule
# {impl#N} + {use#N} def-paths). Proves cross-FILE submodule def-path parity (root + cursor + unescape).
MFDIR="$WD/lexer"; mkdir -p "$MFDIR"
for f in lib cursor unescape; do
  [ -s "$MFDIR/$f.rs" ] || timeout 25 curl -s "$BASE/compiler/rustc_lexer/src/$f.rs" -o "$MFDIR/$f.rs" 2>/dev/null
done
if [ -s "$MFDIR/lib.rs" ] && [ -s "$MFDIR/cursor.rs" ] && [ -s "$MFDIR/unescape.rs" ]; then
  ( cd "$MFDIR" && rustc +nightly -Zunpretty=hir-tree --edition 2021 --crate-type=lib --crate-name rustc_lexer lib.rs 2>/dev/null ) \
    | grep -oE 'owner_id: DefId\(0:[0-9]+ ~ [a-z_]+\[[0-9a-f]+\]::[^),]+\)' \
    | sed -E 's/.*\[[0-9a-f]+\]:://; s/\)$//' | grep -vE '^std$|^core$|^alloc$|prelude|^\{use#' | sort -u | sed 's/^/rustc_lexer|/' > "$SP/rc_lexermf.expect"
  { echo '//@CRATE'
    echo "//@UNIT crate=rustc_lexer path=lib.rs root=1 file=$MFDIR/lib.rs"
    echo "//@UNIT crate=rustc_lexer path=cursor.rs file=$MFDIR/cursor.rs"
    echo "//@UNIT crate=rustc_lexer path=unescape.rs file=$MFDIR/unescape.rs"; } > "$SP/rc_lexermf.in"
  "$SP/s1.rc" < "$SP/rc_lexermf.in" 2>/dev/null > "$SP/rc_lexermf.rc"
  "$SP/s1.rz" < "$SP/rc_lexermf.in" 2>/dev/null > "$SP/rc_lexermf.rz"
  if ! cmp -s "$SP/rc_lexermf.rc" "$SP/rc_lexermf.rz"; then echo "  FAIL BOOTSTRAP (rustc_lexer-multifile): rz != rc"; fail=1
  else
    grep -oE '^CRATERES .*' "$SP/rc_lexermf.rc" | sed 's/^CRATERES //' | sort -u > "$SP/rc_lexermf.actual"
    if cmp -s "$SP/rc_lexermf.actual" "$SP/rc_lexermf.expect"; then echo "  ok   MULTIFILE (rustc_lexer): 3-unit cross-file def-paths == rustc ($(wc -l < "$SP/rc_lexermf.expect"))"
    else echo "  RED  MULTIFILE (rustc_lexer): != rustc (expected $(wc -l < "$SP/rc_lexermf.expect"), got $(wc -l < "$SP/rc_lexermf.actual"))"; diff "$SP/rc_lexermf.expect" "$SP/rc_lexermf.actual" | head -12 | sed 's/^/      /'; fail=1; fi
  fi
else echo "  SKIP (rustc_lexer-multifile): fetch failed (offline?)"; fi

# --- MULTIFILE rustc_ast_ir (CD-18): a 2nd real 2-file rustc crate (lib.rs + visit.rs, ~4.6KB, rz-valid). The driver
# already matches rustc EXACTLY (68 def-paths incl the visit.rs cfg-attributed items) -- a clean multi-file coverage guard.
AID="$WD/ast_ir"; mkdir -p "$AID"
for f in lib visit; do [ -s "$AID/$f.rs" ] || timeout 25 curl -s "$BASE/compiler/rustc_ast_ir/src/$f.rs" -o "$AID/$f.rs" 2>/dev/null; done
if [ -s "$AID/lib.rs" ] && [ -s "$AID/visit.rs" ]; then
  ( cd "$AID" && rustc +nightly -Zunpretty=hir-tree --edition 2021 --crate-type=lib --crate-name rustc_ast_ir lib.rs 2>/dev/null ) \
    | grep -oE 'owner_id: DefId\(0:[0-9]+ ~ [a-z_]+\[[0-9a-f]+\]::[^),]+\)' \
    | sed -E 's/.*\[[0-9a-f]+\]:://; s/\)$//' | grep -vE '^std$|^core$|^alloc$|prelude|^\{use#' | sort -u | sed 's/^/rustc_ast_ir|/' > "$SP/rc_astir.expect"
  { echo '//@CRATE'; echo "//@UNIT crate=rustc_ast_ir path=lib.rs root=1 file=$AID/lib.rs"; echo "//@UNIT crate=rustc_ast_ir path=visit.rs file=$AID/visit.rs"; } > "$SP/rc_astir.in"
  "$SP/s1.rc" < "$SP/rc_astir.in" 2>/dev/null > "$SP/rc_astir.rc"
  "$SP/s1.rz" < "$SP/rc_astir.in" 2>/dev/null > "$SP/rc_astir.rz"
  if ! cmp -s "$SP/rc_astir.rc" "$SP/rc_astir.rz"; then echo "  FAIL BOOTSTRAP (rustc_ast_ir-multifile): rz != rc"; fail=1
  else
    grep -oE '^CRATERES .*' "$SP/rc_astir.rc" | sed 's/^CRATERES //' | sort -u > "$SP/rc_astir.actual"
    if cmp -s "$SP/rc_astir.actual" "$SP/rc_astir.expect"; then echo "  ok   MULTIFILE (rustc_ast_ir): 2-unit cross-file def-paths == rustc ($(wc -l < "$SP/rc_astir.expect"))"
    else echo "  RED  MULTIFILE (rustc_ast_ir): != rustc"; diff "$SP/rc_astir.expect" "$SP/rc_astir.actual" | head | sed 's/^/      /'; fail=1; fi
  fi
else echo "  SKIP (rustc_ast_ir-multifile): fetch failed (offline?)"; fi

# --- MULTIFILE rustc_serialize (CD-19): a 5-unit real crate (lib.rs + serialize/int_overflow/leb128/opaque, ~42KB rz-valid).
# FULLY closed via the CD-19 macro-arm-registration fix (()-bodied + empty-first-arm) + the module fn-gen (CD-16),
# 2-impls-per-repetition (CD-17), and peano-recursive tuple!/peel! (CD-18/19) emit-path detectors.
SED="$WD/serialize"; mkdir -p "$SED"
for f in lib serialize int_overflow leb128 opaque; do [ -s "$SED/$f.rs" ] || timeout 25 curl -s "$BASE/compiler/rustc_serialize/src/$f.rs" -o "$SED/$f.rs" 2>/dev/null; done
if [ -s "$SED/lib.rs" ] && [ -s "$SED/serialize.rs" ] && [ -s "$SED/leb128.rs" ]; then
  ( cd "$SED" && rustc +nightly -Zunpretty=hir-tree --edition 2021 --crate-type=lib --crate-name rustc_serialize lib.rs 2>/dev/null ) \
    | grep -oE 'owner_id: DefId\(0:[0-9]+ ~ [a-z_]+\[[0-9a-f]+\]::[^),]+\)' \
    | sed -E 's/.*\[[0-9a-f]+\]:://; s/\)$//' | grep -vE '^std$|^core$|^alloc$|prelude|^\{use#' | sort -u | sed 's/^/rustc_serialize|/' > "$SP/rc_ser.expect"
  { echo '//@CRATE'; echo "//@UNIT crate=rustc_serialize path=lib.rs root=1 file=$SED/lib.rs"
    for f in serialize int_overflow leb128 opaque; do echo "//@UNIT crate=rustc_serialize path=$f.rs file=$SED/$f.rs"; done; } > "$SP/rc_ser.in"
  "$SP/s1.rc" < "$SP/rc_ser.in" 2>/dev/null > "$SP/rc_ser.rc"
  "$SP/s1.rz" < "$SP/rc_ser.in" 2>/dev/null > "$SP/rc_ser.rz"
  if ! cmp -s "$SP/rc_ser.rc" "$SP/rc_ser.rz"; then echo "  FAIL BOOTSTRAP (rustc_serialize-multifile): rz != rc"; fail=1
  else
    grep -oE '^CRATERES .*' "$SP/rc_ser.rc" | sed 's/^CRATERES //' | sort -u > "$SP/rc_ser.actual"
    if cmp -s "$SP/rc_ser.actual" "$SP/rc_ser.expect"; then echo "  ok   MULTIFILE (rustc_serialize): 5-unit cross-file def-paths == rustc ($(wc -l < "$SP/rc_ser.expect"))"
    else echo "  RED  MULTIFILE (rustc_serialize): != rustc"; diff "$SP/rc_ser.expect" "$SP/rc_ser.actual" | head | sed 's/^/      /'; fail=1; fi
  fi
else echo "  SKIP (rustc_serialize-multifile): fetch failed (offline?)"; fi

# --- MULTIFILE fn-gen (CD-16): a committed synthetic 2-unit crate with a module-level fn-generating macro in a submodule
# (the rustc_serialize leb128 impl_*_leb128! shape). Proves `<mod>::<fnname>` def-path parity for a submodule fn-gen macro.
# (The real rustc_serialize crate stays s1.rc-only + RED on its serialize::{impl#N} macro-impl numbering rung.)
FGD="$HERE/witnesses-fngen"
if [ -s "$FGD/lib.rs" ] && [ -s "$FGD/m.rs" ]; then
  ( cd "$FGD" && rustc +nightly -Zunpretty=hir-tree --edition 2021 --crate-type=lib --crate-name fngen lib.rs 2>/dev/null ) \
    | grep -oE 'owner_id: DefId\(0:[0-9]+ ~ [a-z_]+\[[0-9a-f]+\]::[^),]+\)' \
    | sed -E 's/.*\[[0-9a-f]+\]:://; s/\)$//' | grep -vE '^std$|^core$|^alloc$|prelude|^\{use#' | sort -u | sed 's/^/fngen|/' > "$SP/rc_fngen.expect"
  { echo '//@CRATE'; echo "//@UNIT crate=fngen path=lib.rs root=1 file=$FGD/lib.rs"; echo "//@UNIT crate=fngen path=m.rs file=$FGD/m.rs"; } > "$SP/rc_fngen.in"
  "$SP/s1.rc" < "$SP/rc_fngen.in" 2>/dev/null > "$SP/rc_fngen.rc"
  "$SP/s1.rz" < "$SP/rc_fngen.in" 2>/dev/null > "$SP/rc_fngen.rz"
  if ! cmp -s "$SP/rc_fngen.rc" "$SP/rc_fngen.rz"; then echo "  FAIL BOOTSTRAP (fngen): rz != rc"; fail=1
  else
    grep -oE '^CRATERES .*' "$SP/rc_fngen.rc" | sed 's/^CRATERES //' | sort -u > "$SP/rc_fngen.actual"
    if cmp -s "$SP/rc_fngen.actual" "$SP/rc_fngen.expect"; then echo "  ok   MULTIFILE-FNGEN (fngen): submodule fn-gen macro def-paths == rustc ($(wc -l < "$SP/rc_fngen.expect"))"
    else echo "  RED  MULTIFILE-FNGEN (fngen): != rustc"; diff "$SP/rc_fngen.expect" "$SP/rc_fngen.actual" | head | sed 's/^/      /'; fail=1; fi
  fi
else echo "  SKIP (fngen): witness missing"; fi

# --- MULTIFILE multi-impl (CD-17): a committed synthetic crate whose submodule macro generates 2 impls per repetition group
# (the rustc_serialize direct_serialize_impls! shape: Encodable + Decodable per $ty). Proves interleaved {impl#N} numbering
# (impl#0::e, impl#1::d, impl#2::e, ..) == rustc. (rustc_serialize itself stays s1.rc-only + RED on the tuple! RECURSIVE
# macro cascade, a deferred recursive-macro rung distinct from this per-repetition count.)
MID="$HERE/witnesses-multiimpl"
if [ -s "$MID/lib.rs" ] && [ -s "$MID/m.rs" ]; then
  ( cd "$MID" && rustc +nightly -Zunpretty=hir-tree --edition 2021 --crate-type=lib --crate-name mi lib.rs 2>/dev/null ) \
    | grep -oE 'owner_id: DefId\(0:[0-9]+ ~ [a-z_]+\[[0-9a-f]+\]::[^),]+\)' \
    | sed -E 's/.*\[[0-9a-f]+\]:://; s/\)$//' | grep -vE '^std$|^core$|^alloc$|prelude|^\{use#' | sort -u | sed 's/^/mi|/' > "$SP/rc_mi.expect"
  { echo '//@CRATE'; echo "//@UNIT crate=mi path=lib.rs root=1 file=$MID/lib.rs"; echo "//@UNIT crate=mi path=m.rs file=$MID/m.rs"; } > "$SP/rc_mi.in"
  "$SP/s1.rc" < "$SP/rc_mi.in" 2>/dev/null > "$SP/rc_mi.rc"
  "$SP/s1.rz" < "$SP/rc_mi.in" 2>/dev/null > "$SP/rc_mi.rz"
  if ! cmp -s "$SP/rc_mi.rc" "$SP/rc_mi.rz"; then echo "  FAIL BOOTSTRAP (multiimpl): rz != rc"; fail=1
  else
    grep -oE '^CRATERES .*' "$SP/rc_mi.rc" | sed 's/^CRATERES //' | sort -u > "$SP/rc_mi.actual"
    if cmp -s "$SP/rc_mi.actual" "$SP/rc_mi.expect"; then echo "  ok   MULTIFILE-MULTIIMPL (mi): 2-impls-per-repetition {impl#N} == rustc ($(wc -l < "$SP/rc_mi.expect"))"
    else echo "  RED  MULTIFILE-MULTIIMPL (mi): != rustc"; diff "$SP/rc_mi.expect" "$SP/rc_mi.actual" | head | sed 's/^/      /'; fail=1; fi
  fi
else echo "  SKIP (multiimpl): witness missing"; fi

# --- MULTIFILE cfg-off block (CD-22): the rz-LIFT RESOLUTION. A submodule fn body with a `#[cfg(feature=..)] { use..; }`
# block statement -- rustc drops the block; the driver must too. Resolved via strip_cfg_off_block_toks (a TOKEN-stream
# pre-parse strip, NO Parser struct growth -- a 23rd field mis-lowers rz's extern-crate codegen, the 3x-confirmed LIFT).
# This closes rustc_abi multi-file (544, s1.rc-only: 106KB > rz ceiling) + the general cfg-off-block-stmt case.
CBD="$HERE/witnesses-cfgblock"
if [ -s "$CBD/lib.rs" ] && [ -s "$CBD/m.rs" ]; then
  ( cd "$CBD" && rustc +nightly -Zunpretty=hir-tree --edition 2021 --crate-type=lib --crate-name cb lib.rs 2>/dev/null ) \
    | grep -oE 'owner_id: DefId\(0:[0-9]+ ~ [a-z_]+\[[0-9a-f]+\]::[^),]+\)' \
    | sed -E 's/.*\[[0-9a-f]+\]:://; s/\)$//' | grep -vE '^std$|^core$|^alloc$|prelude|^\{use#' | sort -u | sed 's/^/cb|/' > "$SP/rc_cb.expect"
  { echo '//@CRATE'; echo "//@UNIT crate=cb path=lib.rs root=1 file=$CBD/lib.rs"; echo "//@UNIT crate=cb path=m.rs file=$CBD/m.rs"; } > "$SP/rc_cb.in"
  "$SP/s1.rc" < "$SP/rc_cb.in" 2>/dev/null > "$SP/rc_cb.rc"
  "$SP/s1.rz" < "$SP/rc_cb.in" 2>/dev/null > "$SP/rc_cb.rz"
  if ! cmp -s "$SP/rc_cb.rc" "$SP/rc_cb.rz"; then echo "  FAIL BOOTSTRAP (cfgblock): rz != rc"; fail=1
  else
    grep -oE '^CRATERES .*' "$SP/rc_cb.rc" | sed 's/^CRATERES //' | sort -u > "$SP/rc_cb.actual"
    if cmp -s "$SP/rc_cb.actual" "$SP/rc_cb.expect"; then echo "  ok   MULTIFILE-CFGBLOCK (cb): cfg-off block-stmt use dropped == rustc ($(wc -l < "$SP/rc_cb.expect"))"
    else echo "  RED  MULTIFILE-CFGBLOCK (cb): != rustc"; diff "$SP/rc_cb.expect" "$SP/rc_cb.actual" | head | sed 's/^/      /'; fail=1; fi
  fi
else echo "  SKIP (cfgblock): witness missing"; fi

[ "$fail" = 0 ] && echo "== s3-realcrate: BOOTSTRAP green (REALCRATE RED until cfg/macro/submodule gaps close) ==" || echo "== s3-realcrate: BOOTSTRAP broken =="
exit $fail
