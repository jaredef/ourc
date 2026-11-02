// odrv -- the DRIVER: module expansion, prelude, monomorphization passes, lowering/codegen loop, CLI
struct ModEdit {
    at: u32,
    ins: u32,
    rem: u32,
}
// R2a P5: the first line of every metadata unit `--emit-meta` writes
const META_HEADER: &str = "//@ourc-meta v1";
fn dir_of(path: &str) -> String {
    let b = path.as_bytes();
    let mut cut = 0;
    let mut i = 0;
    while i < b.len() {
        if b[i] == 47 {
            cut = i + 1;
        }
        i = i + 1;
    }
    String::from(&path[..cut])
}
// find `mod NAME;` and splice in the file's text as an inline module
fn expand_mods(src: &str, dir: &str, eds: &mut Vec<ModEdit>) -> String {
    let b = src.as_bytes();
    let mut out = String::from("");
    let mut i = 0;
    // P-MC-MODSCAN: this scan is over RAW BYTES, so `mod X;` inside a COMMENT or a STRING used to be taken
    // for a module declaration -- `// find \`mod NAME;\` and splice ..` in this very file made the compiler
    // try to splice a module named NAME. Track the three non-code regions and only match inside code.
    // (rustc is of course inert to both; probed.)
    let mut inline = false;
    let mut inblock = false;
    let mut instr = false;
    let mut inch = false;
    let mut esc = false;
    while i < b.len() {
        let mut matched = false;
        if inline {
            if b[i] == 10 {
                inline = false;
            }
        } else if inblock {
            if b[i] == 42 && i + 1 < b.len() && b[i + 1] == 47 {
                inblock = false;
                out.push('*');
                i = i + 1;
            }
        } else if instr {
            if esc {
                esc = false;
            } else if b[i] == 92 {
                esc = true;
            } else if b[i] == 34 {
                instr = false;
            }
        } else if inch {
            if esc {
                esc = false;
            } else if b[i] == 92 {
                esc = true;
            } else if b[i] == 39 {
                inch = false;
            }
        } else if b[i] == 47 && i + 1 < b.len() && b[i + 1] == 47 {
            inline = true;
        } else if b[i] == 47 && i + 1 < b.len() && b[i + 1] == 42 {
            inblock = true;
        } else if b[i] == 34 {
            instr = true;
        } else if b[i] == 39 {
            inch = true;
        }
        let incode = !inline && !inblock && !instr && !inch;
        if incode
            && i + 4 < b.len()
            && b[i] == 109
            && b[i + 1] == 111
            && b[i + 2] == 100
            && b[i + 3] == 32
        {
            // read the name, then require `;`
            let mut j = i + 4;
            while j < b.len() && b[j] == 32 {
                j = j + 1;
            }
            let ns = j;
            while j < b.len()
                && ((b[j] >= 97 && b[j] <= 122)
                    || (b[j] >= 65 && b[j] <= 90)
                    || (b[j] >= 48 && b[j] <= 57)
                    || b[j] == 95)
            {
                j = j + 1;
            }
            let name = String::from(&src[ns..j]);
            let mut k = j;
            while k < b.len() && b[k] == 32 {
                k = k + 1;
            }
            if name.len() > 0 && k < b.len() && b[k] == 59 {
                // MODFILE (R2a P1): rustc's lookup -- X.rs, then X/mod.rs; .src kept for the legacy corpus
                let mut body = String::from("");
                let mut sfx = 0;
                while sfx < 3 && body.len() == 0 {
                    let mut path = String::from(dir);
                    path.push_str(name.as_str());
                    if sfx == 0 {
                        path.push_str(".rs");
                    } else if sfx == 1 {
                        path.push_str("/mod.rs");
                    } else {
                        path.push_str(".src");
                    }
                    body = match std::fs::read_to_string(&path) {
                        Ok(t) => t,
                        Err(_e) => String::from(""),
                    };
                    sfx = sfx + 1;
                }
                if body.len() > 0 {
                    let mut rep = String::from("mod ");
                    rep.push_str(name.as_str());
                    rep.push_str(" { ");
                    rep.push_str(body.as_str());
                    rep.push_str(" }");
                    eds.push(ModEdit {
                        at: out.len() as u32,
                        ins: rep.len() as u32,
                        rem: ((k + 1) - i) as u32,
                    });
                    out.push_str(rep.as_str());
                    i = k + 1;
                    matched = true;
                }
            }
        }
        // UTF8-STR: copy a WHOLE UTF-8 sequence -- a one-byte slice of a multi-byte char is not a str (rustc panics; SEM0 decoded
        // past the slice end, so `é` became U+00C0 U+0240 and every non-ASCII literal was mangled)
        if !matched {
            let n = if b[i] < 128 {
                1
            } else if b[i] < 224 {
                2
            } else if b[i] < 240 {
                3
            } else {
                4
            };
            let e = if i + n <= b.len() { i + n } else { b.len() };
            out.push_str(&src[i..e]);
            i = e;
        }
    }
    out
}
// map an expanded-source position back to the user's file (a position INSIDE a spliced module reports the
// `mod NAME;` site -- honest about the limit rather than printing a meaningless offset)
fn unmap_pos(q: u32, eds: &Vec<ModEdit>) -> u32 {
    let mut r = q;
    let mut i = 0;
    while i < eds.len() {
        if q >= eds[i].at && q < eds[i].at + eds[i].ins {
            return eds[i].at;
        }
        if q >= eds[i].at + eds[i].ins {
            if r >= eds[i].ins {
                r = r - eds[i].ins;
            }
            r = r + eds[i].rem;
        }
        i = i + 1;
    }
    r
}
fn unmap_diags(ds: &mut Vec<Diag>, eds: &Vec<ModEdit>) {
    if eds.len() == 0 {
        return;
    }
    let mut i = 0;
    while i < ds.len() {
        ds[i].lo = unmap_pos(ds[i].lo, eds);
        ds[i].hi = unmap_pos(ds[i].hi, eds);
        i = i + 1;
    }
}
fn __arena_cp() -> i64 {
    0
}
fn __arena_reset(_m: i64) {}
// V1.2 DRIVE-LINK: when ourc compiles itself these two are intercepted by name (olower) as libc `system` and a file write;
// these bodies are only what the SEM0-built ourc runs, which has neither (it says so, and `--emit-ir` still works)
fn __ourc_system(_c: &str) -> i64 {
    0 - 1
}
fn __ourc_writefile(_p: &str, _t: &str) -> i64 {
    1
}
// V5.3 BUDGET: arm the compile time limit (SIGALRM -> diagnostic + exit 1) and the memory cap (RLIMIT_AS; a failed
// allocation -> diagnostic + exit 1); a no-op in the SEM0 build
fn __ourc_budget(_secs: i64, _gb: i64) -> i64 {
    0
}
// NULL-PTR-RET: the value a body ends in is `std::ptr::null()` / `null_mut()` (through lets and statement sequences)
fn tail_is_ptrnull(e: &oast::E) -> bool {
    match e {
        oast::E::Call(n, _a, _l, _h) => n.as_str() == "$ptrnull",
        oast::E::Seq(_a, b, _l, _h) => tail_is_ptrnull(b),
        oast::E::Let(_n, _v, b, _l, _h) => tail_is_ptrnull(b),
        oast::E::LetT(_n, _t, _v, b, _l, _h) => tail_is_ptrnull(b),
        _ => false,
    }
}
// `'path'` for a shell command: a path holding `'` is refused rather than quoted wrong
fn sh_quote(p: &str) -> String {
    let mut q = String::from("'");
    q.push_str(p);
    q.push('\'');
    q
}
// V1.2: `-o OUT` -- write the IR beside OUT, run llc + cc (each overridable by $LLC / $CC), remove the temporaries; the
// exit code is the toolchain's
fn link_exe(ir: &str, out: &str) -> i64 {
    let b = out.as_bytes();
    let mut k = 0;
    while k < b.len() {
        if b[k] == 39 {
            eprintln!("ourc: -o {}: a path containing `'` is not supported", out);
            return 1;
        }
        k = k + 1;
    }
    let mut ll = String::from(out);
    ll.push_str(".ourc.ll");
    let mut obj = String::from(out);
    obj.push_str(".ourc.o");
    if __ourc_writefile(ll.as_str(), ir) != 0 {
        eprintln!("ourc: -o: cannot write `{}` (this ourc build cannot write files: use the self-built ourc', or --emit-ir)", ll);
        return 1;
    }
    let mut c = String::from("command -v \"${LLC:-llc}\" >/dev/null 2>&1 || { echo \"ourc: llc not found (set LLC=, or install LLVM)\" >&2; rm -f ");
    c.push_str(sh_quote(ll.as_str()).as_str());
    c.push_str("; exit 127; }; \"${LLC:-llc}\" -O2 -filetype=obj ");
    c.push_str(sh_quote(ll.as_str()).as_str());
    c.push_str(" -o ");
    c.push_str(sh_quote(obj.as_str()).as_str());
    c.push_str(" && \"${CC:-cc}\" -no-pie ");
    c.push_str(sh_quote(obj.as_str()).as_str());
    c.push_str(" -o ");
    c.push_str(sh_quote(out).as_str());
    c.push_str(" -lm; r=$?; rm -f ");
    c.push_str(sh_quote(ll.as_str()).as_str());
    c.push(' ');
    c.push_str(sh_quote(obj.as_str()).as_str());
    c.push_str("; exit $r");
    let r = __ourc_system(c.as_str());
    if r < 0 {
        eprintln!("ourc: -o: this ourc build cannot run llc / cc (use the self-built ourc', or --emit-ir)");
        return 1;
    }
    r
}
// `__usrc__` (the last item) marks where the USER source begins: token-level checks that the prelude's own idiom would trip
// (A2 UNSIZED-STR: the prelude takes `s: str` by value) start after it
fn prelude() -> String {
    String::from("fn rangeto(b: int) -> int { b } fn rangetoinc(b: int) -> int { b } fn f64_tieup(x: f64, p: int, s: str) -> str { let l = f64_snprintf(p + 40, x); let n = len(l); let mut d = 0; while d < n && str_at(l, d) != 46 { d = d + 1; } let q = d + 1 + p; if q >= n || str_at(l, q) != 53 { return s; } let mut k = q + 1; while k < n { if str_at(l, k) != 48 { return s; } k = k + 1; } let mut tl = q; if p == 0 { tl = d; } let t = str_sub(l, 0, tl); if s != t { return s; } let mut out = \"\"; let mut i = len(t) - 1; let mut carry = 1; while i >= 0 { let c = str_at(t, i); if carry == 1 && c >= 48 && c <= 57 { if c == 57 { out = str_cat(\"0\", out); } else { out = str_cat(str_sub(\"0123456789\", c - 47, 1), out); carry = 0; } } else { out = str_cat(str_sub(t, i, 1), out); } i = i - 1; } if carry == 1 { if str_at(t, 0) == 45 { out = str_cat(\"-1\", str_sub(out, 1, len(out) - 1)); } else { out = str_cat(\"1\", out); } } out } fn f64_big(x: f64) -> str { let b = f64_bits(x); let mut p = 0; let mut s = \"\"; while p <= 17 { s = f64_snprintf_e(p, x); if f64_bits(f64_parse(s)) == b { break; } p = p + 1; } let n = len(s); let mut i = 0; let mut neg = 0; if str_at(s, 0) == 45 { neg = 1; i = 1; } let mut dig = \"\"; while i < n && str_at(s, i) != 101 { if str_at(s, i) != 46 { dig = str_cat(dig, str_sub(s, i, 1)); } i = i + 1; } i = i + 1; let mut esg = 1; if str_at(s, i) == 45 { esg = 0 - 1; i = i + 1; } else if str_at(s, i) == 43 { i = i + 1; } let mut e = 0; while i < n { e = e * 10 + (str_at(s, i) - 48); i = i + 1; } e = e * esg; let z = e - (len(dig) - 1); let mut o = dig; if z > 0 { o = str_cat(o, fmt_rep(\"0\", z)); } if neg == 1 { o = str_cat(\"-\", o); } o } fn fmt_rep(c: str, n: int) -> str { let mut o = \"\"; let mut i = 0; while i < n { o = str_cat(o, c); i = i + 1; } o } fn fmt_pad(s: str, w: int, al: int, f: str) -> str { let n = s.len() as int; let mut nc = 0; let mut q = 0; while q < n { let b = str_at(s, q); if b < 128 || b >= 192 { nc = nc + 1; } q = q + 1; } if nc >= w { return s; } let p = w - nc; if al == 1 { return str_cat(s, fmt_rep(f, p)); } if al == 3 { let l = p / 2; return str_cat(str_cat(fmt_rep(f, l), s), fmt_rep(f, p - l)); } if al == 4 { if str_sub(s, 0, 1) == \"-\" { return str_cat(\"-\", str_cat(fmt_rep(\"0\", p), str_sub(s, 1, n))); } return str_cat(fmt_rep(\"0\", p), s); } str_cat(fmt_rep(f, p), s) } fn vstr_join(v: Vec<str>, sep: str) -> str { let n = seq_len(v); let mut out = \"\"; let mut i = 0; while i < n { if i > 0 { out = str_cat(out, sep); } out = str_cat(out, v.get(i)); i = i + 1; } out } fn vstr_concat(v: Vec<str>) -> str { vstr_join(v, \"\") } #[derive(Copy)] struct HashMap { ks: Vec<int>, vs: Vec<int> } impl HashMap { fn new() -> HashMap { HashMap { ks: vec_new(), vs: vec_new() } } } fn hmi_find(m: HashMap, k: int) -> int { let n = seq_len(m.ks); let mut i = 0; while i < n { if m.ks.get(i) == k { return i; } i = i + 1; } 0 - 1 } fn hms_find(m: HashMap, k: str) -> int { let n = seq_len(m.ks); let mut i = 0; while i < n { if streq(m.ks.get(i), k) == 1 { return i; } i = i + 1; } 0 - 1 } fn hmi_insert(m: HashMap, k: int, v: int) -> int { let i = hmi_find(m, k); if i >= 0 { m.vs.set(i, v); 0 } else { m.ks.push(k); m.vs.push(v); 0 } } fn hms_insert(m: HashMap, k: str, v: int) -> int { let i = hms_find(m, k); if i >= 0 { m.vs.set(i, v); 0 } else { m.ks.push(k); m.vs.push(v); 0 } } fn hmi_remove(m: HashMap, k: int) -> Option { let i = hmi_find(m, k); if i < 0 { return None; } let v = m.vs.get(i); let l = seq_len(m.ks) - 1; m.ks[i] = m.ks.get(l); m.vs[i] = m.vs.get(l); m.ks.pop(); m.vs.pop(); Some(v) } fn hms_remove(m: HashMap, k: str) -> Option { let i = hms_find(m, k); if i < 0 { return None; } let v = m.vs.get(i); let l = seq_len(m.ks) - 1; m.ks[i] = m.ks.get(l); m.vs[i] = m.vs.get(l); m.ks.pop(); m.vs.pop(); Some(v) } fn hmi_get(m: HashMap, k: int) -> Option { let i = hmi_find(m, k); if i >= 0 { Some(m.vs.get(i)) } else { None } } fn hms_get(m: HashMap, k: str) -> Option { let i = hms_find(m, k); if i >= 0 { Some(m.vs.get(i)) } else { None } } fn hmi_contains(m: HashMap, k: int) -> bool { hmi_find(m, k) >= 0 } fn hms_contains(m: HashMap, k: str) -> bool { hms_find(m, k) >= 0 } fn hmi_idx(m: HashMap, k: int) -> int { let i = hmi_find(m, k); m.vs.get(i) } fn hms_idx(m: HashMap, k: str) -> int { let i = hms_find(m, k); m.vs.get(i) } fn hm_len(m: HashMap) -> int { seq_len(m.ks) } fn hm_empty(m: HashMap) -> bool { seq_len(m.ks) == 0 } fn hm_keys(m: HashMap) -> Vec<int> { let o: Vec<int> = vec_new(); let n = seq_len(m.ks); let mut i = 0; while i < n { o.push(m.ks.get(i)); i = i + 1; } o }  enum Ordering { Less, Equal, Greater } fn int_cmp(a: int, b: int) -> Ordering { if a < b { Ordering::Less } else if a > b { Ordering::Greater } else { Ordering::Equal } } fn ord_is_gt(o: Ordering) -> bool { match o { Ordering::Greater => true, _ => false } } fn vsort_by(v: Vec<int>, f: Fn) -> int { let n = seq_len(v); let mut i = 1; while i < n { let x = v.get(i); let mut j = i; while j > 0 && ord_is_gt(f(v.get(j - 1), x)) { v.set(j, v.get(j - 1)); j = j - 1; } v.set(j, x); i = i + 1; } 0 } macro_rules! matches { ($e:expr, $p:pat) => { match $e { $p => true, _ => false } }; } enum Option { None, Some(v) } enum Result { Ok(v), Err(e) } fn map<T>(v: Vec<T>, f: Fn) -> Vec<T> { let o: Vec<int> = vec_new(); for x in v { o.push(f(x)); } o } fn filter<T>(v: Vec<T>, f: Fn) -> Vec<T> { let o: Vec<int> = vec_new(); for x in v { if f(x) == 1 { o.push(x); } } o } fn fold<T>(v: Vec<T>, init: int, f: Fn) -> int { let mut a = init; for x in v { a = f(a, x); } a } fn sum<T>(v: Vec<T>) -> int { let mut a = 0; for x in v { a = a + x; } a } fn oproduct<T>(v: Vec<T>) -> int { let mut a = 1; for x in v { a = a * x; } a } fn vmax(v: Vec<int>) -> int { let mut m = v.get(0); for x in v { if m < x { m = x; } } m } fn range(n: int) -> Vec<int> { let o: Vec<int> = vec_new(); for i in 0..n { o.push(i); } o } fn str_from_int(n: int) -> str { if n == 0 { return \"0\"; } let mut v = n; let mut neg = 0; if v < 0 { neg = 1; v = 0 - v; } let mut r = \"\"; while v > 0 { let d = v - v / 10 * 10; r = str_cat(str_sub(\"0123456789\", d, 1), r); v = v / 10; } if neg == 1 { r = str_cat(\"-\", r); } r } fn str_split(s: str, sep: str) -> Vec<str> { let o: Vec<str> = vec_new(); let m = len(sep); if m == 0 { o.push(s); return o; } let mut rest = s; let mut go = 1; while go == 1 { let i = str_find(rest, sep); if i < 0 { o.push(rest); go = 0; } else { o.push(str_sub(rest, 0, i)); rest = str_sub(rest, i + m, len(rest) - i - m); } } o } fn str_find(h: str, nd: str) -> int { let n = len(h); let m = len(nd); let mut i = 0; while i + m <= n { if str_sub(h, i, m) == nd { return i; } i = i + 1; } 0 - 1 } fn str_has(h: str, nd: str) -> int { if str_find(h, nd) < 0 { 0 } else { 1 } } fn say(label: str, n: int) -> int { print(str_cat(label, str_from_int(n))) } fn dbghex(c: int) -> str { let h = \"0123456789abcdef\"; if c < 16 { return str_sub(h, c, 1); } str_cat(str_sub(h, c / 16, 1), str_sub(h, c - c / 16 * 10 - c / 16 * 6, 1)) } fn dbgq(s: str) -> int { print_char(34); let n = len(s); let mut i = 0; while i < n { let c = str_at(s, i); if c == 34 { print_char(92); print_char(34); } else if c == 92 { print_char(92); print_char(92); } else if c == 10 { print_char(92); print_char(110); } else if c == 9 { print_char(92); print_char(116); } else if c == 13 { print_char(92); print_char(114); } else if c == 0 { print_char(92); print_char(48); } else if c < 32 { print_char(92); print_char(117); print_char(123); print_raw(dbghex(c)); print_char(125); } else { print_raw(str_sub(s, i, 1)); } i = i + 1; } print_char(34) } fn dbgc(c: int) -> int { print_char(39); if c == 39 { print_char(92); print_char(39); } else if c == 92 { print_char(92); print_char(92); } else if c == 10 { print_char(92); print_char(110); } else if c == 9 { print_char(92); print_char(116); } else if c == 13 { print_char(92); print_char(114); } else if c == 0 { print_char(92); print_char(48); } else if c < 32 { print_char(92); print_char(117); print_char(123); print_raw(dbghex(c)); print_char(125); } else if c > 127 { let ch: char = c; print_raw(ch.to_string()); } else { print_char(c); } print_char(39) } fn dbg_vi(v: Vec<int>) -> int { print_raw(\"[\"); let n = seq_len(v); let mut i = 0; while i < n { if i > 0 { print_raw(\", \"); } print_int_raw(v.get(i)); i = i + 1; } print_raw(\"]\") } fn dbg_vb(v: Vec<int>) -> int { print_raw(\"[\"); let n = seq_len(v); let mut i = 0; while i < n { if i > 0 { print_raw(\", \"); } if v.get(i) == 1 { print_raw(\"true\"); } else { print_raw(\"false\"); } i = i + 1; } print_raw(\"]\") } fn fis_nan(x: f64) -> bool { x != x } fn fis_inf(x: f64) -> bool { x == 1.0 / 0.0 || x == -1.0 / 0.0 } fn fis_fin(x: f64) -> bool { !(x != x) && !(x == 1.0 / 0.0 || x == -1.0 / 0.0) } fn fsignum(x: f64) -> f64 { if x != x { return x; } if f64_bits(x) < 0 { return -1.0; } 1.0 } fn dbg_vf(v: Vec<f64>) -> int { print_raw(\"[\"); let n = seq_len(v); let mut i = 0; while i < n { if i > 0 { print_raw(\", \"); } print_raw(f64_str(v.get(i), 1)); i = i + 1; } print_raw(\"]\") } fn dbg_vc(v: Vec<int>) -> int { print_raw(\"[\"); let n = seq_len(v); let mut i = 0; while i < n { if i > 0 { print_raw(\", \"); } dbgc(v.get(i)); i = i + 1; } print_raw(\"]\") } fn dbg_vs(v: Vec<str>) -> int { print_raw(\"[\"); let n = seq_len(v); let mut i = 0; while i < n { if i > 0 { print_raw(\", \"); } dbgq(v.get(i)); i = i + 1; } print_raw(\"]\") } fn dbg_vvi(v: Vec<Vec<int>>) -> int { print_raw(\"[\"); let n = seq_len(v); let mut i = 0; while i < n { if i > 0 { print_raw(\", \"); } dbg_vi(v.get(i)); i = i + 1; } print_raw(\"]\") } fn dbg_ai(a: arr) -> int { print_raw(\"[\"); let n = seq_len(a); let mut i = 0; while i < n { if i > 0 { print_raw(\", \"); } print_int_raw(seq_get(a, i)); i = i + 1; } print_raw(\"]\") } fn sempty(s: str) -> int { if len(s) == 0 { 1 } else { 0 } } fn shas(s: str, nd: str) -> int { if str_find(s, nd) < 0 { 0 } else { 1 } } fn sfind(s: str, nd: str) -> Option { let i = str_find(s, nd); if i < 0 { return None; } Some(i) } fn srfind(s: str, nd: str) -> Option { let m = len(nd); let mut i = len(s) - m; while i >= 0 { if str_sub(s, i, m) == nd { return Some(i); } i = i - 1; } None } fn sstarts(s: str, p: str) -> int { let m = len(p); if m > len(s) { return 0; } if str_sub(s, 0, m) == p { 1 } else { 0 } } fn sends(s: str, p: str) -> int { let m = len(p); let n = len(s); if m > n { return 0; } if str_sub(s, n - m, m) == p { 1 } else { 0 } } fn sws(c: int) -> int { if c == 32 { 1 } else { if c == 9 { 1 } else { if c == 10 { 1 } else { if c == 13 { 1 } else { 0 } } } } } fn strims(s: str) -> str { let n = len(s); let mut i = 0; while i < n { if sws(str_at(s, i)) == 1 { i = i + 1; } else { break; } } str_sub(s, i, n - i) } fn strime(s: str) -> str { let mut n = len(s); while n > 0 { if sws(str_at(s, n - 1)) == 1 { n = n - 1; } else { break; } } str_sub(s, 0, n) } fn strim(s: str) -> str { strime(strims(s)) } fn ccase_up(c: int) -> int { if c >= 97 && c <= 122 { return c - 32; } if c >= 224 && c <= 254 && c != 247 { return c - 32; } if c == 255 { return 376; } if c >= 256 && c <= 311 { if c - c / 2 * 2 == 1 { return c - 1; } return c; } if c >= 330 && c <= 375 { if c - c / 2 * 2 == 1 { return c - 1; } return c; } if c >= 313 && c <= 328 { if c - c / 2 * 2 == 0 { return c - 1; } return c; } if c >= 377 && c <= 382 { if c - c / 2 * 2 == 0 { return c - 1; } return c; } if c == 962 { return 931; } if c >= 945 && c <= 969 { return c - 32; } if c >= 1072 && c <= 1103 { return c - 32; } if c >= 1104 && c <= 1119 { return c - 80; } c } fn ccase_low(c: int) -> int { if c >= 65 && c <= 90 { return c + 32; } if c >= 192 && c <= 222 && c != 215 { return c + 32; } if c == 376 { return 255; } if c >= 256 && c <= 311 { if c - c / 2 * 2 == 0 { return c + 1; } return c; } if c >= 330 && c <= 375 { if c - c / 2 * 2 == 0 { return c + 1; } return c; } if c >= 313 && c <= 328 { if c - c / 2 * 2 == 1 { return c + 1; } return c; } if c >= 377 && c <= 382 { if c - c / 2 * 2 == 1 { return c + 1; } return c; } if c >= 913 && c <= 937 && c != 930 { return c + 32; } if c >= 1040 && c <= 1071 { return c + 32; } if c >= 1024 && c <= 1039 { return c + 80; } c } fn s_nonascii(s: str) -> bool { let n = len(s); let mut i = 0; while i < n { if str_at(s, i) > 127 { return true; } i = i + 1; } false } fn ccase(c: int, up: int) -> int { if up == 1 { return ccase_up(c); } ccase_low(c) } fn scase8(s: str, up: int) -> str { let v = str_chars_vec(s); let mut r = \"\"; for c in v { let ch: char = ccase(c, up); r = str_cat(r, ch.to_string()); } r } fn supper(s: str) -> str { if s_nonascii(s) { return scase8(s, 1); } let n = len(s); let mut r = \"\"; let mut i = 0; while i < n { let c = str_at(s, i); if c >= 97 && c <= 122 { r = str_cat(r, str_sub(\"ABCDEFGHIJKLMNOPQRSTUVWXYZ\", c - 97, 1)); } else { r = str_cat(r, str_sub(s, i, 1)); } i = i + 1; } r } fn slower(s: str) -> str { if s_nonascii(s) { return scase8(s, 0); } let n = len(s); let mut r = \"\"; let mut i = 0; while i < n { let c = str_at(s, i); if c >= 65 && c <= 90 { r = str_cat(r, str_sub(\"abcdefghijklmnopqrstuvwxyz\", c - 65, 1)); } else { r = str_cat(r, str_sub(s, i, 1)); } i = i + 1; } r } fn srepeat(s: str, k: int) -> str { let mut r = \"\"; let mut i = 0; while i < k { r = str_cat(r, s); i = i + 1; } r } fn sreplace(s: str, f: str, t: str) -> str { let m = len(f); if m == 0 { return sins_empty(s, t, 1099511627776); } let mut r = \"\"; let mut rest = s; while len(rest) > 0 { let i = str_find(rest, f); if i < 0 { r = str_cat(r, rest); rest = \"\"; } else { r = str_cat(r, str_sub(rest, 0, i)); r = str_cat(r, t); rest = str_sub(rest, i + m, len(rest) - i - m); } } r } fn sreplacen(s: str, f: str, t: str, n: int) -> str { let m = len(f); if m == 0 { return sins_empty(s, t, n); } let mut r = \"\"; let mut rest = s; let mut k = 0; while len(rest) > 0 && k < n { let i = str_find(rest, f); if i < 0 { r = str_cat(r, rest); rest = \"\"; } else { r = str_cat(r, str_sub(rest, 0, i)); r = str_cat(r, t); rest = str_sub(rest, i + m, len(rest) - i - m); k = k + 1; } } str_cat(r, rest) } fn sins_empty(s: str, t: str, n: int) -> str { let l = len(s); let mut r = \"\"; let mut k = 0; let mut i = 0; while i <= l { if k < n { r = str_cat(r, t); k = k + 1; } if i == l { i = l + 1; } else { let mut j = i + 1; while j < l && str_at(s, j) >= 128 && str_at(s, j) < 192 { j = j + 1; } r = str_cat(r, str_sub(s, i, j - i)); i = j; } } r } fn vsort_s(v: Vec<str>) -> int { let n = seq_len(v); let mut i = 1; while i < n { let mut j = i; while j > 0 && scmp(v.get(j - 1), v.get(j)) > 0 { let t = v.get(j); v[j] = v.get(j - 1); v[j - 1] = t; j = j - 1; } i = i + 1; } 0 } fn scmp(a: str, b: str) -> int { let na = len(a); let nb = len(b); let mut i = 0; while i < na && i < nb { let x = str_at(a, i); let y = str_at(b, i); if x < y { return 0 - 1; } if y < x { return 1; } i = i + 1; } if na < nb { return 0 - 1; } if nb < na { return 1; } 0 } fn ounwrap(o: Option) -> int { match o { Some(v) => v, None => panic_exit() } } fn ounwrap_or(o: Option, d: int) -> int { match o { Some(v) => v, None => d } } fn ois_some(o: Option) -> int { match o { Some(v) => 1, None => 0 } } fn ois_none(o: Option) -> int { match o { Some(v) => 0, None => 1 } } fn omap(o: Option, f: Fn) -> Option { match o { Some(v) => Some(f(v)), None => None } } fn omap_or(o: Option, d: int, f: Fn) -> int { match o { Some(v) => f(v), None => d } } fn icount_ones(x: int, bits: int) -> int { let mut c = 0; let mut i = 0; while i < bits { let b = (x >> i) & 1; c = c + b; i = i + 1; } c } fn ileading_zeros(x: int, bits: int) -> int { let mut c = 0; let mut i = bits - 1; while i >= 0 { let b = (x >> i) & 1; if b == 1 { return c; } c = c + 1; i = i - 1; } c } fn itrailing_zeros(x: int, bits: int) -> int { let mut c = 0; let mut i = 0; while i < bits { let b = (x >> i) & 1; if b == 1 { return c; } c = c + 1; i = i + 1; } c } fn isat_add(a: int, b: int, lo: int, hi: int) -> int { let s = a + b; if s < lo { return lo; } if s > hi { return hi; } s } fn isat_sub(a: int, b: int, lo: int, hi: int) -> int { let s = a - b; if s < lo { return lo; } if s > hi { return hi; } s } fn isat_mul(a: int, b: int, lo: int, hi: int) -> int { let s = a * b; if s < lo { return lo; } if s > hi { return hi; } s } fn ichk_add(a: int, b: int, lo: int, hi: int) -> Option { let s = a + b; if s < lo { return None; } if s > hi { return None; } Some(s) } fn ichk_sub(a: int, b: int, lo: int, hi: int) -> Option { let s = a - b; if s < lo { return None; } if s > hi { return None; } Some(s) } fn ichk_mul(a: int, b: int, lo: int, hi: int) -> Option { let s = a * b; if s < lo { return None; } if s > hi { return None; } Some(s) } fn ichecked_div(x: int, d: int) -> Option { if d == 0 { return None; } Some(x / d) } fn ounwrap_or_else(o: Option, f: Fn) -> int { match o { Some(v) => v, None => f() } } fn oand_then(o: Option, f: Fn) -> Option { match o { Some(v) => f(v), None => None } } fn ipow(x: int, e: int) -> int { let mut r = 1; let mut i = 0; while i < e { r = r * x; i = i + 1; } r } fn iabs(x: int) -> int { if x < 0 { return 0 - x; } x } fn imax(x: int, y: int) -> int { if x > y { x } else { y } } fn imin(x: int, y: int) -> int { if x < y { x } else { y } } fn ook_or(o: Option, e: int) -> Result { match o { Some(v) => Ok(v), None => Err(e) } } fn runwrap_or(r: Result, d: int) -> int { match r { Ok(v) => v, Err(e) => d } } fn rok_opt(r: Result) -> Option { match r { Ok(v) => Some(v), Err(e) => None } } fn ofilter(o: Option, f: Fn) -> Option { match o { Some(v) => if f(v) == 1 { Some(v) } else { None }, None => None } } fn cis_digit(c: int) -> bool { c >= 48 && c <= 57 } fn cis_alpha(c: int) -> bool { (c >= 65 && c <= 90) || (c >= 97 && c <= 122) || (c >= 192 && c <= 591 && c != 215 && c != 247) || (c >= 880 && c <= 1327 && c != 894 && c != 903) } fn cis_alnum(c: int) -> bool { cis_digit(c) || cis_alpha(c) } fn cis_ws(c: int) -> bool { c == 32 || c == 9 || c == 10 || c == 13 } fn rmap(r: Result, f: Fn) -> Result { match r { Ok(v) => Ok(f(v)), Err(e) => Err(e) } } fn rmaperr(r: Result, f: Fn) -> Result { match r { Ok(v) => Ok(v), Err(e) => Err(f(e)) } } fn bnext(s: str) -> Option { if len(s) == 0 { return None; } Some(str_at(s, 0)) } fn eargs() -> Vec<str> { let o: Vec<int> = vec_new(); let n = argc(); let mut i = 0; while i < n { o.push(argv_at(i)); i = i + 1; } o } fn freadok(p: str) -> Result { if fileok(p) == 0 { return Err(0); } Ok(readfile(p)) } fn vremove(v: Vec<int>, i: int) -> int { let n = seq_len(v); let out = v.get(i); let mut k = i; while k + 1 < n { v.set(k, v.get(k + 1)); k = k + 1; } let _d = vpop(v); out } fn vinsert(v: Vec<int>, i: int, x: int) -> int { let n = seq_len(v); v.push(x); let mut k = n; while k > i { v.set(k, v.get(k - 1)); k = k - 1; } v.set(i, x); 0 } fn vpop(v: Vec<int>) -> Option { if seq_len(v) == 0 { return None; } Some(vec_popraw(v)) } fn rexit(r: Result) -> int { match r { Ok(v) => 0, Err(e) => 1 } } fn runwrap(r: Result) -> int { match r { Ok(v) => v, Err(e) => panic_exit() } } fn rok(r: Result) -> int { match r { Ok(v) => 1, Err(e) => 0 } } fn rerr(r: Result) -> int { match r { Ok(v) => 0, Err(e) => 1 } } fn sparse(s: str) -> Result { let n = len(s); if n == 0 { return Err(0); } let mut i = 0; let mut neg = 0; if str_at(s, 0) == 45 { neg = 1; i = 1; } if i >= n { return Err(0); } let mut acc = 0; while i < n { let c = str_at(s, i); if c < 48 || c > 57 { return Err(0); } acc = acc * 10 + (c - 48); i = i + 1; } if neg == 1 { return Ok(0 - acc); } Ok(acc) } fn omax(v: Vec<int>) -> Option { if seq_len(v) == 0 { return None; } let mut m = v.get(0); for x in v { if m < x { m = x; } } Some(m) } fn omin(v: Vec<int>) -> Option { if seq_len(v) == 0 { return None; } let mut m = v.get(0); for x in v { if x < m { m = x; } } Some(m) } fn ofirst(v: Vec<int>) -> Option { if seq_len(v) == 0 { return None; } Some(v.get(0)) } fn olast(v: Vec<int>) -> Option { let n = seq_len(v); if n == 0 { return None; } Some(v.get(n - 1)) } fn vbsearch(v: Vec<int>, x: int) -> Result { let mut size = seq_len(v); if size == 0 { return Err(0); } let mut base = 0; while size > 1 { let half = size / 2; let mid = base + half; if v.get(mid) > x { base = base; } else { base = mid; } size = size - half; } let e = v.get(base); if e == x { return Ok(base); } if e < x { return Err(base + 1); } Err(base) } fn opos(v: Vec<int>, f: Fn) -> Option { let n = seq_len(v); let mut i = 0; while i < n { if f(v.get(i)) == 1 { return Some(i); } i = i + 1; } None } fn vclone(v: Vec<int>) -> Vec<int> { let o: Vec<int> = vec_new(); let n = seq_len(v); let mut i = 0; while i < n { o.push(v.get(i)); i = i + 1; } o } fn ssplit_ws(s: str) -> Vec<str> { let o: Vec<str> = vec_new(); let n = len(s); let mut i = 0; while i < n { while i < n && sws(str_at(s, i)) == 1 { i = i + 1; } let st = i; while i < n && sws(str_at(s, i)) == 0 { i = i + 1; } if i > st { o.push(str_sub(s, st, i - st)); } } o } fn arr_vec(a: arr) -> Vec<int> { let o: Vec<int> = vec_new(); let n = seq_len(a); let mut i = 0; while i < n { o.push(seq_get(a, i)); i = i + 1; } o } fn strim_endc(s: str, c: int) -> str { let mut n = len(s); while n > 0 && str_at(s, n - 1) == c { n = n - 1; } str_sub(s, 0, n) } fn strim_startc(s: str, c: int) -> str { let n = len(s); let mut i = 0; while i < n && str_at(s, i) == c { i = i + 1; } str_sub(s, i, n - i) } fn strim_c(s: str, c: int) -> str { strim_startc(strim_endc(s, c), c) } fn str_char_idx(s: str) -> Vec<tup2> { let o: Vec<tup2> = vec_new(); let n = len(s); let mut i = 0; while i < n { let c = str_at(s, i); if c < 128 { o.push((i, c)); i = i + 1; } else if c < 224 { o.push((i, (c - 192) * 64 + (str_at(s, i + 1) - 128))); i = i + 2; } else if c < 240 { o.push((i, ((c - 224) * 64 + (str_at(s, i + 1) - 128)) * 64 + (str_at(s, i + 2) - 128))); i = i + 3; } else { o.push((i, (((c - 240) * 64 + (str_at(s, i + 1) - 128)) * 64 + (str_at(s, i + 2) - 128)) * 64 + (str_at(s, i + 3) - 128))); i = i + 4; } } o } fn vwindows(v: Vec<int>, k: int) -> Vec<Vec<int>> { let o: Vec<Vec<int>> = vec_new(); let n = seq_len(v); let mut i = 0; while i + k <= n { let w: Vec<int> = vec_new(); let mut j = 0; while j < k { w.push(v.get(i + j)); j = j + 1; } o.push(w); i = i + 1; } o } fn vchunks(v: Vec<int>, k: int) -> Vec<Vec<int>> { let o: Vec<Vec<int>> = vec_new(); let n = seq_len(v); let mut i = 0; while i < n { let w: Vec<int> = vec_new(); let mut j = 0; while j < k && i + j < n { w.push(v.get(i + j)); j = j + 1; } o.push(w); i = i + k; } o } fn vsum_any(v: Vec<int>) -> int { let mut a = 0; let n = seq_len(v); let mut i = 0; while i < n { a = a + v.get(i); i = i + 1; } a } fn vsum_f(v: Vec<f64>) -> f64 { let mut a = 0.0; let n = seq_len(v); let mut i = 0; while i < n { a = a + v.get(i); i = i + 1; } a } fn vproduct_f(v: Vec<f64>) -> f64 { let mut a = 1.0; let n = seq_len(v); let mut i = 0; while i < n { a = a * v.get(i); i = i + 1; } a } fn hm_pairs(m: HashMap) -> Vec<tup2> { let o: Vec<tup2> = vec_new(); let n = seq_len(m.ks); let mut i = 0; while i < n { o.push((m.ks.get(i), m.vs.get(i))); i = i + 1; } o } fn t2_lt(a: tup2, b: tup2, sk: int) -> bool { let c = if sk == 1 { scmp(a.0, b.0) } else { if a.0 < b.0 { 0 - 1 } else { if b.0 < a.0 { 1 } else { 0 } } }; if c != 0 { return c < 0; } a.1 < b.1 } fn vsort_t2(v: Vec<tup2>, sk: int) -> int { let n = seq_len(v); let mut i = 1; while i < n { let x = v.get(i); let mut j = i; while j > 0 && t2_lt(x, v.get(j - 1), sk) { v.set(j, v.get(j - 1)); j = j - 1; } v.set(j, x); i = i + 1; } 0 } fn vsort_t2s(v: Vec<tup2>) -> int { vsort_t2(v, 1) } fn vsort_t2i(v: Vec<tup2>) -> int { vsort_t2(v, 0) } fn dbgq_s(s: str) -> str { let mut r = \"\\\"\"; let n = len(s); let mut i = 0; while i < n { let c = str_at(s, i); if c == 34 { r = str_cat(r, \"\\\\\\\"\"); } else if c == 92 { r = str_cat(r, \"\\\\\\\\\"); } else if c == 10 { r = str_cat(r, \"\\\\n\"); } else if c == 9 { r = str_cat(r, \"\\\\t\"); } else { r = str_cat(r, str_sub(s, i, 1)); } i = i + 1; } str_cat(r, \"\\\"\") } fn dbgc_s(c: int) -> str { let ch: char = c; let b = if c == 39 { \"\\\\'\" } else if c == 92 { \"\\\\\\\\\" } else if c == 10 { \"\\\\n\" } else { ch.to_string() }; str_cat(str_cat(\"'\", b), \"'\") } fn str_bytes_vec(s: str) -> Vec<int> { let o: Vec<int> = vec_new(); let n = len(s); let mut i = 0; while i < n { o.push(str_at(s, i)); i = i + 1; } o } fn ofind(v: Vec<int>, f: Fn) -> Option { let n = seq_len(v); let mut i = 0; while i < n { if f(v.get(i)) == 1 { return Some(v.get(i)); } i = i + 1; } None } fn opart_t(v: Vec<int>, f: Fn) -> Vec<int> { let mut a: Vec<int> = vec_new(); for x in v { if f(x) == 1 { a.push(x); } } a } fn opart_g(v: Vec<int>, f: Fn) -> Vec<int> { let mut b: Vec<int> = vec_new(); for x in v { if f(x) == 1 { } else { b.push(x); } } b } fn vstepby(v: Vec<int>, k: int) -> Vec<int> { let o: Vec<int> = vec_new(); let n = seq_len(v); let mut i = 0; while i < n { o.push(v.get(i)); i = i + k; } o } fn vretain(v: Vec<int>, f: Fn) -> int { let n = seq_len(v); let mut w = 0; let mut i = 0; while i < n { if f(v.get(i)) == 1 { v.set(w, v.get(i)); w = w + 1; } i = i + 1; } while seq_len(v) > w { let _d = vpop(v); } 0 } fn vsort(v: Vec<int>) -> int { let n = seq_len(v); let mut i = 1; while i < n { let x = v.get(i); let mut j = i; while j > 0 && v.get(j - 1) > x { v.set(j, v.get(j - 1)); j = j - 1; } v.set(j, x); i = i + 1; } 0 } fn vreverse(v: Vec<int>) -> int { let n = seq_len(v); let mut i = 0; let mut j = n; while i + 1 < j { j = j - 1; let t = v.get(i); v.set(i, v.get(j)); v.set(j, t); i = i + 1; } 0 } fn vswap(v: Vec<int>, i: int, j: int) -> int { let t = v.get(i); v.set(i, v.get(j)); v.set(j, t); 0 } fn vtruncate(v: Vec<int>, k: int) -> int { while seq_len(v) > k { let _d = vpop(v); } 0 } fn vdedup(v: Vec<int>) -> int { let n = seq_len(v); if n == 0 { return 0; } let mut w = 1; let mut i = 1; while i < n { if v.get(i) != v.get(w - 1) { v.set(w, v.get(i)); w = w + 1; } i = i + 1; } while seq_len(v) > w { let _d = vpop(v); } 0 } fn vappend(a: Vec<int>, b: Vec<int>) -> int { let n = seq_len(b); let mut i = 0; while i < n { a.push(b.get(i)); i = i + 1; } while seq_len(b) > 0 { let _d = vpop(b); } 0 } fn vsortbykey(v: Vec<int>, f: Fn) -> int { let n = seq_len(v); let mut i = 1; while i < n { let x = v.get(i); let kx = f(x); let mut j = i; while j > 0 && f(v.get(j - 1)) > kx { v.set(j, v.get(j - 1)); j = j - 1; } v.set(j, x); i = i + 1; } 0 } fn vmaxbykey(v: Vec<int>, f: Fn) -> Option { let n = seq_len(v); if n == 0 { return None; } let mut best = v.get(0); let mut bk = f(best); let mut i = 1; while i < n { let x = v.get(i); let k = f(x); if k >= bk { bk = k; best = x; } i = i + 1; } Some(best) } fn vminbykey(v: Vec<int>, f: Fn) -> Option { let n = seq_len(v); if n == 0 { return None; } let mut best = v.get(0); let mut bk = f(best); let mut i = 1; while i < n { let x = v.get(i); let k = f(x); if k < bk { bk = k; best = x; } i = i + 1; } Some(best) } fn vextend(a: Vec<int>, b: Vec<int>) -> int { let n = seq_len(b); let mut i = 0; while i < n { a.push(b.get(i)); i = i + 1; } 0 } fn ounwrap_or_default(o: Option) -> int { match o { Some(v) => v, None => 0 } } fn vclear(v: Vec<int>) -> int { while seq_len(v) > 0 { let _d = vpop(v); } 0 } fn vswapremove(v: Vec<int>, i: int) -> int { let n = seq_len(v); let out = v.get(i); v.set(i, v.get(n - 1)); let _d = vpop(v); out } fn vnth(v: Vec<int>, i: int) -> Option { if i < seq_len(v) { return Some(v.get(i)); } None } fn vreduce(v: Vec<int>, f: Fn) -> Option { let n = seq_len(v); if n == 0 { return None; } let mut a = v.get(0); let mut i = 1; while i < n { a = f(a, v.get(i)); i = i + 1; } Some(a) } fn fpowi(x: f64, e: int) -> f64 { let mut r = 1.0; let mut n = e; if n < 0 { n = 0 - n; } let mut i = 0; while i < n { r = r * x; i = i + 1; } if e < 0 { 1.0 / r } else { r } } fn fmax(a: f64, b: f64) -> f64 { if a != a { return b; } if b != b { return a; } if a < b { b } else { a } } fn fmin(a: f64, b: f64) -> f64 { if a != a { return b; } if b != b { return a; } if a < b { a } else { b } } fn fabs(x: f64) -> f64 { if x < 0.0 { 0.0 - x } else { x } } fn ffloor(x: f64) -> f64 { if x > 9007199254740992.0 { return x; } if x < 0.0 - 9007199254740992.0 { return x; } let t = (x as i64) as f64; if t > x { t - 1.0 } else { t } } fn fceil(x: f64) -> f64 { if x > 9007199254740992.0 { return x; } if x < 0.0 - 9007199254740992.0 { return x; } let t = (x as i64) as f64; if t < x { t + 1.0 } else { t } } fn fround(x: f64) -> f64 { if x >= 0.0 { ffloor(x + 0.5) } else { fceil(x - 0.5) } } fn vflatmap(v: Vec<int>, f: Fn) -> Vec<int> { let o: Vec<int> = vec_new(); let n = seq_len(v); let mut i = 0; while i < n { let inner: Vec<int> = f(v.get(i)); let m = seq_len(inner); let mut j = 0; while j < m { o.push(inner.get(j)); j = j + 1; } i = i + 1; } o } fn vchain(a: Vec<int>, b: Vec<int>) -> Vec<int> { let o: Vec<int> = vec_new(); let n = seq_len(a); let mut i = 0; while i < n { o.push(a.get(i)); i = i + 1; } let m = seq_len(b); let mut j = 0; while j < m { o.push(b.get(j)); j = j + 1; } o } fn venum(v: Vec<int>) -> Vec<tup2> { let o: Vec<tup2> = vec_new(); let n = seq_len(v); let mut i = 0; while i < n { o.push((i, v.get(i))); i = i + 1; } o } fn vzip(a: Vec<int>, b: Vec<int>) -> Vec<tup2> { let o: Vec<tup2> = vec_new(); let n = seq_len(a); let m = seq_len(b); let lim = if n < m { n } else { m }; let mut i = 0; while i < lim { o.push((a.get(i), b.get(i))); i = i + 1; } o } fn range2(a: int, b: int) -> Vec<int> { let o: Vec<int> = vec_new(); let mut i = a; while i < b { o.push(i); i = i + 1; } o } fn vmin(v: Vec<int>) -> int { let mut m = v.get(0); for x in v { if x < m { m = x; } } m } fn vrev(v: Vec<int>) -> Vec<int> { let o: Vec<int> = vec_new(); let n = seq_len(v); let mut i = n; while i > 0 { i = i - 1; o.push(v.get(i)); } o } fn vlast(v: Vec<int>) -> int { v.get(seq_len(v) - 1) } fn vempty(v: Vec<int>) -> int { if seq_len(v) == 0 { 1 } else { 0 } } fn vany(v: Vec<int>, f: Fn) -> int { for x in v { if f(x) == 1 { return 1; } } 0 } fn vall(v: Vec<int>, f: Fn) -> int { for x in v { if f(x) == 0 { return 0; } } 1 } fn vpos(v: Vec<int>, f: Fn) -> int { let n = seq_len(v); let mut i = 0; while i < n { if f(v.get(i)) == 1 { return i; } i = i + 1; } 0 - 1 } fn vtake(v: Vec<int>, k: int) -> Vec<int> { let o: Vec<int> = vec_new(); let n = seq_len(v); let mut i = 0; while i < n && i < k { o.push(v.get(i)); i = i + 1; } o } fn vtakewhile(v: Vec<int>, f: Fn) -> Vec<int> { let o: Vec<int> = vec_new(); let n = seq_len(v); let mut i = 0; while i < n { if f(v.get(i)) == 1 { o.push(v.get(i)); i = i + 1; } else { i = n; } } o } fn vskipwhile(v: Vec<int>, f: Fn) -> Vec<int> { let o: Vec<int> = vec_new(); let n = seq_len(v); let mut i = 0; let mut sk = 1; while i < n { if sk == 1 && f(v.get(i)) == 1 { } else { sk = 0; o.push(v.get(i)); } i = i + 1; } o } fn vskip(v: Vec<int>, k: int) -> Vec<int> { let o: Vec<int> = vec_new(); let n = seq_len(v); let mut i = k; while i < n { o.push(v.get(i)); i = i + 1; } o } fn vhas(v: Vec<int>, y: int) -> int { for x in v { if x == y { return 1; } } 0 } fn f64_trim(s: str) -> str { let dot = str_find(s, \".\"); if dot < 0 { return s; } let mut n = len(s); while n > dot + 1 { if str_sub(s, n - 1, 1) == \"0\" { n = n - 1; } else { break; } } if n == dot + 1 { n = dot; } str_sub(s, 0, n) } fn f64_exp10(s: str) -> int { let mut t = s; if str_sub(t, 0, 1) == \"-\" { t = str_sub(t, 1, len(t) - 1); } let dot = str_find(t, \".\"); if str_sub(t, 0, 1) == \"0\" { if dot < 0 { return 0; } let mut i = dot + 1; let mut z = 0; while i < len(t) { if str_sub(t, i, 1) == \"0\" { z = z + 1; i = i + 1; } else { break; } } return 0 - z - 1; } if dot < 0 { return len(t) - 1; } dot - 1 } fn f64_sci(x: f64) -> str { let mut p = 0; let mut s = \"\"; while p <= 17 { s = f64_snprintf_e(p, x); if f64_bits(f64_parse(s)) == f64_bits(x) { break; } p = p + 1; } let e = str_find(s, \"e\"); let mant = f64_trim(str_sub(s, 0, e)); let mut ex = str_sub(s, e + 1, len(s) - e - 1); let mut neg = 0; if str_sub(ex, 0, 1) == \"+\" { ex = str_sub(ex, 1, len(ex) - 1); } else { if str_sub(ex, 0, 1) == \"-\" { neg = 1; ex = str_sub(ex, 1, len(ex) - 1); } } while len(ex) > 1 { if str_sub(ex, 0, 1) == \"0\" { ex = str_sub(ex, 1, len(ex) - 1); } else { break; } } let mut r = str_cat(mant, \"e\"); if neg == 1 { r = str_cat(r, \"-\"); } str_cat(r, ex) } fn f64_str(x: f64, dbg: int) -> str { if f64_isnan(x) == 1 { return \"NaN\"; } let b = f64_bits(x); if b == 9218868437227405312 { return \"inf\"; } if b == 0 - 4503599627370496 { return \"-inf\"; } if dbg == 0 && (x >= 1e16 || x <= -1e16) { return f64_big(x); } let mut p = 0; let mut s = \"\"; while p <= 340 { s = f64_snprintf(p, x); if f64_bits(f64_parse(s)) == b { break; } p = p + 1; } s = f64_tieup(x, p, s); let t = f64_trim(s); if dbg == 0 { return t; } let ex = f64_exp10(s); if ex < 16 && 0 - 5 < ex { if str_find(t, \".\") < 0 { return str_cat(t, \".0\"); } return t; } f64_sci(x) } fn str_chars_vec(s: str) -> Vec<char> { let o: Vec<int> = vec_new(); let n = len(s); let mut i = 0; while i < n { let c = str_at(s, i); if c < 128 { o.push(c); i = i + 1; } else if c < 224 { o.push((c - 192) * 64 + (str_at(s, i + 1) - 128)); i = i + 2; } else if c < 240 { o.push(((c - 224) * 64 + (str_at(s, i + 1) - 128)) * 64 + (str_at(s, i + 2) - 128)); i = i + 3; } else { o.push((((c - 240) * 64 + (str_at(s, i + 1) - 128)) * 64 + (str_at(s, i + 2) - 128)) * 64 + (str_at(s, i + 3) - 128)); i = i + 4; } } o } fn vchar_string(v: Vec<int>) -> str { let mut r = \"\"; for c in v { let ch: char = c; r = str_cat(r, ch.to_string()); } r } fn cis_lower(c: int) -> bool { ccase_up(c) != c || c == 223 } fn cis_upper(c: int) -> bool { ccase_low(c) != c } fn iter_dflt_sum<T: Iterator>(mut it: T) -> int { let mut s = 0; while let Some(x) = it.next() { s = s + x; } s } fn iter_dflt_count<T: Iterator>(mut it: T) -> int { let mut c = 0; while let Some(_x) = it.next() { c = c + 1; } c }  fn __char_from_u32(v: int) -> Option<char> { if v >= 0 && v <= 1114111 && (v < 55296 || v > 57343) { Some(v as char) } else { None } } fn __usrc__() {}")
} // ITER-DFLT: provided Iterator methods over a user next()
fn with_prelude(src: &str) -> String {
    let mut s = prelude();
    s.push_str(src);
    s
}
// the same diagnostic can be produced once per walk of the expression that made it; a reader cannot act
// on sixty copies, so identical (offset, message) pairs collapse here
fn dedupe_of(ds: &Vec<Diag>) -> Vec<Diag> {
    let mut uq: Vec<Diag> = Vec::new();
    let mut i = 0;
    while i < ds.len() {
        let mut dup = false;
        let mut j = 0;
        while j < uq.len() {
            if uq[j].lo == ds[i].lo
                && uq[j].hi == ds[i].hi
                && uq[j].msg.as_str() == ds[i].msg.as_str()
            {
                dup = true;
                j = uq.len();
            } else {
                j = j + 1;
            }
        }
        if !dup {
            uq.push(Diag {
                level: ds[i].level,
                msg: String::from(ds[i].msg.as_str()),
                lo: ds[i].lo,
                hi: ds[i].hi,
            });
        }
        i = i + 1;
    }
    uq
}
fn shift_diags(ds: &mut Vec<Diag>, plen: u32) {
    let mut i = 0;
    while i < ds.len() {
        if ds[i].lo >= plen {
            ds[i].lo = ds[i].lo - plen;
        } else {
            ds[i].lo = 0;
        }
        if ds[i].hi >= plen {
            ds[i].hi = ds[i].hi - plen;
        } else {
            ds[i].hi = 0;
        }
        i = i + 1;
    }
}
// MECHANISM (not a plea): an ACCEPTED program must not call a name we never emitted. Silent acceptance
// of IR that does not build was the worst failure class we had -- "compiles fine, cannot link" -- so the
// driver verifies its own output and turns that into a diagnostic. Returns the offending name, or "".
fn ir_name_after(l: &str, at: usize) -> String {
    let b = l.as_bytes();
    let mut i = at;
    let mut r = String::from("");
    while i < b.len() {
        let c = b[i] as char;
        if c == '(' || c == ' ' || c == ')' || c == ',' {
            break;
        }
        r.push(c);
        i = i + 1;
    }
    r
}
// does `l` contain `pat` starting at or after `from`? -- index-based, since a &str slice of a borrowed
// line is exactly the shape semc miscompiled (the first version of this check reported a phantom name)
fn ir_find_at(l: &str, pat: &str, from: usize) -> i32 {
    let b = l.as_bytes();
    let q = pat.as_bytes();
    if q.len() == 0 || b.len() < q.len() {
        return 0 - 1;
    }
    let mut i = from;
    while i + q.len() <= b.len() {
        let mut k = 0;
        let mut ok = true;
        while k < q.len() {
            if b[i + k] != q[k] {
                ok = false;
                k = q.len();
            } else {
                k = k + 1;
            }
        }
        if ok {
            return i as i32;
        }
        i = i + 1;
    }
    0 - 1
}
// IR-CALLER (R2a P4): the `define` enclosing the first call to `callee` ("" when none)
fn ir_caller_of(ir: &str, callee: &str) -> String {
    let mut cur = String::from("");
    let mut want = String::from("@");
    want.push_str(callee);
    want.push('(');
    let mut line = String::from("");
    for c in ir.chars() {
        if c == '\n' {
            let l = line.as_str();
            if ir_find_at(l, "define ", 0) == 0 {
                let at = ir_find_at(l, "@", 0);
                if at >= 0 {
                    cur = ir_name_after(l, (at + 1) as usize);
                }
            } else if ir_find_at(l, "@", 0) != 0
                && ir_find_at(l, "call ", 0) >= 0
                && ir_find_at(l, want.as_str(), 0) >= 0
            {
                return cur;
            }
            line = String::from("");
        } else {
            line.push(c);
        }
    }
    String::from("")
}
fn ir_undefined(ir: &str) -> String {
    let mut defs: Vec<String> = Vec::new();
    let mut calls: Vec<String> = Vec::new();
    {
        let mut line = String::from("");
        for c in ir.chars() {
            if c == '\n' {
                {
                    let l = line.as_str();
                    // C-IRGLOBAL: a line that STARTS with @ is a global DEFINITION -- a string constant, say --
                    // and its payload is data, not instructions. Scanning it for calls made the verifier read the
                    // bytes of a literal: a program containing `let s="call @foo";` was refused with "the emitted
                    // program calls foo\00, which was never defined". The define/declare test was the whole rule;
                    // a global is the third shape a definition takes and it was missing.
                    let isdef = ir_find_at(l, "define ", 0) == 0
                        || ir_find_at(l, "declare ", 0) == 0
                        || ir_find_at(l, "@", 0) == 0;
                    if isdef {
                        let at = ir_find_at(l, "@", 0);
                        if at >= 0 {
                            defs.push(ir_name_after(l, (at + 1) as usize));
                        }
                    } else {
                        let mut from = 0;
                        loop {
                            let c2 = ir_find_at(l, "call ", from);
                            if c2 < 0 {
                                break;
                            }
                            let at = ir_find_at(l, "@", (c2 as usize) + 5);
                            if at < 0 {
                                break;
                            }
                            calls.push(ir_name_after(l, (at as usize) + 1));
                            from = (at as usize) + 1;
                        }
                    }
                }
                line = String::from("");
            } else {
                line.push(c);
            }
        }
    }
    let mut i = 0;
    while i < calls.len() {
        let mut seen = false;
        let mut k = 0;
        while k < defs.len() {
            if defs[k].as_str() == calls[i].as_str() {
                seen = true;
                k = defs.len();
            } else {
                k = k + 1;
            }
        }
        if !seen {
            return String::from(calls[i].as_str());
        }
        i = i + 1;
    }
    // a DUPLICATE definition is the same class of defect, caught the same way -- but it is NOT the same
    // fault, so it is reported separately (it read as "never defined", which sent a probe the wrong way)
    {
        let mut a = 0;
        while a < defs.len() {
            let mut b2 = a + 1;
            while b2 < defs.len() {
                if defs[a].as_str() == defs[b2].as_str() {
                    let mut r = String::from("$dup$");
                    r.push_str(defs[a].as_str());
                    return r;
                }
                b2 = b2 + 1;
            }
            a = a + 1;
        }
    }
    String::from("")
}
fn compile(src0: &str) -> String {
    compile_in(src0, "")
}
// C-ONEEMIT: the emit path used to run the whole front end THREE times -- once here, once in this
// function's own IR verifier, and once more in main for the IR it prints. The verifier's IR is the same
// IR main wants, so it is handed out through `irout` (a Vec, not a &mut String: pushing through a
// &mut String parameter is a shape our value model miscompiles). The helper is NOT called compile_in_o:
// a name of which another function name is a PREFIX comes back carrying the other one's body through the
// semc toolchain (see the rung-85 record), so the two names are kept prefix-disjoint.
fn compile_in(src0: &str, dir: &str) -> String {
    let mut ir0: Vec<String> = Vec::new();
    report_with_ir(src0, dir, &mut ir0)
}
fn report_with_ir(src0: &str, dir: &str, irout: &mut Vec<String>) -> String {
    // C-ASCIIONLY: this tier's strings are BYTES (C8) and a char IS an integer, so a source byte above 127
    // cannot be carried faithfully -- a literal holding one answers the wrong length and prints the wrong
    // text. Refuse the program instead of answering wrongly. The guard sits HERE, on the raw source,
    // because by the time the lexer has built a token the high byte is already gone (probed: a check in
    // the type checker's string-literal arm never fires).
    // C-ASCIIONLY: this tier's strings are BYTES (C8) and a char IS an integer, so a non-ASCII byte in a
    // STRING or char or code position cannot be carried faithfully (it mangles: "café" -> "caf", len 3 not
    // 5 -- empirically refuted the "over-conservative" hypothesis). BUT a non-ASCII byte inside a COMMENT
    // is skipped by the lexer and is harmless (P-01-NONASCII: proven == rustc), so the guard is now
    // comment-aware -- it refuses only bytes >127 OUTSIDE comments. String-literal non-ASCII stays refused.
    {
        let b = src0.as_bytes();
        let mut i = 0;
        let mut bad = 0 - 1;
        let mut inln = false;
        let mut inblk = false;
        let mut instr = false;
        let mut inch = false;
        let mut esc = false;
        while i < b.len() {
            if inln {
                if b[i] == 10 {
                    inln = false;
                }
            } else if inblk {
                if b[i] == 42 && i + 1 < b.len() && b[i + 1] == 47 {
                    inblk = false;
                    i = i + 1;
                }
            } else if instr {
                if esc {
                    esc = false;
                } else if b[i] == 92 {
                    esc = true;
                } else if b[i] == 34 {
                    instr = false;
                }
            }
            // UTF8-STR: a string literal carries its UTF-8 BYTES (len = bytes, as in rustc); chars() decodes them. Char literals and code stay ASCII-only -- see ledger: the SEM0-built ourc mangles a non-ASCII char between read and token (é -> C3 80 C9 80)
            else if inch {
                if esc {
                    esc = false;
                } else if b[i] == 92 {
                    esc = true;
                } else if b[i] == 39 {
                    inch = false;
                }
            }
            // UTF8-CHAR: a char literal is its code point (the lexer sees decoded chars since the expand_mods fix)
            else if b[i] == 47 && i + 1 < b.len() && b[i + 1] == 47 {
                inln = true;
            } else if b[i] == 47 && i + 1 < b.len() && b[i + 1] == 42 {
                inblk = true;
            } else if b[i] == 34 {
                instr = true;
            } else if b[i] == 39 {
                inch = true;
            } else if b[i] > 127 {
                bad = i as i32;
                i = b.len();
                continue;
            }
            i = i + 1;
        }
        if bad >= 0 {
            let mut m = String::from("error[");
            m.push_str(&oast::u32s2(bad as u32));
            m.push_str("..");
            m.push_str(&oast::u32s2((bad + 1) as u32));
            m.push_str(
                "]: a non-ASCII source byte is not implemented (strings are bytes at this tier)",
            );
            return m;
        }
    }
    let mut eds: Vec<ModEdit> = Vec::new();
    let expanded = expand_mods(src0, dir, &mut eds);
    let pl = prelude();
    let full = with_prelude(expanded.as_str());
    let plen = pl.len() as u32;
    let src = full.as_str();
    let toks = olex::lex(src);
    // LEX-ERR: a malformed literal (bad escape, unterminated raw string/comment, empty char) is a lexer
    // diagnostic, reported before parsing as rustc does
    {
        let mut ti = 0;
        while ti < toks.len() {
            match toks[ti].k {
                olex::Tk::LexErr => {
                    let mut ds0: Vec<Diag> = Vec::new();
                    ds0.push(oerrors::err(
                        String::from("malformed literal (unknown escape, or unterminated)"),
                        toks[ti].lo,
                        toks[ti].hi,
                    ));
                    shift_diags(&mut ds0, plen);
                    unmap_diags(&mut ds0, &eds);
                    return oerrors::render(&ds0[0]);
                }
                _ => {}
            }
            ti = ti + 1;
        }
    }
    let mut prog = oparse::parse_prog(toks);
    mono_structs(&mut prog);
    infer_rets(&mut prog);
    infer_clos_params(&mut prog);
    monomorphize(&mut prog);
    let mut ds: Vec<Diag> = Vec::new();
    if prog.perr > 0 {
        let mut m = String::from("expected ");
        if prog.perrexp.len() > 0 {
            m.push_str("`");
            m.push_str(prog.perrexp.as_str());
            m.push_str("`");
        } else {
            m.push_str("something else");
        }
        m.push_str(", found `");
        m.push_str(prog.perrgot.as_str());
        m.push_str("`");
        ds.push(oerrors::err(m, prog.perr - 1, prog.perr));
    }
    if prog.badfmt > 0 {
        ds.push(oerrors::err(
            String::from("unsupported format specifier (only `{}` is implemented)"),
            prog.badfmt - 1,
            prog.badfmt,
        ));
    }
    if prog.trailing > 0 {
        ds.push(oerrors::err(
            String::from("unexpected input after the program (not understood)"),
            prog.trailing,
            prog.trailing + 1,
        ));
    }
    {
        let mut bds: Vec<Diag> = Vec::new();
        oborrowck::check_prog(&prog, &mut bds); // MOVE CHECKING, before the tree is taken apart
                                                // BCK-MUT is a rule of the USER's Rust: the prelude is ourc's own dialect (no `mut`), so a mutability finding
                                                // inside it (positions before plen) is not the program's
        let mut q = 0;
        while q < bds.len() {
            let m = bds[q].msg.as_str();
            let mutmsg =
                oast::str_starts(m, "cannot borrow") || oast::str_starts(m, "cannot assign twice");
            // PRELUDE-MOVES: a MOVE finding inside the prelude (positions before plen) is not the program's either -- with a
            // `T<String>` in play the move checker tracked str values and flagged the prelude's own fmt_rep loop (a false rejection)
            let inprelude = prog.realrs && bds[q].lo < plen && bds[q].hi <= plen;
            if !(mutmsg && (bds[q].lo < plen || !prog.realrs)) && !inprelude {
                // and only in real Rust (`fn main`): .src has no `mut`
                ds.push(Diag {
                    level: bds[q].level,
                    msg: String::from(bds[q].msg.as_str()),
                    lo: bds[q].lo,
                    hi: bds[q].hi,
                });
            }
            q = q + 1;
        }
    }
    let ast = prog.main;
    let mut rib = oresolve::rib_new();
    {
        let mut ii = 0;
        while ii < prog.impls.len() {
            let ti = oast::tridx(&prog.tdefs, prog.impls[ii].tr.as_str());
            // an UNDECLARED trait is an external one (std's `ops::Add`, `Clone`, ..): its impl still provides
            // methods, there is just no obligation list to check against
            if ti < 0 {
            } else {
                let td = &prog.tdefs[ti as usize];
                let mut mi = 0;
                while mi < td.mnames.len() {
                    let mut own = String::from(prog.impls[ii].ty.as_str());
                    own.push_str("$");
                    own.push_str(td.mnames[mi].as_str());
                    if oparse::find_fun_ix(&prog.funs, &own) < 0 {
                        let mut m = String::from("`");
                        m.push_str(prog.impls[ii].ty.as_str());
                        m.push_str("` is missing method `");
                        m.push_str(td.mnames[mi].as_str());
                        m.push_str("` required by trait `");
                        m.push_str(prog.impls[ii].tr.as_str());
                        m.push_str("`");
                        ds.push(oerrors::err(m, prog.impls[ii].lo, prog.impls[ii].hi));
                    }
                    mi = mi + 1;
                }
            }
            ii = ii + 1;
        }
    }
    {
        let mut fi = 0;
        while fi < prog.funs.len() {
            let f = &prog.funs[fi];
            if oast::tridx(&prog.tdefs, f.owner.as_str()) >= 0
                || oast::has_bound(f)
                || oast::has_composite_gp(f)
            {
                fi = fi + 1;
                continue;
            }
            let mut r2 = oresolve::rib_new();
            let mut pi = 0;
            while pi < f.params.len() {
                oresolve::define(&mut r2, &f.params[pi]);
                pi = pi + 1;
            }
            oresolve::resolve_expr(&f.body, &mut r2, &mut ds);
            let mut fe2 = otypeck::fenv_of_t(f, &prog.sdefs, &prog.edefs, &prog.tdefs);
            otypeck::check_calls_t(
                &f.body,
                &prog.funs,
                &mut ds,
                &prog.sdefs,
                &mut fe2,
                &prog.edefs,
                &prog.tdefs,
            );
            fi = fi + 1;
        }
    }
    oresolve::resolve_expr(&ast, &mut rib, &mut ds);
    {
        let mut cenv = oast::Env {
            n: Vec::new(),
            t: Vec::new(),
        };
        otypeck::check_calls_t(
            &ast,
            &prog.funs,
            &mut ds,
            &prog.sdefs,
            &mut cenv,
            &prog.edefs,
            &prog.tdefs,
        );
    }
    if ds.len() > 0 {
        shift_diags(&mut ds, plen);
        unmap_diags(&mut ds, &eds);
        ds = dedupe_of(&ds);
        let mut s = String::from("");
        let mut i = 0;
        while i < ds.len() {
            if i > 0 {
                s.push_str(" ; ");
            }
            s.push_str(&oerrors::render(&ds[i]));
            i = i + 1;
        }
        return s;
    }
    {
        let mut fi = 0;
        while fi < prog.funs.len() {
            let f = &prog.funs[fi];
            if oast::tridx(&prog.tdefs, f.owner.as_str()) >= 0
                || oast::has_bound(f)
                || oast::has_composite_gp(f)
            {
                fi = fi + 1;
                continue;
            }
            let mut nn: Vec<String> = Vec::new();
            let mut tt: Vec<i32> = Vec::new();
            let mut pi = 0;
            while pi < f.params.len() {
                nn.push(String::from(f.params[pi].as_str()));
                let pt = if pi < f.ptys.len() {
                    if otypeck::is_gp(&f.gparams, &f.ptys[pi]) {
                        0 - 3
                    } else {
                        let b0 =
                            otypeck::type_tag(&prog.sdefs, &prog.edefs, &prog.tdefs, &f.ptys[pi]);
                        if b0 >= 0 {
                            b0
                        } else {
                            let ei = oast::eidx(&prog.edefs, &f.ptys[pi]);
                            if ei >= 0 {
                                100 + ei
                            } else {
                                0 - 3
                            }
                        }
                    }
                } else {
                    0
                };
                tt.push(pt);
                pi = pi + 1;
            }
            let fe = oast::Env { n: nn, t: tt };
            let bx = oast::fbody_ix(&prog.funs, fi);
            let mut fe3 = fe;
            let tb = otypeck::check_e(
                &prog.funs[bx].body,
                &mut fe3,
                &mut ds,
                &prog.sdefs,
                &prog.funs,
                &prog.edefs,
                &prog.tdefs,
            );
            let want = if f.ret.len() == 0 {
                0 - 1
            } else if otypeck::is_gp(&f.gparams, &f.ret) {
                0 - 3
            } else {
                otypeck::tag_of_tyname(&prog.sdefs, &f.ret)
            };
            // NULL-PTR-RET (min_const_fn_unsafe_ok): a raw pointer type is its referent's type here, so
            // `fn f<T>() -> *const T { std::ptr::null() }` at T = String "returned str" while the body was the 0 pointer.
            // A null pointer satisfies any raw-pointer return; it is never dereferenced soundly, so nothing reads it as a T.
            if want >= 0
                && tb >= 0
                && !otypeck::tyeq(tb, want)
                && !tail_is_ptrnull(&prog.funs[bx].body)
            {
                let mut m = String::from("function `");
                m.push_str(&f.name);
                m.push_str("` returns ");
                m.push_str(&otypeck::tyname2(want, &prog.sdefs));
                m.push_str(" but body has ");
                m.push_str(&otypeck::tyname2(tb, &prog.sdefs));
                ds.push(oerrors::err(m, oast::e_lo(&f.body), oast::e_hi(&f.body)));
            }
            fi = fi + 1;
        }
    }
    let mut tenv = oast::Env {
        n: Vec::new(),
        t: Vec::new(),
    };
    let t = otypeck::check_e(
        &ast,
        &mut tenv,
        &mut ds,
        &prog.sdefs,
        &prog.funs,
        &prog.edefs,
        &prog.tdefs,
    );
    if ds.len() > 0 {
        shift_diags(&mut ds, plen);
        unmap_diags(&mut ds, &eds);
        ds = dedupe_of(&ds);
        let mut s = String::from("");
        let mut i = 0;
        while i < ds.len() {
            if i > 0 {
                s.push_str(" ; ");
            }
            s.push_str(&oerrors::render(&ds[i]));
            i = i + 1;
        }
        s
    } else {
        let mut lc = olower::lcx_new();
        lc.sdefs = oast::copy_sdefs(&prog.sdefs);
        {
            let mut q = 0;
            while q < prog.fpays.len() {
                lc.fpays.push(String::from(prog.fpays[q].as_str()));
                q = q + 1;
            }
        }
        olower::fill_gops(&mut lc, &prog.funs);
        olower::fill_fsigs(&mut lc, &prog.funs);
        olower::fill_dyn(&mut lc, &prog.impls);
        lc.edefs = oast::copy_edefs(&prog.edefs);
        let hir = olower::lower(&ast, &mut lc);
        let mut body = Body {
            ops: Vec::new(),
            ntmp: 0,
            nloc: 0,
        };
        let rt = omir::build(&hir, &mut body);
        body.ops.push(Op::Ret(rt));
        // CONST-DIV0: rustc rejects a division by a constant-propagated zero at compile time (deny unconditional_panic)
        if omirtrans::const_div0(&body) {
            return String::from("error[0..0]: this operation will panic at runtime: attempt to divide by zero (unconditional_panic)");
        }
        let foldable = !omirtrans::has_call(&body);
        let mut s = String::from("ok ");
        s.push_str(&otypeck::tyname2(t, &prog.sdefs));
        if foldable {
            let r = omirtrans::fold(&body);
            s.push_str(" = ");
            s.push_str(&oast::i64s(r));
        } else {
            s.push_str(" = <runtime>");
        }
        s.push_str(" [hir=");
        s.push_str(&oast::u32s2(ohir::h_id(&hir)));
        s.push_str(" mir=");
        s.push_str(&oast::u32s2(omirtrans::nops(&body)));
        s.push_str("]");
        let ir = ocodegen::emit(&body);
        let mut nl = 0;
        let irb: Vec<char> = {
            let mut v: Vec<char> = Vec::new();
            for c in ir.chars() {
                if c == '\n' {
                    nl = nl + 1;
                }
                v.push(c);
            }
            v
        };
        let _ = irb;
        s.push_str(" ir_lines=");
        s.push_str(&oast::u32s2(nl));
        {
            prog.main = ast; // M-P3: the main body goes back (it was moved out for checking); the parse is reused
            irout.push(emit_ir_prog(prog));
            // CONSTDIV0-LINE (R2a P4): the marker is a whole IR comment LINE -- a program whose own string literal is "; CONSTDIV0"
            // (ourc's source, this very check) put the text inside a global constant and was rejected as a division by zero
            let irm = irout[0].as_str();
            if oast::str_starts(irm, "; CONSTDIV0\n") || oast::str_contains2(irm, "\n; CONSTDIV0\n")
            {
                return String::from("error[0..0]: this operation will panic at runtime: attempt to divide by zero (unconditional_panic)");
            }
            let bad = ir_undefined(irout[0].as_str());
            if bad.len() > 5 {
                let b = bad.as_bytes();
                if b[0] == 36 && b[1] == 100 && b[2] == 117 && b[3] == 112 && b[4] == 36 {
                    let mut m = String::from("two definitions of `");
                    m.push_str(&bad[5..]);
                    m.push_str("` -- an impl for `T` and one for `&T` are the SAME type in our value model (no distinct reference)");
                    let mut d4: Vec<Diag> = Vec::new();
                    d4.push(oerrors::err(m, 0, 1));
                    return oerrors::render(&d4[0]);
                }
            }
            if bad.as_str() == "$enumeq_unsupported" {
                let m=String::from("comparing these enum values is not implemented (a payload whose equality is by CONTENT, not by word)");
                let mut d7: Vec<Diag> = Vec::new();
                d7.push(oerrors::err(m, 0, 1));
                return oerrors::render(&d7[0]);
            }
            if bad.as_str() == "$dbg_unsupported" {
                let m=String::from("`{:?}` on this type is not implemented (a sequence, struct, enum and the scalars are)");
                let mut d6: Vec<Diag> = Vec::new();
                d6.push(oerrors::err(m, 0, 1));
                return oerrors::render(&d6[0]);
            }
            if bad.as_str() == "$lazy_io" {
                let m=String::from("an iterator adaptor whose closure PRINTS cannot be run eagerly: rustc runs it during the consuming call, so the output order is part of the answer");
                let mut d5: Vec<Diag> = Vec::new();
                d5.push(oerrors::err(m, 0, 1));
                return oerrors::render(&d5[0]);
            }
            if bad.as_str() == "$dflt_enum_unsupported" {
                let m=String::from("`Default` for an enum is not implemented (rustc takes the `#[default]` variant, an attribute we do not read)");
                let mut d8: Vec<Diag> = Vec::new();
                d8.push(oerrors::err(m, 0, 1));
                return oerrors::render(&d8[0]);
            }
            if bad.as_str() == "$private_field_E0616" {
                let m = String::from("field is private: it is read outside the module that declares its struct (E0616)");
                let mut d9: Vec<Diag> = Vec::new();
                d9.push(oerrors::err(m, 0, 1));
                return oerrors::render(&d9[0]);
            }
            if bad.as_str() == "$dyn_no_impl" {
                let m=String::from("a trait object needs an `impl Trait for <that type>`; no impl of that trait names this type");
                let mut d3: Vec<Diag> = Vec::new();
                d3.push(oerrors::err(m, 0, 1));
                return oerrors::render(&d3[0]);
            }
            if bad.len() > 0 {
                let mut m = String::from("internal: the emitted program calls `");
                m.push_str(bad.as_str());
                m.push_str(
                    "`, which was never defined (a monomorphization or trait-resolution gap)",
                );
                // IR-CALLER (R2a P4): name the function that makes the call -- this diagnostic had no position at all, so a
                // self-compile wall of this class could only be found by guessing
                let cf = ir_caller_of(irout[0].as_str(), bad.as_str());
                if cf.len() > 0 {
                    m.push_str(" (called from `");
                    m.push_str(cf.as_str());
                    m.push_str("`)");
                }
                let mut d2: Vec<Diag> = Vec::new();
                d2.push(oerrors::err(m, 0, 1));
                return oerrors::render(&d2[0]);
            }
        }
        s
    }
}
fn rep_is_error(r: &str) -> bool {
    let b = r.as_bytes();
    if b.len() < 5 {
        return false;
    }
    b[0] == 101 && b[1] == 114 && b[2] == 114 && b[3] == 111 && b[4] == 114
}
fn emit_ir(src: &str) -> String {
    emit_ir_in(src, "")
}
fn emit_ir_in(src0: &str, dir: &str) -> String {
    let mut eds: Vec<ModEdit> = Vec::new();
    let src2 = expand_mods(src0, dir, &mut eds);
    let src = src2.as_str();
    let full = with_prelude(src);
    let toks = olex::lex(full.as_str());
    let mut prog = oparse::parse_prog(toks);
    mono_structs(&mut prog);
    infer_rets(&mut prog);
    infer_clos_params(&mut prog);
    monomorphize(&mut prog);
    emit_ir_prog(prog)
}
// M1 SINGLE-FRONT-END (M-P3): emit from an already-parsed, already-monomorphized program. report_with_ir hands over the
// Prog it built (same source, same parse, same four passes) instead of emit_ir_in lexing and parsing it all again --
// under the no-free bump arena that second front end was a second full parse cost (12.5 GB in the self-compile).
fn emit_ir_prog(prog: oast::Prog) -> String {
    let mut prog = prog;
    // DYN-RET (R2a P5): a fn returning a trait object (`-> Box<dyn Tr>`, ret `dyn$Tr`) coerces each TAIL value into the
    // {tag, payload} pair; the bare struct was returned and dispatch read its fields as a tag (`mk().area()` printed 0, a
    // silent WRONG). Tails: block end, both if-branches, every match arm, let bodies; diverging tails are left alone.
    {
        let mut fi = 0;
        while fi < prog.funs.len() {
            let tr = olower::dyn_trait_of(prog.funs[fi].ret.as_str());
            if tr.len() > 0 {
                let tgt = String::from(prog.funs[fi].ret.as_str());
                let b = std::mem::replace(&mut prog.funs[fi].body, oast::E::Int(0, 0, 0));
                prog.funs[fi].body = dyn_ret_wrap(b, &tgt);
            }
            fi = fi + 1;
        }
    }
    let mut out = String::from("");
    let mut lits: Vec<String> = Vec::new();
    let mut fi = 0;
    let mut lc = olower::lcx_new();
    lc.sdefs = oast::copy_sdefs(&prog.sdefs);
    {
        let mut q = 0;
        while q < prog.fpays.len() {
            lc.fpays.push(String::from(prog.fpays[q].as_str()));
            q = q + 1;
        }
    }
    olower::fill_gops(&mut lc, &prog.funs);
    olower::fill_fsigs(&mut lc, &prog.funs);
    olower::fill_dyn(&mut lc, &prog.impls);
    lc.edefs = oast::copy_edefs(&prog.edefs);
    while fi < prog.funs.len() {
        let f = &prog.funs[fi];
        if oast::tridx(&prog.tdefs, f.owner.as_str()) >= 0
            || oast::has_bound(f)
            || oast::has_composite_gp(f)
            || erased_superseded(&prog.funs, fi)
        {
            fi = fi + 1;
            continue;
        }
        olower::lcx_reset(&mut lc);
        lc.curfn = String::from(f.name.as_str()); // VIS-FIELD-READ (R2a P6)
        lc.gsub = oast::copy_strs(&f.gsub); // G1 G-P1: this instance's parameter -> type map
                                            // P-SELF: the owner type of a method or associated item, so `Self::X` in its body (including a
                                            // trait default's SHARED body) names this impl's own item
        {
            let pre = oparse::name_owner(&f.name);
            if pre.len() > 0
                && (oast::is_struct_name(&prog.sdefs, &pre)
                    || oast::is_enum_name(&prog.edefs, &pre))
            {
                lc.selfty = pre;
            } else if f.owner.len() > 0
                && (oast::is_struct_name(&prog.sdefs, &f.owner)
                    || oast::is_enum_name(&prog.edefs, &f.owner))
            {
                lc.selfty = String::from(f.owner.as_str());
            }
        }
        let mut pi = 0;
        while pi < f.params.len() {
            lc.names.push(String::from(f.params[pi].as_str()));
            let mut pt = if pi < f.ptys.len() {
                String::from(f.ptys[pi].as_str())
            } else {
                String::from("")
            };
            if pt.len() == 0 && (pi as i32) < f.ncap {
                pt = olower::capty_of(&lc, f.name.as_str(), pi as u32);
            } /* CLOS-CAPTY */
            let ps = olower::local_sty(&prog.sdefs, &prog.edefs, &pt);
            let ps = if ps.as_str() == "Option" || ps.as_str() == "Result" {
                // OPT-PARAM-PAY (W7): the param's recorded payload makes its slot `Option$T` / `Result$T$E`, like a let-bound local
                let mut key = String::from("@pp:");
                key.push_str(f.name.as_str());
                key.push('#');
                key.push_str(&oast::u32s2(pi as u32));
                key.push('=');
                let mut r = ps;
                let mut q = 0;
                while q < prog.fpays.len() {
                    if oast::str_starts(prog.fpays[q].as_str(), key.as_str()) {
                        r.push('$');
                        r.push_str(&prog.fpays[q][key.len()..]);
                        q = prog.fpays.len();
                    }
                    q = q + 1;
                }
                r
            } else {
                ps
            };
            lc.stypes.push(ps);
            pi = pi + 1;
        }
        let bx = oast::fbody_ix(&prog.funs, fi);
        olower::set_cellvars(&mut lc, f.name.as_str(), &prog.funs[bx].body, &prog.mutcaps);
        lc.curfn = String::from(f.name.as_str()); // FN-BOUND-RET
        olower::rc_params(&mut lc, &prog.funs[fi]); // RC-IDENTITY (R4c)
        let pro = olower::copy_params(&prog.funs[fi], &mut lc); // STRUCT-COPY (W7): by-value Copy-struct params are copies
        let hir0 = olower::lower(&prog.funs[bx].body, &mut lc);
        let hir0 = match pro {
            Some(ph) => {
                let i = olower::fresh(&mut lc);
                ohir::H::SeqH(Box::new(ph), Box::new(hir0), i)
            }
            None => hir0,
        };
        let hir = olower::param_drops(hir0, &prog.funs[fi], &prog.funs[bx].body, &mut lc); // DROP-PARAM
        let mut body = Body {
            ops: Vec::new(),
            ntmp: 0,
            nloc: 0,
        };
        // bind params: %pK -> a temp -> STORE into local slot K (real MIR locals)
        let np = f.params.len() as u32;
        let mut k = 0;
        while k < np {
            let t = body.ntmp;
            body.ntmp = t + 1;
            body.ops.push(Op::ParamOp(t, k));
            body.ops.push(Op::StoreLoc(k, t));
            if k + 1 > body.nloc {
                body.nloc = k + 1;
            }
            k = k + 1;
        }
        let rt = omir::build(&hir, &mut body);
        body.ops.push(Op::Ret(rt));
        if omirtrans::const_div0(&body) {
            out.push_str("; CONSTDIV0\n");
        } // CONST-DIV0: every fn body, not just the top level
        ocodegen::collect_strs(&body.ops, &mut lits);
        out.push_str(&ocodegen::emit_named(&body, &f.name, np));
        fi = fi + 1;
    }
    olower::lcx_reset(&mut lc);
    let hir2 = olower::lower(&prog.main, &mut lc);
    let mut b2 = Body {
        ops: Vec::new(),
        ntmp: 0,
        nloc: 0,
    };
    let rt2 = omir::build(&hir2, &mut b2);
    b2.ops.push(Op::Ret(rt2));
    ocodegen::collect_strs(&b2.ops, &mut lits);
    out.push_str(&ocodegen::emit(&b2));
    let mut gs: Vec<String> = Vec::new();
    {
        let mut fi2 = 0;
        while fi2 < prog.funs.len() {
            let _ = fi2;
            fi2 = fi2 + 1;
        }
    }
    ocodegen::collect_globals(&b2.ops, &mut gs);
    // V2 REPRO: a fixed source_filename -- llc records the module's source file name in the object, so the same IR built
    // from two differently named .ll files gave two different binaries (prime.ll vs ir2.ll: the V2 fixed point held in IR
    // while the executables' hashes differed)
    let mut pre = String::from("source_filename = \"ourc\"\n");
    pre.push_str(&ocodegen::str_globals(&lits));
    pre.push_str(&ocodegen::global_defs(&gs));
    pre.push_str(&out);
    pre
}
fn main() {
    // CLI mode: `ourc input.src -o out.ll [--emit-ir] [-O]`
    let argv: Vec<String> = {
        let mut v: Vec<String> = Vec::new();
        for a in std::env::args() {
            v.push(a);
        }
        v
    };
    if argv.len() > 1 {
        let mut rest: Vec<String> = Vec::new();
        let mut i = 1;
        while i < argv.len() {
            rest.push(String::from(argv[i].as_str()));
            i = i + 1;
        }
        let o = osession::parse_args(&rest);
        // V5.3 BUDGET: a compile that runs past the limit ends with a diagnostic and exit 1, not a hang
        if o.time_limit > 0 || o.mem_limit > 0 {
            __ourc_budget(o.time_limit, o.mem_limit);
        }
        let src = match std::fs::read_to_string(&o.input) {
            Ok(t) => t,
            Err(_e) => String::from(""),
        };
        if src.len() == 0 {
            eprintln!("ourc: cannot read input `{}`", o.input);
            std::process::exit(1);
        } // DRV-EXIT
        let mdir = dir_of(&o.input);
        // a `.rs` input is Rust source (not ourc's `.src` dialect): mark it for the Rust-only parse checks (oparse rs_file)
        let mut src = src;
        let ib = o.input.as_bytes();
        if ib.len() > 3
            && ib[ib.len() - 3] == 46
            && ib[ib.len() - 2] == 114
            && ib[ib.len() - 1] == 115
        {
            src.push_str("\nfn __rsfile__() {}\n");
        }
        // R2a P5 CRATE-META: `--emit-meta` checks this crate as a library and writes its metadata unit -- a header and the
        // crate's module-expanded source (ourc monomorphizes everything, so a downstream crate needs every body anyway)
        if o.emit_meta {
            let mut chk = String::from(src.as_str());
            chk.push_str("\nfn main() {}\n");
            let mut irm: Vec<String> = Vec::new();
            let rep = report_with_ir(&chk, &mdir, &mut irm);
            if rep_is_error(&rep) {
                eprintln!("ourc[{}] {} -> {}", osession::opt_name(&o), o.input, rep);
                std::process::exit(1);
            }
            let mut eds: Vec<ModEdit> = Vec::new();
            let body = expand_mods(&src, &mdir, &mut eds);
            print!("{}\n{}", META_HEADER, body);
            return;
        }
        // `--extern NAME=PATH`: each unit is spliced AFTER the program (so the program's own positions stay true) as `mod NAME`
        let mut src = src;
        {
            let mut q = 0;
            while q < o.externs.len() {
                let e = o.externs[q].as_str();
                let b = e.as_bytes();
                let mut c = 0;
                while c < b.len() && b[c] != 61 {
                    c = c + 1;
                }
                if c == b.len() {
                    eprintln!("ourc: --extern expects NAME=PATH, found `{}`", e);
                    std::process::exit(1);
                }
                let name = String::from(&e[..c]);
                let unit = match std::fs::read_to_string(&e[c + 1..]) {
                    Ok(t) => t,
                    Err(_e) => String::from(""),
                };
                if !oast::str_starts(unit.as_str(), META_HEADER) {
                    eprintln!("ourc: --extern {}: not an ourc metadata unit", name);
                    std::process::exit(1);
                }
                src.push_str("\nmod ");
                src.push_str(name.as_str());
                src.push_str(" {\n");
                // XCRATE-MAC: mark the module as an EXTERN CRATE (`NAME$__xcrate__`) -- its #[macro_export] macros are then
                // reachable as `NAME::mac!(..)`, which in a single crate is E0433 (a macro_export macro lives at the crate root)
                src.push_str("fn __xcrate__() {}\n");
                src.push_str(&unit[META_HEADER.len()..]);
                src.push_str("\n}\n");
                q = q + 1;
            }
        }
        let mut ir0: Vec<String> = Vec::new();
        let rep = report_with_ir(&src, &mdir, &mut ir0);
        // DRV-EXIT (rustc_driver): a rejected program reports on STDERR and exits 1, as rustc does; success stays on stdout, exit 0
        if rep_is_error(&rep) {
            eprintln!("ourc[{}] {} -> {}", osession::opt_name(&o), o.input, rep);
            std::process::exit(1);
        }
        // V1.1 CLEAN-EMIT-IR: the status line goes to STDERR, so stdout carries only IR (`ourc x.rs --emit-ir > x.ll` is a
        // valid LLVM module). Harnesses capture 2>&1 and find the IR at its first `@` / `declare` / `define` line.
        eprintln!("ourc[{}] {} -> {}", osession::opt_name(&o), o.input, rep);
        // emit IR only for a program that TYPECHECKED: emitting after diagnostics handed the harness code
        // for a rejected program (it built, then segfaulted), which reads as a backend bug rather than a
        // compile error.
        if o.link && !o.emit_ir {
            let mut ir = String::new();
            if ir0.len() > 0 {
                ir.push_str(ir0[0].as_str());
            } else {
                ir.push_str(emit_ir_in(&src, &mdir).as_str());
            }
            let r = link_exe(ir.as_str(), o.out.as_str());
            if r != 0 {
                std::process::exit(1);
            }
            return;
        }
        if o.emit_ir && !rep_is_error(&rep) {
            if ir0.len() > 0 {
                print!("{}", ir0[0]);
            } else {
                print!("{}", emit_ir_in(&src, &mdir));
            }
        }
        return;
    }
    let progs = ["let x = 2 + 3 * 4; x", "if 1 < 2 { 10 } else { 20 }", "let a = 5; let b = a - 2; a * b", "1 + true", "if 5 { 1 } else { 2 }", "if 1 < 2 { 1 } else { true }", "y + 1", "let x = 1 == 1; if x { 100 } else { 200 }", "let a = 1; if true { a } else { a }", "if true { let b = 2; b } else { 0 }",
       "fn dbl(x) { x * 2 } dbl(21)",
       "fn add3(a, b, c) { a + b + c } add3(1, 2, 3)",
       "fn fact(n) { if n < 2 { 1 } else { n * fact(n - 1) } } fact(5)",
       "fn one() { 1 } one(1)",
       "nope(1)",
       "let mut i = 0; let mut s = 0; while i < 5 { s = s + i; i = i + 1; }; s",
       "let mut n = 1; while n < 100 { n = n * 3; }; n",
       "fn sumto(k) { let mut a = 0; let mut j = 0; while j < k { a = a + j; j = j + 1; }; a } sumto(10)",
       "while 7 { 1 }",
       "struct P { x, y } let p = P { 3, 4 }; p.x + p.y",
       "struct P { x, y } let p = P { 10, 32 }; p.y",
       "struct V { a, b, c } let v = V { 1, 2, 3 }; v.a + v.b * v.c",
       "struct P { x, y } let p = P { 1 }; p.x",
       "struct P { x, y } let p = P { 1, 2 }; p.z",
       "let n = 5; n.x",
       "struct P { x, y } Q { 1, 2 }",
       "struct P { x, y } let p = P { 1, 2 }; let q = P { 3, 4 }; p.x + q.y",
       "struct P { x: int, y: int } let p = P { 20, 22 }; p.x + p.y",
       "struct F { ok: bool, n: int } let f = F { true, 7 }; if f.ok { f.n } else { 0 }",
       "struct F { ok: bool, n: int } let f = F { 1, 7 }; f.n",
       "fn mk(a: int, b: int) -> int { a + b } mk(19, 23)",
       "fn bad(a: int) -> bool { a + 1 } bad(1)",
       "struct P { x: int, y: int } fn sum(p: P) -> int { p.x + p.y } let p = P { 19, 23 }; sum(p)",
       "struct P { x: int, y: int } fn mk() -> P { P { 5, 6 } } let q = mk(); q.x + q.y",
       "struct In { a: int } struct Out { i: In, b: int } let o = Out { In { 7 }, 35 }; o.i.a + o.b",
       "fn need(c: bool) -> int { if c { 1 } else { 0 } } need(5)",
       "let a = 3; let b = 9; if a <= 3 && b >= 9 { 42 } else { 0 }",
       "let a = 1; if a > 5 || a != 1 { 1 } else { 7 }",
       "let t = true; if !t { 1 } else { 13 }",
       "let n = 5; if n < 3 { 1 } else if n < 7 { 42 } else { 9 }",
       "let mut i = 0; let mut c = 0; while i <= 4 { if i != 2 { c = c + i; }; i = i + 1; }; c",
       "if 1 && true { 1 } else { 0 }",
       "enum Opt { None, Some(v) } let o = Opt::Some(42); match o { Some(v) => v, None => 0 }",
       "enum Opt { None, Some(v) } let o = Opt::None; match o { Some(v) => v, None => 7 }",
       "enum Sh { Sq(s), Rect(w, h) } let r = Sh::Rect(6, 7); match r { Sq(s) => s * s, Rect(w, h) => w * h }",
       "enum Opt { None, Some(v) } let o = Opt::Some(1); match o { Some(v) => v }",
       "enum Opt { None, Some(v) } let o = Opt::Some(1); match o { Nope(v) => v, _ => 0 }",
       "let n = 3; match n { Some(v) => v, _ => 0 }",
       "let a = [10, 20, 12]; a[0] + a[1] + a[2]",
       "let a = [1, 2, 3, 4, 5]; let mut i = 0; let mut s = 0; while i < 5 { s = s + a[i]; i = i + 1; }; s",
       "let a = [7]; a[0] * 6",
       "let a = [1, 2]; a[true]",
       "let n = 5; n[0]",
       "let a = [1, 2, 3]; a[1] = 40; a[0] + a[1] + a[2]",
       "struct P { x: int, y: int } let p = P { 1, 2 }; p.x = 40; p.x + p.y",
       "let a = [0, 0, 0, 0, 0]; let mut i = 0; while i < 5 { a[i] = i * i; i = i + 1; }; a[4] + a[3]",
       "let a = [1]; a[0] = true",
       "fn id<T>(x: T) -> T { x } id(42)",
       "fn pick<T>(c: bool, a: T, b: T) -> T { if c { a } else { b } } pick(true, 42, 7)",
       "fn first<T>(a: arr) -> int { a[0] } let xs = [42, 1]; first(xs)",
       "fn idb<T>(x: T) -> T { x } let t = true; if idb(t) { 42 } else { 0 }",
       "struct P { x: int, y: int } impl P { fn sum(self) -> int { self.x + self.y } } let p = P { 19, 23 }; p.sum()",
       "struct P { x: int, y: int } impl P { fn scaled(self, k: int) -> int { (self.x + self.y) * k } } let p = P { 3, 4 }; p.scaled(6)",
       "enum Opt { None, Some(v) } impl Opt { fn unwrap_or(self, d: int) -> int { match self { Some(v) => v, None => d } } } let o = Opt::None; o.unwrap_or(42)",
       "struct P { x: int, y: int } let p = P { 1, 2 }; p.nope()",
       "trait Area { fn area(self) -> int; } struct Sq { s: int } impl Area for Sq { fn area(self) -> int { self.s * self.s } } let q = Sq { 7 }; q.area() - 7",
       "trait Shape { fn base(self) -> int; fn twice(self) -> int { self.base() * 2 } } struct R { n: int } impl Shape for R { fn base(self) -> int { self.n } } let r = R { 21 }; r.twice()",
       "trait T { fn v(self) -> int { 10 } } struct A { n: int } struct B { n: int } impl T for A { fn v(self) -> int { self.n } } impl T for B { } let a = A { 32 }; let b = B { 0 }; a.v() + b.v()",
       "trait T { fn go(self) -> int; } struct S { n: int } impl T for S { } let s = S { 1 }; s.go()",
       "struct S { n: int } impl Nope for S { fn go(self) -> int { 1 } } let s = S { 1 }; s.n",
       "trait Area { fn area(self) -> int; } struct Sq { s: int } impl Area for Sq { fn area(self) -> int { self.s * self.s } } fn show<T: Area>(x: T) -> int { x.area() } let q = Sq { 7 }; show(q) - 7",
       "trait Area { fn area(self) -> int; } struct Sq { s: int } struct Rc { w: int, h: int } impl Area for Sq { fn area(self) -> int { self.s * self.s } } impl Area for Rc { fn area(self) -> int { self.w * self.h } } fn tot<T: Area>(x: T) -> int { x.area() } let a = Sq { 5 }; let b = Rc { 3, 6 }; tot(a) - tot(b) + 35",
       "trait N { fn n(self) -> int; fn dbl(self) -> int { self.n() * 2 } } struct W { v: int } impl N for W { fn n(self) -> int { self.v } } fn via<T: N>(x: T) -> int { x.dbl() } let w = W { 21 }; via(w)",
       "len(\"hello\") + 37",
       "print(\"ourc says hello\"); 42",
       "len(7)",
       "print_int(42); 42",
       "let a = \"abc\"; if a == \"abc\" { 42 } else { 0 }",
       "fn same(a: str, b: str) -> int { if a == b { 1 } else { 0 } } same(\"xy\", \"xy\") * 21 + same(\"xy\", \"zz\") + 21",
       "let v = vec_new(); vec_push(v, 40); vec_push(v, 2); vec_get(v, 0) + vec_get(v, 1)",
       "let v = vec_new(); let mut i = 0; while i < 20 { vec_push(v, i); i = i + 1; }; vec_len(v) + vec_get(v, 19) + 3",
       "vec_push(7, 1)",
       "str_at(\"A\", 0) - 23",
       "len(str_cat(\"abc\", \"de\")) + 37",
       "if str_sub(\"hello world\", 6, 5) == \"world\" { 42 } else { 0 }",
       "str_cat(\"a\", 1)",
       "fn f(n) { if n < 0 { return 0; } n * 2 } f(21)",
       "fn g(n) { let mut i = 0; while i < 100 { if i == n { return i * 2; } i = i + 1; } 0 } g(21)",
       "fn h(n) { return n + 1; } h(41)",
       "let p = (40, 2); p.0 + p.1",
       "let t = (1, 2, 39); t.0 + t.1 + t.2",
       "fn mk(a, b) -> (int, int) { (a, b) } let q = mk(20, 22); q.0 + q.1",
       "let p = (1, 2); p.5",
       "let x = 7; x.0",
       "trait Area { fn area(self) -> int; } struct Sq { s: int } impl Area for Sq { fn area(self) -> int { self.s * self.s } } fn show(x: dyn Area) -> int { x.area() } let a = Sq { 5 }; show(a) + 17",
       "trait Area { fn area(self) -> int; } struct Sq { s: int } impl Area for Sq { fn area(self) -> int { self.s } } fn show(x: dyn Area) -> int { x.nope() } let a = Sq { 5 }; show(a)",
       "trait Area { fn area(self) -> int; } struct Sq { s: int } struct Rc { w: int, h: int } impl Area for Sq { fn area(self) -> int { self.s * self.s } } impl Area for Rc { fn area(self) -> int { self.w * self.h } } let v: Vec<dyn Area> = vec_new(); vec_push(v, Sq { 4 }); vec_push(v, Rc { 2, 13 }); let a = vec_get(v, 0); let b = vec_get(v, 1); a.area() + b.area()",
       "let z: Nope = 1; z",
       "struct P { x: int, y: int } let ps: Vec<P> = vec_new(); vec_push(ps, P { 20, 22 }); let p = vec_get(ps, 0); p.x + p.y",
       "struct P { x: int } let v: Vec<int> = vec_new(); vec_push(v, P { 1 }); 0",
       "let v: Vec<int> = vec_new(); vec_push(v, 40); vec_push(v, 2); vec_get(v, 0) + vec_get(v, 1)",
       "struct Pt { x: int, y: int } impl Pt { fn new(a: int, b: int) -> Self { Pt { a, b } } fn sum(self) -> int { self.x + self.y } } let p = Pt::new(20, 22); p.sum()",
       "struct Pt { x: int } impl Pt { fn origin() -> Self { Pt { 0 } } } let o = Pt::origin(); o.x + 42",
       "let add = |a, b| a + b; add(20, 22)",
       "let k = 7; let scale = |x| x * k; scale(6)",
       "fn apply(f: Fn, x) { f(x) } let inc = |n| n + 1; apply(inc, 41)",
       "let add = |a, b| a + b; add(1)",
       "let mut s = 0; for i in 0..10 { s = s + i; } s - 3",
       "let v: Vec<int> = vec_new(); vec_push(v, 20); vec_push(v, 22); let mut t = 0; for x in v { t = t + x; } t",
       "let mut x = 1; if 1 == 1 { let x = 7; 0 } else { 0 }; x + 41",
       "enum Res { Ok(v), Err(e) } fn half(n) -> Res { if n == n / 2 * 2 { Res::Ok(n / 2) } else { Res::Err(n) } } fn q(n) -> Res { let h = half(n)?; Res::Ok(h) } fn show(r: Res) -> int { match r { Ok(v) => v, Err(e) => 0 - e } } show(q(84))",
       "let n = 7; n?",
       "struct V { x: int, y: int } impl V { fn add(self, o: V) -> V { V { self.x + o.x, self.y + o.y } } fn tot(self) -> int { self.x + self.y } } let a = V { 20, 1 }; let b = V { 20, 1 }; (a + b).tot()",
       "struct P { x: int } fn take(p: P) -> int { p.x } let q = P { 1 }; take(q) + take(q)",
       "struct P { x: int } fn look(p: &P) -> int { p.x } let q = P { 21 }; look(&q) + look(&q)",
       "struct P { x: int } fn take(p: P) -> int { p.x } let q = P { 1 }; let mut s = 0; let mut i = 0; while i < 2 { s += take(q); i += 1; } s",
       "struct P { x: int, y: int } fn first<A, B>(a: A, b: B) -> A { a } let p = P { 20, 22 }; let s = \"hi\"; first(&p, s).x + first(&p, s).y",
       "let a: u8 = 300; a as u8 - 2",
       "let s = \"x\"; s as u8"];
    let mut i = 0;
    while i < progs.len() {
        let __m = __arena_cp();
        println!("[{}] {}", i, compile(progs[i]));
        __arena_reset(__m);
        i = i + 1;
    }
    println!("--- IR for prog0 ---");
    print!("{}", emit_ir(progs[0]));
}
// DYN-RET: wrap every tail value of a trait-object-returning fn body in `$dyncoerce$<target>(..)` (olower lowers it through
// lower_coerce, which leaves an already-dyn value alone)
fn dyn_ret_wrap(e: oast::E, tgt: &str) -> oast::E {
    match e {
        oast::E::Seq(a, b, l, h) => oast::E::Seq(a, Box::new(dyn_ret_wrap(*b, tgt)), l, h),
        oast::E::Let(n, v, b, l, h) => oast::E::Let(n, v, Box::new(dyn_ret_wrap(*b, tgt)), l, h),
        oast::E::LetT(n, t, v, b, l, h) => {
            oast::E::LetT(n, t, v, Box::new(dyn_ret_wrap(*b, tgt)), l, h)
        }
        oast::E::If(c, t, el, l, h) => oast::E::If(
            c,
            Box::new(dyn_ret_wrap(*t, tgt)),
            Box::new(dyn_ret_wrap(*el, tgt)),
            l,
            h,
        ),
        oast::E::Match(sc, arms, l, h) => {
            let mut arms = arms;
            let mut k = 0;
            while k < arms.len() {
                let b = std::mem::replace(&mut arms[k].body, oast::E::Int(0, 0, 0));
                arms[k].body = dyn_ret_wrap(b, tgt);
                k = k + 1;
            }
            oast::E::Match(sc, arms, l, h)
        }
        oast::E::RetE(v, l, h) => oast::E::RetE(v, l, h),
        oast::E::BreakE(l, h) => oast::E::BreakE(l, h),
        oast::E::ContE(l, h) => oast::E::ContE(l, h),
        other => {
            let l = oast::e_lo(&other);
            let h = oast::e_hi(&other);
            let mut n = String::from("$dyncoerce$");
            n.push_str(tgt);
            let mut a: Vec<oast::E> = Vec::new();
            a.push(other);
            oast::E::Call(n, a, l, h)
        }
    }
}
// G1 G-P2 MONO-ALL: an UNBOUNDED generic that has at least one monomorphized instance is not emitted in erased form -- its
// instances carry the real types (a `size_of::<T>()` in the erased body is unresolvable). A call site whose T did not resolve
// then names a missing fn: a refusal, never erased code. A generic with NO instance keeps its erased body (nothing regresses).
fn erased_superseded(fs: &Vec<oast::Fun>, fi: usize) -> bool {
    let f = &fs[fi];
    if f.gparams.len() == 0 || f.gsub.len() > 0 {
        return false;
    }
    // G1 G-P2 (full mono): an UNINSTANTIATED generic is not code at all (rustc never codegens it) -- `pub fn sz<T>()` in a
    // library checked on its own has no caller, and its erased body`s `size_of::<T>()` was refused
    if !gen_body_needs_t(f) {
        // a body that never names T`s identity still works erased: kept, so nothing that ran before regresses
    } else {
        return true;
    }
    let mut pre = String::from(f.name.as_str());
    pre.push('$');
    let mut k = 0;
    while k < fs.len() {
        if fs[k].gsub.len() > 0 && oast::str_starts(fs[k].name.as_str(), pre.as_str()) {
            return true;
        }
        k = k + 1;
    }
    false
}
// G1 G-P2: does a generic body depend on its type parameter's IDENTITY (a layout query on it), so an erased copy cannot be code?
fn gen_body_needs_t(f: &oast::Fun) -> bool {
    let mut k = 0;
    while k < f.gparams.len() {
        let mut a = String::from("$sizeof$");
        a.push_str(f.gparams[k].as_str());
        let mut b = String::from("$alignof$");
        b.push_str(f.gparams[k].as_str());
        if e_calls(&f.body, a.as_str()) || e_calls(&f.body, b.as_str()) {
            return true;
        }
        k = k + 1;
    }
    false
}
fn e_calls(e: &oast::E, n: &str) -> bool {
    match e {
        oast::E::Call(c, a, _l, _h) => {
            if c.as_str() == n {
                return true;
            }
            let mut k = 0;
            while k < a.len() {
                if e_calls(&a[k], n) {
                    return true;
                }
                k = k + 1;
            }
            false
        }
        oast::E::Seq(x, y, _l, _h) => e_calls(x, n) || e_calls(y, n),
        oast::E::Let(_v, x, y, _l, _h) => e_calls(x, n) || e_calls(y, n),
        oast::E::LetT(_v, _t, x, y, _l, _h) => e_calls(x, n) || e_calls(y, n),
        oast::E::If(c, t, el, _l, _h) => e_calls(c, n) || e_calls(t, n) || e_calls(el, n),
        oast::E::Bin(_o, x, y, _l, _h) => e_calls(x, n) || e_calls(y, n),
        oast::E::RetE(x, _l, _h) => e_calls(x, n),
        oast::E::MethodCall(r, _m, a, _l, _h) => {
            if e_calls(r, n) {
                return true;
            }
            let mut k = 0;
            while k < a.len() {
                if e_calls(&a[k], n) {
                    return true;
                }
                k = k + 1;
            }
            false
        }
        _ => false,
    }
}
