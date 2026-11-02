#!/usr/bin/env bash
# t1prime-board.sh -- the T1' RESOLVE board: for each crate, does SEM0 //@S3RESOLVE agree with the ORACLE at
# the resolve boundary? Verdicts:
#   PASS          oracle ACCEPTS the crate (resolves clean) AND SEM0 resolves clean (exit 0, RESPATH emitted)
#   FAIL          oracle accepts but SEM0 refuses (rc 101) or crashes -- the T1' frontier (SEM0 under-resolves)
#   OVERACCEPT    oracle REJECTS (resolve errors) but SEM0 resolves clean -- Doc-731 over-acceptance
#   ORACLE-ABSENT the crate needs rustc-internal features beyond bootstrap (scope-out, not a verdict)
# F1 measures the resolve OUTCOME (accept/refuse), not a proxy. F3 names RUNG (SEM0=$SP/wf.gen1, size-asserted)
# and ORACLE (stable rustc + RUSTC_BOOTSTRAP=1 + oracle-class vendor). ORACLE-class inputs never enter the chain.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
SRC="${SRC:-/tmp/rustc-src/rust-1.95.0}"; ORACLE_SRC="${ORACLE_SRC:-/tmp/rustc-src/rustc-1.95.0-src}"; export ORACLE_SRC; VEN="${VEN:-/tmp/rustc-oracle-vendor}"
SP="${SP:?set SP}"; FE="$SP/SEM0.rzrs"; [ -x "$FE" ] || FE="$SP/wf.gen1"
CRATES="${CRATES:-$(ls -d "$SRC"/compiler/*/ 2>/dev/null | xargs -n1 basename | tr "\n" " ")}"
BOARD="${BOARD:-$HERE/t1prime-board.txt}"
[ -x "$FE" ] || { echo "  ok   t1'-board SKIP (no SEM0 rung)"; exit 0; }
sz=$(stat -c%s "$FE"); [ "$sz" -ge 1000000 ] || { echo "  FAIL t1': rung is $sz B (stub)"; exit 1; }
[ -d "$SRC/compiler" ] || { echo "  ok   t1'-board SKIP (oracle source not cached)"; exit 0; }
bash "$HERE/oracle-vendor-setup.sh" >/dev/null 2>&1 || true   # best-effort ensure oracle-class vendor (network/cargo)
W="$(mktemp -d)"; trap 'rm -rf "$W"' EXIT
: > "$BOARD"
for cr in $CRATES; do
  D="$SRC/compiler/$cr/src"; [ -f "$D/lib.rs" ] || { echo "$cr NOLIB" >> "$BOARD"; continue; }
  bash "$HERE/oracle-build.sh" "$cr" >/dev/null 2>&1; orc=$?
  # Feed the SPLICED whole crate (splice-crate.py inlines every non-cfg(test) `mod X;`), not raw lib.rs, so
  # SEM0 sees the same module set rustc's default lib cfg does. This sidesteps SEM0's resolve-mode file-mod
  # splicer, which is cfg-blind and would refuse on a `#[cfg(test)] mod` rustc excludes (rustc_index/bit_set).
  # Harness-side + oracle-class -- no SEM0 change, so no self-emit fragility. Same approach as t2accept-board.sh.
  python3 "$HERE/splice-crate.py" "$D/lib.rs" > "$W/spliced.rs" 2>/dev/null
  { echo '//@S3RESOLVE'; cat "$W/spliced.rs"; } | timeout 90 "$FE" >/dev/null 2>&1; src=$?
  if   [ "$orc" -eq 2 ]; then v=ORACLE-ABSENT
  elif [ "$orc" -eq 0 ] && [ "$src" -eq 0 ]; then v=PASS
  elif [ "$orc" -eq 0 ]; then v=FAIL
  elif [ "$src" -eq 0 ]; then v=OVERACCEPT
  else v=BOTH-REJECT; fi
  echo "$cr $v" >> "$BOARD"
done
sort -o "$BOARD" "$BOARD"
# regression check vs the committed baseline: a crate that was PASS must not drop to FAIL/OVERACCEPT/BOTH-REJECT.
# ORACLE-ABSENT (scope-out) and NOLIB never fail the gate. F2: able-to-FAIL.
BASE="${BASE:-$HERE/t1prime-baseline.txt}"
if [ -f "$BASE" ]; then
  reg=$(join <(sort "$BASE") <(sort "$BOARD") | awk '$2=="PASS" && $3!="PASS" && $3!="ORACLE-ABSENT" {print $1"("$2"->"$3")"}')
  over=$(awk '$2=="OVERACCEPT"{print $1}' "$BOARD")
  if [ -n "$reg" ] || [ -n "$over" ]; then
    echo "  FAIL t1'-board: regressions [${reg:-none}] over-accept [${over:-none}]"; cat "$BOARD" | sed 's/^/    /'; exit 1
  fi
fi
p=$(awk '$2=="PASS"' "$BOARD" | wc -l); a=$(awk '$2=="ORACLE-ABSENT"' "$BOARD" | wc -l); t=$(wc -l < "$BOARD")
echo "  ok   T1' RESOLVE board: $p PASS, $a oracle-absent (scope-out), of $t crates -- no regression"
exit 0
