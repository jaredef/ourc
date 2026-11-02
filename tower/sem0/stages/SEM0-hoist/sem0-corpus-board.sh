#!/usr/bin/env bash
# The MINT-READINESS number: what a hoist would actually cost.
#
# `sem0-hoist-readiness.sh` asks whether SEM0 reproduces s1_parser's front-end behaviour (8/8 today).
# This asks the other question, and it is the one that decides a hoist: **is SEM0 a good enough general Rust
# compiler to be the tier that compiles the NEXT dev target?** Because that is what a hoist makes it.
#
# It scores the 904-probe r2 corpus against SEM0 (gen1, the L6-built artefact a hoist would freeze) and against
# L6 (the tier a hoist retires), and prints both. No gate enforces this today -- which is why it is a script:
# the number was measured once on 2026-08-26 (164), updated to 257, and then went three weeks unwatched while
# ~30 rungs moved it. That is the `northstar-rung` failure shape: the gates measured the retiring tier.
#
#   SP=<scratch with L6.rzrs + cf.hoist> bash sem0-corpus-board.sh
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../../.." && pwd)"
CC="${TOWER_L2:-$ROOT/tower/L2}"
R2="$ROOT/tower/corpus/r2-corpus"
SRC="$HERE/../S1-lex-parse-ast/s1_parser.rs"
: "${SP:?set SP (scratch with L6.rzrs + cf.hoist)}"

W="$(mktemp -d)"; trap 'rm -rf "$W"' EXIT
cp "$SP/cf.hoist" "$W/cf.full"; cp "$SP/cf.hoist" "$W/cf.hoist"; cp "$SP/L6.rzrs" "$W/rzrs.self"

# gen1: the L6-built SEM0. This is the artefact a hoist freezes, so it is the one that must be scored --
# NOT the rustc-built parser, which is only ever an oracle here.
timeout 1800 "$SP/L6.rzrs" < "$SRC" > "$W/sem0.rzir" 2>/dev/null
"$CC/link/rzir-ld.bin" < "$W/sem0.rzir" | "$CC/opt/rzir-opt.bin" | "$CC/ra/rzir-ra.bin" | "$W/cf.full" > "$W/sem0.bin" 2>/dev/null
chmod +x "$W/sem0.bin"
[ -s "$W/sem0.bin" ] || { echo "  FAIL: gen1 did not mint"; exit 1; }

# HAZARDS, each of which produced a confidently wrong number on 2026-08-26 (see r2-corpus/README-sem0-board.md):
# stdout must be redirected to a file; $SP needs cf.full not just cf.hoist; never run two sweeps concurrently;
# always grep -a, because a probe's own stdout can contain a NUL and silently turn the board binary.
ext(){ awk '$2=="OK"||$2=="MISCOMP"||$2=="GAP"||$2=="HANG"||$2=="UNSOUND"||$2=="REJECT-OK"{print $1, $2}' "$1" | sort -u; }

SP="$W" RZ_CC="$W/sem0.bin" RZ_MARK='//@S9' bash "$R2/map.sh" > "$W/sem0.raw" 2>/dev/null
ext "$W/sem0.raw" > "$W/sem0.txt"
[ -s "$W/sem0.raw" ] || { echo "  FAIL: the sweep produced no output at all"; exit 1; }
noelf=$(grep -ac NOELF "$W/sem0.raw"); n=$(wc -l < "$W/sem0.txt")
[ "$noelf" -eq 0 ] || { echo "  FAIL: $noelf NOELF rows -- the backend was missing, the number is meaningless"; exit 1; }
[ "$n" -ge 100 ] || { echo "  FAIL: only $n probes scored"; exit 1; }

sem0=$(awk '$2=="OK"' "$W/sem0.txt" | wc -l)
l6=$(awk '$2=="OK"' <(ext "$R2/baseline.txt") | wc -l)
base=$(awk '$2=="OK"' <(ext "$R2/baseline-sem0.txt") | wc -l)
gap=$((l6 - sem0))

echo "  mint-readiness board: SEM0 $sem0 OK / $n   (L6 $l6, recorded SEM0 baseline $base)"
echo "  gap to the retiring tier: $gap probes are OK under L6 and not under SEM0"
regr=$(join <(ext "$R2/baseline-sem0.txt") "$W/sem0.txt" -o 0,1.2,2.2 2>/dev/null | awk '$2=="OK" && $3!="OK"{print "    " $1 ": OK -> " $3}')
[ -z "$regr" ] || { echo "  regressions vs the recorded SEM0 baseline:"; echo "$regr"; }
if [ "$gap" -eq 0 ]; then echo "== MINT-READY: SEM0 matches the tier it would retire =="; else
  echo "== NOT MINT-READY: hoisting would trade a $l6/$n compiler for a $sem0/$n one =="; fi
