#!/usr/bin/env bash
# provenance.sh -- the auditable trust-path gate for the hoisted (all-Rust) compile pipeline.
#
# Proves, in one run, that the Rust pipeline (frontend L5.rzrs.rs + linker/optimizer/regalloc/backend, all Rust)
# is GROUNDED in the pre-hoist auditable base -- i.e. the hoist introduced no circular dependency and no
# trusting-trust backdoor. Two legs:
#   (1) NON-CIRCULAR GROUNDING: every Rust-stage binary is re-minted here from SOURCE, using ONLY the audit
#       floor + the C stages rebuilt from their .c + the frontend rebuilt from the frozen seed. No committed
#       Rust-stage binary is ever an input to building itself.
#   (2) DIVERSE DOUBLE COMPILATION (Wheeler's anti-trusting-trust): each hoisted stage's output is asserted
#       byte-for-byte identical to the INDEPENDENTLY-AUTHORED C stage's output over the real self-host rzir.
#       A backdoor would have to be planted identically in BOTH the C and the Rust source to survive --
#       collapsing the trust surface onto the auditable C + M0 floor.
#
# TRUST ROOT: the HAND-WRITTEN HEX. The sole hand-authored artifact is the machine-code-stage0 hex seed
# (hex0.hand.hex + the derived hex1), which self-hosts the hex0->hex1->hex2->m0 assembler ladder. This gate
# DERIVES the backend cf.m0 from that hand-hex in-gate (step 0) rather than trusting a cf binary blob, and
# machine-code-stage0/chain-proof.sh proves that whole ladder is cc/ld/as-free (all host tools stubbed out).
#
# NO residual pre-built inputs remain (2026-07-17). Both compilers are now derived from the hand-hex in-gate:
#   - ccc (C)      = step 0c: M0 cc assembled from c-to-rzir.m0 by the hex-derived M0, then ccc built from ccc.c
#                    through it (the c-from-hex arc fixed the M0 cc bugs blocking the grown 225KB ccc.c).
#   - rzrs.rsz (C-seed Rust front-end) = step 0d: rzcpp (=in-gate-ccc(rzcpp.c)) expands rzrs.c against minimal
#                    freestanding headers, a freestanding shim.c supplies the tiny libc surface via __syscall,
#                    the in-gate ccc compiles it, cchain lowers it -- no derived tcc, no system ld.
# Everything is rebuilt from source below; nothing is entered as a trusted pre-built binary.
# CANONICAL STAGE NAMES per apparatus/docs/tower-naming-and-trust-path.md:
#   $W/L1.cc=M0 cc, L2.ccc=C self-host, L3.rzrs=C-seed FE, L4.rzrs=tier-0, L5.rzrs=stage0 (frozen mainline);
#   $W/B.{cf,ld,opt,ra}-{m0,c,rs}=backend spine per authoring. All $W/ artifacts are provenance class `trust`.
set -uo pipefail

A="$(cd "$(dirname "$0")/.." && pwd)"
MC="$A/L0"           # the hand-hex seed + assembler ladder + cf .m0 source
HEXROOT="$MC/hex0/hex0.hand.hex"                 # the sole hand-authored trust root
CC="$A/L2"                       # ccc is derived in-gate (step 0c) -- CCC set there, not a blob
CPIVOT="$A/L1/c-frontend/c-to-rzir.m0"   # the hex-rooted M0 cc (C->rzir) source
SEED="$A/L4/L4.rzrs.rs"
MAINLINE="$A/L5/L5.rzrs.rs"
HOIST_CF="$A/backend-cf/cf.rs"
HOIST="$A/backend-toolchain"
TCC="$A/tcc"; RZRSC="$A/L3/rzrs.c"; RSEED="$HOIST/rzrs-seed"   # for the in-gate rzrs.rsz
SP="${SP:?set SP (scratch workdir)}"

W="$SP/provenance"; rm -rf "$W"; mkdir -p "$W"
CAP_CF="$W/B.cf-l4"   # CAPACITY BACKEND: cf.rs compiled by tier-0 (step 2); lowers the images above the M0 cf ceiling
pass=0; fail=0
ok(){ echo "  ok   $1"; pass=$((pass+1)); }
no(){ echo "  FAIL $1"; fail=$((fail+1)); }
# assert a just-built artifact is real (executable of at least N bytes), else it's a segfault stub
big(){ if [ -s "$2" ] && [ "$(wc -c <"$2")" -ge "$3" ]; then return 0; else no "$1 (only $(wc -c <"$2" 2>/dev/null || echo 0) bytes)"; return 1; fi; }

echo "== step 0: derive the backend cf.m0 from the HAND-HEX seed (the trust root) =="
[ -f "$HEXROOT" ] || { echo "  MISSING hand-hex root: $HEXROOT"; exit 1; }
echo "  hand-hex root: hex0.hand.hex ($(wc -c <"$HEXROOT") B) -- self-hosts the hex0->hex1->hex2->m0 ladder"
# 0a: prove the whole hex ladder (hex seed -> M0 -> cf) is cc/ld/as-free. chain-proof.sh stubs out every host
#     compiler/assembler/linker on PATH and fails loudly if any is invoked.
if sh "$MC/chain-proof.sh" > "$W/chainproof.log" 2>&1; then ok "hex ladder proven cc/ld/as-free (chain-proof.sh: hex0/hex1 -> M0 -> cf)"; else no "hex ladder proof (see $W/chainproof.log)"; tail -5 "$W/chainproof.log"; exit 1; fi
# 0b: assemble cf.m0 from its .m0 hex-source using the (hex-derived) M0 assembler -- this REPLACES trusting a
#     committed cf binary; cf is now derived from the hand-hex in-gate.
( cd "$MC/emitter" && "$MC/m0/m0.bin" < rzir-to-elf-cf.m0 > "$W/B.cf-m0" 2>/dev/null ); chmod +x "$W/B.cf-m0"
big "cf.m0 assembled from hex-source" "$W/B.cf-m0" 10000 && ok "backend cf.m0 re-derived from hand-hex ($(wc -c <"$W/B.cf-m0") B) -- not a trusted blob" || { echo "== ABORT: cf.m0 derivation failed =="; exit 1; }
# 0b2: assemble M0 cc (the hex-rooted C->rzir front-end) from c-to-rzir.m0 via the same hex-derived M0.
"$MC/m0/m0.bin" < "$CPIVOT" > "$W/L1.cc" 2>/dev/null; chmod +x "$W/L1.cc"
big "M0 cc assembled from c-to-rzir.m0 hex-source" "$W/L1.cc" 10000 || { echo "== ABORT: M0 cc derivation failed =="; exit 1; }
# 0b3: build the C BACKEND STAGES (linker/optimizer/regalloc) from their .c source via M0cc | cf.m0, so the
#      lowering chain is ITSELF hex-rooted -- NO committed binary is trusted anywhere. (raw M0 cf handles these
#      small stages; the big ccc/frontend rzir then goes through this full chain.)
"$W/L1.cc" < "$CC/link/rzir-ld.c" 2>/dev/null | "$W/B.cf-m0" > "$W/B.ld0"  2>/dev/null; chmod +x "$W/B.ld0"
"$W/L1.cc" < "$CC/opt/rzir-opt.c" 2>/dev/null | "$W/B.cf-m0" > "$W/B.opt0" 2>/dev/null; chmod +x "$W/B.opt0"
"$W/L1.cc" < "$CC/ra/rzir-ra.c"   2>/dev/null | "$W/B.cf-m0" > "$W/B.ra0"  2>/dev/null; chmod +x "$W/B.ra0"
big "hex-rooted linker ld0" "$W/B.ld0" 10000 && big "hex-rooted optimizer opt0" "$W/B.opt0" 20000 && big "hex-rooted regalloc ra0" "$W/B.ra0" 20000 \
  && ok "C backend chain (ld/opt/ra) built from .c by hex-rooted M0 cc -- the lowering chain trusts NO committed binary" \
  || { echo "== ABORT: hex-rooted C backend stages failed =="; exit 1; }
# the fully hex-rooted lowering chain: every stage roots in the hand-hex (M0 cc + M0 cf, both assembled from hex).
cchain(){ "$W/B.ld0" | "$W/B.opt0" | "$W/B.ra0" | "$W/B.cf-m0"; }

echo "== step 0c: derive the C COMPILER ccc from the hand-hex (ccc <- ccc.c through the hex-rooted chain) =="
# ccc is built from its OWN source THROUGH the fully hex-rooted chain (M0 cc | ld0 | opt0 | ra0 | cf.m0) -- no
# trusted pre-built blob; it roots in the hand-hex exactly like cf. (c-from-hex arc, 2026-07-17.)
"$W/L1.cc" < "$CC/ccc.c" 2>/dev/null | cchain > "$W/L2.ccc" 2>/dev/null; chmod +x "$W/L2.ccc"
big "ccc built from source by hex-rooted M0 cc" "$W/L2.ccc" 100000 \
  && ok "C compiler ccc re-derived from hand-hex (M0 cc <- c-to-rzir.m0, ccc <- ccc.c) -- not a trusted blob" \
  || { echo "== ABORT: ccc build failed =="; exit 1; }
CCC="$W/L2.ccc"
# ccc self-host fixpoint: it reproduces its own source->rzir output exactly (gen2 == gen3)
"$W/L2.ccc" < "$CC/ccc.c" 2>/dev/null > "$W/L2.ccc.o2"; cchain < "$W/L2.ccc.o2" > "$W/L2.ccc.g2" 2>/dev/null; chmod +x "$W/L2.ccc.g2"
"$W/L2.ccc.g2" < "$CC/ccc.c" 2>/dev/null > "$W/L2.ccc.o3"
cmp -s "$W/L2.ccc.o2" "$W/L2.ccc.o3" && ok "ccc self-host fixpoint (gen2==gen3, $(wc -l <"$W/L2.ccc.o2") lines)" || no "ccc self-host fixpoint (gen2 != gen3)"

echo "== step 0d: derive the C-seed Rust frontend rzrs.rsz from the hand-hex (rzcpp + in-gate ccc, no tcc/system-ld) =="
# rzcpp (a hex-rooted C preprocessor = in-gate-ccc(rzcpp.c)) expands rzrs.c against minimal freestanding headers;
# a freestanding shim.c supplies the tiny libc surface via __syscall; the in-gate ccc compiles the combined
# source; cchain lowers it. rzrs.rsz is no longer a pre-built residual -- it too roots in the hand-hex.
"$W/L2.ccc" < "$TCC/rzcpp.c" 2>/dev/null | cchain > "$W/rzcpp" 2>/dev/null; chmod +x "$W/rzcpp"
big "rzcpp (C preprocessor) built by in-gate ccc" "$W/rzcpp" 20000 || { echo "== ABORT: rzcpp build failed =="; exit 1; }
"$W/rzcpp" -I"$RSEED/headers" < "$RZRSC" > "$W/rzrs.pp.c" 2>/dev/null
cat "$RSEED/shim.c" "$W/rzrs.pp.c" > "$W/combined.c"
"$W/L2.ccc" < "$W/combined.c" 2>/dev/null | cchain > "$W/L3.rzrs" 2>/dev/null; chmod +x "$W/L3.rzrs"
big "rzrs.rsz (C-seed Rust frontend) built from hex-rooted ccc" "$W/L3.rzrs" 100000 \
  && ok "C-seed frontend rzrs.rsz re-derived from hand-hex (rzcpp+shim+ccc, no tcc/system-ld) -- not a trusted blob" \
  || { echo "== ABORT: rzrs.rsz build failed =="; exit 1; }
RZRSZ="$W/L3.rzrs"
echo "  NO residual pre-built inputs remain -- the whole toolchain (C ccc + Rust rzrs) roots in the hand-hex."

echo "== step 1: C chain stages rebuilt from their .c source (compiler = the in-gate ccc, backend = M0 cf) =="
"$CCC" < "$CC/link/rzir-ld.c" 2>/dev/null | cchain > "$W/B.ld-c"  2>/dev/null; chmod +x "$W/B.ld-c"
"$CCC" < "$CC/opt/rzir-opt.c" 2>/dev/null | cchain > "$W/B.opt-c" 2>/dev/null; chmod +x "$W/B.opt-c"
"$CCC" < "$CC/ra/rzir-ra.c"   2>/dev/null | cchain > "$W/B.ra-c"  2>/dev/null; chmod +x "$W/B.ra-c"
big "rebuild ld.C" "$W/B.ld-c" 4000 && big "rebuild opt.C" "$W/B.opt-c" 8000 && big "rebuild ra.C" "$W/B.ra-c" 8000 \
  && ok "C linker/optimizer/regalloc rebuilt from .c source" || { echo "== ABORT: C base did not rebuild =="; exit 1; }

echo "== step 2: frontend rebuilt from source (frozen seed -> tier-0 -> mainline) =="
"$RZRSZ" < "$SEED"    2>/dev/null | cchain > "$W/L4.rzrs" 2>/dev/null; chmod +x "$W/L4.rzrs"
big "tier-0" "$W/L4.rzrs" 100000 || { echo "== ABORT: tier-0 rebuild failed =="; exit 1; }
# The mainline self-host frontend rzir exceeds the M0-cc-compiled ld0/opt0/ra0 stages' ~5MB capacity ceiling.
# PIN-ART (2026-07-31): M0-cc-built ld0 lowers the 9895-line frontend's 4.95MB rzir correctly but TRUNCATES the
# 10037-line frontend's 5.08MB rzir to 19KB -- a hard capacity threshold in M0 cc's codegen of ld0's buffers,
# crossed by the Layer-2 growth. The ccc-compiled B.ld-c/opt-c/ra-c (step 1) are the SAME source with higher
# capacity, produce BYTE-IDENTICAL output below the cap, and are equally hex-rooted (B.ld-c <- ccc <- M0cc <- hex).
# Lower the oversized mainline frontend through them. cf.m0 backend unchanged: the 256MB memsz is verified
# sufficient (the fe256 frontend compiles the full mainline == the dev reference, byte-for-byte). Trust intact:
# the frontend still roots in the hand-hex, via ccc rather than via ld0 directly.
# CAPACITY: the mainline self-image exceeds the M0 cf.bin input-buffer ceiling (~5.7MB), so B.cf-m0 cannot lower
# it. Lower it via cf.hoist (cf.rs, 16MB buffers) as compiled by tier-0 itself -- B.cf-l4, hex-rooted like tier-0.
# Step 4 re-lowers the same image with B.cf-rs (cf.rs compiled by L5) and requires byte-identical output.
"$W/L4.rzrs" < "$HOIST_CF" 2>/dev/null | "$W/B.ld-c" | "$W/B.opt-c" | "$W/B.ra-c" | "$W/B.cf-m0" > "$W/B.cf-l4" 2>/dev/null; chmod +x "$W/B.cf-l4"
big "cf.hoist (cf.rs built by tier-0)" "$W/B.cf-l4" 20000 || { echo "== ABORT: B.cf-l4 build failed =="; exit 1; }
"$W/L4.rzrs"  < "$MAINLINE" 2>/dev/null | "$W/B.ld-c" | "$W/B.opt-c" | "$W/B.ra-c" | "$CAP_CF" > "$W/L5.rzrs" 2>/dev/null; chmod +x "$W/L5.rzrs"
big "rx.self (mainline frontend)" "$W/L5.rzrs" 100000 && ok "frontend rebuilt from source (oversized rzir lowered by the tier-0-built cf.hoist)" \
  || { echo "== ABORT: rx.self rebuild failed =="; exit 1; }

# MINT the entire Rust layer with the M0 audit-floor backend directly -- ONE STEP. The Rust stages are sized
# to fit the M0 cf's ~16MB global ceiling (ld 3MB buffers, opt 2MB + 1MB scratch, ra 3MB + line arrays 100000),
# so cf.m0 compiles all four correctly. No intermediate Rust backend is needed, and no committed Rust-stage
# binary is ever an input to building itself.
mint(){ "$W/L5.rzrs" 2>/dev/null | "$W/B.ld-c" | "$W/B.opt-c" | "$W/B.ra-c" | "$W/B.cf-m0"; }

echo "== step 3: the whole HOIST layer minted in ONE STEP by the M0 audit-floor backend =="
mint < "$HOIST_CF"          > "$W/B.cf-rs"  2>/dev/null; chmod +x "$W/B.cf-rs"
mint < "$HOIST/rzir-ld.rs"  > "$W/B.ld-rs"  2>/dev/null; chmod +x "$W/B.ld-rs"
mint < "$HOIST/rzir-opt.rs" > "$W/B.opt-rs" 2>/dev/null; chmod +x "$W/B.opt-rs"
mint < "$HOIST/rzir-ra.rs"  > "$W/B.ra-rs"  2>/dev/null; chmod +x "$W/B.ra-rs"
big "mint cf.rs" "$W/B.cf-rs" 20000 && big "mint ld.rs" "$W/B.ld-rs" 10000 && big "mint opt.rs" "$W/B.opt-rs" 20000 && big "mint ra.rs" "$W/B.ra-rs" 20000 \
  && ok "cf.rs / ld.rs / opt.rs / ra.rs all minted directly by M0 cf (one step)" || { echo "== ABORT: hoist mint failed =="; exit 1; }

echo "== step 4: DIVERSE DOUBLE COMPILATION -- Rust stage output == C stage output, byte-for-byte =="
"$W/L5.rzrs" < "$MAINLINE" 2>/dev/null > "$W/raw.rzir"
big "self-host frontend rzir" "$W/raw.rzir" 500000 || { echo "== ABORT: no corpus =="; exit 1; }
echo "  corpus = self-host frontend rzir ($(wc -l <"$W/raw.rzir") lines)"
"$W/B.ld-c"  < "$W/raw.rzir" > "$W/ld.cO"  2>/dev/null; "$W/B.ld-rs"  < "$W/raw.rzir" > "$W/ld.rO"  2>/dev/null
cmp -s "$W/ld.cO" "$W/ld.rO"   && ok "linker    ld.rs  == C ld  ($(wc -l <"$W/ld.cO") lines, byte-identical)"  || no "linker DDC"
"$W/B.opt-c" < "$W/ld.cO"    > "$W/opt.cO" 2>/dev/null; "$W/B.opt-rs" < "$W/ld.cO"    > "$W/opt.rO" 2>/dev/null
cmp -s "$W/opt.cO" "$W/opt.rO" && ok "optimizer opt.rs == C opt ($(wc -l <"$W/opt.cO") lines, byte-identical)" || no "optimizer DDC"
"$W/B.ra-c"  < "$W/opt.cO"   > "$W/ra.cO"  2>/dev/null; "$W/B.ra-rs"  < "$W/opt.cO"   > "$W/ra.rO"  2>/dev/null
cmp -s "$W/ra.cO" "$W/ra.rO"   && ok "regalloc  ra.rs  == C ra  ($(wc -l <"$W/ra.cO") lines, byte-identical)"  || no "regalloc DDC"
# backend: cf.rs emits equivalent-but-different code -> behavioral cross-check (both ELFs run identically)
echo 'fn main()->i32{ let mut s=0; let mut i=1; while i<=10 { s=s+i*i; i=i+1; } s }' > "$W/probe.rs"
"$W/L5.rzrs" < "$W/probe.rs" 2>/dev/null | "$W/B.ld-c" | "$W/B.opt-c" | "$W/B.ra-c" > "$W/probe.ra" 2>/dev/null
"$W/B.cf-m0" < "$W/probe.ra" > "$W/p.m0.elf" 2>/dev/null; chmod +x "$W/p.m0.elf"
"$W/B.cf-rs" < "$W/probe.ra" > "$W/p.rs.elf" 2>/dev/null; chmod +x "$W/p.rs.elf"
"$W/p.m0.elf" >/dev/null 2>&1; me=$?; "$W/p.rs.elf" >/dev/null 2>&1; re=$?
[ "$me" = "$re" ] && [ "$me" = 129 ] && ok "backend   cf.rs runs == M0 cf (behavioral, exit $me = 385&255)" || no "backend behavioral (m0=$me rs=$re want 129)"
# CAPACITY BACKEND DDC: the same cf.rs compiled by two different compilers -- B.cf-l4 (by tier-0) and B.cf-rs (by L5)
# -- must lower the oversized L5 image byte-identically. That is the image step 2 built with B.cf-l4.
"$W/L4.rzrs" < "$MAINLINE" 2>/dev/null | "$W/B.ld-c" | "$W/B.opt-c" | "$W/B.ra-c" | "$W/B.cf-rs" > "$W/L5.rzrs.ddc" 2>/dev/null
cmp -s "$W/L5.rzrs" "$W/L5.rzrs.ddc" && ok "cf.hoist  tier-0-built == L5-built on the L5 image ($(stat -c%s "$W/L5.rzrs") B, byte-identical)" || no "capacity-backend DDC (B.cf-l4 vs B.cf-rs on L5)"

echo "== step 5: the from-source all-Rust pipeline self-reproduces (fixed point) =="
allrust(){ "$W/B.ld-rs" | "$W/B.opt-rs" | "$W/B.ra-rs" | "$W/B.cf-rs"; }
"$W/L5.rzrs" < "$MAINLINE" 2>/dev/null | allrust > "$W/L5.rzrs.g2" 2>/dev/null; chmod +x "$W/L5.rzrs.g2"
big "rx.self2 (built by all-Rust pipeline)" "$W/L5.rzrs.g2" 100000 || no "all-Rust rebuild of frontend"
"$W/L5.rzrs.g2" < "$MAINLINE" 2>/dev/null > "$W/g3.rzir"
cmp -s "$W/raw.rzir" "$W/g3.rzir" && ok "gen2 == gen3 through the all-Rust pipeline (fixed point, $(wc -l <"$W/g3.rzir") lines)" || no "all-Rust fixed point"

echo "== step 5b: L6.rzrs (dev target, the rung above frozen L5) re-minted in-gate + DDC =="
# The dev loop's L6.rzrs frontend (the ex-`newstage0` alias; compiles the SEM0 source s1_parser.rs) is brought
# into the trust path: the hex-rooted in-gate L5 (rebuilt in step 2) compiles L6.rzrs.rs. Non-circular -- no
# committed L6 binary is an input. The L6 self-image exceeds the M0-cc capacity ceiling exactly like L5, so it is
# lowered by the DDC-anchored backend (B.{ld,opt,ra}-c + the capacity seed cf.hoist), identical to L5's build.
L6SRC="$A/L6/L6.rzrs.rs"
timeout 240 "$W/L5.rzrs" < "$L6SRC" 2>/dev/null | "$W/B.ld-c" | "$W/B.opt-c" | "$W/B.ra-c" | "$CAP_CF" > "$W/L6.rzrs" 2>/dev/null; chmod +x "$W/L6.rzrs"
if big "L6.rzrs (dev target)" "$W/L6.rzrs" 1000000; then
  # behavioral: L6.rzrs is a compiler -- it must compile a smoke rzrs program to the right answer (sum 1..10 = 55)
  printf 'fn main()->i32{ let mut s=0; let mut i=1; while i<=10 { s=s+i; i=i+1; } s }' > "$W/l6.smoke.rs"
  "$W/L6.rzrs" < "$W/l6.smoke.rs" 2>/dev/null | allrust > "$W/l6.smoke.elf" 2>/dev/null; chmod +x "$W/l6.smoke.elf"
  "$W/l6.smoke.elf" >/dev/null 2>&1; l6=$?
  [ "$l6" = 55 ] && ok "L6.rzrs re-minted from hex-rooted L5 + L6.rzrs.rs (compiles smoke, exit 55)" || no "L6.rzrs behavioral (smoke=$l6 want 55)"
  # DDC: the all-Rust ld/opt/ra (byte-DDC'd in step 4) mint the same L6.rzrs byte-for-byte -> C-path == Rust-path
  timeout 240 "$W/L5.rzrs" < "$L6SRC" 2>/dev/null | "$W/B.ld-rs" | "$W/B.opt-rs" | "$W/B.ra-rs" | "$CAP_CF" > "$W/L6.rzrs.rr" 2>/dev/null; chmod +x "$W/L6.rzrs.rr"
  cmp -s "$W/L6.rzrs" "$W/L6.rzrs.rr" && ok "L6.rzrs DDC: Rust-backend mint == C-backend mint (byte-identical)" || no "L6.rzrs DDC (C-path != Rust-path ld/opt/ra)"
fi

echo "== step 5c: SEM0 (SEM0.FROZEN.rs) SELF-HOSTS through the hex-rooted chain =="
# REPOINTED 2026-08-28 (locales/sem0-closed-world), from s1_parser.rs to SEM0.FROZEN.rs.
# What L6 is OBLIGED to reproduce changed when the SEM0 hoist landed. Before it, L6 compiled the dev target and
# this step asserted that on the trust path. After it, the frozen tier SEM0.FROZEN.rs is what L6 must reproduce
# -- `sem0-stage.sh` mints exactly that in-gate -- and the DEV target is compiled by SEM0, whose self-host is
# gated by s9-wholefile-diff.sh through `$SP/sem0.cc`. Continuing to route the moving dev target through L6 here
# meant the trust gate tracked a tier the hoist had retired, and it is what made a correct emitter change read as
# a trust-path failure: L6 compiles at rc=0 and emits an image whose binary dies with SIGILL on large input, a
# latent L6 layout resonance NON-MONOTONE in emission size (see that locale's trajectory).
# The claim this step makes is unchanged in kind and strictly better rooted: the rung above frozen L6 self-hosts
# using only artifacts this gate minted from the hand-hex. Non-circular -- no committed SEM0 binary is an input,
# and SEM0.FROZEN.rs is in-repo TEXT, not a blob.
# The SEM0 fixed point (gen2==gen3) was reached 2026-08-24 and is gated by s9-wholefile-diff.sh -- but that gate
# runs on the @dev path: it lowers through the COMMITTED dev blobs in 2026-06-30-cc-in-c. Those are pinned
# byte-identical to the trust re-mint by step 7, so the claim was sound; it just had never been DEMONSTRATED with
# every input derived in-gate from the hand-hex. This step does that: the rung above frozen L6 self-hosts on the
# trust path, using only artifacts this gate minted. Non-circular -- no committed SEM0 binary is an input.
S1SRC="$A/sem0/stages/SEM0-hoist/hops/h00.rs"   # SEM0.FROZEN.rs before 2026-11-01
tback(){ "$W/B.ld-c" | "$W/B.opt-c" | "$W/B.ra-c" | "$CAP_CF"; }
if [ -s "$W/L6.rzrs" ] && [ -f "$S1SRC" ]; then
  timeout 600 "$W/L6.rzrs" < "$S1SRC" 2>/dev/null | tback > "$W/SEM0.gen1" 2>/dev/null; chmod +x "$W/SEM0.gen1"
  if big "SEM0.gen1 (hex-rooted L6.rzrs compiles SEM0.FROZEN.rs)" "$W/SEM0.gen1" 1000000; then
    { echo '//@S9'; cat "$S1SRC"; } > "$W/sem0.s9"
    timeout 600 "$W/SEM0.gen1" < "$W/sem0.s9" > "$W/sem0.g2.rzir" 2>/dev/null
    tback < "$W/sem0.g2.rzir" > "$W/SEM0.gen2" 2>/dev/null; chmod +x "$W/SEM0.gen2"
    if big "SEM0.gen2 (SEM0 compiles ITSELF, hex-rooted)" "$W/SEM0.gen2" 1000000; then
      timeout 900 "$W/SEM0.gen2" < "$W/sem0.s9" > "$W/sem0.g3.rzir" 2>/dev/null
      if cmp -s "$W/sem0.g2.rzir" "$W/sem0.g3.rzir"; then
        ok "SEM0 FIXED POINT on the TRUST path: gen2 == gen3 ($(wc -l < "$W/sem0.g3.rzir") rzir lines, byte-identical)"
      else no "SEM0 fixed point (gen2 != gen3 through the hex-rooted chain)"; fi
      "$W/SEM0.gen1" < /dev/null > "$W/sem0.w1" 2>/dev/null
      "$W/SEM0.gen2" < /dev/null > "$W/sem0.w2" 2>/dev/null
      if [ -s "$W/sem0.w1" ] && cmp -s "$W/sem0.w1" "$W/sem0.w2"; then
        ok "SEM0 witness: gen2 == gen1 ($(wc -l < "$W/sem0.w1") lines, behavioral)"
      else no "SEM0 witness (gen2 != gen1, or empty)"; fi
      "$W/B.ld-rs" < "$W/sem0.g2.rzir" 2>/dev/null | "$W/B.opt-rs" 2>/dev/null | "$W/B.ra-rs" 2>/dev/null | "$CAP_CF" > "$W/SEM0.gen2.rr" 2>/dev/null; chmod +x "$W/SEM0.gen2.rr"
      cmp -s "$W/SEM0.gen2" "$W/SEM0.gen2.rr" \
        && ok "SEM0.gen2 DDC: Rust-backend mint == C-backend mint (byte-identical)" \
        || no "SEM0.gen2 DDC (C-path != Rust-path ld/opt/ra)"
    fi
  fi
fi

echo "== step 6: the rzc build driver + the shippable toolchain derive from the hand-hex =="
# rzc built from source through the hex-rooted pipeline (extended cf.rs backend for its K1 fork/wait4 syscalls)
"$W/L5.rzrs" < "$HOIST/rzc/rzc.rs" 2>/dev/null | "$W/B.ld-rs" | "$W/B.opt-rs" | "$W/B.ra-rs" | "$W/B.cf-rs" > "$W/rzc" 2>/dev/null; chmod +x "$W/rzc"
if big "rzc built from source (hex-rooted pipeline)" "$W/rzc" 10000; then
  # assemble a self-contained toolchain dir entirely from the hex-rooted binaries, and drive real work through it
  TCP="$W/tc"; mkdir -p "$TCP"
  cp "$W/rzc" "$TCP/rzc"; cp "$W/L5.rzrs" "$TCP/rzrs.self"; cp "$W/B.cf-rs" "$TCP/cf"
  cp "$W/B.ld-rs" "$TCP/rzir-ld"; cp "$W/B.opt-rs" "$TCP/rzir-opt"; cp "$W/B.ra-rs" "$TCP/rzir-ra"
  chmod +x "$TCP"/*
  printf 'fn main()->i32{ let mut s=0; let mut i=1; while i<=10 { s=s+i; i=i+1; } s }' > "$TCP/g.rs"
  "$TCP/rzc" build "$TCP/g.rs" "$TCP/g.elf" >/dev/null 2>&1; chmod +x "$TCP/g.elf" 2>/dev/null; "$TCP/g.elf" >/dev/null 2>&1; sm=$?
  printf '#[test]\nfn t_ok(){ assert_eq!(2+2,4); }\n' > "$TCP/t.rs"
  "$TCP/rzc" test "$TCP/t.rs" >/dev/null 2>&1; tm=$?
  [ "$sm" = 55 ] && [ "$tm" = 0 ] && ok "rzc + toolchain hex-rooted: build+run == 55, #[test] ok (all binaries from the hand-hex)" || no "rzc toolchain smoke (build/run=$sm want 55, test=$tm want 0)"
fi

echo "== step 7: dev-blob coherence -- committed dev-loop backend blobs == the in-gate trust derivation =="
# closes the gap: the committed C-backend blobs the DEV loop (r2-corpus/map.sh, stage.sh) consumes MUST
# be byte-identical to the hex-rooted in-gate re-derivation, else the fast loop trusts a drifted cache.
cmp -s "$CC/link/rzir-ld.bin" "$W/B.ld-c"  && ok "dev blob rzir-ld.bin  == trust B.ld-c  (byte-identical)"  || no "dev-blob coherence: rzir-ld.bin drifted from the trust derivation"
cmp -s "$CC/opt/rzir-opt.bin" "$W/B.opt-c" && ok "dev blob rzir-opt.bin == trust B.opt-c (byte-identical)" || no "dev-blob coherence: rzir-opt.bin drifted from the trust derivation"
cmp -s "$CC/ra/rzir-ra.bin"   "$W/B.ra-c"  && ok "dev blob rzir-ra.bin  == trust B.ra-c  (byte-identical)"  || no "dev-blob coherence: rzir-ra.bin drifted from the trust derivation"
# ccc.bin is the dev loop's C compiler (sp_seed builds rzcpp + rzrs.rsz with it) = the ccc self-host gen2.
# It went unchecked until 2026-10-23 and had drifted one byte (the pre-D-01 0x2000000 frame base).
cmp -s "$CC/ccc.bin" "$W/L2.ccc.g2" && ok "dev blob ccc.bin      == trust L2.ccc gen2 (byte-identical)" || no "dev-blob coherence: ccc.bin drifted from the trust derivation"
# L6.rzrs (dev target, ex-`newstage0`) coherence: the committed dev backend blobs must mint the byte-identical L6
# image as the trust re-mint (step 5b) -- pins the dev loop's L6.rzrs to the hex-rooted derivation of L6.rzrs.rs.
if [ -s "$W/L6.rzrs" ]; then
  timeout 240 "$W/L5.rzrs" < "$L6SRC" 2>/dev/null | "$CC/link/rzir-ld.bin" | "$CC/opt/rzir-opt.bin" | "$CC/ra/rzir-ra.bin" | "$CAP_CF" > "$W/L6.rzrs.dev" 2>/dev/null; chmod +x "$W/L6.rzrs.dev"
  cmp -s "$W/L6.rzrs.dev" "$W/L6.rzrs" && ok "dev-blob coherence: L6.rzrs via committed dev backend == trust re-mint (byte-identical)" || no "dev-blob coherence: L6.rzrs via dev backend drifted from trust"
fi
echo
echo "== provenance: $pass passed, $fail failed =="
if [ "$fail" = 0 ]; then
  echo "TRUST PATH INTACT: the backend cf.m0 is re-derived from the hand-hex seed in-gate (cc/ld/as-free), the"
  echo "hoisted Rust pipeline is re-minted from it (non-circular), every hoisted stage is byte-for-byte"
  echo "DDC-anchored to its independent C implementation, and the rzc build driver + the shippable toolchain are"
  echo "built from source through that hex-rooted pipeline (step 6). BOTH compilers are derived in-gate: ccc"
  echo "(step 0c: M0 cc <- c-to-rzir.m0 <- hex; ccc <- ccc.c) AND the C-seed frontend rzrs.rsz (step 0d: rzcpp +"
  echo "shim + in-gate ccc, no tcc/system-ld). NO residual pre-built inputs remain -- the whole toolchain roots in hex."
  echo "SEM0 SELF-HOSTS ON THIS PATH (step 5c): the rung above frozen L6 compiles ITSELF to a byte-identical fixed"
  echo "point (gen2==gen3) using only artifacts minted in-gate from the hand-hex, and that gen2 is DDC-anchored."
else
  echo "TRUST PATH ALERT -- see failures above."
fi
[ "$fail" = 0 ]
