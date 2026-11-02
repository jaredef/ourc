#!/bin/bash
# ui-full: run ALL of rustc's tests/ui against ourc and record a per-test outcome, for a failure TOPOLOGY.
# (ui-gauge.sh is the cheap 1-in-10 parity reading; this is the whole map.) Sequential -- no parallel jobs.
#   OURC=<ourc> UI=<rustc-src>/tests/ui OUT=<dir> [LIMIT=n] bash ui-full.sh
# Per test, results.tsv gets:  class  outcome  harness-tag  path  first-ourc-error
#   class:   run   (//@ run-pass)  -> ourc must accept, and exit code + stdout must equal rustc's run
#            pass  (check/build-pass) -> ourc must accept (not run)
#            fail  (no pass marker: a compile-fail test) -> ourc must REJECT; accepting is ACCEPT-INVALID
#   outcome: OK | WRONG | CRASH | REJECT | ACCEPT-INVALID | ORACLE-SKIP (rustc itself could not build/run it here)
#   harness-tag: why the test is not plain single-file (aux / flags / edition / revisions / needs / mod / feature / -)
# The topology files rank: normalised first ourc errors, tests/ui subdirectories, and feature gates.
# Sharded:  JOBS=24 (default nproc*3/4) runs JOBS shards (SHARD=k/N, every N-th test) and merges them.
# Guards: ourc 30s, llc/cc 20s (CODEGEN-TIMEOUT), IR over IRMAX bytes skips codegen (IR-BLOAT),
# each test under ulimit -v MEMKB (default 4 GB).
set -u
OURC="${OURC:?set OURC}"; UI="${UI:?set UI to rustc-1.99.1-src/tests/ui}"; OUT="${OUT:?set OUT}"; LLC="${LLC:-llc}"; LIMIT="${LIMIT:-0}"
IRMAX="${IRMAX:-5000000}"; MEMKB="${MEMKB:-4000000}"; SHARD="${SHARD:-}"
mkdir -p "$OUT"
if [ -z "$SHARD" ]; then
  if [ -n "${LIST:-}" ]; then cut -f1 "$LIST" | grep '\.rs$' | sed "s|^|$UI/|" > "$OUT/all.txt"   # LIST=members.txt: re-measure one sub-arc
  else find "$UI" -name '*.rs' -not -path '*/auxiliary/*' | sort > "$OUT/all.txt"; fi
  [ "$LIMIT" -gt 0 ] && head -n "$LIMIT" "$OUT/all.txt" > "$OUT/l" && mv "$OUT/l" "$OUT/all.txt"
  J="${JOBS:-$(( $(nproc) * 3 / 4 ))}"; rm -f "$OUT"/results.*.tsv
  seq 0 $((J-1)) | xargs -P "$J" -I{} env SHARD={}/$J bash "$0"
  cat "$OUT"/results.*.tsv > "$OUT/results.tsv"; n=$(wc -l < "$OUT/results.tsv")
else
k="${SHARD%/*}"; N="${SHARD#*/}"; W="$(mktemp -d)"; trap 'rm -rf "$W"' EXIT
RES="$OUT/results.$k.tsv"; : > "$RES"
norm() { sed 's/.*-> //; s/^error\[[0-9]*\.\.[0-9]*\]: //; s/ ; .*//; s/`[^`]*`/<X>/g; s/[0-9][0-9]*/N/g' | cut -c1-120; }
n=0
awk -v k="$k" -v N="$N" '(NR-1) % N == k' "$OUT/all.txt" | while read -r f; do
  n=$((n+1)); rel="${f#$UI/}"
  if grep -q '^//@ *run-pass' "$f"; then cls=run; elif grep -q '^//@ *\(check-pass\|build-pass\)' "$f"; then cls=pass; else cls=fail; fi
  tag=$(grep -o '^//@ *\(aux-build\|aux-crate\|compile-flags\|edition\|revisions\|needs-[a-z-]*\|only-[a-z0-9_-]*\|ignore-[a-z0-9_-]*\|run-flags\|exec-env\)' "$f" | head -1 | sed 's|//@ *||')
  [ -z "$tag" ] && grep -q '^ *mod [a-z_]* *;' "$f" && tag=mod-file
  [ -z "$tag" ] && grep -q '#!\[feature' "$f" && tag=feature
  [ -z "$tag" ] && tag=-
  # ourc
  timeout 30 "$OURC" "$f" --emit-ir > "$W/o.txt" 2>&1; orc=$?   # no ulimit -v: ourc reserves a huge VA arena and segfaults under any cap
  if [ "$orc" -ge 124 ] && ! grep -q '^define' "$W/o.txt"; then ok_ir=crash; elif grep -q '^define' "$W/o.txt"; then ok_ir=yes; else ok_ir=no; fi
  err=$(head -1 "$W/o.txt" | norm)
  if [ "$cls" = fail ]; then
    if [ "$ok_ir" = yes ]; then out=ACCEPT-INVALID; elif [ "$ok_ir" = crash ]; then out=CRASH; else out=OK; fi
  elif [ "$ok_ir" = crash ]; then out=CRASH
  elif [ "$ok_ir" = no ]; then out=REJECT
  elif [ "$cls" = pass ]; then out=OK
  else
    if [ "$(stat -c %s "$W/o.txt")" -gt "$IRMAX" ]; then out=IR-BLOAT; err="(IR $(stat -c %s "$W/o.txt") bytes)"
    elif ! timeout 60 rustc --edition 2021 -A warnings -C overflow-checks=off -o "$W/r.bin" "$f" >/dev/null 2>&1; then out=ORACLE-SKIP
    else
      # V4 SAME-ARGV0: both binaries run as "$W/prog", so a program that prints argv[0] (match-string-slice) compares equal
      cp "$W/r.bin" "$W/prog"; ( cd "$W" && timeout 10 ./prog > "$W/r.out" 2>/dev/null ); rrc=$?
      sed -n '/^\(@\|declare\|define\)/,$p' "$W/o.txt" > "$W/o.ll"
      ( ulimit -v "$MEMKB"; timeout 20 "$LLC" -filetype=obj "$W/o.ll" -o "$W/o.o" ) 2>/dev/null; lrc=$?
      [ "$lrc" = 0 ] && { timeout 20 cc -no-pie "$W/o.o" -o "$W/o.bin" -lm 2>/dev/null; lrc=$?; }
      if [ "$lrc" -ge 124 ] || [ "$lrc" -gt 128 ]; then out=CODEGEN-TIMEOUT; err="(llc/cc timeout or OOM)"
      elif [ "$lrc" != 0 ]; then out=REJECT; err="(llc/link failed)"
      else
        cp "$W/o.bin" "$W/prog"; ( cd "$W" && timeout 10 ./prog > "$W/o.out" 2>/dev/null ); xrc=$?
        # V4 ORACLE-SIGNAL: the RUSTC binary itself died on a signal (139 SIGSEGV, 134 SIGABRT): the plain-rustc oracle
        # could not build this test faithfully (a --test / proc-macro crate: derive-test) -- not a verdict on ourc
        if [ "$rrc" -gt 128 ] && [ "$rrc" != 137 ]; then out=ORACLE-SKIP; err="(oracle binary died: rc $rrc)"
        elif [ "$xrc" -ge 124 ] && [ "$rrc" -lt 124 ]; then out=CRASH
        elif [ "$xrc" = "$rrc" ] && cmp -s "$W/r.out" "$W/o.out"; then out=OK
        else out=WRONG; fi
      fi
    fi
  fi
  [ "$out" = OK ] && err=-
  printf '%s\t%s\t%s\t%s\t%s\n' "$cls" "$out" "$tag" "$rel" "$err" >> "$RES"
done
exit 0
fi
# topology
{ echo "== outcome by class (class outcome count)"; cut -f1,2 "$OUT/results.tsv" | sort | uniq -c | sort -k2,2 -k1,1nr
  echo; echo "== WRONG / CRASH / ACCEPT-INVALID (the correctness failures)"; awk -F'\t' '$2=="WRONG"||$2=="CRASH"||$2=="ACCEPT-INVALID"||$2=="IR-BLOAT"||$2=="CODEGEN-TIMEOUT"' "$OUT/results.tsv" | cut -f1,2,4 | head -200
  echo; echo "== top normalised first ourc errors among run/pass REJECTs (what to build next)"; awk -F'\t' '($1=="run"||$1=="pass") && $2=="REJECT" {print $5}' "$OUT/results.tsv" | sort | uniq -c | sort -rn | head -60
  echo; echo "== REJECTs by harness tag (run/pass)"; awk -F'\t' '($1=="run"||$1=="pass") && $2=="REJECT" {print $3}' "$OUT/results.tsv" | sort | uniq -c | sort -rn | head -30
  echo; echo "== run/pass OK rate by tests/ui subdirectory (dir ok/total)"; awk -F'\t' '($1=="run"||$1=="pass") { split($4,p,"/"); d=p[1]; t[d]++; if ($2=="OK") o[d]++ } END { for (d in t) printf "%5d/%-5d %s\n", o[d]+0, t[d], d }' "$OUT/results.tsv" | sort -t/ -k2 -nr | head -60
} > "$OUT/topology.txt"
echo "ui-full: $n tests -> $OUT/results.tsv, $OUT/topology.txt"
