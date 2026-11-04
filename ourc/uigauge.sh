#!/usr/bin/env bash
# PARITY GAUGE: run ourc over rustc's own ui test suite and report, honestly, how many it
#   (a) accepts, and (b) compiles to a binary that RUNS and agrees with rustc's binary.
# Only `run-pass`-style tests (no `//~ ERROR`, no `error-pattern`) are counted as gradeable: a test rustc
# REJECTS proves nothing about us accepting it.
set -u
OURC="${OURC:-/tmp/ourc}"; LLC="${LLC:?}"; N="${N:-300}"
T="${T:-/tmp/rustc-src/rust-1.83.0}"
W="$(mktemp -d)"; trap 'rm -rf "$W"' EXIT
grad=0; acc=0; ran=0; agree=0; nob=0
for f in $(find "$T/tests/ui" -name '*.rs' | sort | tail -n +"${OFF:-1}" | awk "NR % ${STEP:-1} == 0" | head -"$N"); do
  grep -q '//~\|error-pattern\|compile-fail\|run-fail\|#!\[feature' "$f" && continue      # rustc rejects it, expects a panic, or needs nightly      # rustc rejects it, or nightly
  grep -q 'fn main' "$f" || continue                                            # needs an entry point
  # rustc's own answer (must build, else the test needs more than a plain rustc invocation). The
  # denominator counts only files rustc ITSELF builds this way -- counting them before this check
  # inflated it by 42% (413 vs 237 on the head-1200 slice) and understated every rate computed from it.
  rustc -O --edition 2015 "$f" -o "$W/r" 2>/dev/null || { nob=$((nob+1)); continue; }
  grad=$((grad+1))
  want="$(timeout 10 "$W/r" </dev/null 2>/dev/null; echo "rc=$?")"
  out="$("$OURC" "$f" --emit-ir 2>&1)"
  echo "$out" | head -1 | grep -q '> ok' || continue
  acc=$((acc+1))
  echo "$out" | sed -n '/^\(@\|declare\|define\)/,$p' > "$W/o.ll"
  "$LLC" -O2 -filetype=obj "$W/o.ll" -o "$W/o.o" 2>/dev/null || continue
  cc -no-pie "$W/o.o" -o "$W/o" -lm 2>/dev/null || continue
  got="$(timeout 5 "$W/o" 2>/dev/null; echo "rc=$?")"
  ran=$((ran+1))
  if [ "$got" = "$want" ]; then agree=$((agree+1)); [ -n "${V:-}" ] && echo "  AGREE   ${f#*/tests/ui/}"
  else [ -n "${V:-}" ] && echo "  DIVERGE ${f#*/tests/ui/}: rustc=[$(echo "$want"|tr '\n' '|')] ourc=[$(echo "$got"|tr '\n' '|')]"; fi
done
echo "ui gauge: $N files (offset ${OFF:-1}, step ${STEP:-1}): comparable=$grad accepted=$acc built+ran=$ran AGREE-WITH-RUSTC=$agree (+$nob skipped: plain rustc -O cannot build them)"
