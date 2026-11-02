#!/usr/bin/env bash
# memcap.sh -- run a command in its own memory-capped cgroup, so a runaway compiler is killed ALONE (exit 137)
# instead of tripping a global OOM. Seen 2026-10-23: an uncapped sem0.llvm.frozen reached 40.7GB RSS from a tmux
# pane; the kernel's global OOM killer took chrome first, then the compiler.
#
#   memcap.sh CMD [ARGS..]      stdin/stdout/stderr and the exit code pass through unchanged
#
#   MEMCAP=48G   per-process cap (cgroup MemoryMax; default 48G). MEMCAP=0 disables the cap.
#   Swap is disallowed (MemorySwapMax=0) so a runaway fails fast instead of sinking into swap.
#   MEMCAP_POOL=name   run inside name.slice, a SHARED budget for a parallel phase (set it once with
#                      `memcap.sh --pool name 100G`). The pool's MemoryMax caps all its jobs together: when it fills,
#                      the kernel kills the largest job IN THE POOL (exit 137), never anything outside it. MEMCAP
#                      stays each job's own runaway guard.
#   MEMCAP_PEAKLOG=file  append "<MEMCAP_TAG> <peak bytes> <rc>" per job (not written when the job is OOM-killed).
#
# A cgroup cap, NOT `ulimit -v`: the tower runtime mmaps a 16-20GB arena up front, so an address-space limit
# would break healthy runs. The cgroup counts resident memory only. Rlimits (ulimit -s unlimited) are inherited.
# Without a reachable systemd user manager the command runs uncapped, with a warning.
set -u
have_systemd(){ command -v systemd-run >/dev/null 2>&1 && systemctl --user show-environment >/dev/null 2>&1; }
if [ "${1:-}" = --pool ]; then   # memcap.sh --pool NAME SIZE: (re)size a shared pool slice
  have_systemd || { echo "memcap: no systemd user manager -- pool $2 NOT capped" >&2; exit 0; }
  exec systemctl --user set-property --runtime "$2.slice" MemoryMax="$3" MemorySwapMax=0
fi
cap="${MEMCAP:-48G}"
[ "$cap" = 0 ] && exec "$@"
if have_systemd; then
  slice=(); [ -n "${MEMCAP_POOL:-}" ] && slice=(--slice="$MEMCAP_POOL.slice")
  if [ -n "${MEMCAP_PEAKLOG:-}" ]; then   # wrap to read the scope's memory.peak before it exits
    exec systemd-run --user --scope --quiet --collect "${slice[@]}" -p MemoryMax="$cap" -p MemorySwapMax=0 -- \
      bash -c '"$@"; rc=$?; cg=$(cut -d: -f3 /proc/self/cgroup)
        echo "${MEMCAP_TAG:-$1} $(cat "/sys/fs/cgroup$cg/memory.peak" 2>/dev/null || echo ?) $rc" >> "$MEMCAP_PEAKLOG"; exit $rc' _ "$@"
  fi
  exec systemd-run --user --scope --quiet --collect "${slice[@]}" -p MemoryMax="$cap" -p MemorySwapMax=0 -- "$@"
fi
echo "memcap: no systemd user manager -- running UNCAPPED: $1" >&2
exec "$@"
