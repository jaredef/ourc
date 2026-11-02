#!/usr/bin/env bash
# scratch.sh -- shared $SP (scratch) conventions for the tower gates. SOURCE this; do not run it.
#
#   . "$A/lib/scratch.sh"     # $A = apparatus/arcs/..  (i.e. the apparatus dir's arcs parent)
#   sp_seed                   # cold-start: mint B.cf.m0, L3.rzrs (old names cf.bin, rzrs.rsz) and B.cf.hoist.l4 if absent
#   sp_chain_rs OUT < rzir    # lower via the all-Rust twins  ($SP/B.{ld,opt,ra})
#   sp_chain_c  OUT < rzir    # lower via the frozen C stages ($CC/rzir-{ld,opt,ra}.bin)
#   sp_require FILE LABEL     # fail loudly on a segfault stub instead of carrying it forward
#
# Three things every gate needs, which each script used to solve alone -- or fail to solve:
#
# 1. COLD START. A fresh $SP holds nothing, and no script minted the two irreducible inputs: cf.bin (the M0
#    backend, assembled from the hand-hex) and rzrs.rsz (the C-seed Rust frontend: rzcpp preprocesses rzrs.c
#    against the minimal freestanding headers, the hex-rooted ccc compiles it). Every gate simply assumed
#    both were already there, so a cleared /tmp meant hand-reconstructing the root before anything would run.
#    sp_seed mints both from in-repo source. Idempotent, and host cc/ld-free.
#
# 2. CAPACITY -- and this is the one that bites silently. M0 cf.bin has a ~5.7MB input-buffer ceiling. EVERY
#    frontend-scale image (L5, L6, the S1 self-emit) has now outgrown it. cf.bin does not report the
#    overflow: it SEGVs mid-write and leaves a ~259-byte ELF stub, which is executable, so `chain` succeeds,
#    `[ -x ... ]` passes, and the stub is carried forward as if it were a compiler. Downstream this reads as
#    "the frontend miscompiles everything" rather than "the backend could not fit the input" -- parity.sh
#    reported 0/90 and bootstrap.sh's spot battery reported three miscompiles, all from this one cause.
#    sp_chain_* lower through $CFCAP, the capacity-capable cf, so the ceiling stops being a tripwire.
#
# 3. STUB DETECTION. sp_require is the guard for the class above: a produced binary below a plausible floor
#    is a build failure, never a test failure. Gates that compare two artifacts must also refuse to call two
#    EMPTY outputs equal -- see bootstrap.sh's fixed-point check, which passed on 0 lines vs 0 lines.
#
# PROVENANCE. $CFCAP resolves to $SP/cf.hoist when a run has minted its own, else $SP/B.cf.hoist.l4: cf.rs compiled
# by L4, which L3 compiles, all lowered by the frozen C stages and M0 cf. It is hex-rooted like everything else
# here; no committed backend binary is used.

SP="${SP:?set SP (scratch dir)}"
_SL_LIB="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
_SL_A="$(cd "$_SL_LIB/.." && pwd)"
_SL_CC="${TOWER_L2:-$_SL_A/L2}"
_SL_MC="$_SL_A/L0"
_SL_CFL4="$SP/B.cf.hoist.l4"   # minted by sp_seed

# sp_name NEW OLD..: artifacts are written under their canonical names (tower-naming-and-trust-path.md, "Artifact
# names"); the old names are hard links to them for the gates that still read those. Hard links, not symlinks:
# gates `stat -c%s` these files, and stat reports a symlink's own size.
sp_name() { local n="$1" o; shift; for o in "$@"; do ln -f "$SP/$n" "$SP/$o"; done; }

# the capacity-capable cf: a run's own freshly minted cf.hoist wins (L6/stage.sh's B.cf.hoist, else
# L5/selfhost.sh's B.cf.hoist.g1); else the hex-rooted B.cf.hoist.l4 that sp_seed mints.
sp_cf() {
  local f
  for f in "$SP/B.cf.hoist" "$SP/B.cf.hoist.g1"; do
    if [ -s "$f" ] && [ "$(stat -c%s "$f")" -ge 20000 ]; then echo "$f"; return; fi
  done
  echo "$_SL_CFL4"
}

# a built artifact below MIN bytes is a segfault stub, not a compiler -- say so and stop.
sp_require() {
  local f="$1" label="$2" min="${3:-100000}" sz
  sz=$(stat -c%s "$f" 2>/dev/null || echo 0)
  if [ "$sz" -lt "$min" ]; then
    echo "FAIL: $label is $sz bytes (< $min) -- a segfault stub, not a usable artifact." >&2
    echo "      Most often the rzir outgrew the backend's input buffer; \$CFCAP=$(sp_cf)" >&2
    return 1
  fi
  return 0
}

sp_chain_rs() { "$SP/B.ld" | "$SP/B.opt" | "$SP/B.ra" | "$(sp_cf)" > "$1"; chmod +x "$1"; }
sp_chain_c()  { "$_SL_CC/link/rzir-ld.bin" | "$_SL_CC/opt/rzir-opt.bin" | "$_SL_CC/ra/rzir-ra.bin" | "$(sp_cf)" > "$1"; chmod +x "$1"; }

# COLD START: mint $SP/B.cf.m0 (M0 cf, from the hand-hex) and $SP/L3.rzrs (C-seed frontend). Idempotent.
sp_seed() {
  mkdir -p "$SP"
  if [ ! -s "$SP/B.cf.m0" ]; then
    "${TOWER_M0:-$_SL_MC/m0/m0.bin}" < "$_SL_MC/emitter/rzir-to-elf-cf.m0" > "$SP/B.cf.m0" 2>/dev/null
    chmod +x "$SP/B.cf.m0"
    [ -s "$SP/B.cf.m0" ] || { echo "FAIL: could not assemble B.cf.m0 from the hand-hex" >&2; return 1; }
    echo "  seeded \$SP/B.cf.m0 ($(stat -c%s "$SP/B.cf.m0") B) -- M0 cf, assembled from the hand-hex"
  fi
  sp_name B.cf.m0 cf.bin
  if [ ! -s "$SP/L3.rzrs" ]; then
    local sd="$_SL_A/backend-toolchain/rzrs-seed"
    local tcc="$_SL_A/tcc" rzrsc="$_SL_A/L3/rzrs.c"
    # L3.rzcpp and L3.rzrs both lower through the FROZEN C stages + M0 cf: small images, under the cap.
    "$_SL_CC/ccc.bin" < "$tcc/rzcpp.c" 2>/dev/null \
      | "$_SL_CC/link/rzir-ld.bin" | "$_SL_CC/opt/rzir-opt.bin" | "$_SL_CC/ra/rzir-ra.bin" | "$SP/B.cf.m0" > "$SP/L3.rzcpp" 2>/dev/null
    chmod +x "$SP/L3.rzcpp"
    "$SP/L3.rzcpp" -I"$sd/headers" < "$rzrsc" > "$SP/L3.rzrs.pp.c" 2>/dev/null
    cat "$sd/shim.c" "$SP/L3.rzrs.pp.c" > "$SP/L3.rzrs.in.c"
    "$_SL_CC/ccc.bin" < "$SP/L3.rzrs.in.c" 2>/dev/null \
      | "$_SL_CC/link/rzir-ld.bin" | "$_SL_CC/opt/rzir-opt.bin" | "$_SL_CC/ra/rzir-ra.bin" | "$SP/B.cf.m0" > "$SP/L3.rzrs" 2>/dev/null
    chmod +x "$SP/L3.rzrs"
    sp_require "$SP/L3.rzrs" "L3.rzrs (C-seed frontend)" 100000 || return 1
    echo "  seeded \$SP/L3.rzrs ($(stat -c%s "$SP/L3.rzrs") B) -- built from rzrs.c by the hex-rooted ccc"
  fi
  sp_name L3.rzcpp rzcpp; sp_name L3.rzrs rzrs.rsz
  # CAPACITY BACKEND: L5's self-image is larger than M0 cf can take, so its first lowering needs cf.hoist (cf.rs,
  # 16 MB buffers) -- which L5 itself normally mints. Break that cycle from below: L3 compiles L4, L4 compiles
  # cf.rs, and the frozen C stages + M0 cf lower both (small images, under the cap). Hex-rooted, no committed blob.
  if [ ! -s "$SP/B.cf.hoist.l4" ]; then
    "$SP/L3.rzrs" < "$_SL_A/L4/L4.rzrs.rs" 2>/dev/null \
      | "$_SL_CC/link/rzir-ld.bin" | "$_SL_CC/opt/rzir-opt.bin" | "$_SL_CC/ra/rzir-ra.bin" | "$SP/B.cf.m0" > "$SP/L4.rzrs.l3" 2>/dev/null
    chmod +x "$SP/L4.rzrs.l3"
    sp_require "$SP/L4.rzrs.l3" "L4.rzrs.l3 (L4 built by L3)" 100000 || return 1
    "$SP/L4.rzrs.l3" < "$_SL_A/backend-cf/cf.rs" 2>/dev/null \
      | "$_SL_CC/link/rzir-ld.bin" | "$_SL_CC/opt/rzir-opt.bin" | "$_SL_CC/ra/rzir-ra.bin" | "$SP/B.cf.m0" > "$SP/B.cf.hoist.l4" 2>/dev/null
    chmod +x "$SP/B.cf.hoist.l4"
    sp_require "$SP/B.cf.hoist.l4" "B.cf.hoist.l4 (cf.rs built by L4)" 20000 || return 1
    echo "  seeded \$SP/B.cf.hoist.l4 ($(stat -c%s "$SP/B.cf.hoist.l4") B) -- the capacity backend, cf.rs compiled by the hex-rooted L4"
  fi
  CFCAP="$(sp_cf)"; export CFCAP
  return 0
}
