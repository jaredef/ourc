#!/bin/bash
# surface.sh -- behavioral-surface probe harness (arc 2026-10-26-ourc-behavioral-surface).
#   OURC=<ourc binary> bash ourc/surface.sh [probe.rs | dir]...      (default: ourc/probes/surface)
# For each probe: rustc (ORACLE ONLY) decides accept/reject and, if accepted, stdout + exit code; ourc must give
# the same verdict: reject <-> reject (class only), accept -> same stdout and exit. Prints one line per probe and a
# tally; exit 1 if any probe differs. The oracle verdict is cached beside the probe as <probe>.oracle.
# PROFILE: ourc models RELEASE arithmetic (P-WIDTH: overflow WRAPS, as `rustc -O`), so the oracle runs with
# `-C overflow-checks=off`; everything else stays the debug profile. Panics that do not depend on the
# profile (index out of bounds, unwrap on None, divide by zero, explicit panic!) are unaffected.
set -u
D="$(cd "$(dirname "$0")" && pwd)"; OURC="${OURC:?set OURC}"; LLC="${LLC:-llc}"
W="$(mktemp -d)"; trap 'rm -rf "$W"' EXIT
[ $# -gt 0 ] || set -- "$D/probes/surface"
oracle(){ local f="$1" o="$1.oracle"
  [ -s "$o" ] && [ "$o" -nt "$f" ] && { cat "$o"; return; }
  if rustc --edition 2021 -A warnings -C overflow-checks=off -o "$W/r" "$f" 2>/dev/null; then
    local out; out=$(timeout 10 "$W/r" 2>/dev/null); echo "accept rc=$? out=$(printf %s "$out" | md5sum | cut -c1-12)" | tee "$o"
  else echo "reject" | tee "$o"; fi; }
ours(){ local f="$1"
  "$OURC" "$f" --emit-ir > "$W/o.out" 2>&1; local orc=$?
  # DRV-EXIT: a rejection must also EXIT NON-ZERO, as rustc does (a reject that exits 0 is its own mismatch)
  if grep -qE '(^|-> )error' "$W/o.out" || ! grep -q '^define' "$W/o.out"; then [ "$orc" != 0 ] && echo reject || echo reject-rc0; return; fi
  sed -n '/^\(@\|declare\|define\)/,$p' "$W/o.out" > "$W/o.ll"
  "$LLC" -O2 -filetype=obj "$W/o.ll" -o "$W/o.o" 2>/dev/null && cc -no-pie "$W/o.o" -o "$W/o.bin" -lm 2>/dev/null || { echo "ir-broken"; return; }
  local out; out=$(timeout 10 "$W/o.bin" 2>/dev/null); echo "accept rc=$? out=$(printf %s "$out" | md5sum | cut -c1-12)"; }
pass=0; fail=0
for a in "$@"; do for f in $( [ -d "$a" ] && find "$a" -name '*.rs' | LC_ALL=C sort || echo "$a"); do
  want=$(oracle "$f"); got=$(ours "$f")
  if [ "$want" = "$got" ]; then pass=$((pass+1)); printf '  ok   %s\n' "${f#$D/}"
  else fail=$((fail+1)); printf '  DIFF %s: rustc[%s] ourc[%s]\n' "${f#$D/}" "$want" "$got"; fi
done; done
echo "surface: $pass ok, $fail differ"; [ "$fail" = 0 ]
