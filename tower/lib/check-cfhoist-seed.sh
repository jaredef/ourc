#!/usr/bin/env bash
# check-cfhoist-seed.sh -- prove the committed L5/cf.hoist.seed does not leak into the produced compiler.
#
#   SP=<scratch, right after L5/selfhost.sh> bash tower/lib/check-cfhoist-seed.sh
#
# cf.hoist.seed is the one committed input the from-hex build cannot derive: L5's self-image overflows the
# M0 cf input ceiling, and cf.hoist is itself minted by L5. selfhost.sh lowers L5 ($SP/L5.rzrs.g1.rzir) with the seed
# into $SP/L5.rzrs.g1, then has L5.rzrs.g1 mint cf.hoist from backend-cf/cf.rs into $SP/B.cf.hoist.g1. This gate
# re-lowers the SAME L5 rzir with that source-minted cf.hoist and requires a byte-identical L5.rzrs.g1: the
# seed then only broke the chicken-and-egg and contributed nothing to the compiler. (Byte-equality of the
# seed itself is not expected -- it predates later cf.rs edits; provenance.sh anchors it behaviorally.)
set -uo pipefail
SP="${SP:?set SP}"
for f in L5.rzrs.g1.rzir L5.rzrs.g1 B.cf.hoist.g1 B.ld B.opt B.ra; do
  [ -s "$SP/$f" ] || { echo "FAIL: \$SP/$f missing -- run L5/selfhost.sh first"; exit 1; }
done
"$SP/B.ld" < "$SP/L5.rzrs.g1.rzir" | "$SP/B.opt" | "$SP/B.ra" | "$SP/B.cf.hoist.g1" > "$SP/L5.rzrs.washed" 2>/dev/null
if cmp -s "$SP/L5.rzrs.g1" "$SP/L5.rzrs.washed"; then
  echo "ok: L5.rzrs.g1 lowered by cf.hoist.seed == lowered by source-minted cf.hoist ($(stat -c%s "$SP/L5.rzrs.g1") B) -- seed washes out"
else
  echo "FAIL: L5.rzrs.g1 differs when lowered by the source-minted cf.hoist -- the committed seed leaks into the compiler"
  exit 1
fi
