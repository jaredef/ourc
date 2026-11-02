#!/bin/bash
# C-CLOSARITY: every call to a lifted closure must pass exactly as many arguments as the closure's
# definition takes. Reads the emitted IR of a program and reports any mismatch. Usage:
#   OURS=<ourc binary> bash probes/closure-arity-audit.sh [program.rs]
set -u
J="${J:?set J to the driver scratch dir}"; O="${OURS:?set OURS to an ourc binary}"
P="${1:-$(dirname "$0")/../progs/real_rust_closarity.rs}"
LLC="${LLC:-$(ls ~/.rustup/toolchains/*/lib/rustlib/*/bin/llc|head -1)}"
(ulimit -s unlimited; timeout 300 "$O" "$P" --emit-ir 2>/dev/null) | sed -n '/^\(@\|declare\|define\)/,$p' > "$J/aud.ll"
python3 - "$J/aud.ll" <<'PY'
import re,sys
L=open(sys.argv[1]).read()
ar={m.group(1):len([x for x in m.group(2).split(',') if x.strip()]) for m in re.finditer(r'define i64 @(\$clos\d+)\(([^)]*)\)', L)}
bad=[]; tot=0
for c in re.finditer(r'call i64 @(\$clos\d+)\(([^)]*)\)', L):
    tot+=1
    na=len([x for x in c.group(2).split(',') if x.strip()])
    if ar.get(c.group(1)) != na: bad.append((c.group(1), ar.get(c.group(1)), na))
print("  closure-arity audit: %d call(s) of %d mismatched" % (len(bad), tot))
for nm,want,got in bad[:8]: print("    %s takes %s, called with %s" % (nm, want, got))
sys.exit(1 if bad else 0)
PY
