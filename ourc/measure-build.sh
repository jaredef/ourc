#!/bin/bash
# measure-build.sh -- run release.sh in its own systemd scope and record its memory use.
#   bash ourc/measure-build.sh OUTDIR
# A sampler logs, once a second, the scope's anonymous memory, its total (anon + file cache) and its largest process
# (OUTDIR/samples.txt: epoch anon-bytes total-bytes rss-kB command). At the end the kernel's memory.peak for the scope,
# which no sample can miss, goes to OUTDIR/result.txt with the wall time and a per-program summary.
set -u
ROOT="$(cd "$(dirname "$0")/.." && pwd)"; OUT="$(mkdir -p "${1:?usage: measure-build.sh OUTDIR}" && cd "$1" && pwd)"
if [ -z "${MEASURE_IN_SCOPE:-}" ]; then
  exec systemd-run --user --scope -q -E MEASURE_IN_SCOPE=1 bash "$0" "$OUT"
fi
CG="/sys/fs/cgroup$(cut -d: -f3 /proc/self/cgroup)"
( while :; do
    anon=$(awk '$1=="anon"{print $2}' "$CG/memory.stat"); cur=$(cat "$CG/memory.current")
    top=$(ps -o rss=,comm= --sort=-rss -p "$(paste -sd, "$CG/cgroup.procs")" 2>/dev/null | head -1)
    echo "$(date +%s) $anon $cur $top"; sleep 1; done ) > "$OUT/samples.txt" &
S=$!; t0=$(date +%s)
bash "$ROOT/ourc/release.sh" "$OUT/rel" > "$OUT/release.log" 2>&1; rc=$?
kill "$S"
{ echo "release.sh rc=$rc, $(( $(date +%s) - t0 )) s, memory.peak $(cat "$CG/memory.peak") bytes (anon + file cache)"
  awk '{if($2>a)a=$2; if($4>r){r=$4; c=$5}} END{printf "peak anon %.2f GB; largest process %s, %.2f GB RSS\n", a/1e9, c, r/1e6}' "$OUT/samples.txt"
  awk -v t0="$t0" '{if($2>a[$5])a[$5]=$2; if(!($5 in f))f[$5]=$1-t0} END{for(k in a) if(a[k]>5e8 && k!="ps") printf "  %-16s from %4d s, scope anon up to %.2f GB\n", k, f[k], a[k]/1e9}' "$OUT/samples.txt" | sort -k3 -n
} > "$OUT/result.txt"
cat "$OUT/result.txt"
