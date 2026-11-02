#!/bin/bash
# P-MC-CHECKFAULT: the minimal in-context trigger for the segfault that blocks C-ENVMISS and C-IFSCOPE.
#
# Reproduce with a compiler built from a tree whose otypeck seq_slice rule propagates NEGATIVES instead of
# masking them to 201 (either spelling -- `if t0<0 { t0 }` or `if t0<0 { 0-3 }`):
#
#     sed -i 's/if t0==0-3 { 0-3 } else if t0==2 { 2 } else { 201 } },/if t0<0 { 0-3 } else if t0==2 { 2 } else { 201 } },/' src/otypeck.rs
#     OURC_OUT=$J/ourc.v2 bash build-ourc.sh
#
# Then this script shows the trigger is a THREE-PART conjunction, all three parts required:
#   (1) oparse truncated to its first 32 top-level fn items, PLUS
#   (2) the one-line definition `pub fn find_fun_ix(fs:&Vec<Fun>, n:&str)->i32{...}` (oparse.rs:1633), PLUS
#   (3) BOTH of its call sites in omono (lines 67 and 72) live. Either one alone does NOT fault.
#
# Refuted along the way: it is not a count/capacity cliff (three unrelated trivial fns added at n=32 are
# fine, and a sweep of 1..64 deliberate diagnostics never faults), and it is not the call sites' own
# argument shapes (renaming the definition leaves the calls dangling with no fault).
set -u
J="${J:?set J to the driver scratch dir}"; V="${V:-$J/ourc.v2}"
cd "$(dirname "$0")/.."
rm -rf "$J/srcx"; cp -r src "$J/srcx"
cat > "$J/trunc.py" <<'PY'
import sys,re
n=int(sys.argv[1]); s=open(sys.argv[2]).read().split("\n")
st=[i for i,l in enumerate(s) if re.match(r'^(pub )?fn ',l)]
print("\n".join(s if n>=len(st) else s[:st[n]]))
PY
mk(){ bash "$J/mkx.sh"; (ulimit -s unlimited; timeout 300 "$V" "$J/kx.rs" >/dev/null 2>&1); echo "  $1 exit=$?"; }
base(){ { python3 "$J/trunc.py" 32 src/oparse.rs; sed -n '1633p' src/oparse.rs; } > "$J/srcx/oparse.rs"; cp src/omono.rs "$J/srcx/omono.rs"; }
python3 "$J/trunc.py" 32 src/oparse.rs > "$J/srcx/oparse.rs"; mk "(1) alone: 32 items, no definition"
base; sed -i 's/oparse::find_fun_ix/oparse::NOPE/g' "$J/srcx/omono.rs"; mk "(1)+(2), no call site"
base; sed -i '67s/oparse::find_fun_ix/oparse::NOPE/' "$J/srcx/omono.rs"; mk "(1)+(2)+ only the :72 call"
base; sed -i '72s/oparse::find_fun_ix/oparse::NOPE/' "$J/srcx/omono.rs"; mk "(1)+(2)+ only the :67 call"
base; mk "all three  -- EXPECT exit=139"
