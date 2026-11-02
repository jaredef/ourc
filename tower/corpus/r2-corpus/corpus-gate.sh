#!/usr/bin/env bash
# r2-corpus ZERO-REGRESSION gate.
#
# WHY THIS EXISTS. The project states its parity discipline as "zero regression per rung", and the r2-corpus
# differential is named as the oracle for it -- but nothing enforced it. The corpus was run by hand, when someone
# remembered. On 2026-08-25 the i64-statics promotion passed ALL 15 standing gates while carrying a live
# miscompile class, because no gate runs the corpus. Prose is not a discipline; this file is.
#
# TIER MIGRATED TO SEM0 (2026-09-25, keeper decision). The gate now scores the SEM0 tier (the dev target /
# telos: tower/sem0/.../s1_parser.rs), not the retiring L6 transpiler. Rationale: the meta-telos is SEM0
# compiling rustc; L6 is being retired. The two tiers have DISJOINT bug sets (measured 2026-09-25: L6 had 9
# MISCOMPs SEM0 has all fixed -- e.g. the presult i64-param-sign class; SEM0 has 13 MISCOMPs L6 handles -- e.g.
# the whilelet/desugar Box/deref class). Gating on SEM0 measures the tier we are actually advancing.
#
# THE SEM0 RECIPE (why the env below is what it is; see GATE-TIER-FINDING-2026-09-25.md):
#   compiler  = $SP/sem0.frozen  with the  //@S9  marker (the executable-rzir path)
#   linker    = $SP/ld.bin  (the i64 ld TWIN). The frozen C ld clamps heaptop at INT32_MAX and CANNOT link
#               SEM0's >2GB arena output (gdef __heap 4199481344) -> it would SEGV every elf (mass false MISCOMP,
#               which is exactly how the now-INVALID baseline-sem0.txt was produced). MUST be the i64 twin.
#   opt/ra    = the frozen C twins (they handle SEM0 output fine); cf = cf.hoist (capacity-capable).
#
# DETERMINISM. Run on a non-full disk with no concurrent tower build (clean orphan /tmp/tmp.* scratch first).
# A not-OK regression here is real, or the disk is full. Do NOT rebaseline lower to "absorb" a flake.
#
# The gate compares the live board against baseline.txt and FAILS on any probe that regresses from OK. It does
# NOT fail on the recorded MISCOMP debt, and it REPORTS improvements so closing the meet shows up as a number.
#
#   SP=<scratch with sem0.frozen + ld.bin + cf.hoist> bash corpus-gate.sh
#   REBASELINE=1 SP=... bash corpus-gate.sh    # rewrite baseline.txt (use ONLY when improvements are intended)
#
# The pre-2026-09-25 L6 board is preserved as baseline-l6.txt (RZ_MARK='' RZ_CC=$SP/L6.rzrs, C ld) for reference.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
SP="${SP:?set SP (scratch with sem0.frozen + ld.bin + cf.hoist)}"
BASE="$HERE/baseline.txt"
[ -f "$BASE" ] || { echo "  FAIL: missing $BASE"; exit 1; }
[ -x "$SP/sem0.frozen" ] || { echo "  FAIL: no $SP/sem0.frozen -- run sem0-stage.sh first"; exit 1; }
[ -x "$SP/ld.bin" ]      || { echo "  FAIL: no $SP/ld.bin (i64 ld twin) -- run build-live-backend.sh first"; exit 1; }
[ -x "$SP/cf.hoist" ]    || { echo "  FAIL: no $SP/cf.hoist"; exit 1; }

# Absolute paths to the real artifacts (map.sh reassigns SP to a temp scratch below).
SEM0_BIN="$SP/sem0.frozen"; LD_I64="$SP/ld.bin"

# map.sh wants $SP/cf.full; point it at the capacity-capable cf without disturbing the shared $SP.
W="$(mktemp -d)"; trap 'rm -rf "$W"' EXIT
cp "$SP/cf.hoist" "$W/cf.full"
cp "$SP/cf.hoist" "$W/cf.hoist"

# BACKEND = LLVM (the meta-telos target backend, decision 2026-09-27). cf here is used ONLY to self-host the
# rzir2ll translator (map.sh builds it from source and assembles with $W/cf.full); RUN correctness of the TARGET
# is judged on rzir -> rzir2ll -> llc -> cc. A prebuilt $SP/r2l.elf is reused if present (skips the per-run build).
LLC_BIN="$(command -v llc 2>/dev/null || ls ~/.rustup/toolchains/*/lib/rustlib/*/bin/llc 2>/dev/null | head -1)"
CC_BIN="$(command -v cc 2>/dev/null || command -v gcc 2>/dev/null)"
if [ -z "$LLC_BIN" ] || [ -z "$CC_BIN" ]; then
  echo "  SKIP: r2-corpus RUN gate needs llc+cc (the LLVM target backend); not found on this host."; exit 0
fi
[ -x "$SP/r2l.elf" ] && cp "$SP/r2l.elf" "$W/r2l.elf"

SP="$W" RZ_CC="$SEM0_BIN" RZ_MARK='//@S9' RZ_LD="$LD_I64" RZ_TIMEOUT=30 \
  RZ_BACKEND=llvm LLC_BIN="$LLC_BIN" CC_BIN="$CC_BIN" bash "$HERE/map.sh" 2>/dev/null \
  | awk '$2=="OK"||$2=="MISCOMP"||$2=="GAP"||$2=="HANG"||$2=="UNSOUND"||$2=="REJECT-OK"{print $1, $2}' \
  | sort -u > "$W/live.txt"

n=$(wc -l < "$W/live.txt")
[ "$n" -ge 100 ] || { echo "  FAIL: only $n probes scored -- the harness did not run (check sem0.frozen/ld.bin/cf)"; exit 1; }

if [ "${REBASELINE:-0}" = 1 ]; then cp "$W/live.txt" "$BASE"; echo "  REBASELINED $BASE ($n probes)"; fi

# regressions: was OK in the baseline, is not OK now. improvements: the converse.
regr=$(join "$BASE" "$W/live.txt" -o 0,1.2,2.2 2>/dev/null | awk '$2=="OK" && $3!="OK"{print "    " $1 ": OK -> " $3}')
impr=$(join "$BASE" "$W/live.txt" -o 0,1.2,2.2 2>/dev/null | awk '$2!="OK" && $3=="OK"{print "    " $1 ": " $2 " -> OK"}')
# a probe present in one board but not the other is itself a finding (probes added/removed without rebaselining)
only=$(join -v1 "$BASE" "$W/live.txt" 2>/dev/null | awk '{print "    " $1 " (in baseline, not scored now)"}')
new=$(join -v2 "$BASE" "$W/live.txt" 2>/dev/null | awk '{print "    " $1 " (scored now, not in baseline)"}')

ok=$(awk '$2=="OK"' "$W/live.txt" | wc -l); bad=$((n - ok))
echo "  board: $ok OK / $bad not-OK of $n probes (baseline: $(awk '$2=="OK"' "$BASE" | wc -l) OK) [tier: SEM0]"
[ -n "$impr" ] && { echo "  IMPROVED:"; echo "$impr"; }
[ -n "$new"  ] && { echo "  UNTRACKED probes (rebaseline to adopt):"; echo "$new"; }
[ -n "$only" ] && { echo "  MISSING probes:"; echo "$only"; }
if [ -n "$regr" ]; then
  echo "  RED  REGRESSION -- these were OK at baseline:"; echo "$regr"
  echo "       Zero-regression is the standing parity discipline. Fix, or rebaseline deliberately."
  exit 1
fi
echo "== r2-corpus zero-regression gate: GREEN =="
