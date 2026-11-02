#!/usr/bin/env bash
# stage.sh -- the FREEZE build for the type-inference level.
#
# The move (one rung up from the seed->tier0->mainline staircase): the current MAINLINE (L5.rzrs.rs, with the
# TI0 substrate, frozen at ../L5/L5.rzrs.FROZEN-AT.md) is now STAGE0. It compiles L6.rzrs.rs -- the TI development
# target -- so rzrs-next does NOT have to self-host while the invasive Ty layer (TI1-TI5) is built. This
# decouples TI development from the gen2==gen3 fixed-point straitjacket that made in-place TI painful.
#
#   C-seed (rzrs.rsz) --once--> tier-0 --compiles--> STAGE0 (frozen L5.rzrs.rs, hex-rooted)
#   STAGE0 --compiles--> rzrs-next  (TI dev; NOT required to self-host until promotion)
#   gate: rzrs-next compiled by STAGE0 passes the battery; promotion-readiness = rzrs-next self-reproduces
#
# PROMOTION: when rzrs-next self-reproduces (g2==g3) AND its corpus >= stage0's, copy L6.rzrs.rs over the
# mainline L5.rzrs.rs, re-run selfhost/parity/provenance, and the cycle repeats one level higher.
#
#   SP=<scratch with L3.rzrs + B.cf.m0 + the all-Rust backend B.{ld,opt,ra}> bash stage.sh
# CANONICAL STAGE NAMES (apparatus/docs/tower-naming-and-trust-path.md) -- this is the @dev loop; its
# scratch artifacts are provenance class `dev` (fast-loop convenience, coherence-pinned to the @trust
# re-derivation by provenance.sh step 7). The TRUST path is ALWAYS provenance.sh's @trust artifacts.
#   writes L4.rzrs.g2 (tier-0), L5.rzrs.g2 (frozen mainline; old name stage0), B.cf.hoist (old name cf.hoist),
#   L6.rzrs (dev; was also copied to rzrs.next) and L6.rzrs.i2 (the self-reproduction check; was rzrs.next2)
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
CC="${TOWER_L2:-$HERE/../L2}"
SEED="$HERE/../L4/L4.rzrs.rs"
STAGE0="$HERE/../L5/L5.rzrs.rs"     # FROZEN stage0 source (current mainline + TI0)
NEXT="$HERE/L6.rzrs.rs"                              # TI development target
. "$HERE/../lib/scratch.sh"
sp_seed || exit 1        # cold start: mint cf.bin + rzrs.rsz from the hand-hex if $SP is fresh
# stage0 and the cf.hoist mint are both frontend-scale images, past M0 cf.bin's ceiling -- lower via $CFCAP
# (lib/scratch.sh). chainH below switches to this run's OWN freshly minted cf.hoist once it exists.
chain(){ "$SP/B.ld" | "$SP/B.opt" | "$SP/B.ra" | "$(sp_cf)" > "$1"; chmod +x "$1"; }
fail=0

# --- origin -> tier-0 (from the frozen seed, hex-rooted) ---
"$SP/L3.rzrs" < "$SEED" 2>/dev/null | { chain "$SP/L4.rzrs.g2"; } || { echo "FAIL: C seed -> tier-0"; exit 1; }
# --- tier-0 -> STAGE0 (the frozen mainline binary) ---
timeout 120 "$SP/L4.rzrs.g2" < "$STAGE0" 2>/dev/null | { chain "$SP/L5.rzrs.g2"; } || { echo "FAIL: tier-0 -> stage0"; exit 1; }
sp_require "$SP/L5.rzrs.g2" "L5.rzrs.g2 (frozen mainline frontend)" 100000 || exit 1; sp_name L5.rzrs.g2 stage0
echo "L5.rzrs.g2 built (frozen mainline): $(stat -c%s "$SP/L5.rzrs.g2") bytes"

# cf Rust-hoist: rzrs-next's self-image rzir (~4.7MB) exceeds the M0 cf.bin input buffer (~4MB). Mint the
# provenance-anchored cf.rs codegen (8MB) via the M0 chain and use it for the oversized rzrs-next image.
HOISTCF="$HERE/../backend-cf/cf.rs"
# Mint to a SIDE PATH, then move into place. chain lowers through $(sp_cf), which prefers $SP/cf.hoist once
# one exists -- so minting straight onto $SP/B.cf.hoist would have the output redirect truncate the very binary
# doing the lowering (it did: cf.hoist -> 0 bytes, and rzrs.next came out an empty stub).
timeout 120 "$SP/L5.rzrs.g2" < "$HOISTCF" 2>/dev/null | { chain "$SP/B.cf.hoist.new"; } || { echo "FAIL: mint cf.hoist"; exit 1; }
sp_require "$SP/B.cf.hoist.new" "B.cf.hoist (minted from cf.rs)" 20000 || exit 1
mv "$SP/B.cf.hoist.new" "$SP/B.cf.hoist"; sp_name B.cf.hoist cf.hoist
chainH(){ "$SP/B.ld" | "$SP/B.opt" | "$SP/B.ra" | "$SP/B.cf.hoist" > "$1"; chmod +x "$1"; }

# --- STAGE0 -> rzrs-next (the decoupled TI compiler) ---
timeout 240 "$SP/L5.rzrs.g2" < "$NEXT" > "$SP/L6.rzrs.rzir" 2>/dev/null || { echo "FAIL: stage0 could not compile rzrs-next"; exit 1; }
chainH "$SP/L6.rzrs" < "$SP/L6.rzrs.rzir"   # cf.hoist: the rzrs-next image may exceed M0 cf.bin's buffer
sz=$(stat -c%s "$SP/L6.rzrs"); [ "$sz" -ge 100000 ] || { echo "FAIL: L6.rzrs only $sz bytes (segfault stub)"; exit 1; }
echo "L6.rzrs built by L5.rzrs.g2: $(wc -l < "$SP/L6.rzrs.rzir") rzir lines, $sz-byte binary"
# The 11 S-tier gates (s1-ast-diff, s9-run-diff, s2..s7, ra-cf-diff) consume $SP/L6.rzrs. It used to be a copy of
# $SP/rzrs.next; it is now the only name (rzrs.next had no other reader).


# --- gate: rzrs-next passes the spot battery ---
pass=0
ck(){ echo "$1" > "$SP/p.rs"; timeout 10 "$SP/L6.rzrs" < "$SP/p.rs" > "$SP/p.rzir" 2>/dev/null; chain "$SP/p.elf" < "$SP/p.rzir" 2>/dev/null
  timeout 10 "$SP/p.elf"; local rc=$?; if [ "$rc" = "$2" ]; then pass=$((pass+1)); else echo "  FAIL ($rc want $2): $1"; fail=$((fail+1)); fi; }
ck 'fn main() { 2 + 3 * 4 }' 14
ck 'fn fib(n: i32) -> i32 { if n < 2 { n } else { fib(n-1) + fib(n-2) } } fn main() { fib(10) }' 55
ck 'fn main(){ let v = vec![1,2,3]; v.iter().sum() }' 6
ck 'fn main(){ let t=(1,7); t.0 + t.1 }' 8
echo "spot battery: $pass passed"

# --- promotion readiness: does rzrs-next self-reproduce (g2 == g3)? (informational until promotion) ---
timeout 120 "$SP/L6.rzrs" < "$NEXT" > "$SP/L6.rzrs.i2.rzir" 2>/dev/null && chainH "$SP/L6.rzrs.i2" < "$SP/L6.rzrs.i2.rzir" 2>/dev/null
if [ -s "$SP/L6.rzrs.i2" ]; then
  timeout 120 "$SP/L6.rzrs.i2" < "$NEXT" > "$SP/L6.rzrs.i3.rzir" 2>/dev/null
  if diff -q "$SP/L6.rzrs.i2.rzir" "$SP/L6.rzrs.i3.rzir" >/dev/null 2>&1; then echo "PROMOTION-READY: rzrs-next self-reproduces (g2==g3)"; else echo "not-yet-self-hosting: rzrs-next g2!=g3 (fine during TI dev -- stage0 compiles it)"; fi
else echo "not-yet-self-hosting: rzrs-next cannot yet compile itself (fine during TI dev)"; fi

echo "== stage: $fail failed =="
[ "$fail" = 0 ]
