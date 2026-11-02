#!/usr/bin/env bash
# Staged self-improvement bootstrap for the Rust-in-Rust compiler (rzrs).
#
# The move: stop letting the C seed (rzrs.rsz, built by the derived tcc) cap the language rzrs can be
# WRITTEN in. rzrs.rsz is used ONCE to mint tier-0 from the FROZEN seed (L4.rzrs.rs, the C-compilable
# ancestor); the MAINLINE L5.rzrs.rs is now tier-1 and is compiled by tier-0 -- exactly rustc's stage0 ->
# stage1 -> stage2. The mainline is free to use constructs the C seed rejects (the w64 workaround is gone,
# replaced by natural `let w = if a || b { 1 } else { 0 };`).
#
#   origin:  rzrs.rsz (C)  --run once on the FROZEN seed-->  tier-0 binary
#   tier-0:  compiles the MAINLINE L5.rzrs.rs (== tier-1, uses if-||)  -->  tier-1 binary
#   tier-1:  self-reproduces (g2 == g3) + passes the spot battery
#   proof:   rzrs.rsz CANNOT compile the mainline  (we have escaped the seed ceiling)
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
SEED="$HERE/L4.rzrs.rs"                             # FROZEN tier-0 seed source (C-seed-compilable ancestor)
T1="$HERE/../L5/L5.rzrs.rs"        # mainline == tier-1 (natural if-||, beyond the C seed)
CC="${TOWER_L2:-$HERE/../L2}"
SP="${SP:?set SP}"                                    # scratch: L3.rzrs, B.cf.m0 + the all-Rust backend B.{ld,opt,ra}
# The backend chain is now the Rust twins (ld/opt/ra), not the frozen C stages. Populate $SP/B.{ld,opt,ra}
# first via 2026-07-16-backend-rust-hoist/build-live-backend.sh (compiles the canonical, provenance-anchored
# toolchain-derivation/rzir-{ld,opt,ra}.rs). CC retained only for that kit's hex-rooted bootstrap.
# tier-1 is a frontend-scale image, past M0 cf.bin's ~5.7MB ceiling -- which overflows SILENTLY to a
# 259-byte stub, so tier-1 "self-reproduced" (0 lines == 0 lines) and then failed the whole spot battery.
# Lower via $CFCAP (lib/scratch.sh).
. "$HERE/../lib/scratch.sh"
sp_seed || exit 1
chain(){ "$SP/B.ld" | "$SP/B.opt" | "$SP/B.ra" | "$(sp_cf)" > "$1"; chmod +x "$1"; }

fail=0

# --- origin -> tier-0 (from the FROZEN seed) ---------------------------------
"$SP/L3.rzrs" < "$SEED" > "$SP/bootstrap.L4.rzrs.rzir" 2>/dev/null || { echo "FAIL: C seed could not build the frozen seed"; exit 1; }
chain "$SP/bootstrap.L4.rzrs" < "$SP/bootstrap.L4.rzrs.rzir"
echo "tier-0 minted from frozen seed (C seed): $(wc -l < "$SP/bootstrap.L4.rzrs.rzir") rzir lines"

# --- escape proof: the C seed must REJECT the tier-1 source -------------------
if "$SP/L3.rzrs" < "$T1" > /dev/null 2>&1; then
  echo "  FAIL(escape): C seed unexpectedly compiled the tier-1 source"; fail=$((fail+1))
else
  echo "  ESCAPE CONFIRMED: C seed rejects the tier-1 source (if-|| beyond its ceiling)"
fi

# --- tier-0 -> tier-1 --------------------------------------------------------
timeout 90 "$SP/bootstrap.L4.rzrs" < "$T1" > "$SP/bootstrap.L5.rzrs.rzir" 2>/dev/null || { echo "  FAIL: tier-0 could not build tier-1"; exit 1; }
chain "$SP/bootstrap.L5.rzrs" < "$SP/bootstrap.L5.rzrs.rzir"
sp_require "$SP/bootstrap.L5.rzrs" "tier-1 frontend" 100000 || exit 1
echo "tier-1 built by tier-0: $(wc -l < "$SP/bootstrap.L5.rzrs.rzir") rzir lines"

# --- tier-1 self-reproduction: g2 == g3 --------------------------------------
timeout 90 "$SP/bootstrap.L5.rzrs"  < "$T1" > "$SP/bootstrap.L5.rzrs.i2.rzir" 2>/dev/null
chain "$SP/bootstrap.L5.rzrs.i2" < "$SP/bootstrap.L5.rzrs.i2.rzir"
timeout 90 "$SP/bootstrap.L5.rzrs.i2" < "$T1" > "$SP/bootstrap.L5.rzrs.i3.rzir" 2>/dev/null
# a fixed point over two EMPTY emits is not a fixed point -- it is two dead binaries agreeing. Require
# a plausible corpus before believing g2 == g3 (this check read GREEN at 0 lines vs 0 lines).
if [ "$(wc -l < "$SP/bootstrap.L5.rzrs.i2.rzir")" -lt 1000 ]; then
  echo "  FAIL: tier-1 self-emit is $(wc -l < "$SP/bootstrap.L5.rzrs.i2.rzir") lines -- tier-1 is not a working frontend"; fail=$((fail+1))
elif diff -q "$SP/bootstrap.L5.rzrs.i2.rzir" "$SP/bootstrap.L5.rzrs.i3.rzir" >/dev/null; then
  echo "  FIXED POINT: tier-1 gen2 == gen3 ($(wc -l < "$SP/bootstrap.L5.rzrs.i2.rzir") lines) -- tier-1 self-reproduces"
else
  echo "  FAIL: tier-1 gen2 != gen3"; fail=$((fail+1))
fi

# --- tier-1 still compiles real programs (spot battery) ----------------------
pass=0; b=0
ck(){ echo "$1" > "$SP/bootstrap.p.rs"; timeout 10 "$SP/bootstrap.L5.rzrs" < "$SP/bootstrap.p.rs" > "$SP/bootstrap.p.rzir" 2>/dev/null; chain "$SP/bootstrap.p.elf" < "$SP/bootstrap.p.rzir" 2>/dev/null
  timeout 10 "$SP/bootstrap.p.elf"; local rc=$?; b=$((b+1)); if [ "$rc" = "$2" ]; then pass=$((pass+1)); else echo "  battery FAIL ($rc want $2): $1"; fail=$((fail+1)); fi; }
ck 'fn fib(n: i32) -> i32 { if n < 2 { n } else { fib(n-1) + fib(n-2) } } fn main() { fib(10) }' 55
ck 'fn fact(n: i32) -> i32 { if n <= 1 { 1 } else { n * fact(n-1) } } fn main() { fact(5) }' 120
ck 'fn main() { let mut i = 0; let mut s = 0; while i < 5 { s = s + i; i = i + 1; } s }' 10
echo "  tier-1 spot battery: $pass/$b"

echo "== rzrs bootstrap: $([ "$fail" = 0 ] && echo PASS || echo "FAIL ($fail)") =="
[ "$fail" = 0 ]
