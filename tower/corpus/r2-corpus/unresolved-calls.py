#!/usr/bin/env python3
"""List call targets in an emitted rzir file that nothing defines.

WHY. emit_s9's method dispatch ends in s9_method_generic, which emits `call <name>` for any method it does not
know. That is CORRECT for a user-defined method (the impl block is emitted as a function) and DANGLING for a
std method, which no emitted function provides. So the method-level gap is measurable POST HOC from the rzir,
with no change to the compiler and no risk to it -- unlike the expression-level gap, which needed a diagnostic
in s9expr's catch-all because an unlowered expression leaves no trace at all.

Runtime builtins (__*) are excluded: they are provided by the emitted preamble, not by an `fn` line.

  <rustc-built s1_parser> < probe.rs > p.rzir && python3 unresolved-calls.py p.rzir
"""
import sys, re
src = open(sys.argv[1]).read()
defined = set(re.findall(r'^fn ([A-Za-z_][\w:]*)', src, re.M))
called = re.findall(r'=\s*call\s+([A-Za-z_][\w:]*)', src)
for m in sorted({c for c in called if c not in defined and not c.startswith('__')}):
    print(m)
