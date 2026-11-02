#!/usr/bin/env python3
# M1 (P-L6RA-DISCIPLINE-LINT) -- make C-L6-LEAN-DISCIPLINE checkable.
#
# Flags the ONE empirically-harmful shape in emit_s9's cycle fns: a MUTATE-THEN-RETURN of a cycle-call result --
# a `let mut X` reassigned from a call (`X = f(...)`) inside a for/while loop, where the fn then returns X. This is
# C-L6-MUTATE-THEN-RETURN (the pre-8e251389 s9_bv bug that L6 miscompiles under register pressure). The lean fix is to
# return the value directly (no mutable-across-cycle-call accumulator) -- see s9_bv (index-tracked early-return).
#
# Scope: fns named s9_* / emit_* / e* in s1_parser.rs (the emitter). Heuristic (brace-depth fn split + per-fn regex),
# a TRIPWIRE not a proof: a flag is a review prompt, not a hard error. Exit 1 if any flag, 0 if clean.
import re, sys

PATH = sys.argv[1] if len(sys.argv) > 1 else (
    __file__.rsplit("/apparatus/", 1)[0] +
    "/apparatus/arcs/2026-08-04-rustc-frontend-compilation/sub-arcs/S1-lex-parse-ast/s1_parser.rs")

src = open(PATH).read().splitlines()

# split into top-level fns by brace depth (a fn starts at a line beginning `fn NAME`)
fns = []
i = 0
while i < len(src):
    m = re.match(r'^fn ([A-Za-z0-9_]+)', src[i])
    if not m:
        i += 1; continue
    name = m.group(1)
    depth = 0; start = i; seen = False
    while i < len(src):
        depth += src[i].count('{') - src[i].count('}')
        if '{' in src[i]:
            seen = True
        i += 1
        if seen and depth <= 0:
            break
    fns.append((name, start + 1, src[start:i]))

EMIT = re.compile(r'^(s9_|s9[a-z]|emit_|e[a-z])')
flags = []
for name, lineno, body in fns:
    if not EMIT.match(name):
        continue
    text = "\n".join(body)
    # mutable locals declared in this fn
    muts = set(re.findall(r'\blet mut ([A-Za-z0-9_]+)\b', text))
    if not muts:
        continue
    # is there a for/while loop? (the cycle-call must be inside one to be the harmful shape)
    if not re.search(r'\b(for|while)\b', text):
        continue
    for v in muts:
        # X reassigned from a RECURSION-CYCLE call -- the proven-harmful shape (the value is held across the cycle call
        # under register pressure). The cycle is s9expr and its members (s9_bv/s9_walk/s9_stmt). A reassignment from a
        # NON-cycle helper (e.g. emit_mod_items' compile-time mk) is benign and not flagged.
        reassign_call = re.search(r'\b' + re.escape(v) + r'\s*=\s*(s9expr|s9_bv|s9_walk|s9_stmt)\s*\(', text)
        if not reassign_call:
            continue
        # ...and the fn RETURNS X (explicit `return X` or a bare tail `X` / `... else { X }`)
        returns_x = re.search(r'\breturn\s+' + re.escape(v) + r'\b', text) or \
                    re.search(r'(?:^|[{;]\s*)' + re.escape(v) + r'\s*\}', text) or \
                    re.search(r'\belse\s*\{\s*' + re.escape(v) + r'\s*\}', text)
        if returns_x:
            flags.append((name, lineno, v))

if flags:
    print("LINT-LEAN-DISCIPLINE: %d flag(s) -- mutate-then-return of a (cycle-)call result (C-L6-MUTATE-THEN-RETURN):" % len(flags))
    for name, lineno, v in flags:
        print("  s1_parser.rs:%d  fn %s: `let mut %s` reassigned from a call in a loop and returned -> lean-fix (return the value directly)" % (lineno, name, v))
    sys.exit(1)
print("LINT-LEAN-DISCIPLINE: clean (no mutate-then-return-of-call in emit cycle fns).")
sys.exit(0)
