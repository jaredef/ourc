#!/bin/bash
# safe-run.sh -- run a binary-under-test (tcc.elf, far-gate probes) inside a
# resource-limited transient cgroup so a miscompile cannot take down the session:
#   MemoryMax=2G      -- killed by the cgroup before the system OOM killer wakes
#   MemorySwapMax=0   -- no swap-grind "machine feels dead" mode
#   TasksMax=64       -- no fork explosion
#   timeout 30s       -- no infinite loop
# Usage: safe-run.sh <binary> [args...]        (stdin/stdout/stderr pass through)
# Override knobs: SAFE_MEM=4G SAFE_TIMEOUT=120 safe-run.sh ...
exec systemd-run --user --scope --quiet \
  -p MemoryMax="${SAFE_MEM:-2G}" -p MemorySwapMax=0 -p TasksMax="${SAFE_TASKS:-64}" \
  timeout "${SAFE_TIMEOUT:-30}" "$@"
