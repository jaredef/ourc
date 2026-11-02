// link.rs -- cross-crate rzir linker, IN RUST (compiled by the self-hosted tower via //@S9), replacing link.py.
// Reads the crate rzirs on stdin, separated by a line "@@@CRATE@@@" (first crate = the ENTRY/main unit).
// Emits one concatenated rzir on stdout: entry (whole) + each lib's exported fns + their transitive callees.
//
// Two transforms, mirroring the proven link.py:
//   (1) strip_qual  -- a `call/faddr/gref Crate::name` whose FULL qualified name is NOT defined in that crate
//       (a genuine cross-crate reference) resolves to its LAST segment `name`, matching the sibling's bare
//       `fn name`. Intra-crate qualified method defs/refs (`fn Parser::new` / `call Parser::new`) stay qualified.
//   (2) reachability -- from each LIB crate keep ONLY the exported fns (referenced-by-entry, not-defined-by-entry)
//       plus their transitive `call` callees plus the gdefs those keep, dropping the lib's runtime prelude / main
//       / dead code (concatenating two full s9_runtime copies is fragile).
//
// ALLOC-REUSE (2026-09-20, whole-compiler-single-binary R2a): at whole-compiler scale (~130MB concat, 73 crates)
// the linker consumed the whole ~4GB bump arena and segfaulted. Two fixes, both keeping output byte-identical:
//   * hot scans byte-walk tokens (tok_eq/tok_str/ntok) instead of a Vec<String> per line via words();
//   * line iteration is a ZERO-ALLOCATION byte scan (each_line via lstart/lend on the text's bytes), NOT
//     `.lines()` -- SEM0's `.lines()` EAGERLY materializes a Vec<&str> with a per-line copy (str.rs ~13352),
//     and the linker iterates the full text ~7 times, which was the dominant churn (~2.3GB at 64MB input).
//     rzir has no '\r', so a split-on-'\n' with no trailing-empty is byte-equivalent to `.lines()`.
// words() is retained ONLY for the rare strip_qual rebuild (lines that carry a cross-crate qualified ref).

use std::io::Read;

// ALLOC-REUSE arena checkpoint/reset (SEM0 intrinsics, recognized by name in s9_call; NATIVELY these are
// no-ops so rustc builds an identical-output linker). __arena_cp reads __heap_cursor; __arena_reset restores it,
// reclaiming every bump allocation made in between. SOUND here because each lib iteration's transient data is
// PRINTED (flushed to stdout) before the reset and nothing allocated in the window outlives the iteration.
fn __arena_cp() -> i64 { 0 }
fn __arena_reset(_cp: i64) {}

fn words(s: &str) -> Vec<String> {
    let mut r: Vec<String> = Vec::new();
    let mut cur = String::new();
    for ch in s.chars() {
        if ch == ' ' || ch == '\t' { if cur.len() > 0 { r.push(cur.clone()); cur = String::new(); } }
        else { cur.push(ch); }
    }
    if cur.len() > 0 { r.push(cur); }
    r
}

// end byte-offset of the line starting at `st` (index of the next '\n', or the text length). No allocation.
fn bword_eq(b: &[u8], ts: usize, te: usize, kw: &str) -> bool {
    let k = kw.as_bytes(); if te - ts != k.len() { return false; }
    let mut i = 0; while i < k.len() { if b[ts + i] != k[i] { return false; } i += 1; } true
}
fn beq2(b: &[u8], ts: usize, te: usize, k0: u8, k1: u8) -> bool { te - ts == 2 && b[ts] == k0 && b[ts+1] == k1 }
fn beq4(b: &[u8], ts: usize, te: usize, k0: u8, k1: u8, k2: u8, k3: u8) -> bool { te - ts == 4 && b[ts] == k0 && b[ts+1] == k1 && b[ts+2] == k2 && b[ts+3] == k3 }
fn beq5(b: &[u8], ts: usize, te: usize, k0: u8, k1: u8, k2: u8, k3: u8, k4: u8) -> bool { te - ts == 5 && b[ts] == k0 && b[ts+1] == k1 && b[ts+2] == k2 && b[ts+3] == k3 && b[ts+4] == k4 }
fn lend(b: &[u8], st: usize) -> usize { let mut i = st; while i < b.len() && b[i] != 10 { i += 1; } i }

// the idx-th whitespace-delimited token as a String (one allocation), or "" if absent. No Vec.
fn tok_str(s: &str, idx: usize) -> String {
    let b = s.as_bytes(); let mut i = 0; let mut k = 0;
    while i < b.len() {
        while i < b.len() && (b[i] == 32 || b[i] == 9) { i += 1; }
        if i >= b.len() { break; }
        let st = i;
        while i < b.len() && b[i] != 32 && b[i] != 9 { i += 1; }
        if k == idx { return format!("{}", &s[st..i]); }
        k += 1;
    }
    String::new()
}
// does the idx-th token equal kw? No allocation.
fn tok_eq(s: &str, idx: usize, kw: &str) -> bool {
    let b = s.as_bytes(); let mut i = 0; let mut k = 0;
    while i < b.len() {
        while i < b.len() && (b[i] == 32 || b[i] == 9) { i += 1; }
        if i >= b.len() { break; }
        let st = i;
        while i < b.len() && b[i] != 32 && b[i] != 9 { i += 1; }
        if k == idx { return &s[st..i] == kw; }
        k += 1;
    }
    false
}
// number of whitespace-delimited tokens. No allocation.
fn ntok(s: &str) -> usize {
    let b = s.as_bytes(); let mut i = 0; let mut k = 0;
    while i < b.len() {
        while i < b.len() && (b[i] == 32 || b[i] == 9) { i += 1; }
        if i >= b.len() { break; }
        while i < b.len() && b[i] != 32 && b[i] != 9 { i += 1; }
        k += 1;
    }
    k
}

// after the last "::" in a token, else the token itself
fn last_seg(s: &str) -> String {
    let b = s.as_bytes(); let mut cut = 0; let mut i = 0;
    while i + 1 < b.len() { if b[i] == 58 && b[i+1] == 58 { cut = i + 2; } i += 1; }
    if cut == 0 { return format!("{}", s); }
    format!("{}", &s[cut..])
}
fn is_qual(s: &str) -> bool {
    let b = s.as_bytes(); let mut i = 0;
    while i + 1 < b.len() { if b[i] == 58 && b[i+1] == 58 { return true; } i += 1; }
    false
}
fn is_refkw(k: &str) -> bool { k == "call" || k == "gref" || k == "faddr" }
fn is_refkw_at(s: &str, idx: usize) -> bool { tok_eq(s, idx, "call") || tok_eq(s, idx, "gref") || tok_eq(s, idx, "faddr") }
fn is_defkw_at(s: &str, idx: usize) -> bool { tok_eq(s, idx, "fn") || tok_eq(s, idx, "ir") || tok_eq(s, idx, "gdef") }
fn starts_us(s: &str) -> bool { let b = s.as_bytes(); b.len() >= 2 && b[0] == 95 && b[1] == 95 }

// the set of names DEFINED in a crate text (fn/ir/gdef)
fn defs_of(text: &str) -> Vec<String> {
    let mut d: Vec<String> = Vec::new();
    let b = text.as_bytes(); let mut p = 0;
    while p < b.len() {
        let e = lend(b, p); let line = &text[p..e];
        if is_defkw_at(line, 0) && ntok(line) >= 2 { d.push(tok_str(line, 1)); }
        p = e + 1;
    }
    d
}
fn vec_has(v: &Vec<String>, s: &str) -> bool { for x in v.iter() { if x == s { return true; } } false }
// last "::"-segment of a name (X-2 dep-closure resolver canonicalization).
fn is_mirror_crate(tag: &str) -> bool { tag == "rustc_public" || tag == "rustc_public_bridge" }
fn crate_tag(s: &str) -> String { let b = s.as_bytes(); let mut i = 0; while i < b.len() { if b[i] == 36 { return format!("{}", &s[..i]); } i += 1; } String::new() }
fn type_qualified(s: &str) -> bool {
    // true if `s` = `...::Method` where the owner segment before the LAST :: begins with an uppercase letter
    // (a Type::method / Type::assoc the tag-aware canon can bind cross-crate); false for lowercase module paths.
    let b = s.as_bytes(); let mut last = 0 - 1; let mut prev = 0 - 1; let mut i = 0;
    while i + 1 < b.len() { if b[i] == 58 && b[i+1] == 58 { prev = last; last = i as i32; } i += 1; }
    if last < 0 { return false; }
    let owner_start = if prev < 0 { 0 } else { (prev as usize) + 2 };
    let oc = b[owner_start]; oc >= 65 && oc <= 90
}
fn is_labeltok(t: &str) -> bool { let b = t.as_bytes(); b.len() >= 5 && b[0]==95 && b[1]==95 && b[2]==76 && b[3] >= 97 && b[3] <= 122 }
fn line_has_label(line: &str) -> bool { let nt = ntok(line); let mut j = 0; while j < nt { if is_labeltok(&tok_str(line, j)) { return true; } j += 1; } false }
fn strip_tag(s: &str) -> String {
    // remove a leading `crate$` namespacing tag (everything up to and incl. the first '$'), leaving the
    // type-qualified suffix `Type::method` (or a bare `method`). `__` runtime names have no '$' -> unchanged.
    let b = s.as_bytes(); let mut i = 0;
    while i < b.len() { if b[i] == 36 { return format!("{}", &s[(i+1)..]); } i += 1; }
    format!("{}", s)
}
fn lseg(s: &str) -> String {
    let b = s.as_bytes(); let mut cut = 0; let mut i = 0;
    while i + 1 < b.len() { if b[i] == 58 && b[i+1] == 58 { cut = i + 2; } i += 1; }
    if cut == 0 { return format!("{}", s); }
    format!("{}", &s[cut..])
}
// ONEBIN: a fully-qualified path call `crate::Type::method` keeps a `::` crate prefix, but defs use `crate$...`.
// Replace the FIRST `::` with `$` so the call name can match the def (getopts::Options::new -> getopts$Options::new).
// No `::` -> unchanged. Only BINDS when the resulting `crate$...` is an actual def (callers gate on gidx_has/ts).
fn first_colon_to_dollar(s: &str) -> String {
    let b = s.as_bytes(); let mut i = 0;
    while i + 1 < b.len() { if b[i] == 58 && b[i+1] == 58 { return format!("{}${}", &s[..i], &s[(i+2)..]); } i += 1; }
    format!("{}", s)
}
// index of `s` in `v`, else -1.
fn vec_idx(v: &Vec<String>, s: &str) -> i32 { let mut i = 0; for x in v.iter() { if x == s { return i; } i += 1; } 0 - 1 }
fn shash(s: &str, nb: i64) -> i64 { let b = s.as_bytes(); let mut h: i64 = 0; let mut i = 0; while i < b.len() { h = (h * 131 + (b[i] as i64)) % nb; i += 1; } h }
// membership of `s` among g_names, via a bucket index gnb (buckets of g_names indices).
fn gidx_has(gnb: &Vec<Vec<usize>>, g_names: &Vec<String>, nb: i64, s: &str) -> bool { let bk = &gnb[shash(s, nb) as usize]; let mut i = 0; while i < bk.len() { if g_names[bk[i]] == s { return true; } i += 1; } false }
fn hmap_idx(bk: &Vec<Vec<usize>>, keys: &Vec<String>, nb: i64, s: &str) -> i32 { let b = &bk[shash(s, nb) as usize]; let mut i = 0; while i < b.len() { if keys[b[i]] == s { return b[i] as i32; } i += 1; } 0 - 1 }
// membership in a hashed string set (buckets of the strings themselves).
fn hset_has(bk: &Vec<Vec<String>>, nb: i64, s: &str) -> bool { let b = &bk[shash(s, nb) as usize]; let mut i = 0; while i < b.len() { if b[i] == s { return true; } i += 1; } false }
// TAG-AWARE canonicalize (X-2 + tag-aware): a ref whose LAST SEGMENT maps to exactly ONE def (uq_seg[k]->uq_full[k])
// resolves to that def's FULL name -- so a bare `call foo` binds a `B::foo` def AND a `C$foo` tagged def (the tag is
// preserved, not stripped). A name whose lseg is ambiguous or absent is left as-is (X-5 type-disambig residual);
// `__` runtime untouched. Works on both the //@S9 path (defs bare -> full==lseg) and the //@SUPERSET path (C$-tagged).
fn bfs_resolve(name: &str, g_names: &Vec<String>, gnb: &Vec<Vec<usize>>, hnb: i64, gls_key: &Vec<String>, gls_val: &Vec<String>, gls_b: &Vec<Vec<usize>>, gts_key: &Vec<String>, gts_val: &Vec<String>, gts_b: &Vec<Vec<usize>>, gtq_key: &Vec<String>, gtq_val: &Vec<String>, gtq_b: &Vec<Vec<usize>>) -> String {
    if starts_us(name) { return format!("{}", name); }
    if gidx_has(gnb, g_names, hnb, name) { return format!("{}", name); }
    { let ds = first_colon_to_dollar(name); if ds.as_bytes().len() != name.as_bytes().len() && gidx_has(gnb, g_names, hnb, &ds) { return ds; } }   // ONEBIN: crate::Type::method path call -> crate$Type::method def (keeps the def reachable)
    let st = strip_tag(name);
    if is_qual(&st) { let t = hmap_idx(gts_b, gts_key, hnb, &st); if t >= 0 { let v = &gts_val[t as usize]; if is_qual(v) || v.len() > 0 { if gidx_has(gnb, g_names, hnb, v) { return format!("{}", v); } } } }
    let k = hmap_idx(gls_b, gls_key, hnb, &lseg(&strip_tag(name))); if k >= 0 { let v = &gls_val[k as usize]; if gidx_has(gnb, g_names, hnb, v) { return format!("{}", v); } }
    if is_qual(&st) == false { let q = hmap_idx(gtq_b, gtq_key, hnb, &lseg(name)); if q >= 0 { let v = &gtq_val[q as usize]; if streq(v, "\x00AMBIG") == false && gidx_has(gnb, g_names, hnb, v) { return format!("{}", v); } } }   // canon-parity tier: a bare call binds a UNIQUE QUALIFIED def by last-segment (mirrors canon's tq), so reachability keeps the def canon will bind (e.g. create_err -> rustc_errors$DiagCtxtHandle::create_err) instead of pruning it
    format!("{}", name)
}
fn drop_last_seg(s: &str) -> String { let b = s.as_bytes(); let mut cut: i64 = -1; let mut i = 0; while i + 1 < b.len() { if b[i] == 58 && b[i+1] == 58 { cut = i as i64; } i += 1; } if cut < 0 { return String::new(); } format!("{}", &s[..(cut as usize)]) }
fn canon(name: &str, caller_mod: &str, uq_k: &Vec<String>, uq_full: &Vec<String>, uq_multi: &Vec<bool>, uq_b: &Vec<Vec<usize>>, ts_k: &Vec<String>, ts_full: &Vec<String>, ts_multi: &Vec<bool>, ts_b: &Vec<Vec<usize>>, tq_k: &Vec<String>, tq_full: &Vec<String>, tq_multi: &Vec<bool>, tq_b: &Vec<Vec<usize>>, nm_k: &Vec<String>, nm_full: &Vec<String>, nm_multi: &Vec<bool>, nm_b: &Vec<Vec<usize>>, self_crate: &str, cs_key: &Vec<String>, cs_full: &Vec<String>, cs_b: &Vec<Vec<usize>>, hnb: i64) -> String {
    if starts_us(name) { return format!("{}", name); }
    let st = strip_tag(name);
    // ONEBIN rung151 CALLER-MODULE tier (SURGICAL: rustc_target only -- the target-spec crate where base-module
    // free fns meta/opts/... are called bare and collide with rustc_ast$AttrItem::meta). Scoped to one crate so
    // it cannot perturb handle_options(rustc_driver_impl)/getopts resolution. Rust scoping: a bare call binds the
    // caller'"'"'s own module first.
    if self_crate == "rustc_target" && caller_mod.len() > 0 && is_qual(&st) == false { let ck = format!("{}${}::{}", self_crate, caller_mod, st); let c = hmap_idx(cs_b, cs_key, hnb, &ck); if c >= 0 { return format!("{}", &cs_full[c as usize]); } }
    // SAME-CALLER-CRATE tier (most specific): an untagged ref `Type::method` (or bare method) whose CALLER crate
    // (the block being emitted) defines it -> bind that crate's def. Resolves cross-crate name collisions using the
    // caller-crate context the untagged call would otherwise lose.
    if self_crate.len() > 0 { let ck = format!("{}${}", self_crate, st); let c = hmap_idx(cs_b, cs_key, hnb, &ck); if c >= 0 { return format!("{}", &cs_full[c as usize]); } }
    // ONEBIN CRATE-PATH tier: a `crate::Type::method` path call -> the crate$Type::method def. The def is kept
    // reachable (bfs_resolve's first_colon_to_dollar), and lands in the ts table under its crate-stripped suffix;
    // bind via that suffix when unique (getopts::Options::new -> suffix Options::new -> getopts$Options::new).
    { let dsd = first_colon_to_dollar(&st); if dsd.as_bytes().len() != st.as_bytes().len() { let rest = strip_tag(&dsd); let t = hmap_idx(ts_b, ts_k, hnb, &rest); if t >= 0 { if ts_multi[t as usize] == false { return format!("{}", &ts_full[t as usize]); } } } }
    // TYPE-SUFFIX tier: a ref whose crate-stripped `Type::method` is globally unique.
    if is_qual(&st) { let t = hmap_idx(ts_b, ts_k, hnb, &st); if t >= 0 { if ts_multi[t as usize] == false { return format!("{}", &ts_full[t as usize]); } } }
    // NON-MIRROR type-suffix tier: `Type::method` ambiguous ONLY because rustc_public(_bridge) mirrors a
    // compiler-internal type -> bind the single NON-mirror def (the stable-MIR mirror is a leaf, never the callee).
    if is_qual(&st) { let n = hmap_idx(nm_b, nm_k, hnb, &st); if n >= 0 { if nm_multi[n as usize] == false { return format!("{}", &nm_full[n as usize]); } } }
    // BARE-lseg tier: a bare ref binds a unique BARE def with that last-segment.
    if is_qual(&st) == false { let k = hmap_idx(uq_b, uq_k, hnb, &lseg(&st)); if k >= 0 { if uq_multi[k as usize] == false { return format!("{}", &uq_full[k as usize]); } } }
    // METHOD-DETERMINES-TYPE tier: a bare ref whose method has exactly ONE type-qualified def globally binds to it
    // (if only one type defines `m`, any call to `m` is on that type -- sound, no receiver typing needed).
    if is_qual(&st) == false { let q = hmap_idx(tq_b, tq_k, hnb, &lseg(name)); if q >= 0 { if tq_multi[q as usize] == false { return format!("{}", &tq_full[q as usize]); } } }
    format!("{}", name)
}
// emit `text`, canonicalizing every def-header name + call/gref/faddr target. A line with no def/ref token passes
// through verbatim; a rewritten line is rebuilt indent + single-space (rzir tokens are single-space separated).
fn emit_canon(text: &str, uq_k: &Vec<String>, uq_full: &Vec<String>, uq_multi: &Vec<bool>, uq_b: &Vec<Vec<usize>>, ts_k: &Vec<String>, ts_full: &Vec<String>, ts_multi: &Vec<bool>, ts_b: &Vec<Vec<usize>>, tq_k: &Vec<String>, tq_full: &Vec<String>, tq_multi: &Vec<bool>, tq_b: &Vec<Vec<usize>>, nm_k: &Vec<String>, nm_full: &Vec<String>, nm_multi: &Vec<bool>, nm_b: &Vec<Vec<usize>>, self_crate: &str, cs_key: &Vec<String>, cs_full: &Vec<String>, cs_b: &Vec<Vec<usize>>, hnb: i64, fnbase: i64) {
    // labels (__LiN/__LmN) are numbered PER-FUNCTION, restarting at 0 in every fn, so a `goto __Li0` collides
    // across the concatenated image -> it can bind to ANOTHER function's __Li0 and jump into its body (seen:
    // __eq_<E>'s goto landing in a struct-Debug routine -> null deref). The old crate-tag rewrite was BOTH
    // insufficient (same tag for every fn WITHIN a crate) AND silent when self_crate could not be derived (a lib
    // whose only kept fns are __-prefixed derive fns has no $-tagged name to read a tag from). Fix: make labels
    // FUNCTION-unique -- prefix every label token with a per-fn id (fnbase + a counter bumped on each `fn` def),
    // consistently for both `label` and `goto`, for ALL crates incl. the entry.
    let mut fnidx: i64 = 0; let mut seen_fn = false; let mut cur_mod = String::new();
    let b = text.as_bytes(); let mut p = 0;
    while p < b.len() {
        let e = lend(b, p); let line = &text[p..e]; p = e + 1;
        let nt = ntok(line);
        if tok_eq(line, 0, "fn") && nt >= 2 { if seen_fn { fnidx += 1; } seen_fn = true; }
        let mut touch = false;
        if is_defkw_at(line, 0) && nt >= 2 { touch = true; }
        { let mut j = 0; while j + 1 < nt { if is_refkw_at(line, j) { touch = true; } j += 1; } }
        if line_has_label(line) { touch = true; }
        let lb = line.as_bytes(); let mut ind = 0; while ind < lb.len() && (lb[ind] == 32 || lb[ind] == 9) { ind += 1; }
        let mut rb = String::from(&line[..ind]);
        let mut j = 0;
        while j < nt {
            let raw = tok_str(line, j);
            let mut t2 = format!("{}", &raw);
            if is_labeltok(&raw) { t2 = format!("__L{}_{}", fnbase + fnidx, &raw[3..]); }
            if j == 1 && is_defkw_at(line, 0) { t2 = canon(&raw, "", uq_k, uq_full, uq_multi, uq_b, ts_k, ts_full, ts_multi, ts_b, tq_k, tq_full, tq_multi, tq_b, nm_k, nm_full, nm_multi, nm_b, self_crate, cs_key, cs_full, cs_b, hnb); }
            else if j >= 1 && is_refkw_at(line, j - 1) { t2 = canon(&raw, &cur_mod, uq_k, uq_full, uq_multi, uq_b, ts_k, ts_full, ts_multi, ts_b, tq_k, tq_full, tq_multi, tq_b, nm_k, nm_full, nm_multi, nm_b, self_crate, cs_key, cs_full, cs_b, hnb); }
            if j == 1 && is_defkw_at(line, 0) { let m = drop_last_seg(&strip_tag(&t2)); cur_mod = format!("{}", &m); }
            if j > 0 { rb.push(' '); }
            rb.push_str(&t2);
            j += 1;
        }
        print!("{}\n", rb);
    }
}

// rewrite genuine cross-crate qualified refs to their last segment
fn strip_qual(text: &str, defs: &Vec<String>, ts_k: &Vec<String>, ts_multi: &Vec<bool>, ts_b: &Vec<Vec<usize>>, hnb: i64) -> String {
    // TAG-AWARE: a ref `Type::method` whose crate-tagged def `Crate$Type::method` is in THIS crate must NOT be
    // reduced to its last segment (which is ambiguous). Precompute the crate-stripped def suffixes once, and treat
    // a ref as "defined" if its strip_tag matches a def OR a stripped-def suffix. (Pre-tagging strip_qual only
    // compared raw names, so it wrongly stripped every same-crate Type::method call on a //@SUPERSET crate.)
    let mut sdefs: Vec<String> = Vec::new();
    { let mut di = 0; while di < defs.len() { let st = strip_tag(&defs[di]); if is_qual(&st) { sdefs.push(st); } di += 1; } }
    let mut out = String::new();
    let b = text.as_bytes(); let mut p = 0;
    while p < b.len() {
        let e = lend(b, p); let line = &text[p..e]; p = e + 1;
        // FAST PATH: a line with no "::" anywhere can never carry a cross-crate qualified ref -> verbatim, no
        // tokenization. (Blank/whitespace lines have no "::" and pass through here too, byte-identical.)
        if !is_qual(line) { out.push_str(line); out.push('\n'); continue; }
        let w = words(line);
        if w.len() == 0 { out.push_str(line); out.push('\n'); continue; }
        let mut needs = false;
        { let mut j = 1; while j < w.len() { if is_refkw(&w[j-1]) && is_qual(&w[j]) && !vec_has(defs, &w[j]) && !vec_has(&sdefs, &strip_tag(&w[j])) { needs = true; } j += 1; } }
        if !needs { out.push_str(line); out.push('\n'); continue; }
        let lb = line.as_bytes(); let mut ind = 0; while ind < lb.len() && (lb[ind] == 32 || lb[ind] == 9) { ind += 1; }
        let mut rebuilt = String::from(&line[..ind]);
        let mut j = 0;
        while j < w.len() {
            let mut tok: String = w[j].clone();
            if j >= 1 { let prev = &w[j-1];
                if is_refkw(prev) { if is_qual(&tok) { if !vec_has(defs, &tok) && !vec_has(&sdefs, &strip_tag(&tok)) { let ls2 = strip_tag(&tok); if type_qualified(&ls2) == false { let ti = hmap_idx(ts_b, ts_k, hnb, &ls2); let keepq = ti >= 0 && ts_multi[ti as usize] == false; if keepq == false { tok = last_seg(&tok); } } } } }   // ONEBIN rung159: keep a globally-unique MODULE-qualified ref (passes::parse) so canon binds it to its unique def instead of colliding on bare parse
            }
            if j > 0 { rebuilt.push(' '); }
            rebuilt.push_str(&tok);
            j += 1;
        }
        out.push_str(&rebuilt);
        out.push('\n');
    }
    out
}

fn main() {
    let mut whole = String::new();
    std::io::stdin().read_to_string(&mut whole).unwrap();
    // Record each crate as a BYTE RANGE into `whole` (delimited by the @@@CRATE@@@ marker line), instead of
    // copying crate text into a Vec<String>. `whole` is the only large persistent buffer; strip_qual runs
    // on-demand from a slice inside each lib's checkpoint/reset window, so neither a `crates` nor an `sc` copy
    // of the input is ever materialized -- essential at whole-compiler scale (~130MB in a ~4GB arena).
    let mut cstart: Vec<usize> = Vec::new();   // inclusive start of each crate's text in `whole`
    let mut cend: Vec<usize> = Vec::new();     // exclusive end (up to, not including, the @@@CRATE@@@ line)
    {
        let wb = whole.as_bytes(); let mut p = 0; let mut seg_start = 0;
        while p < wb.len() {
            let e = lend(wb, p);
            if bword_eq(wb, p, e, "@@@CRATE@@@") { cstart.push(seg_start); cend.push(p); seg_start = e + 1; }   // byte-compare (no &whole[p..e] slice-copy, which added +1.4GB at whole-compiler scale)
            p = e + 1;
        }
        cstart.push(seg_start); cend.push(whole.len());
    }
    if cstart.len() == 0 { return; }

    // entry = crate 0, strip_qual'd once and kept (it is referenced throughout + printed).
    let entry_raw: &str = &whole[cstart[0]..cend[0]];
    let hnb: i64 = 262139;
    // GLOBAL qualified-def-suffix uniqueness map (for strip_qual's keep-guard). Scan RAW `whole` fn/ir/gdef
    // headers: key = crate-stripped suffix (module::name or Type::method), multi=true if >1 def shares it. Built
    // BEFORE the entry strip_qual (and reused for libs) so a cross-crate MODULE-qualified ref whose suffix is a
    // GLOBALLY-UNIQUE def (passes::parse) is KEPT qualified rather than stripped to a colliding bare name. (ts,
    // built later from the kept set, can't be used here -- chicken-and-egg: the entry strip decides what is kept.)
    let mut gqs_k: Vec<String> = Vec::new(); let mut gqs_full: Vec<String> = Vec::new(); let mut gqs_multi: Vec<bool> = Vec::new();
    let mut gqs_b: Vec<Vec<usize>> = Vec::new(); { let mut i: i64 = 0; while i < hnb { gqs_b.push(Vec::new()); i += 1; } }
    { let wb = whole.as_bytes(); let mut gp = 0; while gp < wb.len() { let ge = lend(wb, gp); let gline = &whole[gp..ge]; gp = ge + 1;
        if is_defkw_at(gline, 0) && ntok(gline) >= 2 { let gnm = tok_str(gline, 1); let gst = strip_tag(&gnm); if is_qual(&gst) {
            let gi = hmap_idx(&gqs_b, &gqs_k, hnb, &gst); if gi < 0 { gqs_b[shash(&gst, hnb) as usize].push(gqs_k.len()); gqs_k.push(gst.clone()); gqs_full.push(gnm.clone()); gqs_multi.push(false); } else { if gqs_full[gi as usize] != gnm { gqs_multi[gi as usize] = true; } } } } } }
    let entry: String = strip_qual(entry_raw, &defs_of(entry_raw), &gqs_k, &gqs_multi, &gqs_b, hnb);
    let entry: &str = &entry;
    let entry_defs = defs_of(entry);
    // exported = names entry references (call/gref/faddr) but does NOT define, excluding __ runtime
    let mut exported: Vec<String> = Vec::new();
    {
        let eb = entry.as_bytes(); let mut p = 0;
        while p < eb.len() {
            let e = lend(eb, p); let line = &entry[p..e]; p = e + 1;
            let nt = ntok(line);
            let mut j = 0;
            while j + 1 < nt {
                if is_refkw_at(line, j) { let nm = tok_str(line, j + 1);
                    // Seed entry-referenced names. `__` names are INCLUDED: a per-type generated fn the entry calls
                    // cross-crate (__dbg_<E>/__eq_<E>/__encode_<E>/__decode_<E>) is defined by a lib crate and MUST
                    // be kept. True runtime-prelude __ names (__heap/__str_grow) are provided by runtime.rzir, are
                    // not lib fn-defs, so the BFS's g_names membership check drops them anyway (harmless to seed).
                    if !vec_has(&entry_defs, &nm) { if !vec_has(&exported, &nm) { exported.push(nm); } }
                }
                j += 1;
            }
        }
    }

    // INCREMENTAL OUTPUT: print the entry now, then each lib's kept text as it is computed. No monolithic
    // `result` accumulator -- so the per-lib arena checkpoint/reset below is sound (nothing printed outlives its
    // iteration in the arena).
    let entry_gd = defs_of(entry);

    // GLOBAL REACHABILITY (X-1): one global work-list across ALL crates keeps transitive entry->A->B calls that
    // the per-lib BFS dropped (it seeded only from entry refs and kept only same-lib callees). Pass A builds the
    // global call-graph (each fn's name + its call targets, space-joined); a global BFS from `exported` computes
    // the kept set; pass B emits the kept fns + their referenced gdefs. Memory-bounded: only names + per-fn
    // callee strings are held globally (not block text); block text is re-derived per crate on emit (arena-reset).
    let mut g_names: Vec<String> = Vec::new();
    let mut g_calls: Vec<String> = Vec::new();
    {
        let wb = whole.as_bytes();
        let mut li = 1;
        while li < cstart.len() {
            // NO __arena_cp/reset here: pass A accumulates PERSISTENT g_names/g_calls clones; a reset would
            // reclaim the arena region those clones live in, and the next lib's strip_qual overwrites it
            // (use-after-reset -> SEGV past ~450KB total input, root-caused by pin-art 2026-09-26). Pass A holds
            // only names + per-fn callee strings globally (bounded ~130MB even at whole-compiler scale, < 4GB);
            // the reset belongs ONLY in pass B (emit), whose per-lib text is printed before its reset.
            // Pass A scans the RAW crate text (NO strip_qual) -- it only needs fn-def names (strip_qual never
            // rewrites `fn NAME` headers) and call/gref/faddr targets for the reachability graph; the raw
            // qualified targets resolve through bfs_resolve's same-caller-crate + type-suffix tiers just as the
            // stripped ones did. This avoids materializing every lib's full strip_qual output (~183MB transient
            // at whole-compiler scale -> 4GB arena SEGV); pass B still strip_quals per crate for the actual emit.
            // Byte-direct scan of whole[cs..ce] -- NO str-slice copies (SEM0 `&s[a..b]` allocates), NO
            // tok_str/tok_eq/ntok (each allocated slice copies). Only cur_name/cur_calls (the persistent data)
            // are built, by appending token bytes. This is what keeps pass A's arena bounded at whole-compiler
            // scale (was +3.67GB of slice-copy garbage for a 38MB input -> now ~the g_names/g_calls size).
            let cs = cstart[li]; let ce = cend[li];
            let mut cur_name = String::new(); let mut cur_calls = String::new(); let mut have = false;
            let mut p = cs;
            while p < ce {
                let e = lend(wb, p);
                let mut i = p; let mut tokidx = 0; let mut is_fn = false; let mut prev_kw = false;
                while i < e {
                    while i < e && (wb[i] == 32 || wb[i] == 9) { i += 1; }
                    if i >= e { break; }
                    let ts = i;
                    while i < e && wb[i] != 32 && wb[i] != 9 { i += 1; }
                    let te = i;
                    if tokidx == 0 && beq2(wb, ts, te, 102, 110) { is_fn = true; }
                    if is_fn && tokidx == 1 {
                        if have { g_names.push(cur_name.clone()); g_calls.push(cur_calls.clone()); }
                        cur_name = String::new(); { let mut z = ts; while z < te { cur_name.push(wb[z] as char); z += 1; } } cur_calls = String::new(); have = true;
                    }
                    if prev_kw { let mut z = ts; while z < te { cur_calls.push(wb[z] as char); z += 1; } cur_calls.push(' '); prev_kw = false; }
                    if beq4(wb, ts, te, 99, 97, 108, 108) || beq5(wb, ts, te, 102, 97, 100, 100, 114) || beq4(wb, ts, te, 103, 114, 101, 102) { prev_kw = true; }
                    tokidx += 1;
                }
                p = e + 1;
            }
            if have { g_names.push(cur_name.clone()); g_calls.push(cur_calls.clone()); }
            li += 1;
        }
    }
    // CANON-AWARE reachability maps: a call edge / exported seed is UNTAGGED (`new_cursor`) but the def is TAGGED
    // (`lex$new_cursor`); the raw-name BFS never crossed that boundary, so a cross-crate callee's block was dropped
    // and its (canon-bound) call dangled at RUN time. Build lseg-unique + type-suffix-unique maps over g_names and
    // resolve seeds + edges through them so reachability follows the same binding canon() will emit.
    let mut gls_key: Vec<String> = Vec::new(); let mut gls_val: Vec<String> = Vec::new();
    let mut gts_key: Vec<String> = Vec::new(); let mut gts_val: Vec<String> = Vec::new();
    let mut gtq_key: Vec<String> = Vec::new(); let mut gtq_val: Vec<String> = Vec::new();
    // HASH INDEX (perf): bucket g_names by name -> its indices, and each map key -> its index, so the O(n^2)
    // linear vec_idx/vec_has scans (which timed out on large keeps) become O(1) amortized. SEM0's HashMap is a
    // LINEAR association map, so this is a manual polynomial-hash bucket table (Vec<Vec<idx>>). Byte-identical
    // output (same kept set + same map contents; emit order is index-driven, not BFS-order).
    let mut gnb: Vec<Vec<usize>> = Vec::new(); { let mut i: i64 = 0; while i < hnb { gnb.push(Vec::new()); i += 1; } }
    { let mut bi = 0; while bi < g_names.len() { gnb[shash(&g_names[bi], hnb) as usize].push(bi); bi += 1; } }
    let mut gls_b: Vec<Vec<usize>> = Vec::new(); { let mut i: i64 = 0; while i < hnb { gls_b.push(Vec::new()); i += 1; } }
    let mut gts_b: Vec<Vec<usize>> = Vec::new(); { let mut i: i64 = 0; while i < hnb { gts_b.push(Vec::new()); i += 1; } }
    let mut gtq_b: Vec<Vec<usize>> = Vec::new(); { let mut i: i64 = 0; while i < hnb { gtq_b.push(Vec::new()); i += 1; } }
    { let mut a = 0; while a < g_names.len() { let st = strip_tag(&g_names[a]); if is_qual(&st) {
        let t = hmap_idx(&gts_b, &gts_key, hnb, &st); if t < 0 { gts_b[shash(&st, hnb) as usize].push(gts_key.len()); gts_key.push(st.clone()); gts_val.push(g_names[a].clone()); } else { gts_val[t as usize] = String::from("\x00AMBIG"); }
        let ls = lseg(&st); let q = hmap_idx(&gtq_b, &gtq_key, hnb, &ls); if q < 0 { gtq_b[shash(&ls, hnb) as usize].push(gtq_key.len()); gtq_key.push(ls); gtq_val.push(g_names[a].clone()); } else { gtq_val[q as usize] = String::from("\x00AMBIG"); }
      } else { let ls = lseg(&st); let k = hmap_idx(&gls_b, &gls_key, hnb, &ls); if k < 0 { gls_b[shash(&ls, hnb) as usize].push(gls_key.len()); gls_key.push(ls); gls_val.push(g_names[a].clone()); } else { gls_val[k as usize] = String::from("\x00AMBIG"); } } a += 1; } }
    // global BFS from exported over the whole call-graph, resolving each name to its actual (tagged) g_names def.
    let mut kept: Vec<String> = Vec::new();
    let mut kept_b: Vec<Vec<String>> = Vec::new(); { let mut i: i64 = 0; while i < hnb { kept_b.push(Vec::new()); i += 1; } }
    let mut work: Vec<String> = Vec::new();
    let mut work_b: Vec<Vec<String>> = Vec::new(); { let mut i: i64 = 0; while i < hnb { work_b.push(Vec::new()); i += 1; } }
    for e in exported.iter() { let r = bfs_resolve(e, &g_names, &gnb, hnb, &gls_key, &gls_val, &gls_b, &gts_key, &gts_val, &gts_b, &gtq_key, &gtq_val, &gtq_b); if gidx_has(&gnb, &g_names, hnb, &r) && !vec_has(&entry_defs, &r) { if !hset_has(&work_b, hnb, &r) { work_b[shash(&r, hnb) as usize].push(r.clone()); work.push(r); } } }
    while work.len() > 0 {
        let n = work.pop().unwrap();
        if hset_has(&kept_b, hnb, &n) { continue; }
        kept_b[shash(&n, hnb) as usize].push(n.clone());
        kept.push(n.clone());
        let cc = crate_tag(&n);
        let bkt = &gnb[shash(&n, hnb) as usize];
        let mut bx = 0;
        while bx < bkt.len() {
            let bi = bkt[bx];
            if g_names[bi] == n {
                let cs = &g_calls[bi];
                let nt = ntok(cs); let mut j = 0;
                while j < nt { let c0 = tok_str(cs, j);
                    let cm = drop_last_seg(&strip_tag(&n));
                    let smc = if cc == "rustc_target" && cm.len() > 0 && is_qual(&c0) == false && starts_us(&c0) == false { format!("{}${}::{}", cc, cm, c0) } else { String::new() };
                    let sc = if cc.len() > 0 { format!("{}${}", cc, c0) } else { String::new() };
                    let c = if smc.len() > 0 && gidx_has(&gnb, &g_names, hnb, &smc) { smc } else if sc.len() > 0 && gidx_has(&gnb, &g_names, hnb, &sc) { sc } else { bfs_resolve(&c0, &g_names, &gnb, hnb, &gls_key, &gls_val, &gls_b, &gts_key, &gts_val, &gts_b, &gtq_key, &gtq_val, &gtq_b) };
                    if gidx_has(&gnb, &g_names, hnb, &c) && !vec_has(&entry_defs, &c) && !hset_has(&kept_b, hnb, &c) { work.push(c); } j += 1; }
            }
            bx += 1;
        }
    }
    // X-2 dep-closure resolver: last-segments UNIQUE among emitted defs (entry defs + kept fns) -> canon() resolves
    // bare<->qualified so a `call foo` binds a `B::foo` def (or vice versa). Ambiguous last-segs stay full (X-5).
    let mut alld: Vec<String> = Vec::new();
    for d in entry_defs.iter() { alld.push(d.clone()); }
    for kk in kept.iter() { alld.push(kk.clone()); }
    // canon() unique-key maps, hash-indexed (one bucket per map + a multi flag; canon returns a hit only when
    // !multi). O(n) build via distinct-full-per-key (was O(alld^2)). Separate Vec params (SEM0 mishandles Vec fields
    // of a &struct -- returns the cell address); byte-identical to the counted form.
    let mut uq_k: Vec<String> = Vec::new(); let mut uq_full: Vec<String> = Vec::new(); let mut uq_multi: Vec<bool> = Vec::new();
    let mut uq_b: Vec<Vec<usize>> = Vec::new(); { let mut i: i64 = 0; while i < hnb { uq_b.push(Vec::new()); i += 1; } }
    { let mut a = 0; while a < alld.len() { if is_qual(&strip_tag(&alld[a])) == false { let ls = lseg(&strip_tag(&alld[a])); let idx = hmap_idx(&uq_b, &uq_k, hnb, &ls); if idx < 0 { uq_b[shash(&ls, hnb) as usize].push(uq_k.len()); uq_k.push(ls); uq_full.push(alld[a].clone()); uq_multi.push(false); } else { if uq_full[idx as usize] != alld[a] { uq_multi[idx as usize] = true; } } } a += 1; } }
    let mut ts_k: Vec<String> = Vec::new(); let mut ts_full: Vec<String> = Vec::new(); let mut ts_multi: Vec<bool> = Vec::new();
    let mut ts_b: Vec<Vec<usize>> = Vec::new(); { let mut i: i64 = 0; while i < hnb { ts_b.push(Vec::new()); i += 1; } }
    { let mut a = 0; while a < alld.len() { let st = strip_tag(&alld[a]); if is_qual(&st) { let idx = hmap_idx(&ts_b, &ts_k, hnb, &st); if idx < 0 { ts_b[shash(&st, hnb) as usize].push(ts_k.len()); ts_k.push(st.clone()); ts_full.push(alld[a].clone()); ts_multi.push(false); } else { if ts_full[idx as usize] != alld[a] { ts_multi[idx as usize] = true; } } } a += 1; } }
    let mut tq_k: Vec<String> = Vec::new(); let mut tq_full: Vec<String> = Vec::new(); let mut tq_multi: Vec<bool> = Vec::new();
    let mut tq_b: Vec<Vec<usize>> = Vec::new(); { let mut i: i64 = 0; while i < hnb { tq_b.push(Vec::new()); i += 1; } }
    { let mut a = 0; while a < alld.len() { let st = strip_tag(&alld[a]); if is_qual(&st) { let ls = lseg(&alld[a]); let idx = hmap_idx(&tq_b, &tq_k, hnb, &ls); if idx < 0 { tq_b[shash(&ls, hnb) as usize].push(tq_k.len()); tq_k.push(ls); tq_full.push(alld[a].clone()); tq_multi.push(false); } else { if tq_full[idx as usize] != alld[a] { tq_multi[idx as usize] = true; } } } a += 1; } }
    let mut nm_k: Vec<String> = Vec::new(); let mut nm_full: Vec<String> = Vec::new(); let mut nm_multi: Vec<bool> = Vec::new();
    let mut nm_b: Vec<Vec<usize>> = Vec::new(); { let mut i: i64 = 0; while i < hnb { nm_b.push(Vec::new()); i += 1; } }
    { let mut a = 0; while a < alld.len() { let st = strip_tag(&alld[a]); if is_qual(&st) && is_mirror_crate(&crate_tag(&alld[a])) == false { let idx = hmap_idx(&nm_b, &nm_k, hnb, &st); if idx < 0 { nm_b[shash(&st, hnb) as usize].push(nm_k.len()); nm_k.push(st.clone()); nm_full.push(alld[a].clone()); nm_multi.push(false); } else { if nm_full[idx as usize] != alld[a] { nm_multi[idx as usize] = true; } } } a += 1; } }
    let mut cs_key: Vec<String> = Vec::new(); let mut cs_full: Vec<String> = Vec::new();
    let mut cs_b: Vec<Vec<usize>> = Vec::new(); { let mut i: i64 = 0; while i < hnb { cs_b.push(Vec::new()); i += 1; } }
    { let mut a = 0; while a < alld.len() { let st = strip_tag(&alld[a]); let ct = crate_tag(&alld[a]); if is_qual(&st) && ct.len() > 0 { let key = format!("{}${}", ct, st); if hmap_idx(&cs_b, &cs_key, hnb, &key) < 0 { cs_b[shash(&key, hnb) as usize].push(cs_key.len()); cs_key.push(key); cs_full.push(alld[a].clone()); } } a += 1; } }
    let entry_crate: String = { let mut r = String::new(); let eb = entry.as_bytes(); let mut pp = 0; while pp < eb.len() { let e = lend(eb, pp); let line = &entry[pp..e]; pp = e + 1; if tok_eq(line, 0, "fn") && ntok(line) >= 2 { let ct = crate_tag(&tok_str(line, 1)); if ct.len() > 0 { r = ct; break; } } } r };
    emit_canon(entry, &uq_k, &uq_full, &uq_multi, &uq_b, &ts_k, &ts_full, &ts_multi, &ts_b, &tq_k, &tq_full, &tq_multi, &tq_b, &nm_k, &nm_full, &nm_multi, &nm_b, &entry_crate, &cs_key, &cs_full, &cs_b, hnb, 0);
    // pass B: emit each crate.s KEPT fn blocks + their referenced gdefs (re-strip_qual per crate, arena-reset)
    // GLOBAL gdef DEDUP: a shared `__` runtime global (__heap/__heap_cursor/...) is emitted by EVERY crate that uses
    // it; without deduping, N copies -> N distinct bump-allocator cursors -> crates allocate over each other's data
    // -> heap corruption at RUN. Track emitted gdef names across all crates (seed with the entry's) and emit once.
    let mut emit_g: Vec<String> = Vec::new();
    let mut emit_g_b: Vec<Vec<usize>> = Vec::new(); { let mut i: i64 = 0; while i < hnb { emit_g_b.push(Vec::new()); i += 1; } }
    { let eb = entry.as_bytes(); let mut pp = 0; while pp < eb.len() { let e = lend(eb, pp); let line = &entry[pp..e]; pp = e + 1; if tok_eq(line, 0, "gdef") && ntok(line) >= 2 { let gn = tok_str(line, 1); if hmap_idx(&emit_g_b, &emit_g, hnb, &gn) < 0 { emit_g_b[shash(&gn, hnb) as usize].push(emit_g.len()); emit_g.push(gn); } } } }
    {
        let mut li = 1;
        while li < cstart.len() {
            let cp = __arena_cp();
            let lib_raw: &str = &whole[cstart[li]..cend[li]];
            let lib_owned: String = strip_qual(lib_raw, &defs_of(lib_raw), &gqs_k, &gqs_multi, &gqs_b, hnb);
            let lib: &str = &lib_owned;
            let mut cur_name = String::new(); let mut cur_block = String::new(); let mut have = false; let mut keep_this = false;
            let mut kept_text = String::new(); let mut gdef_lines: Vec<String> = Vec::new();
            let lbb = lib.as_bytes(); let mut p = 0;
            while p < lbb.len() {
                let e = lend(lbb, p); let line = &lib[p..e]; p = e + 1;
                if tok_eq(line, 0, "fn") && ntok(line) >= 2 {
                    if have && keep_this { kept_text.push_str(&cur_block); }
                    cur_name = tok_str(line, 1); cur_block = String::new(); have = true; keep_this = hset_has(&kept_b, hnb, &cur_name);
                }
                if tok_eq(line, 0, "gdef") && ntok(line) >= 2 { gdef_lines.push(format!("{}", line)); }
                if have && tok_eq(line, 0, "gdef") == false { cur_block.push_str(line); cur_block.push('\n'); }
            }
            if have && keep_this { kept_text.push_str(&cur_block); }
            let mut grefd: Vec<String> = Vec::new();
            let kb = kept_text.as_bytes(); let mut q = 0;
            while q < kb.len() { let e = lend(kb, q); let line = &kept_text[q..e]; q = e + 1; let nt = ntok(line); let mut j = 0; while j + 1 < nt { if tok_eq(line, j, "gref") { let g = tok_str(line, j + 1); if !vec_has(&grefd, &g) { grefd.push(g); } } j += 1; } }
            for gl in gdef_lines.iter() { if ntok(gl) >= 2 { let gn = tok_str(gl, 1); if vec_has(&grefd, &gn) && !vec_has(&entry_gd, &gn) && starts_us(&gn) == false && hmap_idx(&emit_g_b, &emit_g, hnb, &gn) < 0 { print!("{}\n", gl); emit_g_b[shash(&gn, hnb) as usize].push(emit_g.len()); emit_g.push(format!("{}", gn)); } } }
            let self_crate: String = { let mut r = String::new(); let sb = kept_text.as_bytes(); let mut sp = 0; while sp < sb.len() { let e = lend(sb, sp); let line = &kept_text[sp..e]; sp = e + 1; if tok_eq(line, 0, "fn") && ntok(line) >= 2 { let ct = crate_tag(&tok_str(line, 1)); if ct.len() > 0 { r = ct; break; } } } r };
            emit_canon(&kept_text, &uq_k, &uq_full, &uq_multi, &uq_b, &ts_k, &ts_full, &ts_multi, &ts_b, &tq_k, &tq_full, &tq_multi, &tq_b, &nm_k, &nm_full, &nm_multi, &nm_b, &self_crate, &cs_key, &cs_full, &cs_b, hnb, (li as i64) * 1000000);
            __arena_reset(cp);
            li += 1;
        }
    }
}
