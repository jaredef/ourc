#!/usr/bin/env bash
# derive-seeds.sh -- mint every committed dev-loop seed binary from the hand-hex, into $SP/seeds/.
#
#   SP=/tmp/sp bash tower/lib/derive-seeds.sh
#
# The default (from-hex) mode of build-tower.sh runs this first, then points every stage at $SP/seeds/ via
# TOWER_L2 / TOWER_M0 instead of the committed blobs. `build-tower.sh --dev` skips it and uses the blobs.
#
# Base: the two self-hosting hand-audited seeds hex0.bin (rebuilt here from hex0.hand.hex) and hex1.bin
# (rebuilt here from derived-hex1.labeled.hex) -- the trust base chain-proof.sh names. Everything above them
# is derived below, with the same recipe as provenance.sh steps 0-1:
#   hex1 -> hex2 -> m0 -> { cf (M0 backend), M0 cc } -> ld0/opt0/ra0 -> ccc -> L2 ld/opt/ra (via ccc)
#
# The one input NOT derived is L5/cf.hoist.seed: L5's self-image exceeds the M0 cf input ceiling (~5.7MB),
# and cf.hoist is itself minted by L5, so the seed breaks that cycle. build-tower.sh asserts that the
# from-hex chain re-mints it byte-for-byte (selfhost's $SP/cf.hoist == the seed), so it is checked every run.
#
# Each derived seed is compared to its committed blob; drift is reported (not fatal -- the derived one is used).
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; A="$(cd "$HERE/.." && pwd)"
SP="${SP:?set SP (scratch dir)}"
L0="$A/L0"; L2="$A/L2"; S="$SP/seeds"; W="$S/.work"
rm -rf "$S"; mkdir -p "$W" "$S/L0/m0"
die(){ echo "FAIL: $1" >&2; exit 1; }
big(){ [ -s "$1" ] && [ "$(stat -c%s "$1")" -ge "$2" ] || die "$3 is $(stat -c%s "$1" 2>/dev/null || echo 0) B (< $2) -- a segfault stub"; chmod +x "$1"; }
drift=0
same(){ # same DERIVED COMMITTED LABEL
  if cmp -s "$1" "$2"; then echo "  $3: derived == committed"
  else echo "  $3: DRIFT -- derived ($(stat -c%s "$1") B) != committed $(realpath --relative-to="$A/.." "$2") ($(stat -c%s "$2") B); using derived"; drift=$((drift+1)); fi
}

echo "== derive seeds from the hand-hex -> $S =="
# hex ladder: each rung is built by the one below (hex0/hex1 additionally self-host from their sources)
"$L0/hex0/hex0.bin" < "$L0/hex0/hex0.hand.hex" > "$W/hex0"; big "$W/hex0" 200 hex0
cmp -s "$W/hex0" "$L0/hex0/hex0.bin" || die "hex0.bin does not rebuild itself from hex0.hand.hex"
"$L0/hex1/hex1.bin" < "$L0/hex1/derived-hex1.labeled.hex" > "$W/hex1"; big "$W/hex1" 300 hex1
cmp -s "$W/hex1" "$L0/hex1/hex1.bin" || die "hex1.bin does not rebuild itself from derived-hex1.labeled.hex"
"$W/hex1" < "$L0/hex2/derived-hex2.labeled.hex" > "$W/hex2"; big "$W/hex2" 300 hex2
"$W/hex2" < "$L0/m0/m0.labeled.hex" > "$S/L0/m0/m0.bin"; big "$S/L0/m0/m0.bin" 500 m0
echo "  hex0 + hex1 (self-hosting base) -> hex2 -> m0"
M0="$S/L0/m0/m0.bin"
( cd "$L0/emitter" && "$M0" < rzir-to-elf-cf.m0 > "$S/cf.bin" 2>/dev/null ); big "$S/cf.bin" 10000 "cf (M0 backend)"
"$M0" < "$A/L1/c-frontend/c-to-rzir.m0" > "$W/m0cc" 2>/dev/null; big "$W/m0cc" 10000 "M0 cc"
echo "  m0 -> cf.bin (M0 backend), M0 cc"
# C backend stages via M0 cc | cf, then ccc through that chain (provenance step 0b3/0c)
for st in link/rzir-ld opt/rzir-opt ra/rzir-ra; do
  n="$(basename "$st")"; "$W/m0cc" < "$L2/$st.c" 2>/dev/null | "$S/cf.bin" > "$W/$n.0" 2>/dev/null; big "$W/$n.0" 10000 "$n (M0 cc)"
done
chain0(){ "$W/rzir-ld.0" | "$W/rzir-opt.0" | "$W/rzir-ra.0" | "$S/cf.bin"; }
"$W/m0cc" < "$L2/ccc.c" 2>/dev/null | chain0 > "$W/ccc.0" 2>/dev/null; big "$W/ccc.0" 100000 "ccc (gen1, M0 cc)"
# the committed ccc.bin is the ccc SELF-HOST (gen2: ccc compiling ccc.c), so mint that generation
"$W/ccc.0" < "$L2/ccc.c" 2>/dev/null | chain0 > "$W/ccc.1" 2>/dev/null; big "$W/ccc.1" 100000 "ccc (gen2, self-host)"
echo "  M0 cc -> ld0/opt0/ra0 -> ccc (gen1) -> ccc (gen2, self-host)"
# lay out a mirror of tower/L2: sources symlinked, the four binaries replaced by the derived ones
cp -rs "$L2" "$S/L2"
for b in ccc.bin link/rzir-ld.bin opt/rzir-opt.bin ra/rzir-ra.bin; do rm -f "$S/L2/$b"; done
cp "$W/ccc.1" "$S/L2/ccc.bin"
for st in link/rzir-ld opt/rzir-opt ra/rzir-ra; do
  "$W/ccc.0" < "$L2/$st.c" 2>/dev/null | chain0 > "$S/L2/$st.bin" 2>/dev/null; big "$S/L2/$st.bin" 4000 "$st (ccc)"
done
echo "  ccc (gen1) -> L2 ld/opt/ra   (same recipe as provenance step 1)"
echo "== coherence vs committed blobs =="
same "$M0" "$L0/m0/m0.bin" "m0.bin"
same "$S/L2/ccc.bin" "$L2/ccc.bin" "ccc.bin"
for st in link/rzir-ld opt/rzir-opt ra/rzir-ra; do same "$S/L2/$st.bin" "$L2/$st.bin" "$(basename "$st").bin"; done
echo "  cf.hoist.seed: not derivable from hex (capacity cycle) -- re-mint checked after selfhost"
rm -rf "$W"
echo "== seeds ready (drift: $drift) =="
