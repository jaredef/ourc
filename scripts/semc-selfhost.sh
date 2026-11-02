#!/usr/bin/env bash
# semc self-host gate: sem0.semc (SEM0 compiled to native via the LLVM backend) reproduces
# sem0.head's emit BYTE-IDENTICALLY — on small programs AND on its own 28k-line source (the fixpoint).
# Exit 0 = self-host verified. (R6/sem0-stage-llvm.sh grounds this in the hex-rooted cf bootstrap
# instead of the rustc-built sem0.head.)
set -u
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
WORK="${SEMC_DIR:-$(mktemp -d)}"
DEV="$ROOT/tower/sem0/stages/S1-lex-parse-ast/s1_parser.rs"
R2L_BOOT="${R2L:-$WORK/r2l.elf}"
LLC="${LLC:-$HOME/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/llc}"
RZIR2LL="$ROOT/tower/backend-llvm/rzir2ll.rs"
T="$(mktemp -d)"; trap 'rm -rf "$T"' EXIT
fail(){ echo "semc-selfhost: FAIL — $1" >&2; exit 1; }
ulimit -s unlimited 2>/dev/null || true
[ -x "$LLC" ] || fail "llc not found ($LLC)"
echo "semc-selfhost: [1/5] build sem0.head (rustc -O s1_parser.rs)"
rustc -O "$DEV" -o "$T/sem0.head" 2>/dev/null || fail "sem0.head build"
echo "semc-selfhost: [2/5] rebuild mmap-heap translator r2l.mmap.elf from rzir2ll.rs"
[ -x "$R2L_BOOT" ] || fail "bootstrap translator r2l.elf not found ($R2L_BOOT) — set R2L="
{ printf '//@S9\n'; cat "$RZIR2LL"; } | "$T/sem0.head" > "$T/r2l.rzir" 2>/dev/null || fail "rzir2ll emit"
"$R2L_BOOT" < "$T/r2l.rzir" > "$T/r2l2.ll" 2>/dev/null || fail "bootstrap-translate rzir2ll"
"$LLC" -O0 -filetype=obj "$T/r2l2.ll" -o "$T/r2l2.o" 2>/dev/null && cc -no-pie "$T/r2l2.o" -o "$T/r2l.mmap.elf" 2>/dev/null && chmod +x "$T/r2l.mmap.elf" || fail "r2l.mmap build"
echo "semc-selfhost: [3/5] sem0.head emits its own source -> self.rz"
{ printf '//@S9\n'; cat "$DEV"; } | "$T/sem0.head" > "$T/self.rz" 2>/dev/null || fail "self-emit"
echo "semc-selfhost: [4/5] mmap-r2l -> llc -> cc  ->  sem0.semc ($(wc -l <"$T/self.rz") rzir lines)"
"$T/r2l.mmap.elf" < "$T/self.rz" > "$T/self.ll" 2>/dev/null || fail "self xlate"
"$LLC" -O0 -filetype=obj "$T/self.ll" -o "$T/self.o" 2>/dev/null || fail "self llc"
cc -no-pie "$T/self.o" -o "$T/sem0.semc" 2>/dev/null && chmod +x "$T/sem0.semc" || fail "self link"
echo "semc-selfhost: [5/5] verify fixpoint + small-program parity"
printf 'fn main(){ let x=1+2; println!("{}",x*3); }' > "$T/p.rs"
{ printf '//@S9\n'; cat "$T/p.rs"; } | "$T/sem0.semc" > "$T/p.semc.rz" 2>/dev/null
{ printf '//@S9\n'; cat "$T/p.rs"; } | "$T/sem0.head" > "$T/p.head.rz" 2>/dev/null
cmp -s "$T/p.semc.rz" "$T/p.head.rz" || fail "sem0.semc != sem0.head on a small program"
{ printf '//@S9\n'; cat "$DEV"; } | "$T/sem0.semc" > "$T/self.viasemc.rz" 2>/dev/null || fail "sem0.semc crashed on the self-image"
cmp -s "$T/self.viasemc.rz" "$T/self.rz" || fail "FIXPOINT — sem0.semc(s1_parser) != sem0.head(s1_parser)"
echo "semc-selfhost: OK — sem0.semc reproduces sem0.head byte-identically (small + $(wc -l <"$T/self.rz")-line self-image fixpoint)"
