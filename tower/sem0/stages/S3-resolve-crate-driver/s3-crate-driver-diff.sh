#!/usr/bin/env bash
# S3 CRATE-DRIVER differential harness (CD-0b instrument).
# A crate-driver fixture is a DIRECTORY under witnesses-crate/<name>/ : real .rs files + a manifest.txt
#   (<file> crate=<name> [root=1] [mod=1] [dep=1]) describing the module/crate tree.
#
# EXPECTED = rustc's cross-crate resolution map, built from the fixture as REAL FILES (the true crate graph):
#   each dep crate compiled `--crate-type=lib --crate-name <c>` -> rlib; the root crate compiled with `--extern`
#   (sibling `mod` files are real files rustc finds). The def-path set of each crate (from `-Zunpretty=hir-tree`,
#   crate-hash stripped, std/prelude/{use#} filtered) is emitted as `<crate>|<defpath>`, sorted-unique. That IS the
#   resolution target -- every item/mod/use/path binds to the same crate::def-path as rustc.
# ACTUAL = s1_parser's `//@CRATE`-mode resolution map (lines `CRATERES <crate>|<defpath>`) over the SAME fixture
#   presented as ONE DELIMITED STDIN IMAGE (//@CRATE + //@UNIT crate=/path=/[kind=dep]/[root=] boundaries). Until the
#   //@CRATE driver lands (CD-1+), the marker is an inert `//` comment and ACTUAL is empty -> RED (driver owed).
# BOOTSTRAP = rz==rc on the delimited image (the same dual as the other stages). Requires SP (L6.rzrs + cf.hoist) + CC.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; S1="$HERE/../S1-lex-parse-ast"
SP="${SP:?set SP}"; CC="${CC:?set CC}"
P="$S1/s1_parser.rs"
. "$(dirname "$0")/../s1-build.sh"   # builds $SP/s1.rc + $SP/s1.rz once per input set, then reuses them

# def-path set of ONE compiled crate (args after the crate name are extra rustc flags, e.g. --extern)
crate_defpaths(){ # $1=cratename $2=rootfile ; rest=flags
  local cn="$1" rf="$2"; shift 2
  rustc +nightly -Zunpretty=hir-tree --edition 2021 --crate-name "$cn" "$@" "$rf" 2>/dev/null \
    | grep -oE 'owner_id: DefId\(0:[0-9]+ ~ [a-z_]+\[[0-9a-f]+\]::[^),]+\)' \
    | sed -E 's/.*\[[0-9a-f]+\]:://; s/\)$//' | grep -vE '^std$|prelude|^\{use#' | sort -u \
    | sed "s/^/$cn|/"; }

fail=0
for d in "$HERE"/witnesses-crate/*/; do
  [ -f "$d/manifest.txt" ] || continue
  n="$(basename "$d")"; work="$SP/cd_$n"; rm -rf "$work"; mkdir -p "$work"
  # parse manifest into arrays
  roots=(); mods=(); deps=(); allcr=""
  while read -r f rest; do
    [ -z "${f:-}" ] && continue; case "$f" in \#*) continue;; esac
    cp "$d/$f" "$work/$f"
    cr="$(echo "$rest" | grep -oE 'crate=[A-Za-z0-9_]+' | sed 's/crate=//')"
    echo "$allcr" | grep -qw "$cr" || allcr="$allcr $cr"
    case "$rest" in *root=1*) roots+=("$cr:$f");; esac
    case "$rest" in *dep=1*) deps+=("$cr:$f");; esac
  done < "$d/manifest.txt"
  ( cd "$work"
    # EXPECTED: build deps as rlibs, then each root crate with --extern of every dep
    externs=""
    for de in "${deps[@]}"; do dc="${de%%:*}"; df="${de##*:}"; rustc +nightly --edition 2021 --crate-type=lib --crate-name "$dc" "$df" -o "lib$dc.rlib" 2>/dev/null; externs="$externs --extern $dc=lib$dc.rlib"; done
    : > oracle.txt
    for de in "${deps[@]}"; do dc="${de%%:*}"; df="${de##*:}"; crate_defpaths "$dc" "$df" --crate-type=lib >> oracle.txt; done
    for ro in "${roots[@]}"; do rc="${ro%%:*}"; rf="${ro##*:}"; crate_defpaths "$rc" "$rf" $externs >> oracle.txt; done
    sort -u oracle.txt -o oracle.txt
    # E0603 privacy oracle: the set of names rustc reports private (`NAME` is private) across the root crate builds.
    : > priv.txt; : > no.txt
    for ro in "${roots[@]}"; do rc="${ro%%:*}"; rf="${ro##*:}"
      err="$(rustc --edition 2021 --crate-name "$rc" $externs "$rf" 2>&1)"
      echo "$err" | grep -oE '`[a-zA-Z0-9_]+` is private' | sed -E 's/`//g; s/ is private//' >> priv.txt
      # E0432 unresolved-import: the missing NAME from `no `NAME` in ..`
      echo "$err" | grep -oE 'no `[a-zA-Z0-9_]+` in' | sed -E 's/no `//; s/` in//' >> no.txt
    done
    sort -u priv.txt -o priv.txt; sort -u no.txt -o no.txt
  )
  cp "$work/oracle.txt" "$SP/$n.cd.expect"
  cp "$work/priv.txt" "$SP/$n.cd.priv.expect"
  cp "$work/no.txt" "$SP/$n.cd.no.expect"
  # the DELIMITED STDIN IMAGE (the driver's input)
  { echo '//@CRATE'
    while read -r f rest; do
      [ -z "${f:-}" ] && continue; case "$f" in \#*) continue;; esac
      cr="$(echo "$rest" | grep -oE 'crate=[A-Za-z0-9_]+' | sed 's/crate=//')"
      kd=""; case "$rest" in *dep=1*) kd=" kind=dep";; esac
      rt=""; case "$rest" in *root=1*) rt=" root=1";; esac
      echo "//@UNIT crate=$cr path=$f$kd$rt"
      cat "$d/$f"
    done < "$d/manifest.txt"
  } > "$SP/$n.cd.in"
  "$SP/s1.rc" < "$SP/$n.cd.in" 2>/dev/null > "$SP/$n.cd.rc"
  "$SP/s1.rz" < "$SP/$n.cd.in" 2>/dev/null > "$SP/$n.cd.rz"
  if ! cmp -s "$SP/$n.cd.rc" "$SP/$n.cd.rz"; then echo "  FAIL BOOTSTRAP ($n): rz != rc"; fail=1; continue; fi
  grep -oE '^CRATERES .*' "$SP/$n.cd.rc" | sed 's/^CRATERES //' | sort -u > "$SP/$n.cd.actual"
  if cmp -s "$SP/$n.cd.actual" "$SP/$n.cd.expect"; then echo "  ok   CRATE ($n): crate-driver map == rustc ($(wc -l < "$SP/$n.cd.expect") def-paths)"
  else echo "  RED  CRATE ($n): driver owed (expected $(wc -l < "$SP/$n.cd.expect") crate::def-paths, got $(wc -l < "$SP/$n.cd.actual"))"; fail=1; fi
  # CRATEPRIV set -- cross-unit private-use == rustc E0603 (`NAME` is private)
  grep -oE '^CRATEPRIV .*' "$SP/$n.cd.rc" | sed 's/^CRATEPRIV //' | sort -u > "$SP/$n.cd.priv.actual"
  if cmp -s "$SP/$n.cd.priv.actual" "$SP/$n.cd.priv.expect"; then echo "  ok   PRIV  ($n): cross-unit private-use == rustc E0603 ($(wc -l < "$SP/$n.cd.priv.expect"))"
  else echo "  RED  PRIV  ($n): != rustc E0603"; diff "$SP/$n.cd.priv.expect" "$SP/$n.cd.priv.actual" | head -4; fail=1; fi
  # CRATENO set -- cross-unit unresolved-import == rustc E0432 (`no NAME in ..`)
  grep -oE '^CRATENO .*' "$SP/$n.cd.rc" | sed 's/^CRATENO //' | sort -u > "$SP/$n.cd.no.actual"
  if cmp -s "$SP/$n.cd.no.actual" "$SP/$n.cd.no.expect"; then echo "  ok   NO    ($n): cross-unit unresolved == rustc E0432 ($(wc -l < "$SP/$n.cd.no.expect"))"
  else echo "  RED  NO    ($n): != rustc E0432"; diff "$SP/$n.cd.no.expect" "$SP/$n.cd.no.actual" | head -4; fail=1; fi
  # CD-7 FILE mode: the SAME fixture as a manifest of //@UNIT file=<abspath> lines (NO inline content); the driver reads
  # the REAL .rs files from disk (fs::read_to_string). Its full output must == the inline image's (same resolution) + rz==rc.
  { echo '//@CRATE'
    while read -r f rest; do
      [ -z "${f:-}" ] && continue; case "$f" in \#*) continue;; esac
      cr="$(echo "$rest" | grep -oE 'crate=[A-Za-z0-9_]+' | sed 's/crate=//')"
      kd=""; case "$rest" in *dep=1*) kd=" kind=dep";; esac
      rt=""; case "$rest" in *root=1*) rt=" root=1";; esac
      echo "//@UNIT crate=$cr path=$f$kd$rt file=$d/$f"
    done < "$d/manifest.txt"
  } > "$SP/$n.cdf.in"
  "$SP/s1.rc" < "$SP/$n.cdf.in" 2>/dev/null | grep -E '^CRATE' | sort > "$SP/$n.cdf.rc"
  "$SP/s1.rz" < "$SP/$n.cdf.in" 2>/dev/null | grep -E '^CRATE' | sort > "$SP/$n.cdf.rz"
  grep -E '^CRATE' "$SP/$n.cd.rc" | sort > "$SP/$n.cd.inline.sorted"
  if ! cmp -s "$SP/$n.cdf.rc" "$SP/$n.cdf.rz"; then echo "  FAIL BOOTSTRAP-FILE ($n): rz != rc (file mode)"; fail=1;
  elif cmp -s "$SP/$n.cdf.rc" "$SP/$n.cd.inline.sorted"; then echo "  ok   FILE  ($n): real-file read == inline resolution ($(grep -cE '^CRATE' "$SP/$n.cdf.rc") lines)"
  else echo "  RED  FILE  ($n): real-file read != inline"; diff "$SP/$n.cd.inline.sorted" "$SP/$n.cdf.rc" | head -4; fail=1; fi
done
[ "$fail" = 0 ] && echo "== s3-crate-driver: BOOTSTRAP green (CRATE RED until the //@CRATE driver lands) ==" || echo "== s3-crate-driver: BOOTSTRAP broken =="
exit $fail
