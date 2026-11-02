#!/usr/bin/env python3
# S3 crate driver: read a crate's module TREE from the filesystem and assemble it into a single
# inline-module source (mirrors L6's P-MOD-FILE-SPLICE, but as a standalone driver component so
# s1_parser stays dual-compilable on stdin -- reading module files itself needs a stage0 fs-read
# idiom, a separate keeper-gated hoist). Each `mod NAME;` -> `mod NAME { <NAME's assembled source> }`.
# Rust module-path rule: for `mod NAME;` in a file whose module directory is D, NAME's file is
# D/NAME.rs or D/NAME/mod.rs; NAME's own submodules live in D/NAME/.
import sys, os, re
MOD_DECL = re.compile(r'\bmod\s+([A-Za-z_][A-Za-z0-9_]*)\s*;')
def assemble(path, moddir):
    src = open(path).read()
    def repl(m):
        name = m.group(1)
        f_rs  = os.path.join(moddir, name + ".rs")
        f_mod = os.path.join(moddir, name, "mod.rs")
        if os.path.exists(f_rs):
            child = assemble(f_rs, os.path.join(moddir, name))   # submodules of NAME live in D/NAME/
        elif os.path.exists(f_mod):
            child = assemble(f_mod, os.path.join(moddir, name))
        else:
            return m.group(0)   # unresolved mod file -> leave the decl (rustc would E0583; not our concern here)
        return "mod %s { %s }" % (name, child)
    return MOD_DECL.sub(repl, src)
def main():
    root = sys.argv[1]
    sys.stdout.write(assemble(root, os.path.dirname(root) or "."))
main()
