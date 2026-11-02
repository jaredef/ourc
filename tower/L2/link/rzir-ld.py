#!/usr/bin/env python3
"""rzir-ld -- cross-file rzir linker (prototype; C port is rzir-link M5).
Reads N rzir units (files, or stdin). Symbol resolution across units:
  - globals: collect `gdef NAME SIZE`, assign 8-aligned addresses from 0x700000 (above code, below frame); rewrite
    every `vN: i32 = gref NAME` -> `const <addr>`; drop gdef lines.
  - functions: split into `ir main`/`fn NAME` blocks, DEDUP by name (headers define helpers
    in many units), emit the ir-main block first.
  - report unresolved globals (gref with no gdef) and unresolved calls (call NAME with no
    fn/ir NAME) to stderr; --strict errors on unresolved.
Usage: rzir-ld.py [--strict] unit1.rzir unit2.rzir ...   (or stdin for one unit)
"""
import sys, re
TARGETS = {
    'x86_64-linux': {'gbase': 0x700000},
    'arm64-linux':  {'gbase': 0x1000000},  # W9 (cf-arm64-op-widening): bumped 0x420000 → 0x1000000 to eliminate RX/RW vaddr overlap in ccc1.elf (RX can reach past 0x420000 for programs > ~130 KB code)
}
DEFAULT_TARGET = 'x86_64-linux'

# R2: Linux x86_64 -> Linux aarch64 syscall-nr translation.
# Applied only under --target=arm64-linux. rzlibc's C sources are authored
# against the Linux x86_64 syscall ABI (e.g. exit_group=60, write=1); on
# arm64-linux those must become aarch64 nrs (exit_group=94, write=64, ...).
# Applied at rzir-ld so it composes with GBASE (this file's R1 duty), stays
# upstream of rzir-opt/ra, and needs no ccc rebuild.
SYSCALL_XLAT = {
    'arm64-linux': {
        0: 63,     # read
        1: 64,     # write
        2: 1024,   # open   (aarch64 has no `open`; 1024 is the compat nr)
        3: 57,     # close
        8: 62,     # lseek
        12: 214,   # brk
        60: 94,    # exit_group
    },
}

def main():
    argv = sys.argv[1:]
    strict = "--strict" in argv
    target = DEFAULT_TARGET
    rest = []
    for a in argv:
        if a == "--strict":
            continue
        if a.startswith("--target="):
            target = a.split("=", 1)[1]
            continue
        rest.append(a)
    if target not in TARGETS:
        sys.stderr.write("rzir-ld: unrecognized --target=%s; recognized: %s\n"
                         % (target, ", ".join(sorted(TARGETS))))
        sys.exit(2)
    GBASE = TARGETS[target]['gbase']
    args = rest
    texts = [open(f).read() for f in args] if args else [sys.stdin.read()]

    # pass 1: global symbol table
    gaddr = {}; cur = GBASE
    for t in texts:
        for line in t.splitlines():
            m = re.match(r'gdef (\S+) (\d+)\s*$', line)
            if m and m.group(1) not in gaddr:
                gaddr[m.group(1)] = cur; cur += (int(m.group(2)) + 7) // 8 * 8

    # pass 2: strip gdefs, resolve grefs, collect function blocks (dedup by name)
    unresolved_g = set(); blocks = {}; order = []; have_main = [False]
    def flush(name, buf):
        if name is None: return
        if name not in blocks:
            blocks[name] = "\n".join(buf); order.append(name)
        if name == "main": have_main[0] = True
    for t in texts:
        cur_name = None; buf = []
        for line in t.splitlines():
            if re.match(r'gdef \S+ \d+\s*$', line): continue
            m = re.match(r'(\s*v\d+: i32 = )gref (\S+)\s*$', line)
            if m:
                nm = m.group(2)
                if nm in gaddr: line = "%sconst %d" % (m.group(1), gaddr[nm])
                else: unresolved_g.add(nm)
            h = re.match(r'(ir main|fn ([A-Za-z_0-9]+))', line)
            if h:
                flush(cur_name, buf)
                cur_name = "main" if h.group(1) == "ir main" else h.group(2)
                buf = [line]
            elif cur_name is not None:
                buf.append(line)
        flush(cur_name, buf)

    called = set()
    for b in blocks.values():
        called |= set(re.findall(r'\bcall ([A-Za-z_0-9]+)', b))
    unresolved_c = sorted(called - set(blocks))

    for g in sorted(unresolved_g): sys.stderr.write("rzir-ld: unresolved global: %s\n" % g)
    for c in unresolved_c:        sys.stderr.write("rzir-ld: unresolved call:   %s\n" % c)
    sys.stderr.write("rzir-ld: %d units, %d globals, %d functions; %d unresolved globals, %d unresolved calls\n"
                     % (len(texts), len(gaddr), len(blocks), len(unresolved_g), len(unresolved_c)))
    if strict and (unresolved_g or unresolved_c): sys.exit(2)

    ordered = (["main"] if have_main[0] else []) + [n for n in order if n != "main"]

    # R2: per-function syscall-nr translation under --target=arm64-linux.
    # For each `syscall vNr vA vB vC` line, look up vNr's `const K` def; if K
    # is a known x86_64 syscall nr, insert a fresh `const K_new` above the
    # syscall line and rewrite the syscall's first operand to that fresh vN.
    # Fresh-vN + insertion (not in-place const rewrite) avoids collision when
    # the same vN is reused elsewhere as a plain literal (K in {0,1,2,3,...}
    # is common in unrelated code).
    xlat = SYSCALL_XLAT.get(target)
    if xlat:
        n_rewrites = 0
        SYSC = re.compile(r'^(\s*v(\d+):\s*i32 = )syscall\s+v(\d+)((?:\s+v\d+)*)\s*$')
        CDEF = re.compile(r'^\s*v(\d+):\s*i32 = const (-?\d+)\s*$')
        for name in ordered:
            lines = blocks[name].split("\n")
            # collect const defs and used vN slots
            cdef = {}         # vN -> K
            used = set()
            for ln in lines:
                m = CDEF.match(ln)
                if m: cdef[int(m.group(1))] = int(m.group(2))
                for mm in re.finditer(r'\bv(\d+)\b', ln): used.add(int(mm.group(1)))
            next_v = (max(used) + 1) if used else 0
            out = []
            for ln in lines:
                m = SYSC.match(ln)
                if m:
                    nr_v = int(m.group(3))
                    if nr_v in cdef and cdef[nr_v] in xlat:
                        new_k = xlat[cdef[nr_v]]
                        indent = re.match(r'(\s*)', ln).group(1)
                        out.append("%sv%d: i32 = const %d" % (indent, next_v, new_k))
                        ln = "%ssyscall v%d%s" % (m.group(1), next_v, m.group(4))
                        used.add(next_v); next_v += 1; n_rewrites += 1
                out.append(ln)
            blocks[name] = "\n".join(out)
        sys.stderr.write("rzir-ld: syscall-nr xlat (%s): %d rewrites\n" % (target, n_rewrites))

    # CFML per-program frame base: arena_top (cur) > 128MB -> prepend a `heaptop <cur>` directive so cf places
    # the frame arena / sp-save / memsz above the arenas (else the fixed frame region collides with a large
    # program's heap). Below 128MB no directive -> byte-identical output. opt/ra forward the pre-block line.
    prefix = ("heaptop %d\n" % cur) if cur > 134217728 else ""
    sys.stdout.write(prefix + "\n".join(blocks[n] for n in ordered).rstrip("\n") + "\n")

main()
