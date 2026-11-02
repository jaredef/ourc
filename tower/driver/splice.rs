// splice.rs -- flatten a crate into ONE source file: each non-#[cfg(test)] `mod X;` is inlined as
// `mod X { <recursively spliced X.rs or X/mod.rs> }`, faithful to Rust's file-module == inline-module rule.
// Port of tower/census/splice-crate.py (no Python in the tower), built by semc with the tower's own SEM0.
//
// stdin protocol (SEM0-built binaries cannot yet read argv, and fs::read_to_string on a missing file aborts
// instead of returning Err, so the caller supplies both the arguments and the file-existence facts):
//   line 1:  <root lib.rs path>
//   line 2:  tests=0 | tests=1          (tests=1 = SPLICE_TESTS: inline #[cfg(test)] file modules)
//   rest:    every existing .rs path the crate may reach (the caller's `find`), one per line
// stdout: the spliced crate.
use std::collections::HashSet;
use std::io::Read;

fn is_ws(c: char) -> bool { c == ' ' || c == '\t' || c == '\r' }
fn is_word(c: char) -> bool { c.is_ascii_alphanumeric() || c == '_' }
fn is_name(c: char) -> bool { c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' }
fn skip_ws(cs: &Vec<char>, mut i: usize) -> usize { while i < cs.len() && is_ws(cs[i]) { i += 1; } i }
fn sub(cs: &Vec<char>, a: usize, b: usize) -> String { let mut s = String::new(); let mut i = a; while i < b { s.push(cs[i]); i += 1; } s }
fn at(cs: &Vec<char>, i: usize, w: &str) -> bool {
    let wc: Vec<char> = w.chars().collect();
    if i + wc.len() > cs.len() { return false; }
    let mut k = 0; while k < wc.len() { if cs[i + k] != wc[k] { return false; } k += 1; }
    true
}
// `\s*(?://.*)?$` from i
fn tail_ok(cs: &Vec<char>, i: usize) -> bool { let j = skip_ws(cs, i); j == cs.len() || at(cs, j, "//") }

// `pub\b\s*(?:\([^)]*\)\s*)?` at i -> end index (vis = cs[i..end]); None if no `pub` here
fn vis_end(cs: &Vec<char>, i: usize) -> Option<usize> {
    if !at(cs, i, "pub") { return None; }
    let mut j = i + 3;
    if j < cs.len() && is_word(cs[j]) { return None; }
    j = skip_ws(cs, j);
    if j < cs.len() && cs[j] == '(' {
        let mut k = j + 1;
        while k < cs.len() && cs[k] != ')' { k += 1; }
        if k < cs.len() { j = skip_ws(cs, k + 1); }
    }
    Some(j)
}
// `mod\s+([a-z_0-9]+)\s*` at i -> (name, index after trailing ws)
fn mod_name(cs: &Vec<char>, i: usize) -> Option<(String, usize)> {
    if !at(cs, i, "mod") { return None; }
    let j = i + 3;
    if j >= cs.len() || !is_ws(cs[j]) { return None; }
    let a = skip_ws(cs, j);
    let mut b = a; while b < cs.len() && is_name(cs[b]) { b += 1; }
    if b == a { return None; }
    Some((sub(cs, a, b), skip_ws(cs, b)))
}
// `(pub..)?mod NAME\s*;\s*(//.*)?$` at i -> (vis, name)
fn mod_decl_at(cs: &Vec<char>, i: usize) -> Option<(String, String)> {
    let mut j = i; let mut vis = String::new();
    match vis_end(cs, i) { Some(e) => { vis = sub(cs, i, e); j = e; } None => {} }
    match mod_name(cs, j) {
        Some((name, k)) => { if k < cs.len() && cs[k] == ';' && tail_ok(cs, k + 1) { Some((vis, name)) } else { None } }
        None => None,
    }
}
// whole line `(\s*)(pub..)?mod NAME;` -> (indent, vis, name)
fn mod_decl(ln: &str) -> Option<(String, String, String)> {
    let cs: Vec<char> = ln.chars().collect();
    let i = skip_ws(&cs, 0);
    match mod_decl_at(&cs, i) { Some((vis, name)) => Some((sub(&cs, 0, i), vis, name)), None => None }
}
// whole line `\s*(pub..)?mod NAME\s*\{\s*(//.*)?$` -> name
fn mod_open(ln: &str) -> Option<String> {
    let cs: Vec<char> = ln.chars().collect();
    let mut j = skip_ws(&cs, 0);
    match vis_end(&cs, j) { Some(e) => { j = e; } None => {} }
    match mod_name(&cs, j) {
        Some((name, k)) => { if k < cs.len() && cs[k] == '{' && tail_ok(&cs, k + 1) { Some(name) } else { None } }
        None => None,
    }
}
// `\s*#\[path\s*=\s*"([^"]+)"\]\s*(mod-decl)?\s*(//.*)?$` -> PathAttr (has_decl = the mod decl is on this line).
// A flat struct, not Option<(String, Option<(String,String)>)>: SEM0 miscompiles that nested destructure.
struct PathAttr { rel: String, has_decl: bool, vis: String, name: String }
fn path_attr(ln: &str) -> Option<PathAttr> {
    let cs: Vec<char> = ln.chars().collect();
    let mut j = skip_ws(&cs, 0);
    if !at(&cs, j, "#[path") { return None; }
    j = skip_ws(&cs, j + 6);
    if j >= cs.len() || cs[j] != '=' { return None; }
    j = skip_ws(&cs, j + 1);
    if j >= cs.len() || cs[j] != '"' { return None; }
    let a = j + 1; let mut b = a; while b < cs.len() && cs[b] != '"' { b += 1; }
    if b == a || b >= cs.len() || b + 1 >= cs.len() || cs[b + 1] != ']' { return None; }
    let rel = sub(&cs, a, b);
    let k = skip_ws(&cs, b + 2);
    if tail_ok(&cs, k) { return Some(PathAttr { rel: rel, has_decl: false, vis: String::new(), name: String::new() }); }
    match mod_decl_at(&cs, k) { Some((v, n)) => Some(PathAttr { rel: rel, has_decl: true, vis: v, name: n }), None => None }
}
fn is_extern_self(ln: &str) -> bool {
    let t: Vec<&str> = ln.split_whitespace().collect();
    if t.len() == 5 && t[0] == "extern" && t[1] == "crate" && t[2] == "self" && t[3] == "as" && t[4].ends_with(";") {
        let n: Vec<char> = t[4].chars().collect();
        if n.len() < 2 { return false; }
        let mut k = 0; while k + 1 < n.len() { if !is_name(n[k]) { return false; } k += 1; }
        return true;
    }
    if t.len() == 6 && t[0] == "extern" && t[1] == "crate" && t[2] == "self" && t[3] == "as" && t[5] == ";" {
        let n: Vec<char> = t[4].chars().collect();
        let mut k = 0; while k < n.len() { if !is_name(n[k]) { return false; } k += 1; }
        return n.len() > 0;
    }
    false
}
// `(\s*(?:pub\s+)?impl\b(?:\s*<[^>]*>)?\s+)const\s+(.*)$` -> group1 + group2
fn strip_impl_const(ln: &str) -> Option<String> {
    let cs: Vec<char> = ln.chars().collect();
    let mut j = skip_ws(&cs, 0);
    if at(&cs, j, "pub") && j + 3 < cs.len() && is_ws(cs[j + 3]) { j = skip_ws(&cs, j + 3); }
    if !at(&cs, j, "impl") { return None; }
    j += 4;
    if j < cs.len() && is_word(cs[j]) { return None; }
    let k = skip_ws(&cs, j);
    if k < cs.len() && cs[k] == '<' {
        let mut e = k + 1; while e < cs.len() && cs[e] != '>' { e += 1; }
        if e < cs.len() { j = e + 1; }
    }
    if j >= cs.len() || !is_ws(cs[j]) { return None; }
    let g1 = skip_ws(&cs, j);
    if !at(&cs, g1, "const") || g1 + 5 >= cs.len() || !is_ws(cs[g1 + 5]) { return None; }
    let rest = skip_ws(&cs, g1 + 5);
    let mut s = sub(&cs, 0, g1); s.push_str(&sub(&cs, rest, cs.len()));
    Some(s)
}
// net brace count of a line after removing char literals, then "..." strings, then // comments
fn braces(ln: &str) -> (i64, i64) {
    let cs: Vec<char> = ln.chars().collect();
    let mut a: Vec<char> = Vec::new();
    let mut i = 0;
    while i < cs.len() {
        if cs[i] == '\'' {
            if i + 3 < cs.len() && cs[i + 1] == '\\' && cs[i + 3] == '\'' { i += 4; continue; }
            if i + 2 < cs.len() && cs[i + 1] != '\'' && cs[i + 2] == '\'' { i += 3; continue; }
        }
        a.push(cs[i]); i += 1;
    }
    let mut b: Vec<char> = Vec::new();
    i = 0;
    while i < a.len() {
        if a[i] == '"' {
            let mut k = i + 1; let mut closed = false;
            while k < a.len() {
                if a[k] == '\\' { if k + 1 < a.len() { k += 2; continue; } else { break; } }
                if a[k] == '"' { closed = true; break; }
                k += 1;
            }
            if closed { i = k + 1; continue; }
        }
        b.push(a[i]); i += 1;
    }
    let mut o: i64 = 0; let mut c: i64 = 0;
    i = 0;
    while i < b.len() {
        if b[i] == '/' && i + 1 < b.len() && b[i + 1] == '/' { break; }
        if b[i] == '{' { o += 1; }
        if b[i] == '}' { c += 1; }
        i += 1;
    }
    (o, c)
}
// lexical normalisation (collapse `.` and `..`) so a `#[path = "../x.rs"]` matches the caller's listing
fn norm(p: &str) -> String {
    let abs = p.starts_with("/");
    let mut parts: Vec<String> = Vec::new();
    for seg in split_on(p, '/').iter() {
        let seg: &str = seg;
        if seg == "" || seg == "." { continue; }
        if seg == ".." { if parts.len() > 0 && parts[parts.len() - 1] != ".." { parts.pop(); } else { parts.push(seg.to_string()); } continue; }
        parts.push(seg.to_string());
    }
    let mut s = String::new();
    if abs { s.push('/'); }
    let mut k = 0; while k < parts.len() { if k > 0 { s.push('/'); } s.push_str(&parts[k]); k += 1; }
    s
}
// Split on an ASCII separator (used for paths). Paths are ASCII here, so pushing chars is safe -- SEM0 mis-encodes
// a multi-byte char in String::push, so never rebuild source text this way.
fn split_on(s: &str, c: char) -> Vec<String> {
    let mut v: Vec<String> = Vec::new(); let mut cur = String::new();
    for ch in s.chars() { if ch == c { v.push(cur); cur = String::new(); } else { cur.push(ch); } }
    v.push(cur); v
}
fn dirname(p: &str) -> String { match p.rfind('/') { Some(i) => p[..i].to_string(), None => String::new() } }
fn basename(p: &str) -> String { match p.rfind('/') { Some(i) => p[i + 1..].to_string(), None => p.to_string() } }
fn join(a: &str, b: &str) -> String { if b.starts_with("/") || a == "" { b.to_string() } else { format!("{}/{}", a, b) } }

struct Sp { files: HashSet<String>, seen: HashSet<String>, tests: bool }

fn find_mod(sp: &Sp, moddir: &str, name: &str) -> Option<String> {
    let a = join(moddir, &format!("{}.rs", name));
    if sp.files.contains(&norm(&a)) { return Some(a); }
    let b = join(moddir, &format!("{}/mod.rs", name));
    if sp.files.contains(&norm(&b)) { return Some(b); }
    None
}

fn splice(sp: &mut Sp, rawpath: &str) -> String {
    // `seen` is keyed on the path AS SPELLED (as splice-crate.py did): a second `#[path]` route to the same file is
    // a distinct module in Rust and is spliced again; only an identical spelling (a cycle / cfg twin) is skipped.
    if sp.seen.contains(rawpath) { return String::new(); }
    sp.seen.insert(rawpath.to_string());
    let path = norm(rawpath);
    let d = dirname(rawpath);
    let bn = basename(rawpath);
    let stem = if bn.ends_with(".rs") { bn[..bn.len() - 3].to_string() } else { bn.clone() };
    let base = if stem == "mod" || stem == "lib" || stem == "main" { d.clone() } else { join(&d, &stem) };
    // copy into a fresh String: under SEM0 the String fs::read_to_string returns is only safe to read WHOLE --
    // slicing at an offset, lines() and chars() on it read garbage (a SEM0 runtime-layout bug); the copy is normal
    let raw = std::fs::read_to_string(&path).unwrap();
    let mut text = String::new(); text.push_str(&raw);
    let lines: Vec<&str> = text.split('\n').collect();
    let mut out: Vec<String> = Vec::new();
    let mut istack: Vec<String> = Vec::new();
    let mut iclose: Vec<i64> = Vec::new();
    let mut depth: i64 = 0;
    let mut i = 0;
    while i < lines.len() {
        let mut ln = lines[i].to_string();
        let mut moddir = base.clone();
        let mut q = 0; while q < istack.len() { moddir = join(&moddir, &istack[q]); q += 1; }
        if ln.trim() == "#[cfg(test)]" {
            let mut j = i + 1;
            while j < lines.len() && lines[j].trim() == "" { j += 1; }
            if j < lines.len() {
                match mod_decl(lines[j]) {
                    Some((_ind, _vis, tname)) => {
                        if sp.tests {
                            match find_mod(sp, &moddir, &tname) {
                                Some(f) => { out.push(format!("mod {} {{", tname)); out.push(splice(sp, &f)); out.push("}".to_string()); i = j + 1; continue; }
                                None => {}
                            }
                        } else { i = j + 1; continue; }
                    }
                    None => {}
                }
            }
        }
        if is_extern_self(&ln) { i += 1; continue; }
        match strip_impl_const(&ln) { Some(s) => { ln = s; } None => {} }
        match path_attr(&ln) {
            Some(pa) => {
                if !pa.has_decl {
                    if i + 1 < lines.len() {
                        match mod_decl(lines[i + 1]) {
                            Some((_ind, vis2, nm2)) => {
                                i += 1;
                                let f = join(&moddir, &pa.rel);
                                if sp.files.contains(&norm(&f)) {
                                    out.push(format!("{}mod {} {{", vis2, nm2)); out.push(splice(sp, &f)); out.push("}".to_string()); i += 1; continue;
                                }
                                // file absent: like splice-crate.py, keep the attr line and drop the decl line
                            }
                            None => {}
                        }
                    }
                } else {
                    let f = join(&moddir, &pa.rel);
                    if sp.files.contains(&norm(&f)) {
                        out.push(format!("{}mod {} {{", pa.vis, pa.name)); out.push(splice(sp, &f)); out.push("}".to_string()); i += 1; continue;
                    }
                }
            }
            None => {}
        }
        match mod_decl(&ln) {
            Some((ind, vis, name)) => {
                match find_mod(sp, &moddir, &name) {
                    Some(f) => {
                        out.push(format!("{}{}mod {} {{", ind, vis, name)); out.push(splice(sp, &f)); out.push(format!("{}}}", ind));
                        i += 1; continue;
                    }
                    None => {}
                }
            }
            None => {}
        }
        let (o, c) = braces(&ln);
        match mod_open(&ln) { Some(name) => { if o == 1 && c == 0 { istack.push(name); iclose.push(depth); } } None => {} }
        out.push(ln);
        depth += o - c;
        while iclose.len() > 0 && depth <= iclose[iclose.len() - 1] { istack.pop(); iclose.pop(); }
        i += 1;
    }
    out.join("\n")
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    // indexed, not `it.next()` then `for l in it`: SEM0 restarts a partly consumed `lines()` iterator in a for
    let ls: Vec<&str> = input.lines().collect();
    let root = ls[0].to_string();
    let tests = ls[1].trim() == "tests=1";
    let mut files: HashSet<String> = HashSet::new();
    let mut k = 2; while k < ls.len() { if ls[k] != "" { files.insert(norm(ls[k])); } k += 1; }
    let mut sp = Sp { files: files, seen: HashSet::new(), tests: tests };
    let s = splice(&mut sp, &root);
    println!("{}", s);
}
