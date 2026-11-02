#!/usr/bin/env bash
# assemble.sh -- build a self-contained derived-toolchain directory: rzc + its 5 stage binaries as siblings.
# After this, `<dir>/rzc build foo.rs foo.elf` works from anywhere -- rzc finds its stages via /proc/self/exe.
#
#   SP=<scratch with the mainline frontend rzrs.self> bash assemble.sh <out-dir>
#
# Layout produced:
#   <out-dir>/rzc         the build driver
#   <out-dir>/rzrs.self   the Rust-subset frontend  (rzrs -> rzir)
#   <out-dir>/rzir-ld     linker      \
#   <out-dir>/rzir-opt    optimizer    }  the rzir -> rzir middle pipeline
#   <out-dir>/rzir-ra     regalloc    /
#   <out-dir>/cf          backend     (rzir -> ELF; the K1-extended cf.rs build)
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; A="$(cd "$HERE/../.." && pwd)"; CC="$A/L2"
SP="${SP:?set SP (must contain the mainline frontend rzrs.self)}"
OUT="${1:?usage: SP=<scratch> bash assemble.sh <out-dir>}"
MC="$A/L0"
"$MC/m0/m0.bin" < "$MC/emitter/rzir-to-elf-cf.m0" > "$SP/cf.m0" 2>/dev/null; chmod +x "$SP/cf.m0"
chain(){ "$CC/link/rzir-ld.bin" | "$CC/opt/rzir-opt.bin" | "$CC/ra/rzir-ra.bin" | "$1"; }

mkdir -p "$OUT"
# hermetic: clear the six managed binaries so a stale one can never survive a rebuild (a stale rzrs.self still
# builds good programs, so the smoke test alone would not catch it -- discovered via R0.5)
rm -f "$OUT/rzc" "$OUT/rzrs.self" "$OUT/rzir-ld" "$OUT/rzir-opt" "$OUT/rzir-ra" "$OUT/cf"
# backend cf (from cf.rs, with the K1 syscall ABI extension) -- built first and made executable, since it is
# then used as the backend to lower rzc itself
"$SP/rzrs.self" < "$A/backend-cf/cf.rs" 2>/dev/null | chain "$SP/cf.m0" > "$OUT/cf" 2>/dev/null; chmod +x "$OUT/cf"
"$SP/rzrs.self" < "$HERE/rzc.rs"                       2>/dev/null | chain "$OUT/cf"   > "$OUT/rzc" 2>/dev/null
cp "$SP/rzrs.self" "$OUT/rzrs.self"
cp "$CC/link/rzir-ld.bin" "$OUT/rzir-ld"; cp "$CC/opt/rzir-opt.bin" "$OUT/rzir-opt"; cp "$CC/ra/rzir-ra.bin" "$OUT/rzir-ra"
chmod +x "$OUT"/*
for f in rzc rzrs.self rzir-ld rzir-opt rzir-ra cf; do [ -s "$OUT/$f" ] || { echo "FAIL: $f missing/empty"; exit 1; }; done

# smoke test through the assembled toolchain
printf 'fn main()->i32{ let mut s=0; let mut i=1; while i<=10 { s=s+i; i=i+1; } s }' > "$OUT/.smoke.rs"
"$OUT/rzc" build "$OUT/.smoke.rs" "$OUT/.smoke.elf" >/dev/null 2>&1 && "$OUT/.smoke.elf"; rc=$?
rm -f "$OUT/.smoke.rs" "$OUT/.smoke.elf"
[ "$rc" = 55 ] || { echo "FAIL: assembled toolchain smoke test returned $rc (want 55)"; exit 1; }
echo "toolchain assembled at $OUT ($(du -sh "$OUT" | cut -f1)); smoke test == 55"
echo "  try:  $OUT/rzc build foo.rs foo.elf   |   $OUT/rzc run foo.rs   |   $OUT/rzc test foo.rs"
