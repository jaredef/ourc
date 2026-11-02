#!/usr/bin/env bash
# Prove the self-host loop: C rzrs compiles L5.rzrs.rs -> L5.rzrs.g1 (a Rust-written compiler); L5.rzrs.g1
# compiles a battery of tiny Rust programs, each exiting with the expected code.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; CC="${TOWER_L2:-$HERE/../L2}"; SP="${SP:?set SP}"
# Backend chain = the all-Rust twins. Populate $SP/B.{ld,opt,ra} via
# 2026-07-16-backend-rust-hoist/build-live-backend.sh (canonical toolchain-derivation/rzir-*.rs) before running.
chain(){ "$SP/B.ld" | "$SP/B.opt" | "$SP/B.ra" | "$SP/B.cf.m0" > "$1"; chmod +x "$1"; }
chainH(){ "$SP/B.ld" | "$SP/B.opt" | "$SP/B.ra" | "$CFH" > "$1"; chmod +x "$1"; }
# CAPACITY: the mainline self-image rzir outgrew the M0 cf input ceiling (~5.7MB), so it is lowered through
# cf.hoist (cf.rs, 16MB buffers). The first lowering uses $SP/B.cf.hoist.l4 -- cf.rs compiled by the hex-rooted L4,
# minted by sp_seed below. L5.rzrs.g1 then mints its OWN cf.hoist (B.cf.hoist.g1), and lib/check-cfhoist-ddc.sh
# requires the two to lower L5 byte-identically (diverse double compilation).
# cold start: mint cf.bin + rzrs.rsz from the hand-hex when $SP is fresh (this gate already handled capacity
# correctly -- it is the model lib/scratch.sh generalizes -- but it still assumed the two roots pre-existed).
. "$HERE/../lib/scratch.sh"
sp_seed || exit 1
CFH="$SP/B.cf.hoist.l4"   # chainH lowers through the L4-built cf.hoist until L5 mints B.cf.hoist.g1
# Staged bootstrap: the mainline L5.rzrs.rs is now tier-1 (uses natural if-|| the C seed rejects), so it is
# built by tier-0 -- minted once from the FROZEN seed by the C seed -- not by the C seed directly.
SEED="$HERE/../L4/L4.rzrs.rs"
"$SP/L3.rzrs" < "$SEED" > "$SP/L4.rzrs.rzir" 2>/dev/null || { echo "C seed FAILED on frozen seed"; exit 1; }
chain "$SP/L4.rzrs.m0" < "$SP/L4.rzrs.rzir"
timeout 90 "$SP/L4.rzrs.m0" < "$HERE/L5.rzrs.rs" > "$SP/L5.rzrs.g1.rzir" 2>/dev/null || { echo "tier-0 FAILED on mainline L5.rzrs.rs"; exit 1; }
chainH "$SP/L5.rzrs.g1" < "$SP/L5.rzrs.g1.rzir"   # CAPACITY: lower the oversized self-image via the L4-built cf.hoist (cf.bin overflows)
# cf Rust-hoist: the mainline's self-image rzir (~4.7MB post-TI-promotion) outgrew the M0 cf.bin input
# buffer (~4MB). Build the provenance-anchored cf.rs codegen (8MB buffers, behaviorally == M0 cf per
# provenance.sh) THROUGH the hex-rooted chain (M0 cf.bin mints it -- cf.rs's own rzir is small), and use
# it only for the oversized self-reproduction images. M0 cf (B.cf.m0) still builds tier-0 + the whole
# spot battery, so the minimal anchor still validates the small surface.
HOISTCF="$HERE/../backend-cf/cf.rs"
timeout 120 "$SP/L5.rzrs.g1" < "$HOISTCF" 2>/dev/null | chain "$SP/B.cf.hoist.g1"
[ "$(stat -c%s "$SP/B.cf.hoist.g1" 2>/dev/null)" -ge 20000 ] || { echo "FAIL: cf.hoist re-mint (new L5 could not mint its own cf.hoist)"; exit 1; }
CFH="$SP/B.cf.hoist.g1"; sp_name L5.rzrs.g1 rzrs.self; sp_name B.cf.hoist.g1 cf.hoist
pass=0; fail=0
ck(){ echo "$1" > "$SP/p.rs"; timeout 10 "$SP/L5.rzrs.g1" < "$SP/p.rs" > "$SP/p.rzir" 2>/dev/null; chain "$SP/p.elf" < "$SP/p.rzir" 2>/dev/null
  timeout 10 "$SP/p.elf"; local rc=$?; if [ "$rc" = "$2" ]; then pass=$((pass+1)); else echo "  FAIL ($rc want $2): $1"; fail=$((fail+1)); fi; }
ck 'fn main() { 2 + 3 * 4 }' 14
ck 'fn main() { let a = (2 + 3) * (4 + 1); a }' 25
ck 'fn main() { let a = 100; a / 7 }' 14
ck 'fn main() { let a = 100; a % 7 }' 2
ck 'fn sq(x: i32) -> i32 { x * x } fn main() { sq(5) + 1 }' 26
ck 'fn main() { let x = 7; if x > 5 { 100 } else { 200 } }' 100
ck 'fn main() { let mut i = 0; let mut s = 0; while i < 5 { s = s + i; i = i + 1; } s }' 10
ck 'fn fib(n: i32) -> i32 { if n < 2 { n } else { fib(n-1) + fib(n-2) } } fn main() { fib(10) }' 55
ck 'fn fact(n: i32) -> i32 { if n <= 1 { 1 } else { n * fact(n-1) } } fn main() { fact(5) }' 120
ck 'fn main() { let a = 3; let b = 4; if a < b && b < 10 { 1 } else { 0 } }' 1
ck 'fn main() { let mut n = 5; let mut f = 1; while n > 1 { f = f * n; n = n - 1; } f }' 120
ck 'static mut X: i32 = 0; fn main() -> i32 { unsafe { X = 42; X + 1 } }' 43
ck 'static mut A: [i32; 16] = [0; 16]; fn main() -> i32 { unsafe { let mut i = 0; while i < 16 { A[i] = i * i; i = i + 1; } A[5] } }' 25
ck 'static mut A: [i32; 8] = [0; 8]; fn sum() -> i32 { unsafe { let mut s = 0; let mut i = 0; while i < 8 { s = s + A[i]; i = i + 1; } s } } fn main() -> i32 { unsafe { let mut i = 0; while i < 8 { A[i] = i; i = i + 1; } sum() } }' 28
ck 'static mut B: [u8; 32] = [0; 32]; fn main() -> i32 { unsafe { B[0] = 65; B[1] = 66; (B[0] + B[1]) } }' 131

# --- self-reproduction: L5.rzrs.rs compiles ITSELF to a byte-identical fixed point (gen2 == gen3) ---
timeout 240 "$SP/L5.rzrs.g1" < "$HERE/L5.rzrs.rs" > "$SP/L5.rzrs.i2.rzir" 2>/dev/null
chainH "$SP/L5.rzrs.i2" < "$SP/L5.rzrs.i2.rzir"   # cf.hoist: the i2 rzir may exceed the M0 cf.bin buffer
timeout 240 "$SP/L5.rzrs.i2" < "$HERE/L5.rzrs.rs" > "$SP/L5.rzrs.i3.rzir" 2>/dev/null
if diff -q "$SP/L5.rzrs.i2.rzir" "$SP/L5.rzrs.i3.rzir" >/dev/null; then echo "  FIXED POINT: gen2 == gen3 ($(wc -l < "$SP/L5.rzrs.i2.rzir") lines) -- L5.rzrs.rs self-reproduces"; pass=$((pass+1)); else echo "  FAIL: gen2 != gen3"; fail=$((fail+1)); fi
echo "== self-host: $pass passed, $fail failed =="
[ "$fail" = 0 ]
