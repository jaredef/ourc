#!/usr/bin/env bash
# sem0-stage-llvm.sh -- ONE-HOP LLVM mint of the frozen SEM0 binary (arc 2026-09-28-llvm-mint-promotion).
#
# WHY: the cf mint (sem0-stage.sh) needs a 26-stage staircase because a cf-BUILT frozen SIGSEGVs past ~N emit lines
# (the emit-capacity wall). An LLVM-BUILT frozen has no such wall -> it mints the full dev in ONE hop. This takes the
# hex-rooted cf bootstrap frozen ($SP/sem0.frozen from sem0-stage.sh), builds the canonical frozen via the rzir->LLVM
# bridge, and GUARDS it with the C2 differential: LLVM-frozen emit MUST equal the cf bootstrap emit, byte-identical,
# or the mint FAILS. cf stays the HEX ROOT; LLVM is trusted BY RESULT (the emit match), never by provenance.
#
# TRANSLATOR: built FROM SOURCE via the tower (cf frozen -> rzir2ll.rzir -> spine), NO rustc, NO pre-built seed.
# This became possible once rzir2ll's O(n^2) is_term arena-exhaustion was fixed (arc 2026-09-28-tower-rzir2ll-miscompile)
# and its output was made STREAMING (print per-function, not accumulate) so a 731k self-emit fits the bump arena.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; A="$HERE/.."
SP="${SP:?set SP}"; CC="${CC:?set CC}"
ulimit -s unlimited 2>/dev/null || ulimit -s 1048576 2>/dev/null || true   # T36: s9_init_ty's deep else-if chain overflows the default 8MB stack when the LLVM-minted frozen emits it; the frozen is byte-identical-correct under an adequate stack (substrate-traps.md T36 -- "C2 gates must set ulimit -s unlimited").
DEV="$A/sem0/stages/S1-lex-parse-ast/s1_parser.rs"
. "$A/lib/scratch.sh"   # sp_name
FE="$SP/SEM0.cf"
[ -x "$FE" ]  || { echo "sem0-stage-llvm: SKIP (no cf bootstrap $SP/SEM0.cf)"; exit 0; }
LLC="$(command -v llc 2>/dev/null || ls ~/.rustup/toolchains/*/lib/rustlib/*/bin/llc 2>/dev/null | head -1)"
CCB="$(command -v cc 2>/dev/null || command -v gcc 2>/dev/null)"
[ -n "$LLC" ] && [ -n "$CCB" ] || { echo "sem0-stage-llvm: SKIP (llc/cc not found)"; exit 0; }
back(){ "$SP/B.ld" 2>/dev/null | "$CC/opt/rzir-opt.bin" 2>/dev/null | "$CC/ra/rzir-ra.bin" 2>/dev/null | "$SP/B.cf.hoist" > "$1" 2>/dev/null; chmod +x "$1" 2>/dev/null; }
cc9(){ local t; t="$(mktemp)"; { echo '//@S9'; cat; } > "$t"; "$1" < "$t"; local r=$?; rm -f "$t"; return $r; }

# 1. build the bridge translator FROM SOURCE via the tower (cf frozen -> rzir -> spine). No rustc, no seed.
cc9 "$FE" < "$HERE/rzir2ll.rs" > "$SP/r2l.rzir" 2>/dev/null
grep -q UNLOWERED "$SP/r2l.rzir" && { echo "sem0-stage-llvm: FAIL (rzir2ll UNLOWERED)"; exit 1; }
back "$SP/r2l.cf" < "$SP/r2l.rzir"
[ -x "$SP/r2l.cf" ] || { echo "sem0-stage-llvm: FAIL (translator build)"; exit 1; }; sp_name r2l.cf r2l.elf

# 1b. PURE two-stage bootstrap (no rustc): the cf-built r2l handles small inputs but a BSS __heap caps it at ~3GB
#     (-no-pie) -- it crashes on the 751k self-image. Use it to build an MMAP-HEAP translator (the `gdef __mmapheap`
#     sentinel makes rzir2ll emit __heap as a runtime mmap region, no BSS cap), which DOES handle the self-image.
#     r2l is 64-bit-addr-clean, so the high mmap heap is safe for it (unlike the default, kept BSS for other progs).
{ echo 'gdef __mmapheap 1'; cat "$SP/r2l.rzir"; } | "$SP/r2l.cf" > "$SP/r2l.mmap.ll" 2>/dev/null
"$LLC" -O0 -filetype=obj "$SP/r2l.mmap.ll" -o "$SP/r2l.mmap.o" 2>/dev/null || { echo "sem0-stage-llvm: FAIL (mmap-translator llc)"; exit 1; }
"$CCB" -no-pie "$SP/r2l.mmap.o" -o "$SP/r2l.llvm.mmap.elf" 2>/dev/null || { echo "sem0-stage-llvm: FAIL (mmap-translator link)"; exit 1; }
chmod +x "$SP/r2l.llvm.mmap.elf"
[ -x "$SP/r2l.llvm.mmap.elf" ] || { echo "sem0-stage-llvm: FAIL (mmap-translator build)"; exit 1; }; sp_name r2l.llvm.mmap.elf r2l.mmap.elf

# 2. one-hop LLVM mint: cf-bootstrap emits the dev self-image, the MMAP tower-built translator -> llc -> link
cc9 "$FE" < "$DEV" > "$SP/self.rz" 2>/dev/null
[ -s "$SP/self.rz" ] || { echo "sem0-stage-llvm: FAIL (empty self-emit)"; exit 1; }
timeout 2400 "$SP/r2l.llvm.mmap.elf" < "$SP/self.rz" > "$SP/self.ll" 2>/dev/null || { echo "sem0-stage-llvm: FAIL (bridge xlate)"; exit 1; }
"$LLC" -O0 -filetype=obj "$SP/self.ll" -o "$SP/self.o" 2>/dev/null || { echo "sem0-stage-llvm: FAIL (llc)"; exit 1; }
"$CCB" -no-pie "$SP/self.o" -o "$SP/SEM0.llvm" 2>/dev/null || { echo "sem0-stage-llvm: FAIL (link)"; exit 1; }
cat > "$SP/SEM0.llvm.s9" <<WRAP
#!/bin/sh
ulimit -s unlimited 2>/dev/null || ulimit -s 1048576 2>/dev/null || true
t="\$(mktemp)"; { echo '//@S9'; cat; } > "\$t"; "$SP/SEM0.llvm" < "\$t"; r=\$?; rm -f "\$t"; exit \$r
WRAP
chmod +x "$SP/SEM0.llvm.s9"; sp_name SEM0.llvm sem0.llvm.frozen; sp_name SEM0.llvm.s9 sem0.llvm.cc

# 3. GUARD C2 (differential vs cf bootstrap) + C4 (fixpoint)
cc9 "$SP/SEM0.llvm" < "$DEV" > "$SP/self.viallvm.rz" 2>/dev/null
cmp -s "$SP/self.viallvm.rz" "$SP/self.rz" || { echo "sem0-stage-llvm: FAIL C2/C4 -- LLVM-frozen emit != cf bootstrap emit (DIVERGED)"; exit 1; }
echo "sem0-stage-llvm: OK -- translator built from source (no rustc/seed); SEM0.llvm minted in ONE HOP; emit == cf (C2) + fixpoint (C4) [$(wc -l < "$SP/self.rz") rzir lines]"
