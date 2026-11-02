#!/usr/bin/env bash
# rustc-front-end coverage instrument (v2, WHOLE-FILE first-drift).
#
# Telos: rzrs compiles the ACTUAL rustc/std source files == rustc. This gauge feeds each REAL file WHOLE to
# rzrs stage0's front end (so the prepass resolves forward-referenced fns/types -- a line-PREFIX search cannot,
# and spuriously capped reach at the first forward call) and reports the FIRST-DRIFT line: the source line of the
# first construct rzrs genuinely cannot parse. Everything before it compiled. This is the honest coverage the arc
# drives toward 100% (== full-file parse).
#
# Requires: the compiler emits a located diagnostic `error: <msg> @<byte-offset>` at each terminal parse-drift
# (eput_at in L6.rzrs.rs); this script maps that offset -> a source line. A file with NO error parses in full.
#
# Usage: SP=<scratch> [RUNG=SEM0|L6] bash measure-reach.sh [TAG]   (TAG default 1.83.0, pinned for repro)
#        RUNG=SEM0 (default) measures the telos rung; RUNG=L6 measures the bootstrap transpiler.
set -u
SP="${SP:?export SP to the dir holding L6.rzrs}"
TAG="${1:-1.83.0}"
WD="${WD:-$SP/rustc-measure}"; mkdir -p "$WD/src"
BASE="https://raw.githubusercontent.com/rust-lang/rust/$TAG"

FILES=(
  compiler/rustc_lexer/src/lib.rs
  compiler/rustc_lexer/src/cursor.rs
  compiler/rustc_lexer/src/unescape.rs
  compiler/rustc_span/src/lib.rs
  compiler/rustc_ast/src/ast.rs
  compiler/rustc_ast/src/token.rs
  compiler/rustc_parse/src/parser/expr.rs
  compiler/rustc_parse/src/lexer/mod.rs
  compiler/rustc_hir/src/hir.rs
  compiler/rustc_resolve/src/lib.rs
  compiler/rustc_data_structures/src/lib.rs
  compiler/rustc_index/src/lib.rs
  library/core/src/option.rs
  library/alloc/src/vec/mod.rs
  library/core/src/slice/mod.rs
)
for f in "${FILES[@]}"; do
  bn=$(echo "$f" | tr '/' '_')
  [ -s "$WD/src/$bn" ] || timeout 20 curl -s "$BASE/$f" -o "$WD/src/$bn" 2>/dev/null
done

# WHICH RUNG (2026-08-25). This instrument fed $SP/L6.rzrs from 2026-08-05, when L6 was the only front end that
# ran whole files. SEM0 (s1_parser.rs) self-hosted 2026-08-24 and is the rung that carries the telos; L6 is the
# bootstrap that compiles it. Measured on the same 15 files: SEM0 parses 15/15 (32009/32009 lines), L6 parses
# 4/15 (3907 = 12%). The instrument was reporting the coverage of a component being retired, and that number
# steered scoping decisions. RUNG defaults to SEM0; RUNG=L6 keeps the old reading as a bootstrap-health check.
# See locales/northstar-rung/ (C-REACH-WRONG-RUNG).
#
# CT-1, 2026-08-26 (locales/rustc-crate-targeting): THIS GAUGE CANNOT FAIL FOR SEM0. Its success test is
# `exit 0 && no error:`, and BOTH halves are unsound. (a) SEM0's diagnostics are eprintln!, which s1_parser.rs
# records as dropped by an L6/SEM0-built parser -- so "no error on stderr" is guaranteed, not observed.
# (b) The parser is TOTAL: it degrades a construct it cannot represent and continues, rather than stopping.
# Measured on one binary: `let x = @@@ ~~~ ;` -> exit 0, empty stderr, `Expr LitInt 0`; and over 192 real rustc
# files the AST dump's own item spans tile 3,157,700 of 3,157,700 bytes. Reach is genuinely 100% and genuinely
# means nothing. The discriminating instrument is a RUSTC-BUILT s1_parser in lowering mode, whose UNLOWERED
# diagnostics are live -- see that locale's inv.sh. Do not quote this script's number as coverage.
#
# CAVEAT, load-bearing: this gauge measures PARSE-REACH ONLY. SEM0 at 100% does NOT mean its AST == rustc; that
# fidelity is gated on 35 fixtures by S1's hand-transcribed goldens (-Zunpretty=ast-tree is nightly-only, the
# toolchain here is stable), and is UNMEASURED on rustc's own source. Do not quote 100% as correctness.
RUNG="${RUNG:-SEM0}"
case "$RUNG" in
  SEM0) FE="$SP/SEM0.rzrs"
        [ -x "$FE" ] || FE="$SP/wf.gen1"
        [ -x "$FE" ] || { echo "no SEM0 binary in \$SP (expected SEM0.rzrs or wf.gen1 -- run s9-wholefile-diff.sh, or RUNG=L6)"; exit 1; } ;;
  L6)   FE="$SP/L6.rzrs"
        [ -x "$FE" ] || { echo "no \$SP/L6.rzrs -- run stage.sh"; exit 1; } ;;
  *) echo "RUNG must be SEM0 or L6"; exit 1 ;;
esac

# whole-file first-drift: feed the WHOLE file; parse the located diagnostic's @offset -> a source line.
reach() {
  local f="$1" total rc msg off line
  total=$(wc -l < "$f")
  timeout 60 "$FE" < "$f" > /dev/null 2> "$WD/e.txt"; rc=$?
  if [ "$rc" -eq 0 ] && ! grep -qiE 'drift|error:' "$WD/e.txt"; then echo "$total $total full"; return; fi
  msg=$(grep -iE 'error:|drift' "$WD/e.txt" | head -1)
  # the parse-drift diagnostic prints raw source context (with newlines) BEFORE the `@<offset>`, so scan ALL of
  # stderr for the offset, not just the first line. Exactly one drift fires per run (it exits at the first).
  off=$(grep -oE '@[0-9]+' "$WD/e.txt" | head -1 | tr -d '@')
  if [ -n "$off" ]; then
    line=$(head -c "$off" "$f" | wc -l); line=$(( line + 1 ))
    # reach = lines that parsed BEFORE the first drift line
    echo "$(( line - 1 )) $total drift:$line"
  else
    # rc!=0 with no located offset: parse completed, a semantic check (moved/mismatch/...) fired -> full parse-reach
    echo "$total $total sem"
  fi
}

echo "rung: $RUNG  ($FE)"
printf "%-40s %6s %6s %5s  %s\n" FILE REACH TOTAL PCT "FIRST-DRIFT"
tr=0; tl=0
for f in "$WD"/src/*; do
  bn=$(basename "$f" | sed 's/\.rs$//')
  read -r r t note < <(reach "$f")
  tr=$(( tr + r )); tl=$(( tl + t ))
  dl=""; case "$note" in drift:*) dl="L${note#drift:}: $(sed -n "${note#drift:}p" "$f" | sed 's/^ *//' | cut -c1-60)";; full) dl="(full parse)";; sem) dl="(full parse; semantic reject)";; esac
  printf "%-40s %6s %6s %4s%%  %s\n" "$bn" "$r" "$t" "$(( r*100/t ))" "$dl"
done
echo "---------------------------------------------------------------------------"
printf "%-40s %6s %6s %4s%%   [rung %s, PARSE-REACH only -- AST fidelity vs rustc is NOT measured here]\n" "AGGREGATE" "$tr" "$tl" "$(( tr*100/tl ))" "$RUNG"
