#!/usr/bin/env bash
# ARENA HEADROOM ASSERTION -- proves the self-emit still succeeds with a DELIBERATELY REDUCED arena.
#
# WHY THIS AND NOT AN RSS THRESHOLD. The obvious instrument -- assert `peak RSS / gdef __heap < X%` -- is
# INVALID on this substrate, measured 2026-08-26: RSS scales WITH the declared arena, because the bump allocator
# touches pages proportionally and RSS also counts the binary, the 16 MB str arena, obuf and stack. At a 1280 MB
# arena the self-emit peaked at 1,314,804 kB -- RSS EXCEEDED the arena -- and still completed correctly with
# g2 == g3. So "RSS approaching the arena" predicts nothing, and "RSS over the arena" is not even a failure.
#
# What IS a valid measurement is the one that found the number: shrink the arena and see whether the self-emit
# still works. This asserts a FLOOR on the margin rather than estimating occupancy.
#
# The failure mode is already CAUGHT elsewhere (arena exhaustion truncates the self-image and breaks the g2==g3
# fixed point, which selfhost and s9-wholefile both check). What was missing is EARLY WARNING -- how much room
# is left before that happens. This provides it, and the constraint that went stale
# ("98.6% full, ~150 MB left") went stale precisely because nothing re-measured it.
#
#   SP=<scratch> CC=<cc-in-c> [MARGIN=0.75] bash arena-headroom.sh
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
SP="${SP:?set SP}"; CC="${CC:?set CC}"
P="$HERE/stages/S1-lex-parse-ast/s1_parser.rs"
MARGIN="${MARGIN:-0.75}"        # require the self-emit to survive at this FRACTION of the declared arena
W="$(mktemp -d)"; trap 'rm -rf "$W"' EXIT
back(){ "$SP/ld.bin" 2>/dev/null | "$CC/opt/rzir-opt.bin" 2>/dev/null | "$CC/ra/rzir-ra.bin" 2>/dev/null | "$SP/cf.hoist"; }

# REPOINTED 2026-08-28 (locales/sem0-closed-world). The two tiers below have DIFFERENT obligations and had
# been sharing one builder and one reference image, both rooted in L6. Since the SEM0 hoist, L6 no longer
# compiles the dev target -- `$SP/sem0.cc` does -- so tier 1 was measuring the RETIRED tier, which is the
# `northstar-rung` shape this arc has already paid for once. Tier 1 now builds the dev target with the hoisted
# tier; tier 2 keeps L6 and compiles what L6 is actually obliged to reproduce, SEM0.FROZEN.rs.
FROZEN="$HERE/stages/SEM0-hoist/hops/h00.rs"   # SEM0.FROZEN.rs before 2026-11-01
[ -f "$FROZEN" ] || { echo "  FAIL: no SEM0.FROZEN.rs -- tier 2 has nothing L6 is obliged to reproduce"; exit 1; }
{ echo '//@S9'; cat "$P"; } > "$W/s9ref.rs"
[ -x "$SP/sem0.cc" ] || { echo "  FAIL: no \$SP/sem0.cc -- the hoisted tier that builds the dev target"; exit 1; }
timeout 1800 "$SP/sem0.cc" < "$P" 2>/dev/null | back > "$W/refg1" 2>/dev/null; chmod +x "$W/refg1"
[ "$(stat -c%s "$W/refg1")" -ge 100000 ] || { echo "  FAIL: reference gen1 is a stub"; exit 1; }
timeout 1800 "$W/refg1" < "$W/s9ref.rs" > "$W/ref.rzir" 2>/dev/null
[ -s "$W/ref.rzir" ] || { echo "  FAIL: reference self-image is empty"; exit 1; }

DECL="$(grep -oE 'gdef __heap [0-9]+' "$P" | head -1 | grep -oE '[0-9]+')"
[ -n "$DECL" ] || { echo "  FAIL: no 'gdef __heap' in s1_parser.rs"; exit 1; }
CAP=8589934592   # 8GB fbase cap (b1: rbx is 64-bit for fbase>=4GB via conditional movabs) (cf.rs:241, 2026-09-09 keeper raise from 3.5GB; verified-safe <3968MB, HW limit ~4080MB). __heap now 3780MB.
if [ "$DECL" -gt "$CAP" ]; then
  echo "  RED  ARENA: declared __heap $DECL exceeds the C-EAS-FBASE-CAP ceiling $CAP"; exit 1
fi
RED="$(python3 -c "print(int($DECL * $MARGIN))")"
echo "  arena declared $((DECL/1048576)) MB; probing at $((RED/1048576)) MB (${MARGIN} of declared)"

sed "s/gdef __heap $DECL/gdef __heap $RED/" "$P" > "$W/red.rs"
grep -q "gdef __heap $RED" "$W/red.rs" || { echo "  FAIL: could not rewrite the gdef"; exit 1; }

timeout 1800 "$SP/sem0.cc" < "$W/red.rs" 2>/dev/null | back > "$W/g1" 2>/dev/null; chmod +x "$W/g1"
[ "$(stat -c%s "$W/g1")" -ge 100000 ] || { echo "  FAIL: reduced-arena gen1 is a stub"; exit 1; }
{ echo '//@S9'; cat "$W/red.rs"; } > "$W/s9.rs"
timeout 1800 "$W/g1" < "$W/s9.rs" > "$W/g2.rzir" 2>/dev/null
back < "$W/g2.rzir" > "$W/g2" 2>/dev/null; chmod +x "$W/g2"
[ "$(stat -c%s "$W/g2")" -ge 100000 ] || { echo "  RED  ARENA: reduced-arena gen2 is a stub -- margin below ${MARGIN}"; exit 1; }

timeout 1800 "$W/g2" < "$W/s9.rs" > "$W/g3.rzir" 2>/dev/null; r=$?
if [ "$r" -ge 128 ]; then
  echo "  RED  ARENA: self-emit SIGSEGVs at $((RED/1048576)) MB -- headroom is under ${MARGIN} of the declared arena"; exit 1
fi
if ! cmp -s "$W/g2.rzir" "$W/g3.rzir"; then
  echo "  RED  ARENA: g2 != g3 at $((RED/1048576)) MB -- the fixed point breaks before the declared arena does"; exit 1
fi
echo "  ok   ARENA/SEM0: self-emit holds g2==g3 at $((RED/1048576)) MB ($(wc -l < "$W/g3.rzir") lines) -- at least $(( (DECL-RED)/1048576 )) MB of margin"

# ---- TIER 2: the L6-EMITTED arena, which governs gen1 -- a DIFFERENT number on a DIFFERENT tier.
# Asserting only the SEM0 tier left this one unwatched, and the two are nothing alike (measured 2026-08-26):
#   SEM0-emitted arena (1792 MB declared): floor between 511 and 767 MB  -> over 1 GB of margin.
#   L6-emitted   arena (1920 MB declared): floor between 1792 and 1856 MB -> 64-128 MB of margin.
# That asymmetry was CLOSED 2026-08-26 by fixing L6's String initial capacity (4096 -> 64 bytes per String;
# every format! was allocating a 4 KB buffer from a no-free bump arena for a few bytes of text). gen1's self-emit
# went 1,902,336 -> 897,280 kB peak RSS with a byte-identical self-image, and this tier now tolerates a 1024 MB
# shave where it previously SIGSEGVd at a 128 MB one. Both tiers are therefore probed by the SAME fraction now.
# Before that fix this tier could only be asserted at an absolute 64 MB, which is what a 3-7% margin allows.
L6SRC="$HERE/../L6/L6.rzrs.rs"
LDECL="$(grep -oE 'gdef __heap [0-9]+' "$L6SRC" | head -1 | grep -oE '[0-9]+')"
[ -n "$LDECL" ] || { echo "  FAIL: no 'gdef __heap' in L6.rzrs.rs"; exit 1; }
LRED="$(python3 -c "print(int($LDECL * $MARGIN))")"   # same fraction as the SEM0 tier
[ -n "${LMARGIN:-}" ] && LRED=$(( LDECL - LMARGIN ))   # optional absolute-shave override, for probing
[ -x "$SP/rzrs.self" ] || { echo "  FAIL: no \$SP/rzrs.self (the L5 that builds L6)"; exit 1; }
sed "s/gdef __heap $LDECL/gdef __heap $LRED/" "$L6SRC" > "$W/l6.rs"
grep -q "gdef __heap $LRED" "$W/l6.rs" || { echo "  FAIL: could not rewrite L6's gdef"; exit 1; }
# Tier 2's reference is built HERE, from L6 at its full arena compiling SEM0.FROZEN.rs -- it can no longer
# borrow tier 1's, which is now a SEM0-built image of a different source.
{ echo '//@S9'; cat "$FROZEN"; } > "$W/s9frozen.rs"
timeout 1800 "$SP/L6.rzrs" < "$FROZEN" 2>/dev/null | back > "$W/lref1" 2>/dev/null; chmod +x "$W/lref1"
[ "$(stat -c%s "$W/lref1")" -ge 100000 ] || { echo "  FAIL: L6 reference gen1 (from SEM0.FROZEN.rs) is a stub"; exit 1; }
timeout 1800 "$W/lref1" < "$W/s9frozen.rs" > "$W/lref.rzir" 2>/dev/null
[ -s "$W/lref.rzir" ] || { echo "  FAIL: L6 reference self-image is empty"; exit 1; }
timeout 1800 "$SP/rzrs.self" < "$W/l6.rs" 2>/dev/null | back > "$W/l6" 2>/dev/null; chmod +x "$W/l6"
[ "$(stat -c%s "$W/l6")" -ge 100000 ] || { echo "  RED  ARENA/L6: reduced-arena L6 is a stub"; exit 1; }
timeout 1800 "$W/l6" < "$FROZEN" 2>/dev/null | back > "$W/lg1" 2>/dev/null; chmod +x "$W/lg1"
[ "$(stat -c%s "$W/lg1")" -ge 100000 ] || { echo "  RED  ARENA/L6: gen1 built by the reduced L6 is a stub"; exit 1; }
timeout 1800 "$W/lg1" < "$W/s9frozen.rs" > "$W/lg1.rzir" 2>/dev/null; lr=$?
if [ "$lr" -ge 128 ]; then
  echo "  RED  ARENA/L6: gen1 self-emit SIGSEGVs at $((LRED/1048576)) MB -- headroom lost"; exit 1
fi
if ! cmp -s "$W/lref.rzir" "$W/lg1.rzir"; then
  echo "  RED  ARENA/L6: self-image differs at $((LRED/1048576)) MB -- silent truncation"; exit 1
fi
echo "  ok   ARENA/L6: gen1 self-emit identical at $((LRED/1048576)) MB of L6's $((LDECL/1048576)) MB arena -- at least $(( (LDECL-LRED)/1048576 )) MB of margin"

