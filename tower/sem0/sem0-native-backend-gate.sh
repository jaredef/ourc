#!/usr/bin/env bash
# sem0-native-backend-gate.sh -- NB-5: the SEM0-native backend, trust-anchored.
#
# WHAT IT ASSERTS. cf.rs (the rzir->ELF backend) is itself Rust, and the hex-rooted SEM0 (minted through the
# sem0-stage staircase into $SP/sem0.frozen) compiles it. This gate proves that the backend assembled by SEM0
# (a maximally-diverse ~19k-line compiler) is BYTE-IDENTICAL to the backend assembled by the tower's cf.hoist
# on real programs -- including a heap program, which exercises cf.rs's byte-buffer + raw-syscall code paths
# (the P-NB-A/P-NB-B pipelines). This is a diverse-double-compilation of the BACKEND ITSELF: a trusting-trust
# attack would have to live identically in BOTH the audit-floor chain AND SEM0 to survive.
#
# WHY NOT provenance step 5d / an L6-seed patch. Provenance's SEM0 is the L6-buildable v1 seed, and patching it
# with the byte-array/__syscall fixes trips L6's non-monotone self-host miscompile resonance (measured; see
# nb5-seed-refreeze-scope.md). $SP/sem0.frozen is minted through the sem0-stage STAIRCASE (each hop by the
# previous frozen, not by L6), already carries the fixes, and is hex-rooted -- so it dodges L6 entirely.
# STRICTLY ADDITIVE: the M0 audit floor and cf.hoist are unchanged and remain the root.
#
#   SP=<scratch: has sem0.frozen + cf.hoist + ld.bin> CC=<cc-in-c> bash sem0-native-backend-gate.sh
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../.." && pwd)"
SP="${SP:?set SP}"; CC="${CC:?set CC}"
CFRS="$HERE/../backend-cf/cf.rs"
RT="$ROOT/ourc/rt/runtime.rzir"
FROZ="$SP/sem0.frozen"; [ -x "$FROZ" ] || FROZ="$SP/sem0.cc"
CFH="$SP/cf.hoist"
for f in "$FROZ" "$CFH" "$SP/ld.bin" "$CC/opt/rzir-opt.bin" "$CC/ra/rzir-ra.bin" "$CFRS" "$RT"; do
  [ -e "$f" ] || { echo "  FAIL sem0-native-backend: missing $f"; exit 1; }
done
W="$(mktemp -d)"; trap 'rm -rf "$W"' EXIT
spine(){ "$SP/ld.bin" 2>/dev/null | "$CC/opt/rzir-opt.bin" 2>/dev/null | "$CC/ra/rzir-ra.bin" 2>/dev/null | "$CFH" 2>/dev/null; }
ra(){ { echo '//@S9'; cat "$1"; } | timeout 120 "$FROZ" 2>/dev/null | "$SP/ld.bin" 2>/dev/null | "$CC/opt/rzir-opt.bin" 2>/dev/null | "$CC/ra/rzir-ra.bin" 2>/dev/null > "$2"; }

# 1. mint the SEM0-native backend: cf.rs compiled by the hex-rooted SEM0 (sem0.frozen), assembled by the spine.
{ echo '//@S9'; cat "$CFRS"; } | timeout 180 "$FROZ" > "$W/cf.s9" 2>/dev/null
grep -q UNLOWERED "$W/cf.s9" && { echo "  FAIL sem0-native-backend: SEM0 left UNLOWERED rzir in cf.rs"; exit 1; }
spine < "$W/cf.s9" > "$W/cf.sem0" 2>/dev/null; chmod +x "$W/cf.sem0"
[ "$(wc -c <"$W/cf.sem0")" -ge 20000 ] || { echo "  FAIL sem0-native-backend: cf.sem0 too small ($(wc -c <"$W/cf.sem0") B)"; exit 1; }

# 2. probes -- incl. a heap program (Vec/String) that drives cf.rs's byte-buffer + syscall paths.
mkdir -p "$W/p"
printf 'fn main(){ let mut s=0; let mut i=1; while i<=10 { s=s+i; i=i+1; } println!("{}", s); }\n' > "$W/p/sum.rs"      # 55
printf 'fn fib(n:u32)->u32{ if n<2 {n} else {fib(n-1)+fib(n-2)} } fn main(){ println!("{}", fib(15)); }\n' > "$W/p/fib.rs" # 610
printf 'fn main(){ let v=vec![5,3,8,1]; let mut m=0; for x in v.iter(){ if *x>m {m=*x;} } println!("max {}", m); }\n' > "$W/p/heap.rs" # max 8
printf 'fn main(){ let s=String::from("hello"); println!("{} {}", s, s.len()); }\n' > "$W/p/str.rs"                       # hello 5

fails=0
for prog in sum fib heap str; do
  ra "$W/p/$prog.rs" "$W/$prog.ra"
  [ -s "$W/$prog.ra" ] || { echo "  FAIL sem0-native-backend [$prog]: no ra"; fails=$((fails+1)); continue; }
  "$CFH"        < "$W/$prog.ra" > "$W/$prog.h.elf" 2>/dev/null; chmod +x "$W/$prog.h.elf"
  "$W/cf.sem0"  < "$W/$prog.ra" > "$W/$prog.s.elf" 2>/dev/null; chmod +x "$W/$prog.s.elf"
  hr="$(timeout 20 "$W/$prog.h.elf" 2>/dev/null)"; sr="$(timeout 20 "$W/$prog.s.elf" 2>/dev/null)"
  if cmp -s "$W/$prog.h.elf" "$W/$prog.s.elf" && [ -n "$hr" ] && [ "$hr" = "$sr" ]; then
    :
  else
    echo "  FAIL sem0-native-backend [$prog]: cf.hoist[$hr] != cf.sem0[$sr] (elf $(cmp -s "$W/$prog.h.elf" "$W/$prog.s.elf" && echo same || echo diff))"
    fails=$((fails+1))
  fi
done

[ "$fails" = 0 ] && echo "  ok   sem0-native-backend: cf.rs assembled by hex-rooted SEM0 == cf.hoist (byte-identical) on sum/fib/heap/str -- the SEM0-native backend is trust-anchored" || exit 1
exit 0
