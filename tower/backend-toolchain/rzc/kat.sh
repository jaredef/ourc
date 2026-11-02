#!/usr/bin/env bash
# kat.sh -- K1 gate: prove the extended syscall ABI (64-bit args + r10/r8) supports process control.
# Builds the backend from the CURRENT cf.rs (so the extension is exercised), then compiles+runs spawn-kat.rs,
# which fork()s a child that exit(42)s and wait4(pid,&status,0,0)s for it -- asserting WEXITSTATUS == 42.
#
#   SP=<scratch containing rzrs.self (mainline frontend)> bash kat.sh
#
# The M0 audit-floor cf is used ONLY to lower cf.rs into a backend binary; the KAT itself runs on the cf.rs
# backend, so a broken r10 (rusage) wiring would EFAULT wait4 and fail the assert.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; A="$(cd "$HERE/../.." && pwd)"; CC="$A/L2"
SP="${SP:?set SP (must contain the mainline frontend rzrs.self)}"
HOIST_CF="$A/backend-cf/cf.rs"
MC="$A/L0"
"$MC/m0/m0.bin" < "$MC/emitter/rzir-to-elf-cf.m0" > "$SP/cf.m0" 2>/dev/null; chmod +x "$SP/cf.m0"
chain(){ "$CC/link/rzir-ld.bin" | "$CC/opt/rzir-opt.bin" | "$CC/ra/rzir-ra.bin" | "$1"; }

# build the cf.rs backend (with the K1 syscall extension) via the M0 cf
"$SP/rzrs.self" < "$HOIST_CF" 2>/dev/null | chain "$SP/cf.m0" > "$SP/cf.rs.bin" 2>/dev/null; chmod +x "$SP/cf.rs.bin"
[ -s "$SP/cf.rs.bin" ] || { echo "FAIL: could not build cf.rs backend"; exit 1; }

run_kat(){ # $1 = src, $2 = expected rc, $3 = label
  "$SP/rzrs.self" < "$1" 2>/dev/null | chain "$SP/cf.rs.bin" > "$SP/kat.elf" 2>/dev/null
  chmod +x "$SP/kat.elf"; [ -s "$SP/kat.elf" ] || { echo "FAIL ($3): frontend/chain rejected $(basename "$1")"; exit 1; }
  "$SP/kat.elf"; local rc=$?
  if [ "$rc" = "$2" ]; then echo "  PASS $3 (== $2)"; else echo "  FAIL $3: got $rc, expected $2"; exit 1; fi
}

echo "== rzc KATs on the extended cf.rs backend =="
# K1: fork + wait4 child exit-code retrieval
run_kat "$HERE/spawn-kat.rs" 42 "K1 fork+wait4 exit-code"
# K2: byte-slice reads (b\"...\"[i]/.len(), str.as_bytes()) -- broken before the ecopy fix
run_kat "$HERE/byteslice-kat.rs" 167 "K2 byte-slice reads"
# K2: fork + execve (path from a b\"...\" literal) + wait4 -- build a known-exit child ELF first
printf 'fn main()->i32{ return 55; }' | "$SP/rzrs.self" 2>/dev/null | chain "$SP/cf.rs.bin" > /tmp/rzc-kat-child 2>/dev/null
chmod +x /tmp/rzc-kat-child
run_kat "$HERE/execve-kat.rs" 55 "K2 fork+execve+wait4"
echo "rzc KATs PASS (K1 syscall ABI + K2 byte-slice/execve process control)"
