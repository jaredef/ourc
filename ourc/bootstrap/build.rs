// build.rs -- embed the vendored source tree (tree/) into the binary, so an installed ourc-bootstrap needs no
// checkout. Without tree/ the list is empty and the binary must be run with --repo <checkout>.
use std::fs;
use std::path::{Path, PathBuf};

fn walk(root: &Path, dir: &Path, out: &mut Vec<PathBuf>) {
    let mut ents: Vec<_> = match fs::read_dir(dir) { Ok(r) => r.filter_map(|e| e.ok()).map(|e| e.path()).collect(), Err(_) => return };
    ents.sort();
    for p in ents {
        if p.is_dir() { walk(root, &p, out) } else { out.push(p) }
    }
}

fn main() {
    let man = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let tree = man.join("tree");
    println!("cargo:rerun-if-changed=tree");
    let mut files = Vec::new();
    walk(&tree, &tree, &mut files);
    let mut s = String::from("pub static TREE: &[(&str, &[u8])] = &[\n");
    for f in &files {
        let rel = f.strip_prefix(&tree).unwrap().to_str().unwrap().to_string();
        s.push_str(&format!("    ({:?}, include_bytes!({:?})),\n", rel, f.to_str().unwrap()));
    }
    s.push_str("];\n");
    let out = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("embed.rs");
    fs::write(out, s).unwrap();
}
