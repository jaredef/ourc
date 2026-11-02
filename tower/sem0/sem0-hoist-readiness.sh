#!/usr/bin/env bash
# SEM0 HOIST READINESS -- run the semantic gate set against SEM0-BUILT SEM0, not L6-built SEM0.
#
# WHY. Every S-tier gate builds its SEM0 binary with `$SP/L6.rzrs` -- i.e. they all test gen1, the L6-built
# SEM0. Only two gates touch gen2 (the SEM0-built SEM0): s9-wholefile-diff.sh and s9-behavior-diff.sh. So the
# artifact that becomes load-bearing the moment SEM0 is hoisted is validated by 35 fixtures and 17 probes and
# nothing else -- while nine semantic gates test the binary being retired.
#
# That is the northstar-rung failure prospectively: the gates would start measuring the wrong rung the instant
# the hoist landed, with no signal saying so. This harness converts "SEM0 self-hosts" from an assumption
# (gen2 == gen3, a fixed point of the EMISSION) into a measurement (does gen2 pass the semantic gates?).
#
#   SP=<scratch> CC=<cc-in-c> bash sem0-hoist-readiness.sh
#
# Exits non-zero if any gate that passes with gen1 fails with gen2. That difference is the readiness verdict.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; B="$HERE/stages"
SP="${SP:?set SP}"; CC="${CC:?set CC}"; export SP CC
P="$B/S1-lex-parse-ast/s1_parser.rs"
W="$(mktemp -d)"; trap 'rm -rf "$W"' EXIT
back(){ "$CC/link/rzir-ld.bin" 2>/dev/null | "$CC/opt/rzir-opt.bin" 2>/dev/null | "$CC/ra/rzir-ra.bin" 2>/dev/null | "$SP/cf.hoist"; }

echo "== SEM0 hoist readiness =="
[ -x "$SP/L6.rzrs" ] || { echo "  FAIL: no \$SP/L6.rzrs (run stage.sh)"; exit 1; }

# gen1 = L6-built SEM0 (today's rung).  gen2 = SEM0-built SEM0 (the rung a hoist would install).
timeout 300 "$SP/L6.rzrs" < "$P" 2>/dev/null | back > "$W/gen1" 2>/dev/null; chmod +x "$W/gen1"
[ "$(stat -c%s "$W/gen1")" -ge 100000 ] || { echo "  FAIL: gen1 is a stub"; exit 1; }
{ echo '//@S9'; cat "$P"; } > "$W/s9.rs"
timeout 600 "$W/gen1" < "$W/s9.rs" > "$W/g2.rzir" 2>/dev/null
[ -s "$W/g2.rzir" ] || { echo "  FAIL: gen1 self-emit is empty"; exit 1; }
back < "$W/g2.rzir" > "$W/gen2" 2>/dev/null; chmod +x "$W/gen2"
[ "$(stat -c%s "$W/gen2")" -ge 100000 ] || { echo "  FAIL: gen2 is a stub"; exit 1; }
echo "  gen1 (L6-built)   $(stat -c%s "$W/gen1") B"
echo "  gen2 (SEM0-built) $(stat -c%s "$W/gen2") B   self-image $(wc -l < "$W/g2.rzir") rzir lines"

# The gates invoke their compiler as `CC < source.rs` and expect rzir on stdout. gen2 needs the //@S9 marker to
# emit rzir rather than dump an AST, so present it behind a shim with L6's interface. This is the ONLY difference
# between the two runs -- same gates, same fixtures, same backend.
# NB: the marker must be prepended into a FILE, not piped. SEM0 truncates piped stdin (its read_to_string
# lowering does not loop to EOF), so `{ echo //@S9; cat; } | gen2` silently feeds it a fragment -- measured:
# 254 rzir lines instead of 285,250. The delim_balance refusal added earlier catches it (0 output, exit 1)
# rather than compiling garbage, which is how this was noticed at all. Recorded as C-SEM0-PIPED-STDIN-TRUNCATES.
cat > "$W/sem0cc" <<SHIM
#!/bin/sh
t="\$(mktemp)"
{ echo '//@S9'; cat; } > "\$t"
"$W/gen2" < "\$t"; r=\$?
rm -f "\$t"
exit \$r
SHIM
chmod +x "$W/sem0cc"

GATES="S1-lex-parse-ast/s1-ast-diff.sh S2-macro-procmacro/s2-expand-diff.sh S3-resolve-crate-driver/s3-resolve-diff.sh
       S4-hir-lowering/s4-hir-diff.sh S5-typeck-trait-solver/s5-typeck-diff.sh S6-generics-mono/s6-mono-diff.sh
       S7-checker-tiers/s7-check-diff.sh S9-lower-codegen/s9-run-diff.sh"
export BASELINE=sem
printf '\n  %-26s %-10s %-10s %s\n' GATE "gen1(L6)" "gen2(SEM0)" VERDICT
fail=0; both=0
for g in $GATES; do
  n="$(basename "$g" .sh)"
  timeout 900 bash "$B/$g" > "$W/a.log" 2>&1; a=$?
  SEM0_CC="$W/sem0cc" timeout 900 bash "$B/$g" > "$W/b.log" 2>&1; b=$?
  av=$([ "$a" = 0 ] && echo ok || echo FAIL); bv=$([ "$b" = 0 ] && echo ok || echo FAIL)
  if [ "$a" = 0 ] && [ "$b" != 0 ]; then v="REGRESSION"; fail=$((fail+1)); cp "$W/b.log" "$SP/.readiness.$n.log"
  elif [ "$a" != 0 ]; then v="(gen1 already failing -- not a hoist signal)"
  else v="ready"; both=$((both+1)); fi
  printf '  %-26s %-10s %-10s %s\n' "$n" "$av" "$bv" "$v"
  [ "$v" = "REGRESSION" ] && sed -n '1,3p' "$W/b.log" | sed 's/^/        /'
done

# GEN2 DIFFERENTIAL (2026-08-26). The eight gates above run gen2 on SEM0's OWN S-tier fixtures, and the
# 904-probe corpus board is scored with gen1 -- measured identical under gen2, so it cannot see this either.
# Nothing here touched gen2 on large real Rust until this lane. Measured on a 192-file sample of rustc 1.83.0:
# gen1 and gen2 diverge on 9 of 192 files, 7 of them gen2 SIGSEGVs. That is the northstar-rung failure one tier
# in -- the artifact a hoist INSTALLS was the one thing not measured on real input. Advisory here (it carries
# its own known-open baseline); it is the readiness signal that matters most and must reach 0 known-open before
# a hoist proposal.
gd=0
if bash "$B/SEM0-hoist/gen2-differential/gen2-differential.sh" > "$W/gd.log" 2>&1; then
  gd_line=$(grep -E 'corpus [0-9]+ files' "$W/gd.log" | head -1)
  printf '  %-26s %s\n' "gen2-differential" "${gd_line:-ok}"
else
  gd=1; printf '  %-26s %s\n' "gen2-differential" "NEW DIVERGENCE -- see below"
  sed -n '1,12p' "$W/gd.log" | sed 's/^/        /'
fi

echo
if [ "$fail" = 0 ]; then
  echo "  VERDICT: $both/8 semantic gates pass with SEM0-built SEM0 -- no gate regresses under the hoist."
  echo "  This is a NECESSARY condition, not a sufficient one: .unwrap() is still a silent no-op"
  echo "  (6 sites, all on read_to_string Results).\n  AST ITEM fidelity vs rustc IS now measured -- 1835/1835 comparable files, 100.0%, 0 missed 0 invented\n  (locales/rustc-crate-targeting/ast-fidelity). That covers item KIND and NAME at every depth; expression\n  bodies, types, patterns and spans are compared by no oracle here and remain unmeasured."
  echo "  panic!/assert! were closed 2026-08-26 and now exit 101 like rustc."
  echo "  AND: the gen2 differential carries $(grep -vcE '^\s*#|^\s*$' "$B/SEM0-hoist/gen2-differential/baseline.txt") known-open divergence(s)"
  echo "  between the L6-built and SEM0-built compilers on real Rust. Those are defects in the artifact a hoist"
  echo "  would install; 8/8 above does not cover them."
else
  echo "  VERDICT: $fail gate(s) REGRESS under SEM0-built SEM0 -- NOT ready to hoist. Logs: \$SP/.readiness.*.log"
fi
[ "$fail" = 0 ] && [ "$gd" = 0 ]
