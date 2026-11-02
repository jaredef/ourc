#!/usr/bin/env bash
# R2 corpus differential map: every probes/*.rs through the derived toolchain AND rustc (oracle), categorized.
# The corpus was grown by a 20-agent parallel fan-out (r2-corpus-growth workflow). Re-run to check for regressions
# or after closing a rung.  SP=<scratch with rzrs.self + cf.full> bash map.sh [--fails]
# CANONICAL STAGE NAMES (apparatus/docs/tower-naming-and-trust-path.md): rzrs.self=L5.rzrs (frozen
# mainline) or the L6.rzrs dev binary via symlink; cf.full=B.cf; rzir-{ld,opt,ra}.bin=B.{ld,opt,ra}
# (@dev, coherence-pinned to @trust by provenance.sh step 7). rustc here is the ORACLE (== check), never in the chain.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; A="$(cd "$HERE/../.." && pwd)"; CC="${TOWER_L2:-$A/L2}"
SP="${SP:?set SP (scratch with rzrs.self + cf.full)}"
ONLYFAIL="${1:-}"
W="$(mktemp -d)"
# RZ_LD: the linker. Default = the frozen C ld (L6 path). SEM0 output uses a >2GB arena the C ld cannot link
# (it clamps heaptop at INT32_MAX), so the SEM0 gate sets RZ_LD=$SP/ld.bin (the i64 ld twin).
RZ_LD="${RZ_LD:-$CC/link/rzir-ld.bin}"
chain(){ "$RZ_LD" | "$CC/opt/rzir-opt.bin" | "$CC/ra/rzir-ra.bin" | "$SP/cf.full"; }
# RZ_BACKEND selects the TARGET codegen: "cf" (Tier-1 hex-rooted, self-host default) or "llvm" (the DECIDED
# meta-telos target backend: rzir -> rzir2ll -> llc -> cc). cf is retained only to self-host the tower + build the
# rzir2ll translator; RUN correctness of the target is judged on LLVM. produce_elf <rzfile> <elffile> dispatches.
RZ_BACKEND="${RZ_BACKEND:-cf}"
LLC_BIN="${LLC_BIN:-$(command -v llc 2>/dev/null || ls ~/.rustup/toolchains/*/lib/rustlib/*/bin/llc 2>/dev/null | head -1)}"
CC_BIN="${CC_BIN:-$(command -v cc 2>/dev/null || command -v gcc 2>/dev/null)}"
R2L_ELF=""
if [ "$RZ_BACKEND" = "llvm" ]; then
  [ -n "$LLC_BIN" ] && [ -n "$CC_BIN" ] || { echo "map.sh: RZ_BACKEND=llvm but llc/cc not found" >&2; exit 2; }
fi
produce_elf(){ # $1=rzfile $2=elffile
  if [ "$RZ_BACKEND" = "llvm" ]; then
    "$R2L_ELF" < "$1" > "$2.ll" 2>/dev/null || return 1
    "$LLC_BIN" -O2 -filetype=obj "$2.ll" -o "$2.o" 2>/dev/null || return 1
    "$CC_BIN" -no-pie "$2.o" -o "$2" 2>/dev/null || return 1
  else chain < "$1" > "$2" 2>/dev/null; fi
  chmod +x "$2" 2>/dev/null; return 0
}
# RZ_CC / RZ_MARK: which compiler the "derived" column measures. Default is $SP/rzrs.self, i.e. L6 -- the tier
# being RETIRED. Set RZ_CC=<sem0 binary> RZ_MARK='//@S9' to score SEM0 instead.
# WHY THIS EXISTS. Measured 2026-08-26: borrow_mutfn, borrow_mutfn2 and closures_fnmut_ref are baseline OK, and
# they are OK -- under L6. Under SEM0 all three MISCOMP (`&mut` of a SCALAR mutates a copy). This 904-probe board
# could not see that, because it only ever measured L6, so a hoist would have regressed them silently. That is
# exactly the northstar-rung failure, on the one gate the readiness harness did not parameterise.
# The marker is prepended into a FILE, never piped: SEM0 truncates piped stdin (C-SEM0-PIPED-STDIN-TRUNCATES).
# C-CORPUS-ASLR-NONDETERMINISM (locales/call-abi-arity CL-A10): a MISCOMPILED probe dereferences a wild pointer,
# and whether that address faults, hits an unmapped page, or lands on something executable depends on the ADDRESS
# SPACE LAYOUT. Measured 2026-08-26 on symbol_interner_hashmap: the SAME byte-identical binary returned
# 139 139 139 139 132 139 132 139 139 139 across ten runs, so its row alternated between MISCOMP and HANG and
# polluted the regression column of every sweep. Under `setarch -R` (ASLR off) the same binary returned 132 ten
# times out of ten. Disabling randomisation makes the CLASSIFICATION deterministic without changing what is
# measured: a correct probe is unaffected, and an incorrect one now fails the same way every time.
# Absent setarch (non-Linux host), NORAND is empty and the harness behaves exactly as it did.
NORAND=""
if command -v setarch >/dev/null 2>&1; then if setarch -R true >/dev/null 2>&1; then NORAND="setarch -R"; fi; fi
RZCC="${RZ_CC:-$SP/rzrs.self}"
RZMARK="${RZ_MARK:-}"
if [ "$RZ_BACKEND" = "llvm" ]; then
  # Build the rzir2ll translator ONCE (Tier-1: the SEM0 under test lowers it, cf assembles the tool). Prefer $SP/r2l.elf.
  if [ -x "$SP/r2l.elf" ]; then R2L_ELF="$SP/r2l.elf"; else
    { [ -n "$RZMARK" ] && echo "$RZMARK"; cat "$HERE/../../backend-llvm/rzir2ll.rs"; } | "$RZCC" > "$W/r2l.rz" 2>/dev/null
    chain < "$W/r2l.rz" > "$W/r2l.elf" 2>/dev/null; chmod +x "$W/r2l.elf"; R2L_ELF="$W/r2l.elf"
  fi
  [ -x "$R2L_ELF" ] && [ -s "$R2L_ELF" ] || { echo "map.sh: RZ_BACKEND=llvm but rzir2ll translator did not build" >&2; exit 2; }
fi
for f in "$HERE"/probes/*.rs; do
  [ -e "$f" ] || continue
  n=$(basename "$f" .rs)
  if rustc -O --edition 2021 -o "$W/$n.o" "$f" 2>/dev/null; then ostat="ok:$("$W/$n.o" 2>/dev/null):$?"; else ostat="REJECT"; fi
  if [ -n "$RZMARK" ]; then { echo "$RZMARK"; cat "$f"; } > "$W/$n.in"; else cp "$f" "$W/$n.in"; fi
  timeout "${RZ_TIMEOUT:-10}" "$RZCC" < "$W/$n.in" > "$W/$n.rz" 2>"$W/$n.e"; fc=$?
  if [ "$fc" = 124 ]; then dstat="HANG"
  elif [ "$fc" != 0 ]; then dstat="REJECT"
  else produce_elf "$W/$n.rz" "$W/$n.elf"
    # The derived binary is run under a TIMEOUT. It was unbounded, which is safe only while the compiler being
    # scored produces terminating code. Measured 2026-08-26: under SEM0 three probes
    # (grow3_labeled_break_value, grow6_labeledbreak_2, grow9_statemachine_15 -- all labeled-break shapes)
    # produce NON-TERMINATING binaries, and the unbounded run stalled the whole 904-probe sweep indefinitely.
    # The `| sort -k2` at the end of the loop means nothing is emitted until every probe finishes, so the stall
    # presented as "zero probes scored" and read like a hang in the harness rather than a runaway child.
    # Inert on the default L6 path: every L6-produced binary terminates, and the board is byte-identical.
    if [ -s "$W/$n.elf" ]; then
      out=$(timeout 5 $NORAND "$W/$n.elf" 2>/dev/null); rc=$?
      if [ "$rc" = 124 ]; then dstat="HANG"; else dstat="ok:$out:$rc"; fi
    else dstat="NOELF"; fi; fi
  if [ "$ostat" = "$dstat" ]; then cat="OK"
  elif [ "$ostat" = "REJECT" ] && [ "$dstat" = "REJECT" ]; then cat="REJECT-OK"
  elif [ "$ostat" = "REJECT" ]; then cat="UNSOUND"
  elif [ "$dstat" = "REJECT" ] || [ "$dstat" = "NOELF" ]; then cat="GAP"
  elif [ "$dstat" = "HANG" ]; then cat="HANG"
  else cat="MISCOMP"; fi
  if [ "$ONLYFAIL" = "--fails" ] && [ "$cat" = "OK" ]; then continue; fi
  printf '%-30s %-9s oracle=%-14s derived=%s\n' "$n" "$cat" "$ostat" "$dstat"
done | sort -k2
rm -rf "$W"
