// genstubs.rs -- stub every DANGLING call in the linked driver image (a call whose target no crate defines).
// Port of tower/backend-llvm/driver-genstubs.py (no Python in the tower), built by semc with the tower's SEM0.
//
// For each dangler `c` (first-seen order):
//   * a unique qualified def `<crate>$...$c` exists (or c is in HARD)  -> alias: forward the params to it
//   * c is a string/path-returning name (STRINGRET, rung219)            -> return a fresh empty String header
//   * otherwise (absent or ambiguous)                                   -> identity: `ret v0`
//
// stdin:  the linked rzir (linked_drv.rz)
// stdout: the stub rzir, then a line `@@@DANG@@@` and the dangler list (`<count>\t<name>`, most-called first),
//         then a line `@@@SUMMARY@@@ danglers=N aliased=A identity=I stringret=S`. (SEM0-built binaries cannot
//         write files yet, so the caller splits the stream.)
use std::collections::{HashMap, HashSet};
use std::io::Read;

fn is_vreg(t: &str) -> bool {
    let b = t.as_bytes();
    if b.len() < 2 || b[0] != b'v' { return false; }
    let mut i = 1; while i < b.len() { if b[i] < b'0' || b[i] > b'9' { return false; } i += 1; } true
}
fn sk(b: &[u8], mut i: usize) -> usize { while i < b.len() && (b[i] == b' ' || b[i] == b'\t' || b[i] == b'\r') { i += 1; } i }
fn lit(b: &[u8], i: usize, w: &[u8]) -> bool {
    if i + w.len() > b.len() { return false; }
    let mut k = 0; while k < w.len() { if b[i + k] != w[k] { return false; } k += 1; } true
}
// re.search(r':\s*i32\s*=\s*param', s)
fn is_param(s: &str) -> bool {
    let b = s.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b':' {
            let mut j = sk(b, i + 1);
            if lit(b, j, b"i32") { j = sk(b, j + 3); if j < b.len() && b[j] == b'=' { j = sk(b, j + 1); if lit(b, j, b"param") { return true; } } }
        }
        i += 1;
    }
    false
}

fn main() {
    let mut s0 = String::new();
    std::io::stdin().read_to_string(&mut s0).unwrap();
    let mut src = String::new(); src.push_str(&s0);   // SEM0: a read String is only safe whole; work on a copy

    let mut defs: HashSet<String> = HashSet::new();
    let mut argc: HashMap<String, i64> = HashMap::new();
    let mut calls: HashMap<String, i64> = HashMap::new();
    let mut order: Vec<String> = Vec::new();          // first-seen order of call targets (Python Counter order)
    let mut cur = String::new(); let mut have_cur = false; let mut pc: i64 = 0;
    for ln in src.lines() {
        let t: Vec<&str> = ln.split_whitespace().collect();
        if t.len() >= 2 && (t[0] == "fn" || t[0] == "ir") {
            if have_cur { argc.insert(cur.clone(), pc); }
            let mut nm = String::new();
            let mut k = 1;
            while k < t.len() { if t[k] == "->" { break; } if k > 1 { nm.push_str(" "); } nm.push_str(t[k]); k += 1; }
            cur = nm; have_cur = true; defs.insert(cur.clone()); pc = 0;
        } else if have_cur && is_param(ln) {
            pc += 1;
        }
        if t.len() >= 5 && t[3] == "call" {
            let mut c = String::new();
            let mut k = 4;
            while k < t.len() { if is_vreg(t[k]) { break; } if k > 4 { c.push_str(" "); } c.push_str(t[k]); k += 1; }
            if c != "" && !c.starts_with("__") {
                if calls.contains_key(&c) { let n = calls[&c]; calls.insert(c.clone(), n + 1); }
                else { calls.insert(c.clone(), 1); order.push(c.clone()); }
            }
        }
    }
    if have_cur { argc.insert(cur.clone(), pc); }

    // bare name (after the last `$`) -> the qualified defs carrying it; only the count and the sole entry matter
    let mut bare_n: HashMap<String, i64> = HashMap::new();
    let mut bare_one: HashMap<String, String> = HashMap::new();
    for d in defs.iter() {
        match d.rfind('$') {
            Some(i) => {
                let b = d[i + 1..].to_string();
                if bare_n.contains_key(&b) { let n = bare_n[&b]; bare_n.insert(b.clone(), n + 1); }
                else { bare_n.insert(b.clone(), 1); bare_one.insert(b.clone(), d.clone()); }
            }
            None => {}
        }
    }

    let mut out = String::new();
    let mut dang: Vec<String> = Vec::new();
    let mut aliased = 0; let mut ident = 0; let mut strret = 0;
    for c in order.iter() {
        if defs.contains(c) { continue; }
        dang.push(c.clone());
        let mut q = String::new();
        if c == "run_compiler" { q = "rustc_driver_impl$run_compiler".to_string(); }
        else if bare_n.contains_key(c) && bare_n[c] == 1 { q = bare_one[c].clone(); }
        if q == "" && c == "path" {
            // rung219: an ambiguous string/path-returning dangler gets a fresh zeroed 16-byte String header, not
            // `self` (the identity stub made sysroot.path() return the &Sysroot struct -> __scat_s OOB SEGV)
            out.push_str(&format!("fn {}\n", c));
            let mut i = 0; while i < 6 { out.push_str(&format!("  v{}: i32 = param\n", i)); i += 1; }
            out.push_str("  v6: i32 = gref __heap\n  v7: i32 = gref __heap_cursor\n  v8: i32 = load v7\n");
            out.push_str("  v9: i32 = add v6 v8\n  v10: i32 = const 16\n  v11: i32 = add v8 v10\n  store v7 v11\n");
            out.push_str("  v12: i32 = const 0\n  store8 v9 v12\n  v13: i32 = const 8\n  v14: i32 = add v9 v13\n");
            out.push_str("  store8 v14 v12\n  ret v9\n");
            strret += 1;
        } else if q != "" {
            let mut n: i64 = 2; if argc.contains_key(&q) { n = argc[&q]; }
            if n < 1 { n = 1; } if n > 6 { n = 6; }
            out.push_str(&format!("fn {}\n", c));
            let mut i = 0; while i < n { out.push_str(&format!("  v{}: i32 = param\n", i)); i += 1; }
            let mut a = String::new();
            i = 0; while i < n { if i > 0 { a.push_str(" "); } a.push_str(&format!("v{}", i)); i += 1; }
            out.push_str(&format!("  v{}: i32 = call {} {}\n", n, q, a));
            out.push_str(&format!("  ret v{}\n", n));
            aliased += 1;
        } else {
            out.push_str(&format!("fn {}\n", c));
            let mut i = 0; while i < 6 { out.push_str(&format!("  v{}: i32 = param\n", i)); i += 1; }
            out.push_str("  ret v0\n");
            ident += 1;
        }
    }
    print!("{}", out);

    // dangler list, most-called first; a stable insertion sort keeps first-seen order among equal counts
    let mut idx: Vec<usize> = Vec::new();
    let mut k = 0;
    while k < dang.len() {
        let ck = calls[&dang[k]];
        let mut p = idx.len();
        while p > 0 && calls[&dang[idx[p - 1]]] < ck { p -= 1; }
        idx.insert(p, k);
        k += 1;
    }
    println!("@@@DANG@@@");
    let mut d = String::new();
    for i in idx.iter() { d.push_str(&format!("{}\t{}\n", calls[&dang[*i]], dang[*i])); }
    print!("{}", d);
    println!("@@@SUMMARY@@@ danglers={} aliased={} identity={} stringret={}", dang.len(), aliased, ident, strret);
}
