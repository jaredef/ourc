#!/usr/bin/env bash
# semc — standalone SEM0 + LLVM Rust-subset compiler.
#   semc file.rs [-o out]   →   native executable (via SEM0 → rzir → r2l → llc → cc)
#
# Pieces come from a tower build (`SP=<dir> bash tower/build-tower.sh --gates`, from the hand-hex by default):
#   SEM0 = $SP/sem0.llvm.frozen (the C2-admitted LLVM mint), else $SP/sem0.frozen (the cf mint). Always the RAW
#          frozen: semc prepends //@S9 itself, and the stages' .cc wrappers would prepend a second one.
#   R2L  = $SP/r2l.elf (rzir2ll, compiled by the tower's own SEM0 -- minted by the llvm-parity / ourc gates).
#   llc  = the host's llc (or rustup's), cc = the host linker. These are the Tier-2 differential backend.
# Override any piece with SEM0= / R2L= / LLC=. SEMC_SP= is accepted as an alias for SP.
# SEMC_MODE=superset compiles in //@SUPERSET mode (crate = the file's basename) instead of //@S9: it carries the
# runtime fixes //@S9 leaves out to keep SEM0's self-emit byte-identical (e.g. fs::read_to_string's 16-byte String
# header). The tower's own tools (tower/driver/*.rs) are built this way.
# SEMC_RT=<file.rzir> appends a runtime rzir to SEM0's output before r2l (the cross-crate linker link.rs needs the
# t3-run-frontier runtime). SEMC_LDFLAGS adds flags to the final cc link (e.g. -lm). Unset = no change.
set -u
SP="${SEMC_SP:-${SP:-}}"
if [ -z "${SEM0:-}" ] && [ -n "$SP" ]; then
  SEM0="$SP/SEM0.llvm"; [ -x "$SEM0" ] || SEM0="$SP/SEM0.cf"
fi
R2L="${R2L:-${SP:+$SP/r2l.cf}}"
LLC="${LLC:-$(command -v llc 2>/dev/null || ls ~/.rustup/toolchains/*/lib/rustlib/*/bin/llc 2>/dev/null | head -1)}"
[ -n "${SEM0:-}" ] && [ -x "$SEM0" ] && [ -n "$R2L" ] && [ -x "$R2L" ] || {
  echo "semc: no SEM0/r2l.cf -- set SP to a tower build scratch dir (SP=<dir> bash tower/build-tower.sh --gates)," >&2
  echo "      or set SEM0= and R2L= explicitly" >&2; exit 2; }
[ -n "$LLC" ] || { echo "semc: llc not found (set LLC=)" >&2; exit 2; }
ulimit -s unlimited 2>/dev/null || true   # deep parse recursion on large inputs (self-host)
MC="$(dirname "$0")/memcap.sh"   # per-process cgroup memory cap (MEMCAP=48G default; MEMCAP=0 disables)
src="${1:?usage: semc file.rs [-o out]}"; out="a.out"
[ "${2:-}" = "-o" ] && out="${3:?-o needs a path}"
n=$(basename "$src" .rs); tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
{ if [ "${SEMC_MODE:-s9}" = superset ]; then printf '//@SUPERSET crate=%s\n' "$n"; else printf '//@S9\n'; fi; cat "$src"; } | "$MC" "$SEM0" > "$tmp/$n.rzir" 2>"$tmp/sem0.err" \
  || { echo "semc: SEM0 front-end failed" >&2; head -3 "$tmp/sem0.err" >&2; exit 1; }
if [ -n "${SEMC_RT:-}" ]; then [ -s "$SEMC_RT" ] || { echo "semc: SEMC_RT=$SEMC_RT is missing or empty" >&2; exit 2; }
  cat "$SEMC_RT" >> "$tmp/$n.rzir"; fi
"$MC" "$R2L" < "$tmp/$n.rzir" > "$tmp/$n.ll" 2>"$tmp/r2l.err" \
  || { echo "semc: rzir->LLVM (r2l) failed" >&2; head -3 "$tmp/r2l.err" >&2; exit 1; }
"$MC" "$LLC" -O2 -filetype=obj "$tmp/$n.ll" -o "$tmp/$n.o" 2>"$tmp/llc.err" \
  || { echo "semc: llc failed" >&2; tail -3 "$tmp/llc.err" >&2; exit 1; }
cc -no-pie "$tmp/$n.o" -o "$out" ${SEMC_LDFLAGS:-} 2>"$tmp/link.err" \
  || { echo "semc: link failed" >&2; head -5 "$tmp/link.err" >&2; exit 1; }
echo "semc: wrote $out"
