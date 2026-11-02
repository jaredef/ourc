#!/usr/bin/env python3
# parse-deps.py <Cargo.toml> -- emit each dependency, classified, for the oracle builder:
#   EXT   <alias> <package> <cargo-dep-line>     external crates.io dep (rename/features preserved)
#   INTREE <alias> <path>                         in-tree path dep
# Includes [dependencies] and portable [target.'cfg(unix|...)'.dependencies] (host is linux); skips windows.
import sys, tomli
d = tomli.load(open(sys.argv[1], 'rb'))
def emit(name, spec):
    if isinstance(spec, str):
        print(f"EXT\t{name}\t{name}\t{name} = \"{spec}\""); return
    if 'path' in spec:
        print(f"INTREE\t{name}\t{spec['path']}"); return
    pkg = spec.get('package', name)
    parts = []
    if 'version' in spec: parts.append(f'version = "{spec["version"]}"')
    if 'package' in spec: parts.append(f'package = "{spec["package"]}"')
    if 'features' in spec: parts.append("features = [" + ", ".join(f'"{f}"' for f in spec['features']) + "]")
    if spec.get('default-features') is False: parts.append("default-features = false")
    line = f"{name} = {{ {', '.join(parts)} }}"
    print(f"EXT\t{name}\t{pkg}\t{line}")
for name, spec in d.get('dependencies', {}).items():
    emit(name, spec)
for tkey, tval in d.get('target', {}).items():
    if 'windows' in tkey.lower() or 'msvc' in tkey.lower(): continue
    for name, spec in tval.get('dependencies', {}).items():
        emit(name, spec)
