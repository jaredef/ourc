#!/usr/bin/env bash
# sem0-stage.sh -- the FREEZE build for the SEM0 tier: a staircase of hops from L6 to a self-hosting SEM0.
#
#   L6.rzrs     --compiles--> hops/h00.rs  ==> $SP/SEM0.h00   (the L6-rooted seed)
#   SEM0.h00    --compiles--> hops/h01.rs  ==> $SP/SEM0.h01   (the seed self-mints the bridge)
#   SEM0.h(N-1) --compiles--> hops/hNN.rs  ==> $SP/SEM0.hNN   (h02..h27; h23 is iterated to a byte fixpoint)
#   SEM0.h27    --compiles--> s1_parser.rs ==> $SP/SEM0.cf    (the dev source, iterated to a byte fixpoint)
#
# SEM0.cf is the hex-rooted SEM0 (old name sem0.frozen) and $SP/SEM0.s9 its //@S9 shim (old name sem0.cc); the
# S-tier gates reach it through SEM0_CC. Names: apparatus/docs/tower-naming-and-trust-path.md, "Artifact names",
# which also maps each hop to its old SEM0.FROZEN.STAGE* name. The hop sources were renamed 2026-11-01; their
# per-stage history (and this script's earlier hand-unrolled form) is in git log.
#
# WHY A STAIRCASE: no frozen SEM0 compiles a much newer SEM0 in one step. Each frozen has an emit-capacity wall
# (it SIGSEGVs past ~N emit lines, N rising with the compiler's own size), and an old frozen can also MISCOMPILE
# a construct a newer dev uses. Each hop is a proven, in-repo snapshot that the previous hop's frozen compiles
# cleanly; the split points were found by bisection. No hop source is a trusted blob: all are in-repo text, and
# rooting stays in the hand-hex (L6 mints the seed exactly as L5 mints L6).
#
# WHY h00 -> h01 (2026-08-30, keeper-accepted self-mint -- see emit-s9-vecpat-slot-null + SEM0.FROZEN-AT.md): the
# L6-minted seed miscompiles ONE `add` mnemonic in delim_balance on a payload-binding-scale target. The seed
# self-minting from h01 (rung-15) produces a frozen whose emit does not carry that corruption.
#
# BACKEND: h00, h01 and h02 lower through the frozen C ld; every later hop through the i64 Rust ld twin ($SP/B.ld),
# because the C ld clamps heaptop at INT32_MAX and cannot link the >2GB self-emit arena (endgame rung 284).
#
# The hops (what each snapshot is, from the hand-unrolled script it replaces):
#   h00  (was SEM0.FROZEN.rs) v1 (087511ae) -- L6-buildable seed source
#   h01  (was STAGE2) v2 (rung-15) -- the seed self-mints the working tier from this (the BRIDGE)
#   h02  (was STAGE2B) v2b (2026-09-05 staircase refresh) -- split-hop intermediate (old STAGE3) so bridge->2b->dev stays in-reach
#   h03  (was STAGE2C) v2c (2026-09-08 mono-land refresh) -- proven pre-mono dev so 2b->2c->dev(+mono) stays in-reach
#   h04  (was STAGE2D) v2d (2026-09-08 drop-glue refresh) -- proven gap-1 dev so 2c->2d->dev(+needs_drop) stays in-reach
#   h05  (was STAGE2E) v2e proven 6/6 dev so 2d->2e->dev(9/9) stays in-reach
#   h06  (was STAGE2F) v2f proven 9/9 base so 2e->2f->dev(+lexer) stays in-reach
#   h07  (was STAGE2G) v2g proven tokenizer HEAD so 2f->2g->dev(+strip_shebang) stays in-reach (+197)
#   h08  (was STAGE2H) v2h proven lexer HEAD so 2g->2h->dev(+graphviz caps) stays in-reach
#   h09  (was STAGE2I) v2i proven graphviz-caps dev so 2h->2i->dev(+multi-param-mono) stays in-reach (+109)
#   h10  (was STAGE2J) v2j proven multi-param-mono dev so 2i->2j->dev(+s9f12 clos-blk fix) stays in-reach (+16)
#   h11  (was STAGE2K) v2k proven mono+s9f12 dev so 2j->2k->dev(+write-toks vec-var fix) stays in-reach (+7)
#   h12  (was STAGE2M) v2m proven write-toks dev so 2k->2m->dev(+single_node bundle +add-CSE) stays in-reach (add-CSE SHRINKS the image)
#   h13  (was STAGE2N) v2n proven single_node+add-CSE dev so 2m->2n->dev(+NodeLabels::len user-method fix) stays in-reach (+7)
#   h14  (was STAGE2O) v2o proven NodeLabels::len dev so 2n->2o->dev(+edges len-guard tighten: require typed <recv>::len fret) stays in-reach (+3)
#   h15  (was STAGE2P) v2p proven edges len-guard dev so 2o->2p->dev(+NodeLabels::len resolved-alias fix + s9_for_iter ref-elem binding: single_edge RAN-CORRECT) stays in-reach (+16)
#   h16  (was STAGE2Q) v2q proven single_edge dev so 2p->2q->dev(+into_iter chain + ctor-map-collect: hasse_diagram RAN-CORRECT) stays in-reach (+45)
#   h17  (was STAGE2R) v2r proven hasse dev so 2q->2r->dev(+trait-default-method dispatch: WithEscStrs node_style default, render no longer segvs) stays in-reach (+~30)
#   h18  (was STAGE2S) v2s proven trait-default dev so 2r->2s->dev(+escape_default + push-by-recv-type + closure capture-type propagation O(N): left_aligned_text RAN-CORRECT) stays in-reach
#   h19  (was STAGE2T) v2t proven graphviz-closed dev so 2s->2t->dev(+[u8;N]-literal byte-pack storage) stays in-reach
#   h20  (was STAGE2U) v2u proven u8-literal dev so 2t->2u->dev(+filter_map chain adapter + gv-gated collect) stays in-reach
#   h21  (was STAGE2V) v2v proven filter_map dev so 2u->2v->dev(+rev chain adapter + __heap 3400->3580MB for headroom) stays in-reach
#   h22  (was STAGE2W) v2w proven rev dev so 2v->2w->dev(-3 dead fns: s9_any_arg_char + emit_*_nested_items) stays in-reach
#   h23  (was STAGE3) v3 (2026-09-01) -- FULL RE-FREEZE: the bridge frozen mints the then-full dev to a byte fixpoint
#   h24  (was STAGE4) v4 (2026-09-17) -- last-gated dev (r0, 19382 L): STAGE3-frozen mints this
#   h25  (was STAGE5) v5 (2026-09-17) -- r3 (016e45dd, 19450 L): the r0-frozen mints this; its emit ceiling then clears the full dev
#   h26  (was STAGE5B) v5b (2026-10-10 re-mint): snapshot 413 (3bbdd3851fbc, 26675 L) -- the r3-frozen bootstraps THIS cleanly; its frozen then clears the full 27843 dev (the r3-frozen miscompiles commit 414 "f64 arith result typing" + beyond, a CONSTRUCT miscompile, not a capacity wall -- empirically bisected 2026-10-10)
#   h27  (was STAGE5C) v5c (2026-10-11 split-hop): midpoint commit c187ae17e (27527 L) -- the 413-frozen mints THIS cleanly (3.71GB, under the 4.2GB ceiling); its frozen then clears the full HEAD dev in ~2.1GB. The 413->HEAD gap was NOT a construct miscompile but a per-function arena peak: s9_call/s9_method DOUBLED its driver-D2 rung intercepts (9->18) post-midpoint, and the 413-era emission of that grown function peaks >4.2GB. The midpoint carries the emission-efficiency improvements; empirically bisected + fixpoint-verified 2026-10-11.
#
#   SP=<scratch with L6.rzrs + B.cf.hoist + the backend spine> bash sem0-stage.sh
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../../.." && pwd)"
CC="${TOWER_L2:-$ROOT/tower/L2}"
DEV="$HERE/../S1-lex-parse-ast/s1_parser.rs"
: "${SP:?set SP (scratch with L6.rzrs + B.cf.hoist)}"
. "$ROOT/tower/lib/scratch.sh"
fail=0
chain(){ "$CC/link/rzir-ld.bin" | "$CC/opt/rzir-opt.bin" | "$CC/ra/rzir-ra.bin" | "$SP/B.cf.hoist" > "$1"; chmod +x "$1"; }
chain3(){ "$SP/B.ld" | "$CC/opt/rzir-opt.bin" | "$CC/ra/rzir-ra.bin" | "$SP/B.cf.hoist" > "$1"; chmod +x "$1"; }
# a //@S9 shim over a given sem0 binary (marker prepended into a FILE -- read_to_string does not loop to EOF on a pipe)
mkshim(){ cat > "$2" <<SHIM
#!/bin/sh
t="\$(mktemp)"; { echo '//@S9'; cat; } > "\$t"; "$1" < "\$t"; r=\$?; rm -f "\$t"; exit \$r
SHIM
chmod +x "$2"; }
big(){ [ "$(stat -c%s "$1" 2>/dev/null || echo 0)" -ge 1000000 ] || { echo "  FAIL: $(basename "$1") truncated"; exit 1; }; }

# hop NAME SRC LOWER: the current frozen ($SP/SEM0.s9) compiles SRC; LOWER makes $SP/NAME; the shim moves to it.
hop(){
  local n="$1" src="$2" lower="$3" l
  timeout 1800 "$SP/SEM0.s9" < "$src" > "$SP/$n.rzir" 2>/dev/null || { echo "  FAIL: $(basename "$src") did not compile ($n)"; exit 1; }
  l=$(wc -l < "$SP/$n.rzir"); [ "$l" -ge 100000 ] || { echo "  FAIL: $n mint truncated ($l lines)"; exit 1; }
  "$lower" "$SP/$n" < "$SP/$n.rzir" 2>/dev/null; big "$SP/$n"
  mkshim "$SP/$n" "$SP/SEM0.s9"
  echo "  $n: minted from $(basename "$src") ($l rzir lines)"
}
# fixpoint NAME SRC: iterate SRC through the current frozen until the emit is a byte fixpoint; the fixpoint is NAME,
# the iterations NAME.i1, NAME.i2, ...
fixpoint(){
  local n="$1" src="$2" it R prev="" l
  for it in 1 2 3 4 5; do
    R="$SP/$n.i$it.rzir"
    timeout 1800 "$SP/SEM0.s9" < "$src" > "$R" 2>/dev/null || { echo "  FAIL: $n iter $it could not compile $(basename "$src")"; exit 1; }
    l=$(wc -l < "$R"); [ "$l" -ge 100000 ] || { echo "  FAIL: $n iter $it truncated ($l lines)"; exit 1; }
    if [ -n "$prev" ] && cmp -s "$R" "$prev"; then
      chain3 "$SP/$n" < "$R" 2>/dev/null; big "$SP/$n"; mkshim "$SP/$n" "$SP/SEM0.s9"
      echo "  $n: byte fixpoint at iter $it ($l rzir lines, $(stat -c%s "$SP/$n")-byte binary; gen2==gen3 met)"
      return
    fi
    chain3 "$SP/$n.i$it" < "$R" 2>/dev/null; big "$SP/$n.i$it"
    mkshim "$SP/$n.i$it" "$SP/SEM0.s9"   # the next iteration emits THROUGH this frozen
    prev="$R"
  done
  echo "  FAIL: $n did not reach a byte fixpoint in 5 iterations"; exit 1
}

# h00: L6 mints the seed (L6 takes the source directly; no //@S9 shim)
timeout 1800 "$SP/L6.rzrs" < "$HERE/hops/h00.rs" > "$SP/SEM0.h00.rzir" 2>/dev/null || { echo "  FAIL: L6 could not compile hops/h00.rs"; exit 1; }
chain "$SP/SEM0.h00" < "$SP/SEM0.h00.rzir" 2>/dev/null; big "$SP/SEM0.h00"
mkshim "$SP/SEM0.h00" "$SP/SEM0.s9"
echo "  SEM0.h00: L6 minted the seed ($(wc -l < "$SP/SEM0.h00.rzir") rzir lines)"
hop SEM0.h01 "$HERE/hops/h01.rs" chain
hop SEM0.h02 "$HERE/hops/h02.rs" chain
for n in 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22; do hop "SEM0.h$n" "$HERE/hops/h$n.rs" chain3; done
fixpoint SEM0.h23 "$HERE/hops/h23.rs"
for n in 24 25 26 27; do hop "SEM0.h$n" "$HERE/hops/h$n.rs" chain3; done
fixpoint SEM0.cf "$DEV"
sp_name SEM0.cf sem0.frozen; sp_name SEM0.s9 sem0.cc

# --- SEM0.cf must compile the DEV target into a WORKING, non-truncated binary ---
timeout 1800 "$SP/SEM0.s9" < "$DEV" > "$SP/sem0_dev.rzir" 2>/dev/null || { echo "  FAIL: SEM0.s9 could not compile the dev target"; fail=1; }
dl=$(wc -l < "$SP/sem0_dev.rzir" 2>/dev/null || echo 0)
[ "$dl" -ge 100000 ] || { echo "  FAIL: dev emission only $dl rzir lines -- truncated"; fail=1; }
grep -aq $'= \x0c\x00\x00 ' "$SP/sem0_dev.rzir" && { echo "  FAIL: corrupt opcode in dev emission"; fail=1; }
chain3 "$SP/.sem0dev.elf" < "$SP/sem0_dev.rzir" 2>/dev/null
timeout 20 setarch -R "$SP/.sem0dev.elf" < /dev/null > /dev/null 2>&1 || { echo "  FAIL: dev binary SIGSEGVs on empty input"; fail=1; }

# --- and a trivial program must run ---
printf 'fn main(){ println!("stage-ok"); }\n' > "$SP/.sem0probe.rs"
chain3 "$SP/.sem0probe.elf" < <("$SP/SEM0.s9" < "$SP/.sem0probe.rs" 2>/dev/null) 2>/dev/null
out=$(timeout 20 setarch -R "$SP/.sem0probe.elf" 2>/dev/null); rc=$?
[ "$out" = "stage-ok" ] && [ "$rc" = 0 ] || { echo "  FAIL: SEM0.cf output does not run (rc=$rc out='$out')"; fail=1; }

echo "== sem0-stage: $fail failed =="
[ "$fail" = 0 ]
