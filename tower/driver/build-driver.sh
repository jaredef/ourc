#!/usr/bin/env bash
# build-driver.sh -- build the ONEBIN driver `tower-rustc` (rustc's own compiler crates, compiled by SEM0 and
# linked into one binary) from IN-REPO scripts, with the TOWER-BUILT SEM0 as the emitter.
#
#   SP=<tower scratch> [RUSTC_SRC=<rustc-X.Y.Z-src>] bash tower/driver/build-driver.sh [PHASE..]
#
#   PHASES (default: all, in order)
#     tools    build splice + genstubs + link with semc (+ PROVENANCE) -> $DRV/tools/
#     ingest   splice each crate into one file           -> $DRV/Lingest/<crate>.rs
#     meta     //@METADATA pass over every crate          -> $DRV/Lingest/all.meta
#     emit     //@SUPERSET pass (+ completeness check)    -> $DRV/Lingest/<crate>.ss.rz
#     link     tools/link over the combined crates        -> $DRV/linked_drv.rz
#     lower    genstubs -> r2l -> llc -O0 -> cc           -> $DRV/tower-rustc
#     smoke    run tower-rustc on hello.rs                -> exit code + where it stopped
#
# Rebuilds what the 2026-10-22 full-driver arc ran from an untracked sidecar ($J, a scratch dir:
# refresh_tower.sh / reemit.sh / drvgauge.sh / buildtower.sh / fix_trunc.sh), which was lost with that machine.
# Recipe sources: apparatus/arcs/2026-10-22-full-driver/HANDOFF.md, 2026-10-13-semc-rustc-testsuite/
# drvgauge-semc.sh, backend-llvm/buildtower-llvm.sh, 2026-10-03-plane-p-provisioning/roundtrip.sh.
#
# Differences from the sidecar, on purpose:
#   * EMITTER = $SP/sem0.llvm.frozen, semc after it compiles itself (gate-checked, see below) -- hex-rooted via C2
#     -- not sem0.head (= `rustc -O s1_parser.rs`). EMITTER=<binary> overrides (e.g. a native sem0.head, to
#     tell an emitter difference from a source difference).
#   * the HANDOFF gotchas are checks, not folklore: every phase deletes its outputs before running (no stale
#     drv.o / tower-rustc can survive a failed step); llc runs -O0 with a 1800s timeout and a timeout is fatal;
#     every .ss.rz is completeness-checked (ends in `gdef __heap_cursor 8`, no goto without its label) and a
#     failed crate is retried serially before the phase fails; linked_drv.rz < 50k lines is fatal.
#   * no Python: the splicer and the stub generator are Rust (tower/driver/splice.rs, genstubs.rs), compiled by
#     semc with this same tower-built SEM0 (//@SUPERSET mode). They replace census/splice-crate.py and
#     backend-llvm/driver-genstubs.py (byte-identical on all 80 crates of 1.99.1; genstubs adds rung219).
#   * EVERY tool is built by semc, through scripts/semc.sh: splice + genstubs (//@SUPERSET) and the cross-crate
#     linker link.rs (//@S9 + SEMC_RT=the t3-run runtime, -lm) -- no hand-copied SEM0->r2l->llc->cc chain.
#     tools/PROVENANCE records the sha256 of the emitter, r2l.elf and each tool + source; every later phase
#     re-checks it and refuses tools built by a different emitter/r2l. A rustc-built emitter (`rustc version` in
#     its .comment, e.g. sem0.head) is refused unless ALLOW_NATIVE_EMITTER=1 (diagnostics only).
#   * the crate list is DERIVED from $RUSTC_SRC/compiler/*/src/lib.rs (+ getopts + lib_{core,alloc,std}),
#     never hard-coded, so moving the source version moves the list with it.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; A="$HERE/.."; ROOT="$A/.."
SP="${SP:?set SP (tower scratch: sem0.frozen + r2l.elf + ld/opt/ra; run build-tower.sh --gates first)}"
RUSTC_SRC="${RUSTC_SRC:-$HOME/Developer/rustc-src/rustc-1.99.1-src}"
DRV="${DRV:-$SP/driver}"; L="$DRV/Lingest"
MEMCAP="${MEMCAP:-48G}"; export MEMCAP   # per-process cap (scripts/memcap.sh)
# The pool budget comes from what the machine can actually spare, NOT from MemTotal: /tmp is tmpfs here (its files
# are RAM) and the desktop needs its share. A fixed 100G on a 125G box drove the KERNEL (not the pool) to global
# OOM on 2026-10-26 and it killed the user's editor. Default: MemAvailable at start minus MEMCAP_HEADROOM_G (32: zram + desktop), max 100G.
MEMCAP_HEADROOM_G="${MEMCAP_HEADROOM_G:-32}"
if [ -z "${MEMCAP_BUDGET:-}" ]; then
  avail_g=$(( $(awk '/^MemAvailable:/{print $2}' /proc/meminfo) / 1048576 ))
  b=$(( avail_g - MEMCAP_HEADROOM_G )); [ "$b" -gt 100 ] && b=100
  [ "$b" -lt 8 ] && { echo "build-driver: only ${avail_g}G available; free memory or set MEMCAP_BUDGET" >&2; exit 1; }
  MEMCAP_BUDGET="${b}G"
fi
# meta + emit are embarrassingly parallel (each crate reads only its own source + all.meta). Every job sits in ONE
# shared pool (the rzdrv slice) capped at MEMCAP_BUDGET, and is ADMITTED BY MEMORY (admit_run): a crate starts only
# when the running jobs' expected peaks plus its own fit the budget, so the big crates (largest-first) run a few at a
# time and the small tail fills up to JOBS (nproc-4) cores. Expected peak = the crate's last recorded peak in
# PEAK_HIST (kept across scratch dirs) x 1.25, or PEAK_DEFAULT_G for a crate never seen. A count-only cap (28 jobs)
# started 28 big crates at once and drove the machine to global OOM (2026-10-26). A job that still overflows the
# pool (rc 137/143) is redone alone by the serial retry; MEMCAP stays each job's runaway guard.
MEMCAP_POOL_NAME=rzdrv
PEAK_HIST="${PEAK_HIST:-$HOME/.cache/rusteze-peaks}"; PEAK_DEFAULT_G="${PEAK_DEFAULT_G:-6}"
if [ -z "${JOBS:-}" ]; then JOBS=$(( $(nproc) - 4 )); [ "$JOBS" -lt 1 ] && JOBS=1; fi
CRATE_TIMEOUT="${CRATE_TIMEOUT:-1800}"
MEMCAP_SH="$ROOT/scripts/memcap.sh"; SEMC="$ROOT/scripts/semc.sh"; T="$DRV/tools"; GETOPTS="$A/vendor/getopts-0.2.24-lib.rs"
LINK_RS="$ROOT/ourc/rt/link.rs"
RT="$ROOT/ourc/rt/runtime.rzir"
ulimit -s unlimited 2>/dev/null || ulimit -s 1048576 2>/dev/null || true   # the frozen recurses deep on real crates
die(){ echo "FAIL: $*" >&2; exit 1; }
say(){ echo "  $*"; }

EMITTER="${EMITTER:-}"
# The default emitter is SEMC AFTER IT COMPILES ITSELF: $SP/sem0.llvm.frozen, minted by build-tower.sh's
# sem0-stage-llvm gate (current s1_parser.rs through the semc pipeline r2l -> llc -> cc, guarded by C2 emit == the
# hex-rooted cf frozen and C4 self-emit fixpoint). No silent fallback to the cf-built sem0.frozen: the gate's OK
# line must exist and post-date the binary. ALLOW_UNGATED_EMITTER=1 skips that check (diagnostics only).
if [ -z "$EMITTER" ]; then
  EMITTER="$SP/sem0.llvm.frozen"
  [ -x "$EMITTER" ] || die "no $EMITTER -- run build-tower.sh --gates (sem0-stage-llvm mints it; the cf sem0.frozen is NOT used)"
  if [ "${ALLOW_UNGATED_EMITTER:-0}" != 1 ]; then
    glog=$(ls "$SP"/.log.sem0_stage_llvm* 2>/dev/null | head -1)
    [ -n "$glog" ] && grep -q '^sem0-stage-llvm: OK' "$glog" && [ ! "$EMITTER" -nt "$glog" ] \
      || die "$EMITTER has no passing sem0-stage-llvm gate log in $SP (C2 + C4 fixpoint) -- run build-tower.sh --gates"
  fi
fi
[ -x "$EMITTER" ] || die "emitter $EMITTER is not executable"
if readelf -p .comment "$EMITTER" 2>/dev/null | grep -q 'rustc version'; then
  [ "${ALLOW_NATIVE_EMITTER:-0}" = 1 ] || die "emitter $EMITTER is rustc-built (native); the driver must be built by the tower SEM0. ALLOW_NATIVE_EMITTER=1 to override (diagnostics only)"
  echo "WARNING: native (rustc-built) emitter $EMITTER -- this build is a DIAGNOSTIC, not a tower build" >&2
fi
[ -x "$SP/r2l.elf" ] || die "no $SP/r2l.elf -- run build-tower.sh --gates (llvm-parity/ourc mint it)"
[ -d "$RUSTC_SRC/compiler" ] || die "no rustc source at $RUSTC_SRC"
LLC="${LLC:-$(command -v llc 2>/dev/null || ls ~/.rustup/toolchains/*/lib/rustlib/*/bin/llc 2>/dev/null | head -1)}"
[ -n "$LLC" ] || die "llc not found (set LLC=)"
command -v cc >/dev/null || die "cc not found"
mkdir -p "$L"

crates_compiler(){ ls -d "$RUSTC_SRC"/compiler/*/src/lib.rs | awk -F/ '{print $(NF-2)}' | LC_ALL=C sort; }
crates_all(){ { crates_compiler; echo getopts; echo lib_alloc; echo lib_core; echo lib_std; } | LC_ALL=C sort; }
src_of(){ case "$1" in getopts) echo "$GETOPTS";; lib_*) echo "$RUSTC_SRC/library/${1#lib_}/src/lib.rs";;
                        *) echo "$RUSTC_SRC/compiler/$1/src/lib.rs";; esac; }
# a complete SUPERSET emit ends in the arena footer and has a label for every goto (a truncated emit -- timeout,
# OOM, SEGV mid-write -- leaves dangling gotos and looks EXACTLY like an emitter bug downstream)
ss_complete(){ local f="$1"; [ -s "$f" ] || return 1
  tail -n 5 "$f" | grep -q '^gdef __heap_cursor 8' || return 1
  awk '{for(i=1;i<NF;i++){ if($i=="goto") g[$(i+1)]=1 } if($1=="label") l[$2]=1}
       END{for(k in g) if(k ~ /^__L[mi][0-9]+$/ && !(k in l)) {bad=1; exit} exit bad}' "$f"; }
export -f ss_complete
# largest spliced source first, so the big crates start early instead of stretching the tail
largest_first(){ while read -r cr; do echo "$(stat -c%s "$L/$cr.rs" 2>/dev/null || echo 0) $cr"; done < "$1" | sort -rn | cut -d' ' -f2; }
# a pool-killed job reports 137 (SIGKILL) or 143 (systemd stopping the scope); both mean "out of memory"
is_oom_rc(){ [ "$1" = 137 ] || [ "$1" = 143 ]; }
pool_open(){ "$MEMCAP_SH" --pool "$MEMCAP_POOL_NAME" "$MEMCAP_BUDGET"; export MEMCAP_POOL="$MEMCAP_POOL_NAME"; }
pool_peaks(){ [ -s "$1" ] && say "peak memory, top 5: $(sort -k2 -rn "$1" | head -5 | awk '{printf "%s %.1fG  ", $1, $2/2^30}')"; }
g2b(){ echo $(( ${1%G} * 1073741824 )); }   # "82G" -> bytes
# admit_run PHASE CMD LIST -- run CMD (bash -c, crate as $1) over LIST largest-first, admitting by expected memory.
admit_run(){
  local phase="$1" cmd="$2" list="$3" hist="$PEAK_HIST/$1" budget used=0 cr e pid
  budget=$(g2b "$MEMCAP_BUDGET"); mkdir -p "$PEAK_HIST"; touch "$hist"
  local -A run=()
  for cr in $(largest_first "$list"); do
    e=$(awk -v c="$cr" '$1==c{p=$2} END{print p+0}' "$hist")
    [ "$e" -gt 0 ] && e=$(( e * 5 / 4 )) || e=$(g2b "${PEAK_DEFAULT_G}G")
    [ "$e" -gt "$budget" ] && e=$budget
    while [ "${#run[@]}" -ge "$JOBS" ] || { [ "${#run[@]}" -gt 0 ] && [ $(( used + e )) -gt "$budget" ]; }; do
      pid=""; wait -n -p pid; [ -n "$pid" ] && [ -n "${run[$pid]:-}" ] && { used=$(( used - run[$pid] )); unset "run[$pid]"; }
    done
    bash -c "$cmd" _ "$cr" & run[$!]=$e; used=$(( used + e ))
  done
  wait
}
# fold this phase's successful peaks into the history (latest wins; a killed job's peak is only a lower bound, so it
# is kept only when larger than what the history has)
peaks_save(){ local hist="$PEAK_HIST/$1" log="$2"; [ -s "$log" ] || return 0; mkdir -p "$PEAK_HIST"; touch "$hist"
  awk 'FNR==NR{h[$1]=$2; next} $2 ~ /^[0-9]+$/ { if ($3==0 || $2+0 > h[$1]+0) h[$1]=$2 } END{for (k in h) print k, h[k]}' \
    "$hist" "$log" | LC_ALL=C sort > "$hist.tmp" && mv "$hist.tmp" "$hist"; }

sha(){ sha256sum "$1" | cut -d' ' -f1; }
# the tools must have been built by THIS emitter + r2l (a stale tools/ from another SP or emitter is refused)
need_tools(){ local p="$T/PROVENANCE" t
  [ -s "$p" ] || die "no $p -- run the tools phase"
  grep -qx "emitter $(sha "$EMITTER") $EMITTER" "$p" || die "tools were built by a different emitter -- re-run the tools phase"
  grep -qx "r2l $(sha "$SP/r2l.elf") $SP/r2l.elf" "$p" || die "tools were built by a different r2l.elf -- re-run the tools phase"
  for t in "$@"; do [ -x "$T/$t" ] && grep -q "^tool $t $(sha "$T/$t") " "$p" || die "tools/$t missing or not the semc-built binary -- re-run the tools phase"; done; }

phase_tools(){
  echo "== tools (splice + genstubs + link, compiled by semc with the tower-built SEM0) =="
  rm -rf "$T"; mkdir -p "$T"
  local t src mode rt lib
  for t in splice genstubs link; do
    src="$HERE/$t.rs"; mode=superset; rt=""; lib=""
    [ "$t" = link ] && { src="$LINK_RS"; mode=s9; rt="$RT"; lib="-lm"; }   # link.rs: //@S9 + the t3-run runtime
    SEMC_MODE="$mode" SEMC_RT="$rt" SEMC_LDFLAGS="$lib" SEM0="$EMITTER" R2L="$SP/r2l.elf" LLC="$LLC" \
      bash "$SEMC" "$src" -o "$T/$t" > "$T/.$t.log" 2>&1 || die "semc could not build $t: $(tail -2 "$T/.$t.log")"
  done
  { echo "emitter $(sha "$EMITTER") $EMITTER"; echo "r2l $(sha "$SP/r2l.elf") $SP/r2l.elf"; echo "llc $LLC"
    for t in splice genstubs link; do src="$HERE/$t.rs"; [ "$t" = link ] && src="$LINK_RS"
      echo "tool $t $(sha "$T/$t") src $(sha "$src") $(realpath --relative-to="$ROOT" "$src")"; done; } > "$T/PROVENANCE"
  say "built by semc ($(basename "$EMITTER") $(sha "$EMITTER" | cut -c1-12)): $(ls "$T" | tr '\n' ' ')"
}

phase_ingest(){
  echo "== ingest ($(crates_all | wc -l) crates from $RUSTC_SRC) =="
  need_tools splice
  rm -f "$L"/*.rs
  find "$RUSTC_SRC/compiler" "$RUSTC_SRC/library" -name '*.rs' > "$DRV/files.txt"
  local n=0
  for cr in $(crates_all); do
    local s; s="$(src_of "$cr")"; [ -f "$s" ] || die "missing source for $cr: $s"
    # the splicer reads its arguments and the reachable-file list from stdin (SEM0-built binaries cannot read argv
    # or test file existence yet); #[path] can leave the crate (core -> stdarch), so list the whole source tree
    { echo "$s"; echo tests=0; cat "$DRV/files.txt"; if [ "$cr" = getopts ]; then echo "$s"; fi; } \
      | "$T/splice" > "$L/$cr.rs" 2>"$L/.$cr.splice.err" || die "splice failed for $cr ($(head -1 "$L/.$cr.splice.err"))"
    n=$((n+1))
  done
  crates_all > "$DRV/crates.txt"
  say "spliced $n crates ($(cat "$L"/*.rs | wc -l) lines) -> $L"
}

phase_meta(){
  echo "== meta (//@METADATA, up to $JOBS parallel, admitted by memory, pool $MEMCAP_BUDGET) =="
  rm -f "$L/all.meta" "$L"/*.meta1 "$L/.peak.meta"
  export EMITTER L CRATE_TIMEOUT MEMCAP_SH; export MEMCAP_PEAKLOG="$L/.peak.meta"
  # each crate's records are independent of the others (metadata mode reads no prior meta), so emit in
  # parallel and concatenate in the fixed sorted order -- bare keys are last-writer-wins, so order matters
  meta_one='ulimit -s unlimited 2>/dev/null; export MEMCAP_TAG="$1"
    { echo "//@METADATA crate=$1"; cat "$L/$1.rs"; } | timeout "$CRATE_TIMEOUT" "$MEMCAP_SH" "$EMITTER" 2>/dev/null \
      | sed "s/^META /\/\/META /" | grep "^//META " > "$L/$1.meta1"; true'
  pool_open
  admit_run meta "$meta_one" "$DRV/crates.txt"
  local empty=""; for cr in $(cat "$DRV/crates.txt"); do [ -s "$L/$cr.meta1" ] || empty="$empty $cr"; done
  if [ -n "$empty" ]; then   # an empty .meta1 is a killed/failed job (pool OOM or crash): redo it alone, whole budget
    say "no META after the parallel pass:$empty -- retrying SERIALLY outside the pool"
    for cr in $empty; do env -u MEMCAP_POOL MEMCAP="$MEMCAP_BUDGET" bash -c "$meta_one" _ "$cr"; done
  fi
  pool_peaks "$L/.peak.meta"; peaks_save meta "$L/.peak.meta"; unset MEMCAP_POOL MEMCAP_PEAKLOG
  empty=""
  for cr in $(cat "$DRV/crates.txt"); do [ -s "$L/$cr.meta1" ] || empty="$empty $cr"; cat "$L/$cr.meta1" >> "$L/all.meta"; done
  rm -f "$L"/*.meta1
  say "all.meta: $(wc -l < "$L/all.meta") records, $(du -h "$L/all.meta" | cut -f1)"
  [ -z "$empty" ] || say "WARN: no META records from:$empty"
}

phase_emit(){
  echo "== emit (//@SUPERSET, up to $JOBS parallel, admitted by memory, pool $MEMCAP_BUDGET, ${CRATE_TIMEOUT}s/crate) =="
  [ -s "$L/all.meta" ] || die "no all.meta -- run the meta phase"
  rm -f "$L"/*.ss.rz "$L/.peak.emit"
  grep -v '^lib_' "$DRV/crates.txt" > "$DRV/emit.txt"     # lib_* are metadata-only (never linked)
  export EMITTER L CRATE_TIMEOUT MEMCAP_SH; export MEMCAP_PEAKLOG="$L/.peak.emit"
  emit_one='ulimit -s unlimited 2>/dev/null; export MEMCAP_TAG="$1"
    { echo "//@SUPERSET crate=$1"; cat "$L/all.meta"; cat "$L/$1.rs"; } | timeout "$CRATE_TIMEOUT" "$MEMCAP_SH" "$EMITTER" > "$L/$1.ss.rz" 2>"$L/.$1.emit.err"
    echo "$1 $?" >> "$L/.emit.rc"'
  : > "$L/.emit.rc"
  pool_open
  admit_run emit "$emit_one" "$DRV/emit.txt"
  local bad=""; for cr in $(cat "$DRV/emit.txt"); do ss_complete "$L/$cr.ss.rz" || bad="$bad $cr"; done
  if [ -n "$bad" ]; then   # pool-OOM-killed, timed out, or crashed: redo each ALONE, outside the pool, whole budget
    say "incomplete after the parallel pass:$bad -- retrying SERIALLY outside the pool"
    for cr in $bad; do env -u MEMCAP_POOL MEMCAP="$MEMCAP_BUDGET" bash -c "$emit_one" _ "$cr"; done
    bad=""; for cr in $(cat "$DRV/emit.txt"); do ss_complete "$L/$cr.ss.rz" || bad="$bad $cr"; done
  fi
  pool_peaks "$L/.peak.emit"; peaks_save emit "$L/.peak.emit"; unset MEMCAP_POOL MEMCAP_PEAKLOG
  [ -z "$bad" ] || { for cr in $bad; do local rc; rc=$(awk -v c="$cr" '$1==c{r=$2} END{print r}' "$L/.emit.rc")
                     is_oom_rc "$rc" && rc="$rc (OOM: hit MEMCAP_BUDGET=$MEMCAP_BUDGET running alone)"
                     say "$cr: rc=$rc $(tail -1 "$L/.$cr.emit.err" 2>/dev/null)"; done
                     die "incomplete SUPERSET emit for:$bad"; }
  say "emitted $(wc -l < "$DRV/emit.txt") crates, all complete ($(cat "$L"/*.ss.rz | wc -l) rzir lines)"
}

phase_link(){
  echo "== link (tools/link, the semc-built cross-crate linker) =="
  rm -f "$DRV/combined_drv.rz" "$DRV/linked_drv.rz"
  need_tools link
  [ -s "$L/rustc_driver_impl.ss.rz" ] || die "no rustc_driver_impl.ss.rz (the entry crate) -- run emit"
  cat "$L/rustc_driver_impl.ss.rz" > "$DRV/combined_drv.rz"
  for cr in $(cat "$DRV/emit.txt"); do [ "$cr" = rustc_driver_impl ] && continue
    echo '@@@CRATE@@@' >> "$DRV/combined_drv.rz"; cat "$L/$cr.ss.rz" >> "$DRV/combined_drv.rz"; done
  timeout 1800 "$MEMCAP_SH" "$T/link" < "$DRV/combined_drv.rz" > "$DRV/linked_drv.rz" 2>"$DRV/.link.err" || die "tools/link failed (rc=$?)"
  local n; n=$(wc -l < "$DRV/linked_drv.rz")
  [ "$n" -ge 50000 ] || die "linked_drv.rz is only $n lines (< 50k = collapsed link; healthy was ~500k on 1.95)"
  say "linked_drv.rz: $n lines (from $(wc -l < "$DRV/combined_drv.rz") combined)"
}

phase_lower(){
  echo "== lower (genstubs -> r2l -> llc -O0 -> cc) =="
  rm -f "$DRV/stubs.rz" "$DRV/drv_full.rz" "$DRV/drv.ll" "$DRV/drv.o" "$DRV/tower-rustc"
  [ -s "$DRV/linked_drv.rz" ] || die "no linked_drv.rz -- run link"
  need_tools genstubs
  # genstubs writes stubs, then `@@@DANG@@@` + the dangler list, then `@@@SUMMARY@@@ ...` on stdout
  "$MEMCAP_SH" "$T/genstubs" < "$DRV/linked_drv.rz" > "$DRV/.genstubs.out" || die "genstubs failed (rc=$?)"
  awk '/^@@@DANG@@@$/{s=1; next} /^@@@SUMMARY@@@/{s=2; next} s==0' "$DRV/.genstubs.out" > "$DRV/stubs.rz"
  awk '/^@@@DANG@@@$/{s=1; next} /^@@@SUMMARY@@@/{s=2; next} s==1' "$DRV/.genstubs.out" > "$DRV/drv.dang"
  grep -q '^@@@SUMMARY@@@' "$DRV/.genstubs.out" || die "genstubs output has no summary (truncated?)"
  say "$(sed -n 's/^@@@SUMMARY@@@ //p' "$DRV/.genstubs.out")"
  cat "$DRV/linked_drv.rz" "$DRV/stubs.rz" > "$DRV/drv_full.rz"
  timeout 1800 "$MEMCAP_SH" "$SP/r2l.elf" < "$DRV/drv_full.rz" > "$DRV/drv.ll" 2>"$DRV/.r2l.err" || die "r2l failed (rc=$?)"
  say "drv.ll: $(wc -l < "$DRV/drv.ll") lines"
  timeout 1800 "$MEMCAP_SH" "$LLC" -O0 -filetype=obj "$DRV/drv.ll" -o "$DRV/drv.o" 2>"$DRV/.llc.err" \
    || die "llc failed or timed out (rc=$?): $(tail -2 "$DRV/.llc.err")"
  cc -no-pie "$DRV/drv.o" -o "$DRV/tower-rustc" -lm 2>"$DRV/.cc.err" || die "link failed: $(head -3 "$DRV/.cc.err")"
  say "tower-rustc: $(stat -c%s "$DRV/tower-rustc") B"
}

phase_smoke(){
  echo "== smoke (tower-rustc hello.rs) =="
  [ -x "$DRV/tower-rustc" ] || die "no tower-rustc -- run lower"
  printf 'fn main(){ let x=1; }\n' > "$DRV/hello.rs"
  ( cd "$DRV" && timeout 120 "$MEMCAP_SH" ./tower-rustc hello.rs > .smoke.out 2>&1 ); local rc=$?
  say "tower-rustc hello.rs -> exit $rc"; tail -5 "$DRV/.smoke.out" | sed 's/^/    /'
  if [ "$rc" -ge 128 ] && command -v gdb >/dev/null; then
    ( cd "$DRV" && timeout 300 gdb -q -batch -ex run -ex 'bt 15' --args ./tower-rustc hello.rs > .smoke.bt 2>&1 )
    say "first frames:"; grep -E '^#[0-9]+' "$DRV/.smoke.bt" | head -8 | sed 's/^/    /'
  fi
}

PHASES=("$@"); [ ${#PHASES[@]} -gt 0 ] || PHASES=(tools ingest meta emit link lower smoke)
echo "== build-driver: emitter=$(basename "$EMITTER")  src=$(cat "$RUSTC_SRC/version" 2>/dev/null)  DRV=$DRV =="
for p in "${PHASES[@]}"; do
  case "$p" in tools|ingest|meta|emit|link|lower|smoke) t0=$(date +%s); "phase_$p"; say "($p: $(( $(date +%s) - t0 ))s)";;
               *) die "unknown phase '$p'";; esac
done
