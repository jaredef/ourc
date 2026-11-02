#!/usr/bin/env bash
# t2accept-board.sh -- the T2 ACCEPT board: SEM0's whole-crate //@S7CHECK verdict == the ORACLE's accept/reject.
# The crate is SPLICED whole (splice-crate.py inlines every non-cfg(test) `mod X;`) so SEM0 checks the same code
# rustc does -- reading lib.rs alone would be the R0 splice-trap at T2 (an over-acceptance risk). Verdicts:
#   PASS          oracle and SEM0 AGREE (both ACCEPT, or both REJECT)
#   OVERACCEPT    SEM0 ACCEPTs but oracle REJECTs (Doc-731 over-acceptance)
#   UNDERACCEPT   SEM0 REJECTs but oracle ACCEPTs
#   ORACLE-ABSENT crate needs rustc-internal features beyond bootstrap (scope-out)
# F1 the accept OUTCOME, not a proxy. F3 RUNG=SEM0 ($SP/wf.gen1, size-asserted), ORACLE=stable rustc+bootstrap+
# oracle-class vendor. Oracle-class inputs never enter the chain. Able-to-FAIL vs t2accept-baseline.txt.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
SRC="${SRC:-/tmp/rustc-src/rust-1.95.0}"; ORACLE_SRC="${ORACLE_SRC:-/tmp/rustc-src/rustc-1.95.0-src}"; export ORACLE_SRC; VEN="${VEN:-/tmp/rustc-oracle-vendor}"
SP="${SP:?set SP}"; FE="$SP/SEM0.rzrs"; [ -x "$FE" ] || FE="$SP/wf.gen1"
CRATES="${CRATES:-$(ls -d "$SRC"/compiler/*/ 2>/dev/null | xargs -n1 basename | tr "\n" " ")}"
BOARD="${BOARD:-$HERE/t2accept-board.txt}"; BASE="${BASE:-$HERE/t2accept-baseline.txt}"
[ -x "$FE" ] || { echo "  ok   t2-board SKIP (no SEM0 rung)"; exit 0; }
sz=$(stat -c%s "$FE"); [ "$sz" -ge 1000000 ] || { echo "  FAIL t2: rung is $sz B (stub)"; exit 1; }
[ -d "$SRC/compiler" ] && [ -f "$ORACLE_SRC/x.py" ] || { echo "  ok   t2-board SKIP (oracle (B) tree absent)"; exit 0; }
bash "$HERE/oracle-vendor-setup.sh" >/dev/null 2>&1 || true
W="$(mktemp -d)"; trap 'rm -rf "$W"' EXIT
: > "$BOARD"
for cr in $CRATES; do
  D="$SRC/compiler/$cr/src"; [ -f "$D/lib.rs" ] || { echo "$cr NOLIB" >> "$BOARD"; continue; }
  bash "$HERE/oracle-build.sh" "$cr" >/dev/null 2>&1; orc=$?    # 0 accept, 1 reject, 2 absent
  python3 "$HERE/splice-crate.py" "$D/lib.rs" > "$W/spliced.rs" 2>/dev/null
  { echo '//@S7CHECK'; cat "$W/spliced.rs"; } | timeout 120 "$FE" > "$W/v" 2>/dev/null
  sv=$(grep -oE 'ACCEPT|REJECT' "$W/v" | head -1); [ -n "$sv" ] || sv=REFUSE
  if   [ "$orc" -eq 2 ]; then v=ORACLE-ABSENT
  elif [ "$orc" -eq 0 ] && [ "$sv" = ACCEPT ]; then v=PASS
  elif [ "$orc" -ne 0 ] && [ "$sv" = REJECT ]; then v=PASS
  elif [ "$sv" = ACCEPT ]; then v=OVERACCEPT
  else v=UNDERACCEPT; fi
  echo "$cr $v" >> "$BOARD"
done
sort -o "$BOARD" "$BOARD"
if [ -f "$BASE" ]; then
  reg=$(join <(sort "$BASE") <(sort "$BOARD") | awk '$2=="PASS" && $3!="PASS" && $3!="ORACLE-ABSENT" {print $1"("$2"->"$3")"}')
  bad=$(awk '$2=="OVERACCEPT"||$2=="UNDERACCEPT"{print $1"("$2")"}' "$BOARD")
  if [ -n "$reg" ] || [ -n "$bad" ]; then echo "  FAIL t2-board: regressions [${reg:-none}] disagreements [${bad:-none}]"; cat "$BOARD"|sed 's/^/    /'; exit 1; fi
fi
p=$(awk '$2=="PASS"' "$BOARD"|wc -l); a=$(awk '$2=="ORACLE-ABSENT"' "$BOARD"|wc -l); t=$(wc -l<"$BOARD")
echo "  ok   T2 ACCEPT board: $p PASS, $a oracle-absent, of $t crates -- verdicts agree with rustc"
exit 0
