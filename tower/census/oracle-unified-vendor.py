#!/usr/bin/env python3
# oracle-unified-vendor.py <compiler_dir> -- emit ONE unified [dependencies] over the UNION of every compiler
# crate's EXTERNAL deps, merging feature sets per crate so cargo unifies features (e.g. smallvec gets
# const_generics + union + may_dangle from whichever crate needs each). Path (in-tree) deps are excluded.
# Windows/msvc target deps excluded (host is linux). Prints a Cargo.toml [dependencies] block.
import sys, os, tomli
comp = sys.argv[1]
# crate-name -> {package, versions:set, features:set, default_features:bool}
acc = {}
def add(name, spec):
    if isinstance(spec, str):
        e = acc.setdefault(name, {"package": name, "vers": set(), "feats": set(), "def": True})
        e["vers"].add(spec); return
    if "path" in spec: return
    pkg = spec.get("package", name)
    e = acc.setdefault(name, {"package": pkg, "vers": set(), "feats": set(), "def": True})
    e["package"] = pkg
    if "version" in spec: e["vers"].add(spec["version"])
    for f in spec.get("features", []): e["feats"].add(f)
    if spec.get("default-features") is False: e["def"] = False
for d in sorted(os.listdir(comp)):
    ct = os.path.join(comp, d, "Cargo.toml")
    if not os.path.isfile(ct): continue
    data = tomli.load(open(ct, "rb"))
    for name, spec in data.get("dependencies", {}).items(): add(name, spec)
    for tkey, tval in data.get("target", {}).items():
        if "windows" in tkey.lower() or "msvc" in tkey.lower(): continue
        for name, spec in tval.get("dependencies", {}).items(): add(name, spec)
print("[dependencies]")
for name in sorted(acc):
    e = acc[name]
    ver = sorted(e["vers"])[-1] if e["vers"] else "*"   # highest requested version req
    parts = [f'version = "{ver}"']
    if e["package"] != name: parts.append(f'package = "{e["package"]}"')
    if e["feats"]: parts.append("features = [" + ", ".join(f'"{f}"' for f in sorted(e["feats"])) + "]")
    if not e["def"]: parts.append("default-features = false")
    print(f"{name} = {{ {', '.join(parts)} }}")
