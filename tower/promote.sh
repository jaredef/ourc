#!/usr/bin/env bash
# promote.sh -- the L6 -> L5 promotion, with its naming convention MECHANIZED.
#
# WHY THIS EXISTS. Seventeen promotions were done by hand between 2026-07-31 and 2026-08-09, and produced FIVE
# incompatible backup-naming conventions (bare hash, layerN-hash, subarcNN-hash, topic-hash, and two different
# `.pre-*` file suffixes), none of which carried a date or a rung. They were hidden dot-directories, so they were
# invisible in listings, and they sorted meaninglessly -- `layer10` before `layer5`, `ast10` between `0875c9b2`
# and `b8b264ad`. The convention was prose in tower-naming-and-trust-path.md; prose does not enforce.
#
# The rule, now enforced here rather than remembered:
#
#   apparatus/arcs/L5/promotions/<YYYY-MM-DD>-L6-to-L5-<label>/
#       L5.rzrs.rs.pre                 the mainline as it stood BEFORE this promotion
#       L5.rzrs.FROZEN-AT.md.pre       the freeze log as it stood before
#
# Date first so the directory sorts chronologically; rung pair explicit so the direction is readable without
# opening it; <label> is the rung/topic name so the promotion is identifiable, with the short SHA appended for
# uniqueness (two promotions have landed on the same day more than once).
#
#   bash apparatus/promote.sh <label>            # e.g. `bash apparatus/promote.sh i64-statics`
#   DRY=1 bash apparatus/promote.sh <label>      # show what would happen, touch nothing
#
# This script does the BACKUP and the COPY and appends a freeze-log stub. It does NOT run the gates and it does
# NOT commit -- promotion is keeper-authorized ("stage the case, do not flip L5 unprompted"), so the
# operator runs `SP=<fresh> bash apparatus/build-tower.sh --gates` and records the result in the stub by hand.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
L5="$HERE/L5/L5.rzrs.rs"
L6="$HERE/L6/L6.rzrs.rs"
MARK="$HERE/L5/L5.rzrs.FROZEN-AT.md"
PROMO="$HERE/L5/promotions"
DRY="${DRY:-0}"

label="${1:-}"
unlock="${2:-}"
[ -n "$label" ] || { echo "usage: [DRY=1] bash apparatus/promote.sh <label> \"<what this unlocks>\""; exit 2; }
# --- the freeze criterion, stated rather than assumed ------------------------------------------------
# The promotion criterion is #3 AUTHORING-UNLOCK: "freeze when the coverage becomes the language you need to write
# the NEXT layer, not on self-repro+corpus alone." In practice 27 promotions (2026-07-31..08-25, ten of them on a
# single day) recorded only `g2==g3` plus a corpus number, and those corpus numbers turn out not to be comparable
# across entries -- 904 -> 744 -> 874 in three days, each claiming no regression. The i64-statics promotion moved
# no telos metric at all. See apparatus/docs/promotion-cadence-review-2026-08-25.md.
# So: say what this unlocks. It is stamped into the freeze marker and cannot be left blank.
if [ -z "$unlock" ]; then
  echo "FAIL: no authoring-unlock given."
  echo "      Promotion flips the TRUST ANCHOR and rewrites every artifact below the freeze line. State what the"
  echo "      new L5 makes authorable that the old one did not, or what telos metric this moves (whole-file reach,"
  echo "      r2-corpus OK count). If the honest answer is \"nothing yet\", that is a reason not to promote."
  echo "      usage: bash apparatus/promote.sh $label \"<what this unlocks>\""
  exit 2
fi
case "$label" in
  *[!a-z0-9-]*) echo "FAIL: label '$label' must be lowercase kebab-case -- it becomes a directory name"; exit 2;;
  stage*) echo "FAIL: label '$label' uses the banned bare word 'stage' (tower-naming-and-trust-path.md) -- use the rung"; exit 2;;
esac
for f in "$L5" "$L6" "$MARK"; do [ -f "$f" ] || { echo "FAIL: missing $f"; exit 1; }; done

# Refuse a no-op promotion: it would mint a backup directory that records nothing.
if cmp -s "$L5" "$L6"; then echo "FAIL: L6 is already byte-identical to L5 -- nothing to promote"; exit 1; fi

# --- C-S0I64-M0CF-ARENA-BUDGET: the emitted-program arena budget an L5 may NOT exceed ---------------
# An L5's `gdef __heap` / `gdef __str_arena` govern the programs it EMITS, and selfhost.sh + parity.sh lower
# those emitted programs with M0 `cf.bin` -- deliberately, so the minimal hex-rooted anchor still validates the
# small surface. M0 cf IGNORES `heaptop` and hands every program a FIXED 16 units (256MB); only cf.hoist (cf.rs)
# derives p_memsz from the arena. Measured 2026-08-25 by sweeping the gdef in a fixed p.rzir and lowering it both
# ways: cf.bin's p_memsz stayed 16 at every heap from 1MB to 1.79GB while cf.hoist's tracked 64 -> 172, and the
# emitted program ran at 209MB of heap and SIGSEGVd at 228MB.
#
# So an L5 carrying a large arena is not "slower" or "wasteful" -- every program it emits segfaults, and the
# tower goes 13/15 RED including provenance. This is structural: M0 cf is frozen hex-rooted and cannot be fixed.
# The 2026-08-25 i64-statics promotion hit exactly this and was reverted; the fix was to promote everything
# EXCEPT the arena gdefs, which are an L6-only dev-tier setting (L6 needs 1.6GB for its SEM0 self-emit; L5, whose
# only job is to compile L6, has never needed more than 1MB).
CAP=$((224 * 1024 * 1024))
heap=$(grep -o 'gdef __heap [0-9]*' "$L6" | head -1 | awk '{print $3}')
arena=$(grep -o 'gdef __str_arena [0-9]*' "$L6" | head -1 | awk '{print $3}')
tot=$(( ${heap:-0} + ${arena:-0} ))
if [ "$tot" -gt "$CAP" ]; then
  echo "FAIL: L6 emits programs with __heap+__str_arena = $((tot/1048576))MB, over M0 cf.bin's $((CAP/1048576))MB budget."
  echo "      Promoting this verbatim makes EVERY program the new L5 emits segfault (measured: 0/16 selfhost,"
  echo "      13/15 gates RED incl. provenance). M0 cf ignores heaptop and is frozen hex-rooted."
  echo "      Promote everything EXCEPT the arena gdefs -- they are an L6-only dev setting. See"
  echo "      RA-alloc-codegen/locales/stage0-i64-storage/constraints-ledger.md C-S0I64-M0CF-ARENA-BUDGET."
  exit 1
fi

sha="$(cd "$HERE/.." && git rev-parse --short HEAD 2>/dev/null || echo nogit)"
dir="$PROMO/$(date +%F)-L6-to-L5-${label}-${sha}"
[ -e "$dir" ] && { echo "FAIL: $dir already exists -- pick a distinct label"; exit 1; }

echo "  from  L6 $(wc -l < "$L6") lines, $(stat -c%s "$L6") bytes"
echo "  over  L5 $(wc -l < "$L5") lines, $(stat -c%s "$L5") bytes"
echo "  back  ${dir#"$HERE/"}"
[ "$DRY" = 1 ] && { echo "== DRY: nothing written =="; exit 0; }

mkdir -p "$dir"
cp "$L5" "$dir/L5.rzrs.rs.pre"
cp "$MARK" "$dir/L5.rzrs.FROZEN-AT.md.pre"
cp "$L6" "$L5"

# Prepend the new entry: the log reads newest-first, as the seventeen hand-written entries already do.
tmp="$(mktemp)"
{
  echo "# L5.rzrs freeze marker"
  echo "# ============================================================================"
  echo "# ${label^^} HOIST $(date +%F): L6.rzrs.rs ($(wc -l < "$L5") L, $(stat -c%s "$L5") B) frozen as the new L5."
  echo "#   Promoted from $sha by apparatus/promote.sh. Prior L5 at promotions/$(basename "$dir")/."
  echo "#   UNLOCKS: $unlock"
  echo "#   GATES: <run \`SP=\$(mktemp -d) bash apparatus/build-tower.sh --gates\` and record the result here --"
  echo "#     16/16 incl. provenance AND the r2-corpus zero-regression gate. Record whole-file reach before/after"
  echo "#     (instrument/measure-reach.sh) so the ratchet is a measurement, not a claim.>"
  echo "#   KEEPER-AUTHORIZED: <record the authorization here>"
  echo "#"
  tail -n +1 "$MARK"
} > "$tmp"
mv "$tmp" "$MARK"

echo "== promoted. NOT gated, NOT committed. =="
echo "   next: SP=\$(mktemp -d) bash apparatus/build-tower.sh --gates   # expect 15/15 incl. provenance"
echo "   then: fill the GATES/KEEPER lines at the top of ${MARK#"$HERE/"}"
