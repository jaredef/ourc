#!/usr/bin/env python3
# splice-crate.py <lib.rs> -- emit the whole crate as ONE file for whole-crate checking (T2). Each
# non-#[cfg(test)] `mod X;` is inlined as `mod X { <recursively spliced X.rs or X/mod.rs> }`, faithful to
# Rust's file-module == inline-module equivalence. MEASUREMENT-ONLY harness tooling (oracle-class): the output
# is fed to SEM0 for a T2 verdict, never into the produced chain.
import sys, os, re
def splice(path, seen):
    if path in seen: return ""
    seen.add(path)
    d = os.path.dirname(path)
    stem = os.path.basename(path)[:-3] if path.endswith('.rs') else os.path.basename(path)
    base = d if stem in ('mod', 'lib', 'main') else os.path.join(d, stem)   # a FILE module foo.rs has its submodules in foo/, not the parent dir (Rust module-path rule)
    out = []
    lines = open(path, encoding='utf-8', errors='replace').read().split('\n')
    i = 0
    istack = []      # names of enclosing INLINE `mod X {` blocks (nested file-mods live under base/X/..)
    iclose = []      # brace-depth below which each inline mod's body has closed
    depth = 0        # running brace depth (string/comment-stripped)
    def _strip(t):   # remove // comments, "..." strings, '.' chars so their braces don't perturb the count
        return re.sub(r'//.*$', '', re.sub(r'"(?:\\.|[^"\\])*"', '', re.sub(r"'(?:\\.|[^'])'", '', t)))
    while i < len(lines):
        ln = lines[i]
        moddir = os.path.join(base, *istack) if istack else base
        # a #[cfg(test)] mod (and its `mod X;`): SKIP by default (T2/LOWER accept boards must not see tests,
        # to keep their baselines). With SPLICE_TESTS=1 (the T3-RUN/harness flow) INLINE it instead -- drop the
        # cfg attr and splice the test module in like any other file module, so the harness can discover its
        # #[test] fns. See apparatus/arcs/2026-09-20-entry-harness-in-chain (R2).
        if re.match(r'\s*#\[cfg\(test\)\]\s*$', ln):
            j = i+1
            while j < len(lines) and lines[j].strip()=='':
                j += 1
            mt = re.match(r'\s*(pub\b\s*(?:\([^)]*\)\s*)?)?mod\s+([a-z_0-9]+)\s*;\s*(?://.*)?$', lines[j]) if j < len(lines) else None
            if mt:
                if os.environ.get('SPLICE_TESTS'):
                    tname = mt.group(2)
                    cand = [os.path.join(moddir, tname+'.rs'), os.path.join(moddir, tname, 'mod.rs')]
                    f = next((c for c in cand if os.path.isfile(c)), None)
                    if f:
                        out.append(f"mod {tname} {{"); out.append(splice(f, seen)); out.append("}")
                        i = j+1; continue
                    # no file (inline #[cfg(test)] mod X { .. }) -> fall through so the block body is emitted
                else:
                    i = j+1; continue
        # `extern crate self as NAME;` is a no-op for resolution (it only aliases the current crate for
        # derive-generated paths); SEM0's resolver would treat it as a cross-crate file splice and refuse. Drop it.
        if re.match(r'\s*extern\s+crate\s+self\s+as\s+[a-z_0-9]+\s*;\s*$', ln):
            i += 1; continue
        # RFC-2632 `impl<..> const Trait for T` -> strip the `const` (SEM0's impl parser only elides `const`
        # ADJACENT to `impl`, not after the generic-param list; const-ness is inert to resolve/accept). See
        # locales/rustc-data-structures. Off the SEM0 self-emit critical path.
        mci = re.match(r'(\s*(?:pub\s+)?impl\b(?:\s*<[^>]*>)?\s+)const\s+(.*)$', ln)
        if mci:
            ln = mci.group(1) + mci.group(2)
        # `#[path = "rel/file.rs"] mod NAME;` -- explicit module path (same line, or attr on the line above).
        mp = re.match(r'\s*#\[path\s*=\s*"([^"]+)"\]\s*(?:(pub\b\s*(?:\([^)]*\)\s*)?)?mod\s+([a-z_0-9]+)\s*;)?\s*(?://.*)?$', ln)
        if mp and mp.group(3) is None and i+1 < len(lines):
            nx = re.match(r'\s*(pub\b\s*(?:\([^)]*\)\s*)?)?mod\s+([a-z_0-9]+)\s*;\s*(?://.*)?$', lines[i+1])
            if nx:
                rel, vis2, nm2 = mp.group(1), (nx.group(1) or ''), nx.group(2); i += 1
                f = os.path.join(moddir, rel)
                if os.path.isfile(f):
                    out.append(f"{vis2}mod {nm2} {{"); out.append(splice(f, seen)); out.append("}"); i += 1; continue
        if mp and mp.group(3) is not None:
            rel, vis2, nm2 = mp.group(1), (mp.group(2) or ''), mp.group(3)
            f = os.path.join(moddir, rel)
            if os.path.isfile(f):
                out.append(f"{vis2}mod {nm2} {{"); out.append(splice(f, seen)); out.append("}"); i += 1; continue
        m = re.match(r'(\s*)(pub\b\s*(?:\([^)]*\)\s*)?)?mod\s+([a-z_0-9]+)\s*;\s*(?://.*)?$', ln)
        if m:
            ind, vis, name = m.group(1), (m.group(2) or ''), m.group(3)
            cand = [os.path.join(moddir, name+'.rs'), os.path.join(moddir, name, 'mod.rs')]
            f = next((c for c in cand if os.path.isfile(c)), None)
            if f:
                out.append(f"{ind}{vis}mod {name} {{")
                out.append(splice(f, seen))
                out.append(f"{ind}}}")
                i += 1; continue
        out.append(ln)
        _c = _strip(ln); o = _c.count('{'); cl = _c.count('}')
        mo = re.match(r'\s*(?:pub\b\s*(?:\([^)]*\)\s*)?)?mod\s+([a-z_0-9]+)\s*\{\s*(?://.*)?$', ln)
        if mo and o == 1 and cl == 0:
            istack.append(mo.group(1)); iclose.append(depth)
        depth += o - cl
        while iclose and depth <= iclose[-1]:
            istack.pop(); iclose.pop()
        i += 1
    return '\n'.join(out)
seen = set()
main_out = splice(sys.argv[1], seen)
# SPLICE_ALLMODS (opt-in, LOWER-coverage only): after faithful splicing, some file-modules are declared inside
# crate-specific macros (declare_passes!, supported_targets!) the text-splicer cannot follow, so their bodies
# never reach the parser. For whole-crate LOWERING coverage, append every on-disk .rs under the crate root that
# normal splicing did not `seen`, each wrapped in a uniquely-named module (avoids name collision; module PATH is
# irrelevant to per-item LOWERING). Default OFF so T2/resolve gates keep faithful structure.
if os.environ.get('SPLICE_ALLMODS'):
    root = os.path.dirname(sys.argv[1])
    extra = []
    n = 0
    for dp, _dn, fns in os.walk(root):
        for fn in sorted(fns):
            if not fn.endswith('.rs'):
                continue
            fp = os.path.join(dp, fn)
            if fp in seen:
                continue
            if fn in ('tests.rs',) or os.sep+'tests'+os.sep in fp:
                continue
            body = splice(fp, seen)
            if body.strip():
                extra.append(f"mod __covmod_{n} {{"); extra.append(body); extra.append("}"); n += 1
    if extra:
        main_out = main_out + "\n// --- SPLICE_ALLMODS coverage completion (" + str(n) + " extra modules) ---\n" + "\n".join(extra)
print(main_out)
