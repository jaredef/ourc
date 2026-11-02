#!/usr/bin/env bash
# oracle-build.sh <crate> -- the ORACLE verdict for a rustc compiler crate, via the (B) FULL-BUILD oracle:
# `x.py check compiler/<crate>` inside the real rustc-1.95.0 workspace (exact dep versions, feature resolution,
# build-env, and stage0 that rustc itself uses; deps cached so a targeted check is ~seconds). This replaces the
# standalone dep-reconstruction (which could not reproduce rustc's exact build for the deep crates).
#   exit 0 = ACCEPT (rustc accepts the crate)   1 = REJECT (diagnostics on stdout)   2 = ORACLE-ABSENT/no tree
# ORACLE-class, MEASUREMENT-ONLY, never in the produced chain. The workspace + stage0 + build artifacts live under
# $ORACLE_SRC (a /tmp cache), like the source cache; oracle-vendor-setup.sh provisions it.
set -uo pipefail
CR="${1:?usage: oracle-build.sh <crate>}"
ORACLE_SRC="${ORACLE_SRC:-/tmp/rustc-src/rustc-1.95.0-src}"
[ -d "$ORACLE_SRC/compiler/$CR" ] || { echo "ORACLE-ABSENT: no compiler/$CR under $ORACLE_SRC"; exit 2; }
[ -f "$ORACLE_SRC/x.py" ] || { echo "ORACLE-ABSENT: no x.py (run oracle-vendor-setup.sh)"; exit 2; }
W="$(mktemp -d)"; trap 'rm -rf "$W"' EXIT
( cd "$ORACLE_SRC" && python3 x.py check "compiler/$CR" ) > "$W/diag" 2>&1
rc=$?
if [ "$rc" -ne 0 ]; then grep -E '^error' "$W/diag" | head -8; fi
exit "$rc"
