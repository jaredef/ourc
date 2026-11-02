#!/usr/bin/env bash
# check-cfhoist-ddc.sh -- diverse double compilation of the capacity backend, cf.hoist (backend-cf/cf.rs).
#
#   SP=<scratch, right after L5/selfhost.sh> bash tower/lib/check-cfhoist-ddc.sh
#
# Two builds of the same cf.rs by two different compilers: B.cf.hoist.l4 (compiled by L4, which L3 compiles from the
# hand-hex) and B.cf.hoist.g1 (compiled by L5.rzrs.g1). selfhost.sh lowered L5 ($SP/L5.rzrs.g1.rzir) with the first
# into $SP/L5.rzrs.g1. This gate re-lowers the SAME rzir with the second and requires a byte-identical L5.rzrs.g1.
set -uo pipefail
SP="${SP:?set SP}"
for f in L5.rzrs.g1.rzir L5.rzrs.g1 B.cf.hoist.l4 B.cf.hoist.g1 B.ld B.opt B.ra; do
  [ -s "$SP/$f" ] || { echo "FAIL: \$SP/$f missing -- run L5/selfhost.sh first"; exit 1; }
done
"$SP/B.ld" < "$SP/L5.rzrs.g1.rzir" | "$SP/B.opt" | "$SP/B.ra" | "$SP/B.cf.hoist.g1" > "$SP/L5.rzrs.ddc" 2>/dev/null
if cmp -s "$SP/L5.rzrs.g1" "$SP/L5.rzrs.ddc"; then
  echo "ok: L5.rzrs.g1 lowered by the L4-built cf.hoist == lowered by the L5-built cf.hoist ($(stat -c%s "$SP/L5.rzrs.g1") B)"
else
  echo "FAIL: L5.rzrs.g1 differs between the L4-built and the L5-built cf.hoist"
  exit 1
fi
