#!/usr/bin/env bash
# ourc-gate.sh -- the TOP of the source-language staircase: build `ourc` (19 crates we authored + the odrv
# driver) WITH THE TOWER'S OWN SEM0, and assert it still compiles real Rust to a binary that behaves as the
# committed golden says.
#
# Why this gate exists: ourc was built only by `ourc/build-ourc.sh`,
# which takes its frontend, its rzir->LLVM translator and its llc/cc from the ENVIRONMENT -- in practice a
# rustc-built SEM0 sitting in a scratch dir. So the tower could be green from the hand-hex up to SEM0 while
# nothing connected ourc to it, and a SEM0 regression that broke ourc was invisible here. This wires the same
# build to the tower's in-gate artifacts: frontend = the minted frozen ($SEM0_CC if a stage0 was promoted,
# else $SP/sem0.frozen), translator = $SP/r2l.elf (minted the way llvm-parity.sh mints it, reused if present).
#
# ORACLE-FREE: the claim is ourc's own behaviour on a committed fixture vs a committed golden, not a rustc run.
# SKIPS cleanly when llc/cc are absent (the ourc path emits LLVM IR, so it is host-gated the same way the
# bridge board is -- a capability gate, not a hex-rooted trust gate).
set -uo pipefail
# The frozen recurses deeply enough on real crates to blow the default 8MB stack -- it SEGVs mid-emit, leaving
# TRUNCATED rzir with no marker, which then links into a silent stub. The `.cc` wrappers the stages mint raise
# the limit for exactly this reason; a gate that calls the frozen directly must raise it too.
ulimit -s unlimited 2>/dev/null || ulimit -s 1048576 2>/dev/null || true
HERE="$(cd "$(dirname "$0")" && pwd)"; A="$HERE/.."; SP="${SP:?set SP}"
. "$A/lib/scratch.sh"; sp_seed || exit 1
ARC="$A/../ourc"
# The frontend must be the RAW frozen, never $SEM0_CC: the `.cc` artifacts the stages mint are WRAPPERS that
# prepend `//@S9` to their stdin (they exist to be "the compiler that compiles the dev source"), so every
# other directive line -- `//@METADATA`, `//@SUPERSET` -- stops being the first line and is ignored. Pointing
# this gate at the wrapper made ourc build with no metadata and no crate-tagged symbols, and the result
# segfaulted; the frozen itself honours every directive. Prefer the C2-admitted LLVM mint, else the cf mint.
FE="$SP/sem0.llvm.frozen"; [ -x "$FE" ] || FE="$SP/sem0.frozen"
[ -x "$FE" ] || { echo "  ourc-gate: SKIP (no frozen at $SP/sem0{.llvm,}.frozen -- run sem0-stage first)"; exit 0; }
LLC="$(command -v llc 2>/dev/null || ls ~/.rustup/toolchains/*/lib/rustlib/*/bin/llc 2>/dev/null | head -1)"
CCB="$(command -v cc 2>/dev/null || command -v gcc 2>/dev/null)"
[ -n "$LLC" ] && [ -n "$CCB" ] || { echo "  ourc-gate: SKIP (llc/cc not found; the ourc path is host-gated)"; exit 0; }

# 1. the rzir->LLVM translator, self-hosted (reuse llvm-parity.sh's if it already minted one this run)
if [ ! -x "$SP/r2l.elf" ]; then
  CF="$(sp_cf)"
  { echo '//@S9'; cat "$A/backend-llvm/rzir2ll.rs"; } | timeout 120 "$FE" > "$SP/r2l.rzir" 2>/dev/null \
    || { echo "  ourc-gate: FAIL (SEM0 could not lower rzir2ll.rs)"; exit 1; }
  grep -q UNLOWERED "$SP/r2l.rzir" && { echo "  ourc-gate: FAIL (rzir2ll.rs UNLOWERED)"; exit 1; }
  "$SP/ld.bin" < "$SP/r2l.rzir" | "$SP/opt.bin" | "$SP/ra.bin" | "$CF" > "$SP/r2l.elf" 2>/dev/null
  chmod +x "$SP/r2l.elf" 2>/dev/null
  [ -x "$SP/r2l.elf" ] || { echo "  ourc-gate: FAIL (translator did not link)"; exit 1; }
fi

# 1b. The frontend must answer the `//@METADATA` directive, because ourc's build is a metadata pass followed
# by a per-crate `//@SUPERSET` pass: a frontend that ignores the directive still LINKS and then segfaults,
# which is a crash whose cause is three steps upstream. Assert the directive is live before building, so the
# failure names itself. (This is what the first cold run actually hit -- pointed at the `//@S9`-prepending
# wrapper, not at the frozen.)
md=$({ echo "//@METADATA crate=ospan"; cat "$ARC/src/ospan.rs"; } | timeout 120 "$FE" 2>/dev/null | grep -c "^META ")
if [ "${md:-0}" -lt 1 ]; then
  echo "  ourc-gate: FAIL -- $FE emits 0 META lines for //@METADATA (expected the cross-crate table)."
  echo "             A frontend that ignores the directive builds an ourc that links and then crashes."; exit 1
fi

# 1c. ourc's linked rzir (~19 MB) is past the cf-built r2l's fixed arena, which SIGSEGVs at its end. Build ourc with
# the mmap-heap translator instead, the way ourc/release.sh does: rzir2ll.rs through semc.sh, translated by the
# bridge sem0-stage-llvm.sh leaves in $SP. Without the bridge, fall back to the cf-built r2l.elf above.
R2L="$SP/r2l.elf"
if [ -x "$SP/r2l.llvm.mmap.elf" ]; then
  SEM0="$FE" R2L="$SP/r2l.llvm.mmap.elf" LLC="$LLC" bash "$ARC/../scripts/semc.sh" "$A/backend-llvm/rzir2ll.rs" -o "$SP/r2l.llvm" > "$SP/.r2l.llvm.build" 2>&1 \
    || { echo "  ourc-gate: FAIL (could not build the mmap-heap r2l.llvm)"; tail -3 "$SP/.r2l.llvm.build"; exit 1; }
  R2L="$SP/r2l.llvm"
fi

# 2. build ourc through the tower -- all 19 crates + odrv, metadata pass, our linker, our backend
SEM0="$FE" R2L="$R2L" LLC="$LLC" OURC_OUT="$SP/ourc" bash "$ARC/build-ourc.sh" > "$SP/.ourc.build" 2>&1 \
  || { echo "  ourc-gate: FAIL (build-ourc.sh)"; tail -3 "$SP/.ourc.build"; exit 1; }
[ -s "$SP/ourc" ] || { echo "  ourc-gate: FAIL (no ourc binary)"; exit 1; }

# 2b. a truncated emit must not reach the linker: a crate whose rzir ends without `pool` was cut off mid-fn
# (the stack-overflow signature above), and the linked image is then a stub that runs and prints nothing
if grep -q "TRUNCATED" "$SP/.ourc.build" 2>/dev/null; then echo "  ourc-gate: FAIL (a crate's rzir was truncated)"; exit 1; fi

# 3. behaviour: ourc compiles the fixture, the emitted IR builds, and the program behaves == the golden
F="$HERE/fixture.rs"; G="$HERE/fixture.golden"
"$SP/ourc" "$F" --emit-ir 2>/dev/null | sed -n '/^\(@\|declare\|define\)/,$p' > "$SP/ourc.fx.ll"
[ -s "$SP/ourc.fx.ll" ] || { echo "  ourc-gate: FAIL (ourc emitted no IR for the fixture)"; exit 1; }
"$LLC" -O2 -filetype=obj "$SP/ourc.fx.ll" -o "$SP/ourc.fx.o" 2>/dev/null \
  && "$CCB" -no-pie "$SP/ourc.fx.o" -o "$SP/ourc.fx" -lm 2>/dev/null \
  || { echo "  ourc-gate: FAIL (ourc's IR does not build)"; exit 1; }
timeout 20 "$SP/ourc.fx" > "$SP/ourc.fx.out" 2>/dev/null; ec=$?
printf 'exit %d\n' "$ec" >> "$SP/ourc.fx.out"
cmp -s "$G" "$SP/ourc.fx.out" || { echo "  ourc-gate: FAIL (behaviour differs from the golden)"; diff "$G" "$SP/ourc.fx.out" | head -8; exit 1; }
echo "  ourc-gate: ok -- ourc built by the tower's own SEM0 (19 crates + odrv), fixture behaves == golden"
