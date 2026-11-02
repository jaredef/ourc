#!/usr/bin/env bash
# build-live-backend.sh -- populate $SP with the all-Rust backend the daily gates run.
#
# The live gate chain used to be the frozen hex-rooted C stages ($CC/rzir-{ld,opt,ra}.bin) + cf.bin.
# The backend is now Rust: the CANONICAL, provenance-anchored twins live in
#   apparatus/arcs/backend-toolchain/rzir-{ld,opt,ra}.rs
# (DDC byte-identical to the C stages, minted from the hand-hex in provenance.sh). This kit compiles
# them into $SP/B.{ld,opt,ra} (old names {ld,opt,ra}.bin) so selfhost.sh / parity.sh / bootstrap.sh can run the all-Rust chain.
#
# Bootstrap is hex-rooted: the FROZEN C stages ($CC, the committed trust anchors) + $SP/cf.bin build a
# frontend from the frozen seed, and that frontend compiles the Rust twins. No system cc/ld is used.
# Prereqs in $SP (same convention as the gates): L3.rzrs, B.cf.m0 (sp_seed mints both).
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
CC="${TOWER_L2:-$HERE/../L2}"
SEED="$HERE/../L4/L4.rzrs.rs"
MAINLINE="$HERE/../L5/L5.rzrs.rs"
TW="$HERE/../backend-toolchain"          # canonical, provenance-anchored twin sources
. "$HERE/../lib/scratch.sh"   # sp_seed, sp_cf, sp_chain_c -- no local lowering copy
sp_seed || exit 1        # cold start: mint B.cf.m0 + L3.rzrs from the hand-hex if $SP is fresh

# bootstrap the frontend with the FROZEN C backend (hex-rooted anchors) -- L3.rzrs -> L4.rzrs -> L5.rzrs
# The FRONTEND image (tier-0's compile of the mainline) is ~233K rzir lines -- far past M0 cf.bin's ~5.7MB
# input-buffer ceiling, which overflows SILENTLY into a 259-byte stub. Lower through $CFCAP (see lib/scratch.sh).
"$SP/L3.rzrs" < "$SEED"     2>/dev/null | sp_chain_c "$SP/L4.rzrs"   || { echo "FAIL: C seed -> tier-0"; exit 1; }
timeout 90 "$SP/L4.rzrs" < "$MAINLINE" 2>/dev/null | sp_chain_c "$SP/L5.rzrs" || { echo "FAIL: tier-0 -> frontend"; exit 1; }
sp_require "$SP/L5.rzrs" "L5.rzrs (bootstrap frontend)" 100000 || exit 1

# compile each canonical Rust twin through the frozen C backend -> $SP/B.{ld,opt,ra}
for st in ld opt ra; do
  timeout 90 "$SP/L5.rzrs" < "$TW/rzir-$st.rs" 2>/dev/null | sp_chain_c "$SP/B.$st" \
    || { echo "FAIL: compile rzir-$st.rs"; exit 1; }
  sz=$(stat -c%s "$SP/B.$st")
  [ "$sz" -ge 10000 ] || { echo "FAIL: B.$st only $sz bytes (segfault stub)"; exit 1; }
  sp_name "B.$st" "$st.bin"; echo "built $SP/B.$st ($sz B) from canonical rzir-$st.rs"
done
echo "live backend ready: \$SP/B.{ld,opt,ra} + B.cf.m0 -- the daily gates now run the all-Rust chain"
