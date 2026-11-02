# llvm-backend.sh -- shared TARGET-codegen backend for RUN gates (sourced, not executed).
#
# The meta-telos backend is the rzir->LLVM bridge (Route B, decision 2026-09-27): a RUN gate judges target
# behaviour on  rzir -> rzir2ll -> llc -> cc , NOT on cf. cf is the hex-rooted self-host backend ONLY (it still
# builds the rzir2ll translator here, which is legitimate Tier-1: self-hosting the tool). This file gives every
# RUN gate one backend-switchable producer so the migration is one line per gate instead of duplicated plumbing.
#
# CONTRACT (set before sourcing):
#   SP   = scratch (needs ld.bin + cf.hoist; optionally a prebuilt r2l.elf)
#   CC   = cc-in-c toolchain root (opt/ra twins)
#   RZ_R2LCC = a SEM0 binary that lowers rzir2ll.rs (e.g. $SP/s1.rz / L6.rzrs / sem0.frozen)  [only needed to BUILD r2l]
#   RZ_BACKEND = cf | llvm   (default llvm -- the shipped backend)
# PROVIDES:
#   rz_backend_ready  -> MUST be called in the CURRENT shell (not $(...)): sets RZ_BACKEND_EFFECTIVE + RZ_R2L_ELF
#                        and RZ_BACKEND_MSG; returns 0 ready / 1 skip. Call as: rz_backend_ready || { skip; exit 0; }
#   rz_back           -> reads rzir on stdin, writes an ELF to stdout (so `... | rz_back > out.elf` works on both)
# A gate SKIPs cleanly (exit 0) when RZ_BACKEND=llvm but llc/cc/translator are unavailable -- capability-gated,
# host-free CI stays green, exactly like llvm-parity.sh.

RZ_BACKEND="${RZ_BACKEND:-llvm}"
_RZ_HERE_LIB="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RZ_LLC="${LLC_BIN:-$(command -v llc 2>/dev/null || ls ~/.rustup/toolchains/*/lib/rustlib/*/bin/llc 2>/dev/null | head -1)}"
RZ_CCB="${CC_BIN:-$(command -v cc 2>/dev/null || command -v gcc 2>/dev/null)}"
RZ_R2LRS="${RZ_R2LRS:-$_RZ_HERE_LIB/../backend-llvm/rzir2ll.rs}"
RZ_R2L_ELF=""

_rz_cf_back(){ "$SP/ld.bin" 2>/dev/null | "$CC/opt/rzir-opt.bin" 2>/dev/null | "$CC/ra/rzir-ra.bin" 2>/dev/null | "$SP/cf.hoist" 2>/dev/null; }

rz_backend_ready(){
  RZ_BACKEND_MSG=""
  if [ "$RZ_BACKEND" != "llvm" ]; then RZ_BACKEND_EFFECTIVE="cf"; RZ_BACKEND_MSG="cf"; return 0; fi
  if [ -z "$RZ_LLC" ] || [ -z "$RZ_CCB" ]; then RZ_BACKEND_MSG="llc/cc not found (LLVM target backend unavailable)"; return 1; fi
  if [ -x "$SP/r2l.elf" ] && [ -s "$SP/r2l.elf" ]; then RZ_R2L_ELF="$SP/r2l.elf";
  else
    [ -x "$RZ_R2LCC" ] || { RZ_BACKEND_MSG="no RZ_R2LCC to build the rzir2ll translator"; return 1; }
    { echo '//@S9'; cat "$RZ_R2LRS"; } | "$RZ_R2LCC" 2>/dev/null | _rz_cf_back > "$SP/r2l.elf" 2>/dev/null
    chmod +x "$SP/r2l.elf" 2>/dev/null; RZ_R2L_ELF="$SP/r2l.elf"
  fi
  [ -x "$RZ_R2L_ELF" ] && [ -s "$RZ_R2L_ELF" ] || { RZ_BACKEND_MSG="rzir2ll translator did not build"; return 1; }
  RZ_BACKEND_EFFECTIVE="llvm"; RZ_BACKEND_MSG="llvm"; return 0
}

rz_back(){
  if [ "$RZ_BACKEND_EFFECTIVE" = "llvm" ]; then
    local t; t="$(mktemp -d)"
    if "$RZ_R2L_ELF" > "$t/o.ll" 2>/dev/null && "$RZ_LLC" -O2 -filetype=obj "$t/o.ll" -o "$t/o.o" 2>/dev/null && "$RZ_CCB" -no-pie "$t/o.o" -o "$t/o.elf" 2>/dev/null; then cat "$t/o.elf"; fi
    rm -rf "$t"
  else _rz_cf_back; fi
}
