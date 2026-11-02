#!/usr/bin/env bash
# The MINT-GAP pipeline board: runs every fixture behind the four pipelines, SEM0 vs rustc.
#
# This is the falsifier for `mint-gap-pipeline-oracle.md`. Each fixture is one claim. The CONTROL set is the
# probe-the-probe lane: those must AGREE, and if they ever diverge the board is measuring its own harness rather
# than SEM0 (that failure mode has produced confident wrong numbers on this arc more than once).
#
#   SP=<scratch with sem0.frozen + cf.hoist> bash mint-gap-board.sh
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../../../.." && pwd)"
CC="${TOWER_L2:-$ROOT/tower/L2}"
: "${SP:?set SP (scratch containing sem0.frozen and cf.hoist)}"
[ -x "$SP/sem0.frozen" ] || { echo "FAIL: no $SP/sem0.frozen -- mint it with sem0-stage.sh first"; exit 1; }
[ -s "$SP/cf.hoist" ]    || { echo "FAIL: no $SP/cf.hoist -- the backend is missing, any number here is meaningless"; exit 1; }
command -v rustc >/dev/null || { echo "FAIL: rustc absent; this board is a DIFFERENTIAL and cannot run one-sided"; exit 1; }
# The backend must EXIST. A wrong ROOT (one `..` short -> apparatus/apparatus/) silently yields no ld/opt/ra,
# and cf.hoist still emits bytes from empty input -- so the `-s` ELF guard passes and EVERY fixture reports
# CRASH. That exact defect produced a fully bogus board on 2026-08-28, and the same `..` miscount had already
# happened once in sem0-corpus-board.sh. Assert the binaries, not just their output.
for b in link/rzir-ld.bin opt/rzir-opt.bin ra/rzir-ra.bin; do
  [ -x "$CC/$b" ] || { echo "FAIL: missing backend $CC/$b -- ROOT resolved wrong; every row would read CRASH"; exit 1; }
done

W="$(mktemp -d)"; trap 'rm -rf "$W"' EXIT
fail=0; ctlfail=0; n=0

verdict(){ # verdict <file> -> echoes AGREE|<divergence class>
  local f="$1" t="$W/w"; rm -rf "$t"; mkdir -p "$t"
  { echo '//@S9'; cat "$f"; } > "$t/p.rs"          # marker MUST reach a FILE: C-SEM0-PIPED-STDIN-TRUNCATES
  timeout 120 "$SP/sem0.frozen" < "$t/p.rs" > "$t/r" 2>/dev/null
  [ -s "$t/r" ] && ! grep -q UNLOWERED "$t/r" || { echo "REFUSED-OR-UNLOWERED"; return; }
  "$CC/link/rzir-ld.bin" < "$t/r" 2>/dev/null | "$CC/opt/rzir-opt.bin" 2>/dev/null \
    | "$CC/ra/rzir-ra.bin" 2>/dev/null | "$SP/cf.hoist" > "$t/e" 2>/dev/null
  [ -s "$t/e" ] || { echo "NOELF"; return; }
  chmod +x "$t/e"; local so sr; so="$(timeout 10 setarch -R "$t/e" 2>/dev/null)"; sr=$?
  rustc --edition 2021 -O -o "$t/x" "$f" >/dev/null 2>&1 || { echo "RUSTC-REJECTS"; return; }
  local ro; ro="$(timeout 10 "$t/x" 2>/dev/null)"
  if [ "$so" = "$ro" ]; then echo "AGREE"
  elif [ $sr -ge 128 ]; then echo "CRASH"
  elif echo "$so" | grep -qE '(^|[^0-9])[123][0-9]{7}([^0-9]|$)'; then echo "ADDRLEAK"
  elif [ -z "$so" ]; then echo "EMPTY"
  elif echo "$so" | grep -qE '(^|[^0-9])0([^0-9]|$)'; then echo "ZERODEF"
  else echo "VALUEDIV"; fi
}

# P0 is an INVERTED lane. Its fixtures call methods that DO NOT EXIST. rustc rejects them (E0599); the claim
# under test is that SEM0 compiles them anyway and returns a default. This is the root claim of the whole board:
# if these ever start refusing, the fail-open mouth is closed and P1/P2 should be re-measured from scratch.
echo "== P0-failopen (INVERTED: rustc MUST reject; SEM0 compiling them IS the defect)"
p0fail=0
for f in "$HERE"/fixtures/P0-failopen/*.rs; do
  [ -e "$f" ] || continue
  b="$(basename "$f" .rs)"; n=$((n+1))
  if rustc --edition 2021 -O -o "$W/x0" "$f" >/dev/null 2>&1; then
    printf '   %-24s %s\n' "$b" "INVALID-FIXTURE (rustc ACCEPTED it; it no longer tests fail-open)"; fail=1; continue
  fi
  rm -rf "$W/w0"; mkdir -p "$W/w0"; { echo '//@S9'; cat "$f"; } > "$W/w0/p.rs"
  timeout 120 "$SP/sem0.frozen" < "$W/w0/p.rs" > "$W/w0/r" 2>/dev/null
  if [ -s "$W/w0/r" ] && ! grep -q UNLOWERED "$W/w0/r"; then
    printf '   %-24s %s\n' "$b" "FAIL-OPEN (SEM0 emitted code for a method that does not exist)"; p0fail=$((p0fail+1))
  else
    printf '   %-24s %s\n' "$b" "REFUSED -- fail-open CLOSED here"
  fi
done
echo "   -> $p0fail of the P0 fixtures still compile silently"
echo

for d in "$HERE"/fixtures/*/; do
  case "$(basename "$d")" in P0-failopen) continue;; esac
  p="$(basename "$d")"; echo "== $p"
  for f in "$d"*.rs; do
    [ -e "$f" ] || continue
    v="$(verdict "$f")"; n=$((n+1)); b="$(basename "$f" .rs)"
    printf '   %-24s %s\n' "$b" "$v"
    if [ "$p" = control ]; then
      [ "$v" = AGREE ] || { ctlfail=$((ctlfail+1)); fail=1; }
    fi
  done
done

echo
echo "  fixtures run: $n"
if [ "$ctlfail" -ne 0 ]; then
  echo "== BOARD INVALID: $ctlfail CONTROL fixtures diverged. These constructs were measured working on"
  echo "   2026-08-28; if they now diverge, fix that regression before reading any row above. =="
  exit 1
fi
echo "== controls all AGREE -- the harness is measuring SEM0, not itself =="
exit $fail
