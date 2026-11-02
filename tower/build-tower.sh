#!/usr/bin/env bash
# build-tower.sh -- build the whole tower into a scratch dir, from cold, in dependency order.
#
#   SP=/tmp/sp bash tower/build-tower.sh               # build from the hand-hex + the tower gates
#   SP=/tmp/sp bash tower/build-tower.sh --gates       # ... and every sub-arc (S1..S9) gate + provenance
#   SP=/tmp/sp bash tower/build-tower.sh --dev         # opt-in fast loop: seed from the committed blobs
#   SP=/tmp/sp bash tower/build-tower.sh --mint        # leanest SEM0 remint: --dev + skip parity + LLVM mint, no gates
#                                                      # (UNVERIFIED, provenance class dev: run --gates before pinning)
#
# DEFAULT = FROM HEX. lib/derive-seeds.sh first mints every dev-loop seed binary (m0, cf, ccc, the L2
# ld/opt/ra) from hex0.hand.hex into $SP/seeds/, and every stage below runs on those (TOWER_L2/TOWER_M0)
# instead of the committed blobs; drift between the two is reported. The one committed input left is
# L5/cf.hoist.seed (capacity cycle -- see derive-seeds.sh), and lib/check-cfhoist-seed.sh proves after
# selfhost that it washes out of the produced compiler. --dev skips the derivation and uses the committed
# blobs directly (the old behavior). No host cc/ld is used anywhere in the produced chain in either mode.
#
# Why this file exists: the order below is a real dependency chain -- build-live-backend mints the all-Rust
# twins that every later gate's `chain` runs, and stage.sh produces the L6.rzrs that all 11 sub-arc gates
# consume -- but it lived only in the arc docs, so a cleared /tmp meant rediscovering it a step at a time.
# One command, one order, one summary.
#
# NOTE ON TRUST: this is the @dev fast loop (provenance class `dev`). provenance.sh is the TRUST gate -- it
# re-derives everything from the hand-hex in-gate and DDC-anchors it. Run it (via --gates) before believing
# any claim about the produced chain. See apparatus/docs/tower-naming-and-trust-path.md.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; A="$HERE"
SP="${SP:?set SP (scratch dir; may be empty)}"; export SP
mkdir -p "$SP"
GATES=0; DEV=0; MINT=0
for arg in "$@"; do
  case "$arg" in
    --gates) GATES=1 ;;
    --dev)   DEV=1 ;;
    --mint)  DEV=1; MINT=1 ;;
    *) echo "unknown argument '$arg' (use: --gates, --dev, --mint)" >&2; exit 2 ;;
  esac
done
fail=0; skipped=0; declare -a SUMMARY=()
# Memory caps (scripts/memcap.sh; 2026-10-23 an uncapped sem0.llvm.frozen hit 40.7GB and tripped a GLOBAL OOM).
# Each gate runs in its own cgroup capped at GATE_MEMCAP (gates run serially, so one cap = the whole budget); the
# tools a gate calls through semc.sh get their own MEMCAP (default 48G) scope. GATE_MEMCAP=0 disables.
MEMCAP_SH="$A/../scripts/memcap.sh"; GATE_MEMCAP="${GATE_MEMCAP:-96G}"; INNER_MEMCAP="${MEMCAP:-48G}"
CAPPED=(env MEMCAP="$GATE_MEMCAP" "$MEMCAP_SH" env MEMCAP="$INNER_MEMCAP")   # prefix: "${CAPPED[@]}" CMD..
# Per-gate wall-clock limit. Was a fixed 2400 s; arena headroom alone now does ~6 full 1M-line SEM0 self-compiles (~10 min
# each), so it overran it (rc=124). GATE_TIMEOUT=<seconds> overrides.
GATE_TIMEOUT="${GATE_TIMEOUT:-7200}"

# Parallel gate lane. After the serial prefix (seeds .. sem0-stage-llvm + the shared s1 build) the lane's gates are
# independent, so up to GATES_JOBS groups run at once (GATES_JOBS=1 = the old serial loop in the shared SP). Each
# group runs in $SP/.gates/<group>/, a btrfs REFLINK clone of SP (copy-on-write: ~instant, no space until written)
# that is also its cwd -- gates write fixed names into $SP and relative names into the cwd, which would collide.
# All gates share one memory pool (GATE_POOL) on top of each gate's own GATE_MEMCAP; a gate the pool kills (rc
# 137/143) is re-run alone afterwards. Results print in table order, in the serial format.
GATES_JOBS="${GATES_JOBS:-8}"
# Pool = MemAvailable minus 32G headroom (zram + desktop), max 100G -- see build-driver.sh: a fixed 100G on a 125G
# box sent the kernel to global OOM on 2026-10-26.
if [ -z "${GATE_POOL:-}" ]; then
  gp=$(( $(awk '/^MemAvailable:/{print $2}' /proc/meminfo) / 1048576 - 32 )); [ "$gp" -gt 100 ] && gp=100; [ "$gp" -lt 8 ] && gp=8
  GATE_POOL="${gp}G"
fi
logname(){ echo "$SP/.log.$(echo "$1" | tr -c 'a-zA-Z0-9' '_')"; }
report() { # report LABEL RC LOG -- print one gate result line and record it
  local label="$1" rc="$2" log="$3"
  printf '  %-34s ' "$label"
  if [ "$rc" = 0 ] && grep -q -E '(^| )SKIP([^A-Za-z]|$)' "$log" 2>/dev/null; then
    # a gate that exits 0 but only SKIPPED (missing input) is not a pass -- it hid crate census for two runs
    printf 'skip (%s)\n' "$(grep -m1 -o -E 'SKIP.*' "$log" | cut -c1-80)"; SUMMARY+=("skip $label"); skipped=$((skipped+1))
  elif [ "$rc" = 0 ]; then
    printf 'ok\n'; SUMMARY+=("ok   $label")
  else
    [ "$rc" = 137 ] && echo "OOM: hit GATE_MEMCAP=$GATE_MEMCAP" >> "$log"
    printf 'FAIL rc=%s (log: %s)\n' "$rc" "$log"; SUMMARY+=("FAIL $label"); fail=$((fail+1))
    tail -4 "$log" | sed 's/^/        /'
  fi
}
run() { # run LABEL SCRIPT [ARGS..] -- serial, in the shared SP
  local label="$1"; shift
  local log; log="$(logname "$label")"
  local rc=0; timeout "$GATE_TIMEOUT" "${CAPPED[@]}" bash "$@" > "$log" 2>&1 || rc=$?
  report "$label" "$rc" "$log"
}
lane_row(){ # lane_row IDX -> sets L_LABEL L_SCRIPT L_GROUP
  local row="${GATES_LANE[$1]}" rest; L_LABEL="${row%%|*}"; rest="${row#*|}"; L_SCRIPT="${rest%%|*}"
  L_GROUP="${rest#"$L_SCRIPT"}"; L_GROUP="${L_GROUP#|}"; [ -n "$L_GROUP" ] || L_GROUP="g$1"; }
run_group(){ # run_group GROUP IDX.. (backgrounded): clone SP, then run the members in table order
  local g="$1"; shift; local c="$SP/.gates/$g" idx rc
  mkdir -p "$c"
  ( cd "$SP" && find . -mindepth 1 -maxdepth 1 ! -name .gates ! -name '.log.*' -print0 | xargs -0 -r cp -a --reflink=auto -t "$c" )
  for idx in "$@"; do lane_row "$idx"; rc=0
    ( cd "$c" && SP="$c" MEMCAP_POOL=rztower timeout "$GATE_TIMEOUT" "${CAPPED[@]}" bash "$A/$L_SCRIPT" ) > "$(logname "$L_LABEL")" 2>&1 || rc=$?
    echo "$rc" > "$SP/.gates/.rc.$idx"
  done
}
run_lane_parallel(){
  local G="$SP/.gates" i g n="${#GATES_LANE[@]}"; rm -rf "$G"; mkdir -p "$G"
  local -a gorder=(); local -A gmem=()
  for ((i = 0; i < n; i++)); do lane_row "$i"; [ -n "${gmem[$L_GROUP]+x}" ] || gorder+=("$L_GROUP"); gmem[$L_GROUP]+="$i "; done
  "$MEMCAP_SH" --pool rztower "$GATE_POOL"
  echo "  [gate lane: $n gates in ${#gorder[@]} groups, $GATES_JOBS at a time, memory pool $GATE_POOL, clones in $G]"
  for g in "${gorder[@]}"; do
    while [ "$(jobs -rp | wc -l)" -ge "$GATES_JOBS" ]; do wait -n; done
    # shellcheck disable=SC2086
    run_group "$g" ${gmem[$g]} &
  done
  wait
  local rc retry=()
  for ((i = 0; i < n; i++)); do rc=$(cat "$G/.rc.$i" 2>/dev/null || echo 255); case "$rc" in 137|143) retry+=("$i");; esac; done
  for i in "${retry[@]}"; do lane_row "$i"   # pool-killed: re-run ALONE, outside the pool, in its group's clone
    echo "  [retry alone (pool OOM rc=$(cat "$G/.rc.$i")): $L_LABEL]"; rc=0
    ( cd "$G/$L_GROUP" && SP="$G/$L_GROUP" timeout "$GATE_TIMEOUT" "${CAPPED[@]}" bash "$A/$L_SCRIPT" ) > "$(logname "$L_LABEL")" 2>&1 || rc=$?
    echo "$rc" > "$G/.rc.$i"
  done
  for ((i = 0; i < n; i++)); do lane_row "$i"; report "$L_LABEL" "$(cat "$G/.rc.$i" 2>/dev/null || echo 255)" "$(logname "$L_LABEL")"; done
}

# the cold-start roots in $SP are minted from whichever seeds this run uses -- never reuse another mode's
rm -f "$SP/B.cf.m0" "$SP/L3.rzrs" "$SP/L3.rzcpp" "$SP/cf.bin" "$SP/rzrs.rsz" "$SP/rzcpp"
if [ "$DEV" = 1 ]; then
  unset TOWER_L2 TOWER_M0
  echo "== tower build (SP=$SP) -- DEV: seeded from the committed blobs =="
else
  echo "== tower build (SP=$SP) -- from the hand-hex =="
  "${CAPPED[@]}" bash "$A/lib/derive-seeds.sh" | sed 's/^/  /'
  [ "${PIPESTATUS[0]}" = 0 ] || { echo "== ABORT: could not derive the seeds from the hand-hex =="; exit 1; }
  export TOWER_L2="$SP/seeds/L2" TOWER_M0="$SP/seeds/L0/m0/m0.bin"
fi
export CC="${TOWER_L2:-$A/L2}"
run "backend twins (ld/opt/ra)" "$A/backend-live/build-live-backend.sh"
[ "$fail" = 0 ] || { echo "== ABORT: the backend twins are the floor; nothing below can run =="; exit 1; }
run "bootstrap (seed -> tier-0 -> tier-1)" "$A/L4/bootstrap.sh"
run "selfhost (L5 g2==g3 + battery)"       "$A/L5/selfhost.sh"
[ "$DEV" = 1 ] || run "cf.hoist.seed washes out"   "$A/lib/check-cfhoist-seed.sh"
[ "$MINT" = 1 ] || run "parity (90 programs vs rustc)"        "$A/parity/parity.sh"
run "stage (L5 -> L6.rzrs)"            "$A/L6/stage.sh"
# SEM0 HOIST 2026-08-28: one tier above `stage`. L6 mints the FROZEN SEM0 in-gate, and from here the 14 S-tier
# gates compile the dev target with it (SEM0_CC now defaults to $SP/sem0.cc, was $SP/L6.rzrs). Same trust shape
# as stage0: frozen source in-repo, binary minted from the tier below, rooting unchanged in the hand-hex.
run "sem0-stage (L6 -> SEM0.cf)"       "$A/sem0/stages/SEM0-hoist/sem0-stage.sh"
[ "$fail" = 0 ] || { echo "== ABORT: SEM0 is now the tier that compiles the dev target; the gates below cannot run without it =="; exit 1; }

if [ "$MINT" = 1 ]; then
  # the LLVM frozen is what ourc/ourc-gate prefer; its C2 byte-identical-emit check is the admission, so it stays
  run "sem0-stage-llvm (C2: llvm-frozen emit == cf; stage0 mint)" "$A/backend-llvm/sem0-stage-llvm.sh"
  echo "== MINT (unverified): $SP/SEM0.cf $( [ -x "$SP/SEM0.llvm" ] && echo "+ $SP/SEM0.llvm") =="
  for f in "$SP/SEM0.cf" "$SP/SEM0.llvm"; do [ -f "$f" ] && echo "  pin $(sha256sum "$f" | cut -c1-12)  $f"; done
fi
if [ "$GATES" = 1 ]; then
  echo "== sub-arc gates =="
  # The gate lane is a table (see tower/gates.md for each gate's rationale + the past incident it guards).
  # Two gates stay EXPLICIT around the loop: the C2 stage0 mint (it mints the frozen the S-tier gates
  # compile with, aborts if the LLVM frozen diverges, and selects SEM0_CC) and provenance (the trust gate).
  run "sem0-stage-llvm (C2: llvm-frozen emit == cf; stage0 mint)" "$A/backend-llvm/sem0-stage-llvm.sh"
  [ "$fail" = 0 ] || { echo "== ABORT: C2 failed -- the LLVM-minted frozen diverges from the hex-rooted cf frozen; refusing to promote it as stage0 =="; exit 1; }
  if [ -x "$SP/SEM0.llvm.s9" ]; then export SEM0_CC="$SP/SEM0.llvm.s9"; echo "  [stage0 = LLVM-minted frozen (C2-admitted, Tier-2 by-result)]"; else echo "  [stage0 = cf frozen \$SP/SEM0.s9 (LLVM mint skipped: llc/cc absent)]"; fi
  # S1-S7 do not read BASELINE; S9+ do. Set once here -- behavior-identical to the former per-S9 export.
  export BASELINE=sem
  # Build the s1.rc + s1.rz pair every S-gate (and S9 whole-file's gen1) differentials against ONCE, in the shared
  # SP, so every gate -- and every parallel clone -- starts with a cache hit (sem0/stages/s1-build.sh).
  export P="$A/sem0/stages/S1-lex-parse-ast/s1_parser.rs"
  run "s1 compilers (rustc + SEM0_CC, shared)" "$A/sem0/stages/s1-build.sh"
  unset P
  [ "$fail" = 0 ] || { echo "== ABORT: could not build the s1 compilers the gate lane differentials against =="; exit 1; }
  # host-gated gates (bridge, ourc, multicrate) self-skip when llc/cc are absent.
  # Row = "LABEL|SCRIPT[|GROUP]". Rows sharing a GROUP run in table order in ONE scratch clone (they hand files to
  # each other: S9 whole-file mints wf.gen1/wf.gen2, which behavioural, RUN-slice and multi-crate consume).
  GATES_LANE=(
    "S1 AST differential|sem0/stages/S1-lex-parse-ast/s1-ast-diff.sh"
    "S2 macro expand|sem0/stages/S2-macro-procmacro/s2-expand-diff.sh"
    "S3 resolve|sem0/stages/S3-resolve-crate-driver/s3-resolve-diff.sh"
    "S4 HIR lowering|sem0/stages/S4-hir-lowering/s4-hir-diff.sh"
    "S5 typeck|sem0/stages/S5-typeck-trait-solver/s5-typeck-diff.sh"
    "S6 generics/mono|sem0/stages/S6-generics-mono/s6-mono-diff.sh"
    "S7 checker tiers|sem0/stages/S7-checker-tiers/s7-check-diff.sh"
    "S9 run differential (sem)|sem0/stages/S9-lower-codegen/s9-run-diff.sh"
    "S9 whole-file self-compilation|sem0/stages/S9-lower-codegen/s9-wholefile-diff.sh|s9chain"
    "S9 behavioural differential|sem0/stages/S9-lower-codegen/s9-behavior-diff.sh|s9chain"
    "r2-corpus zero-regression|corpus/r2-corpus/corpus-gate.sh"
    "D0 over-acceptance (vs rustc)|sem0/stages/D0-derived-substrate/overaccept-board.sh"
    "crate census (T0 regression)|census/census-gate.sh|s9chain"
    "arena headroom (self-emit at 75%)|sem0/arena-headroom.sh"
    "harness synth (tower-built == golden)|../apparatus/arcs/2026-09-20-entry-harness-in-chain/harness-equiv-gate.sh"
    "RUN-slice (SEM0-linked cross-crate == golden)|../apparatus/arcs/2026-09-22-t3-run-pipeline-slices/run-slice-gate/run-slice-gate.sh|s9chain"
    "multi-crate RUN (pf -> lexer == golden)|../apparatus/arcs/2026-09-24-meta-telos-completion/locales/Plane-I-integration/multicrate-pf-lexer/multicrate-gate.sh|s9chain"
    "SEM0-native backend (cf.rs via SEM0 == cf.hoist)|sem0/sem0-native-backend-gate.sh"
    "rzir->LLVM bridge parity (board)|backend-llvm/llvm-parity.sh"
    "ourc (19 crates, tower-built SEM0 == golden)|ourc/ourc-gate.sh"
    "semc (r2-corpus, tower-built SEM0 vs rustc)|semc/semc-gate.sh"
  )
  # a gate whose script is not in this checkout (the three hosted in apparatus/, absent from the public export)
  # is reported as a skip, not run
  kept=()
  for row in "${GATES_LANE[@]}"; do
    r="${row#*|}"
    if [ -f "$A/${r%%|*}" ]; then kept+=("$row")
    else printf '  %-34s skip (not in this checkout: %s)\n' "${row%%|*}" "${r%%|*}"; SUMMARY+=("skip ${row%%|*}"); skipped=$((skipped+1)); fi
  done
  GATES_LANE=("${kept[@]}")
  if [ "$GATES_JOBS" -le 1 ]; then
    for row in "${GATES_LANE[@]}"; do r="${row#*|}"; run "${row%%|*}" "$A/${r%%|*}"; done
  else
    run_lane_parallel
  fi

  echo "== trust gate =="
  run "provenance (DDC, hex-rooted)" "$A/backend-toolchain/provenance.sh"
fi

echo
printf '%s\n' "${SUMMARY[@]}"
echo "== build-tower ($([ "$DEV" = 1 ] && echo "dev blobs" || echo "from hex")): $([ "$fail" = 0 ] && echo PASS || echo "FAIL ($fail)")$([ "$skipped" = 0 ] || echo " -- $skipped SKIPPED")) =="
[ "$fail" = 0 ]
