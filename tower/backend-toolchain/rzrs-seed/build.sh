#!/usr/bin/env bash
# build.sh -- reconstruct rzrs.rsz (the C-seed Rust frontend) from the HAND-HEX, cc/ld-free, WITHOUT the
# derived tcc (which roots in external tinycc). Path: rzcpp (hex-rooted) preprocesses rzrs.c against MINIMAL
# freestanding headers; a freestanding shim (shim.c) supplies the small libc surface (printf/fprintf/fwrite/
# fread/read/strlen/strncmp/strcmp/memcpy/exit, no malloc) via __syscall + ccc's __va builtins; ccc compiles
# the combined source; ld+cf lower it. Verifies the result == the committed rzrs.rsz on the frozen seed.
#
# STATUS: UNBLOCKED. A pin-art series of three ccc.c miscompiles was root-caused and fixed (all self-repro-
# neutral): Bug 1 global char* string literals (prescan/parse_init/parse_string string-awareness); Bug 2
# `(TYPE)e;` cast-statements mis-routed to a NULL faddr (parse_stmt cast handler); Bug 3 `long v>2147483647L`
# emitted a 32-bit `gt` (signed relational < <= > >= had no *8 promotion -- R162, mirrors R160 equality). The
# hex-rooted ccc now compiles the combined shim+rzrs.c cleanly; tier-0 == the reference (sum(1..5)=15, works).
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; ROOT="$(cd "$HERE/../../.." && pwd)"
CC="$ROOT/tower/L2"; TCC="$ROOT/tower/tcc"
RZRSC="$ROOT/tower/L3/rzrs.c"
SEED="$ROOT/tower/L4/L4.rzrs.rs"
W="${SP:?set SP}/rzrs-seed"; rm -rf "$W"; mkdir -p "$W"
MC="$ROOT/tower/L0"
"$MC/m0/m0.bin" < "$MC/emitter/rzir-to-elf-cf.m0" > "$W/cf" 2>/dev/null; chmod +x "$W/cf"; CF="$W/cf"
chain(){ "$CC/link/rzir-ld.bin" | "$CC/opt/rzir-opt.bin" | "$CC/ra/rzir-ra.bin" | "$CF"; }
"$CC/ccc.bin" < "$TCC/rzcpp.c" | chain > "$W/rzcpp" 2>/dev/null; chmod +x "$W/rzcpp"     # rzcpp = cf(ccc(rzcpp.c))
"$W/rzcpp" -I"$HERE/headers" < "$RZRSC" > "$W/rzrs.pp.c" 2>/dev/null                     # preprocess against minimal headers
cat "$HERE/shim.c" "$W/rzrs.pp.c" > "$W/combined.c"                                      # + freestanding shim
if "$CC/ccc.bin" < "$W/combined.c" | chain > "$W/rzrs.rsz" 2>/dev/null && chmod +x "$W/rzrs.rsz" && [ -s "$W/rzrs.rsz" ]; then
  "$W/rzrs.rsz" < "$SEED" > "$W/t0.hex.rzir" 2>/dev/null
  "$SP/rzrs.rsz" < "$SEED" > "$W/t0.ref.rzir" 2>/dev/null
  cmp -s "$W/t0.hex.rzir" "$W/t0.ref.rzir" && echo "rzrs.rsz RECONSTRUCTED FROM HAND-HEX (byte-identical tier-0)" || echo "built but tier-0 differs"
else
  echo "FAILED: ccc could not compile the combined shim+rzrs.c (see header -- the 3 pin-art bugs are fixed; a NEW miscompile would land here)."
fi
