// oparse -- the PARSER: tokens -> Prog (items, expressions, desugars)
pub struct MacRule {
    pub pat: Vec<Tok>,
    pub body: Vec<Tok>,
}
pub struct Mac {
    pub name: String,
    pub rules: Vec<MacRule>,
}
// USE-TREE (R2a P2): does the `use` at `i` contain a `{` group before its `;`?
fn use_has_group(t: &Vec<Tok>, i: usize) -> bool {
    let mut k = i;
    while k < t.len() && !tkv_semi(&t[k].k) {
        if tkv_l_brace(&t[k].k) {
            return true;
        }
        k = k + 1;
    }
    false
}
// USE-TREE: read one use-tree at t[i0..end] under `prefix`; push `use <path> ;` per leaf (the use/`;` tokens are t[ut]/t[st]).
// Returns the index after the tree (at its `,` / `}` / end). `P::{self}` names P itself.
fn use_tree_flat(
    t: &Vec<Tok>,
    i0: usize,
    end: usize,
    prefix: &Vec<Tok>,
    ut: usize,
    st: usize,
    out: &mut Vec<Tok>,
) -> usize {
    let mut i = i0;
    let mut cur: Vec<Tok> = toks_clone(prefix);
    while i < end {
        if tkv_l_brace(&t[i].k) {
            i = i + 1;
            while i < end {
                if tkv_r_brace(&t[i].k) {
                    i = i + 1;
                    break;
                }
                if tkv_comma(&t[i].k) {
                    i = i + 1;
                } else {
                    i = use_tree_flat(t, i, end, &cur, ut, st, out);
                }
            }
            return i;
        }
        if tkv_comma(&t[i].k) || tkv_r_brace(&t[i].k) {
            break;
        }
        cur.push(tok_clone(&t[i]));
        i = i + 1;
    }
    let n = cur.len();
    if n >= 2 && word_is(&cur[n - 1], "self") && tkv_path_sep(&cur[n - 2].k) {
        let _ = cur.pop();
        let _ = cur.pop();
    }
    out.push(tok_clone(&t[ut]));
    let mut q = 0;
    while q < cur.len() {
        out.push(tok_clone(&cur[q]));
        q = q + 1;
    }
    out.push(tok_clone(&t[st]));
    i
}
pub fn tok_clone(t: &Tok) -> Tok {
    Tok {
        k: t.k.clone(),
        lo: t.lo,
        hi: t.hi,
    }
}
pub fn toks_clone(v: &Vec<Tok>) -> Vec<Tok> {
    let mut o: Vec<Tok> = Vec::new();
    let mut i = 0;
    while i < v.len() {
        o.push(tok_clone(&v[i]));
        i = i + 1;
    }
    o
}
// V5.0 TOK-NOALLOC: the token is matched by reference -- `t.k.clone()` copied the whole token (a literal's string too) and leaked it
// under the never-freeing heap, on every call; 330 call sites, 31 of them full-file scans, made ourc's self-compile pass 26 GB
pub fn tok_word(t: &Tok) -> String {
    match &t.k {
        Tk::Ident(x) => String::from(x.as_str()),
        _ => String::new(),
    }
}
// V5.0: is this token an identifier at all? -- the non-allocating form of `tok_word(t).len() > 0`
pub fn is_ident(t: &Tok) -> bool {
    match &t.k {
        Tk::Ident(x) => x.len() > 0,
        _ => false,
    }
}
// V5.0: is this token the identifier `s`? -- the non-allocating form of `tok_word(t).as_str() == s`
pub fn word_is(t: &Tok, s: &str) -> bool {
    match &t.k {
        Tk::Ident(x) => x.as_str() == s,
        _ => s.len() == 0,
    }
}
// M1 GLOB-NOALLOC: byte-wise compares that never slice (a `&s[a..b]` slice allocates a copy under SEM0)
// does a[a0..a1] equal b (all of it)?
fn bytes_eq_at(a: &[u8], a0: usize, b: &[u8]) -> bool {
    if a0 + b.len() > a.len() {
        return false;
    }
    let mut k = 0;
    while k < b.len() {
        if a[a0 + k] != b[k] {
            return false;
        }
        k = k + 1;
    }
    true
}
// is `<g[p0..]>$<n>` in v (g[p0..] = the glob's prefix part)? nothing is allocated unless it hits
fn has_joined(v: &Vec<String>, g: &[u8], p0: usize, n: &str) -> bool {
    let pl = g.len() - p0;
    let nb = n.as_bytes();
    let want = pl + 1 + nb.len();
    let mut i = 0;
    while i < v.len() {
        // V5.1 LEN-FIRST: `as_bytes()` COPIES the string under ourc's runtime (str_bytes_vec), and this loop runs over every
        // struct / enum name for every identifier the parser resolves (gres_ty <- tyres <- atom): test the length first
        if v[i].len() != want {
            i = i + 1;
            continue;
        }
        let b = v[i].as_bytes();
        if b.len() == want && b[pl] == 36 && bytes_eq_at(b, pl + 1, nb) {
            let mut k = 0;
            let mut ok = true;
            while k < pl && ok {
                if b[k] != g[p0 + k] {
                    ok = false;
                }
                k = k + 1;
            }
            if ok {
                return true;
            }
        }
        i = i + 1;
    }
    false
}
// do two tokens denote the same thing? (kind and, for an identifier, its text)
pub fn tok_same(a: &Tok, b: &Tok) -> bool {
    let wa = tok_word(a);
    let wb = tok_word(b);
    if wa.len() > 0 || wb.len() > 0 {
        return wa.as_str() == wb.as_str();
    }
    let ks = [
        "(", ")", "{", "}", "[", "]", ",", ";", ":", "=", "=>", "::", "$", "+", "-", "*", "/", "%",
        "<", ">", "==", "!=", "<=", ">=", "&", "|", "^", "!", "?", "..", "<<", ">>", ".",
    ];
    let mut i = 0;
    while i < ks.len() {
        let ia = tk_is(&a.k, ks[i]);
        let ib = tk_is(&b.k, ks[i]);
        if ia || ib {
            return ia && ib;
        }
        i = i + 1;
    }
    match a.k.clone() {
        Tk::Num(x) => match b.k.clone() {
            Tk::Num(y) => x == y,
            _ => false,
        },
        Tk::ChL(x) => match b.k.clone() {
            Tk::ChL(y) => x == y,
            _ => false,
        },
        Tk::StrL(x) => match b.k.clone() {
            Tk::StrL(y) => x.as_str() == y.as_str(),
            _ => false,
        },
        _ => false,
    }
}
pub fn tok_opens(t: &Tok) -> bool {
    tkv_l_paren(&t.k) || tkv_l_brace(&t.k) || tkv_l_brack(&t.k)
}
pub fn tok_closes(t: &Tok) -> bool {
    tkv_r_paren(&t.k) || tkv_r_brace(&t.k) || tkv_r_brack(&t.k)
}
pub struct P {
    lvlab: Vec<String>,
    trnames: Vec<String>,
    edflt: Vec<String>,
    farity: Vec<String>,
    refpn: Vec<String>,    // DEREF-PLACE: a `let r = &mut <fixed place>` name ...
    refpe: Vec<E>, // DEREF-PLACE: .. and its place (`*r` is that place while r is live -- the borrow checker forbids other uses)
    gen_fn: bool,  // CONST-GEN-LEN: reading a FN signature (const params admitted only there)
    cgen: Vec<String>, // CONST-GEN-LEN: the const params of the fn being read
    cgty: Vec<String>, // CONST-GEN-ARG: each const param's type
    tfconst: Vec<E>, // CONST-GEN-ARG: a call path's literal turbofish const arguments
    cghid: Vec<String>, // CONST-GEN-ARG: "fn=total|i,j" -- which const params of fn are hidden value params
    closlets: Vec<String>, // CLOS-CAP-CLOS (R2a P4): this fn's `let NAME = <closure>` names -- a capture of one is callable (Fn)
    cbind: Vec<String>,    // CONST-GEN-LEN: "N=param" -- N is the length of array param `param`
    fretn: Vec<String>, // FN-ITEM-RET: "name=ret" per fn, for the closure that wraps a fn used as a value
    lnames: Vec<String>,
    lastpay: String,
    lastgarg: String, // PARAM-GINST (W1): a user enum type's single type argument (`Op<P>` -> "P"), erased from its name
    chantx: Vec<String>, // R4f CHAN: names bound to a channel SENDER (send -> push)
    ptrinfo: Vec<String>, // R4b PTR: "n=@null" or "n=<place root>" for a non-mut raw-pointer / reference binding
    spawncl: Vec<String>, // SPAWN-PANIC: the closures handed to thread::spawn (checked transitively at the end of the parse)
    chanrx: Vec<String>,  // R4f CHAN: names bound to a channel RECEIVER (recv -> pop front)
    lastarrel: String, // R4a XMUTE: the element type of the last `[T; N]` parsed (its name erases to `arr`)
    annel: String,     // R4a XMUTE: the array element of the current let annotation
    fpays: Vec<String>,
    mutcaps: Vec<String>,
    strvals: Vec<u32>,
    immlets: Vec<u32>,
    implinst: String,
    lastfull: String, // R12e: the last parse_tyname result in FULL (`Pair$B$A`), before GEN-IMPL / arg erasure
    lastgargs: String, // R12e: a multi-arg struct type's arguments as written ("B,A"), set by parse_tyname0
    fnrets: Vec<String>, // FN-BOUND-RET: "F=R" from `F: Fn*(..) -> R` bounds of the fn being read
    fnargsv: Vec<String>, // FN-BOUND-ARGS: "F=A,B" from `F: Fn*(A, B)` bounds of the fn being read
    implbase: String,
    hasmain: bool,
    boxid: i32,
    boxvars: Vec<String>,
    nextpub: bool,
    privfns: Vec<String>,
    pcfns: Vec<String>, // PUB-CRATE (R2a P6): "qualified|root$" -- a pub(crate) fn of an extern crate, visible only under root
    nextpubcrate: bool,
    privflds: Vec<String>, // VIS-FIELD (R2a P6): "m$S|f" -- a non-pub field of a struct declared in module m
    usechk: Vec<String>,
    usechklo: Vec<u32>,
    cparams: Vec<String>,
    lbls: Vec<String>,
    plabel: String,
    lentry: bool,
    tfish: String,
    tfish2: String, // R4a XMUTE: a turbofish's SECOND type (`transmute::<A, B>` -> B)
    tfcalls: Vec<String>, // G1 G-P2: "lo|T1|T2" -- a user call written with a turbofish (`sz::<u64>()`), for mono demand
    tfishtx: String, // R4a LAYOUT: the turbofish's FIRST type as raw token text (lossless: `&u8`, `Option < & u8 >`)
    annty: String,
    plname: Vec<String>,
    plidx: Vec<i32>,
    plofs: Vec<i32>,
    mstr: Vec<String>,
    mstri: Vec<i32>,
    mstrt: Vec<String>,
    globs: Vec<String>,
    globm: Vec<String>, // V5.1: each glob's declaring module (the part before `|`), kept apart so a lookup never copies bytes
    globp: Vec<String>, // V5.1: each glob's prefix (the part after `|`)
    fnames: Vec<String>,
    lvhit: Vec<String>,
    loopv: Vec<String>,
    mscal: Vec<String>,
    smut: Vec<String>,
    vfn: Vec<String>,
    atn: Vec<String>,
    lproj: String,
    att: Vec<String>,
    selfty: String,
    usen: Vec<String>,
    uset: Vec<String>,
    namesonly: bool,
    perrexp: String,
    perrgot: String,
    nhyg: u32,
    macs: Vec<Mac>,
    depth: u32,
    vnames: Vec<String>,
    vowner: Vec<String>,
    sfnames: Vec<String>,
    sawcopy: bool,
    sawreprc: bool,
    saword: bool, // R13 DERIVE-ORD: the next struct derives Ord // R4a LAYOUT: the next struct is #[repr(C)]
    aliasn: Vec<String>,
    aliast: Vec<String>,
    perr: u32,
    badfmt: u32,
    consts: Vec<String>,
    ucyc: Vec<String>, // USE-CYCLE: "alias=target$path" for every `use` (single-segment `use e as b` too)
    cpatuse: Vec<String>, // CONST-PAT-TY widening: non-primitive consts used as patterns (checked post-parse)
    ordneed: Vec<String>, // REGRESS-5: "S=fieldty" for each field of a derive(Ord) struct
    ordhave: Vec<String>, // REGRESS-5: derive(Ord) struct names
    ctys: Vec<String>,    // CONST-PAT-TY: "name=type" of each const (or "name=!static")
    t: Vec<Tok>,
    i: usize,
    snames: Vec<String>,
    enames: Vec<String>,
    clos: Vec<Fun>,
    nfor: u32,
    mpfx: String,
    mods: Vec<String>,
    retmut: bool,
    refvars: Vec<String>,
    refret: Vec<String>,
    lastretmut: bool,
}
// an integer literal, whichever KIND it was written as: 1 = a number, 2 = a char literal, 0 = neither.
// Every position that only needs the VALUE (an array repeat count, a range endpoint, a match literal)
// asks this, so a char literal works everywhere a number does without 15 copies of the test.
pub fn ilit(t: &Tok) -> i32 {
    match &t.k {
        Tk::Num(_n) => 1,
        Tk::ChL(_n) => 2,
        _ => 0,
    }
}
pub fn is_float_tok(t: &Tok) -> bool {
    match &t.k {
        Tk::FloatL(_x) => true,
        _ => false,
    }
}
pub fn ilit_val(t: &Tok) -> i64 {
    match &t.k {
        Tk::Num(n) => *n,
        Tk::ChL(n) => *n,
        _ => 0,
    }
}
// the owner prefix of a qualified item name: `S$n` -> `S`, `main` -> ""
// is this expression a PLACE? (read at format time, the way rustc reads a reference)
pub fn is_place_expr(e: &E) -> bool {
    match e {
        E::Var(_n, _l, _h) => true,
        E::Field(_o, _f, _l, _h) => true,
        E::TupIdx(_o, _i, _l, _h) => true,
        E::Index(_b, _i, _l, _h) => true,
        E::Call(n, a, _l, _h) => a.len() == 0 && olower::is_gname(n),
        _ => false,
    }
}
// a `&mut` to a SCALAR needs a place our value model does not have; a `&mut` to an AGGREGATE needs
// nothing, because our aggregates are heap handles and a handle already behaves as a reference (the
// probe's p5/p6 agree today). So the refusal is exactly scalar-shaped.
pub fn find_sname(ss: &Vec<Sdef>, n: &str) -> bool {
    let mut i = 0;
    while i < ss.len() {
        if ss[i].name.as_str() == n {
            return true;
        }
        i = i + 1;
    }
    false
}
pub fn scalar_ty(n: &str) -> bool {
    n == "int" || n == "bool" || n == "char" || n == "f64" || n == "str" || oast::is_int_width(n)
}
pub fn fn_bounded(gps: &Vec<String>, gbs: &Vec<String>, ty: &str) -> bool {
    let mut i = 0;
    while i < gps.len() {
        if gps[i].as_str() == ty && i < gbs.len() {
            let b = gbs[i].as_bytes();
            if b.len() >= 2 && b[0] == 70 && b[1] == 110 {
                return true;
            }
        }
        i = i + 1;
    }
    false
}
pub fn lit_ty(e: &E) -> String {
    match e {
        E::StrLit(_v, _l, _h) => String::from("str"),
        E::Int(_v, _l, _h) => String::from("int"),
        E::Bool(_v, _l, _h) => String::from("bool"),
        E::Cast(_x, t, _l, _h) => String::from(t.as_str()),
        E::StructLit(n, _f, _l, _h) => String::from(n.as_str()),
        E::EnumLit(n, _v, _p, _l, _h) => String::from(n.as_str()),
        E::LetT(_n, t, _v, _b, _l, _h) => String::from(t.as_str()),
        E::TupLit(parts, _l, _h) => {
            let mut es: Vec<String> = Vec::new();
            let mut k = 0;
            while k < parts.len() {
                es.push(lit_ty(&parts[k]));
                k = k + 1;
            }
            tup_name(parts.len() as u32, &es)
        }
        E::Call(n, _a, _l, _h) => {
            if n.as_str() == "f64_parse" {
                String::from("f64")
            } else {
                String::from("")
            }
        }
        E::Bin(_op, a, b, _l, _h) => {
            let la = lit_ty(a);
            let lb = lit_ty(b);
            if la.as_str() == "f64" || lb.as_str() == "f64" {
                String::from("f64")
            } else if la.as_str() == "int" && lb.as_str() == "int" {
                String::from("int")
            } else {
                String::from("")
            }
        } // P-NEG-LIT-DEBUG: `-3` is Bin("-",0,3); an arithmetic literal expr types as its operands
        _ => String::from(""),
    }
}
// A tuple is a NAME like every other type here, and until now that name carried only the ARITY -- so a
// tuple VARIABLE could not be Debug-printed (its elements were unknown) while a tuple LITERAL could (its
// elements were right there). The name now carries the element types: `tup2$int$str`. Only LEAF element
// names go in -- an element that is itself composite (a Vec, an array, another tuple, a closure, a dyn)
// would make the name ambiguous to read back, so such a tuple keeps the bare arity name and keeps the
// refusal it has today. Integer widths normalize to `int` so an annotation `(i64, i64)` and the literal
// `(1, 2)` mint the SAME name.
pub fn tup_leaf(t: &str) -> String {
    if t.len() == 0 {
        return String::from("");
    }
    if t == "Vec" || t == "arr" || t == "Fn" || t == "unit" {
        return String::from("");
    }
    if tup_arity_name(t) >= 0 {
        return String::from("");
    }
    if t == "char" || t == "bool" || t == "str" || t == "f64" {
        return String::from(t);
    }
    if t == "int" || is_int_width(t) {
        return String::from("int");
    }
    // a non-scalar element (Vec$int, Vec$S, a struct/enum name) is carried by ENCODING its inner `$` as
    // `~`, so it does not collide with the `$` that separates tuple leaves; tup_elems decodes it back.
    {
        let b = t.as_bytes();
        let mut r = String::from("");
        let mut i = 0;
        while i < b.len() {
            if b[i] == 36 {
                r.push_str("~");
            } else {
                r.push_str(&t[i..(i + 1)]);
            }
            i = i + 1;
        }
        r
    }
}
// C-SLICEREAD: the methods that WRITE through their receiver. A slice in a value position is a copy in
// this value model, so these would write to the copy and the caller would never see it -- they stay
// refused. Everything else only reads, and rustc accepts those on a slice, so they are admitted.
pub fn slice_mutator(m: &str) -> bool {
    m == "push"
        || m == "pop"
        || m == "push_str"
        || m == "insert"
        || m == "remove"
        || m == "clear"
        || m == "sort"
        || m == "sort_by"
        || m == "sort_unstable"
        || m == "reverse"
        || m == "truncate"
        || m == "set"
        || m == "swap"
        || m == "retain"
        || m == "dedup"
        || m == "fill"
        || m == "extend"
        || m == "append"
        || m == "drain"
        || m == "resize"
}
pub fn tup_arity_name(tn: &str) -> i32 {
    let b = tn.as_bytes();
    if b.len() < 4 {
        return -1;
    }
    if b[0] != 116 || b[1] != 117 || b[2] != 112 {
        return -1;
    }
    let mut v = 0;
    let mut i = 3;
    let mut any = false;
    while i < b.len() {
        if b[i] == 36 {
            break;
        }
        if b[i] < 48 || b[i] > 57 {
            return -1;
        }
        v = v * 10 + ((b[i] - 48) as i32);
        any = true;
        i = i + 1;
    }
    if !any {
        return -1;
    }
    v
}
pub fn tup_name(n: u32, es: &Vec<String>) -> String {
    let mut r = String::from("tup");
    r.push_str(&u32s2(n));
    if (es.len() as u32) != n {
        return r;
    }
    let mut parts = String::from("");
    let mut k = 0;
    while k < es.len() {
        let l = tup_leaf(es[k].as_str());
        if l.len() == 0 {
            return r;
        }
        parts.push('$');
        parts.push_str(l.as_str());
        k = k + 1;
    }
    r.push_str(parts.as_str());
    r
}
// the element types of a tuple name, or empty when the name carries only the arity
pub fn tup_elems(tn: &str) -> Vec<String> {
    let mut o: Vec<String> = Vec::new();
    let b = tn.as_bytes();
    let mut i = 3;
    if tup_arity_name(tn) < 0 {
        return o;
    }
    while i < b.len() {
        if b[i] == 36 {
            break;
        }
        i = i + 1;
    }
    if i >= b.len() {
        return o;
    }
    let mut start = i + 1;
    let mut j = start;
    while j <= b.len() {
        if j == b.len() || b[j] == 36 {
            let raw = &tn[start..j];
            let rb = raw.as_bytes();
            let mut e = String::from("");
            let mut k = 0;
            while k < rb.len() {
                if rb[k] == 126 {
                    e.push_str("$");
                } else {
                    e.push_str(&raw[k..(k + 1)]);
                }
                k = k + 1;
            }
            o.push(e);
            start = j + 1;
        }
        j = j + 1;
    }
    o
}
pub fn name_owner(n: &str) -> String {
    let b = n.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == 36 {
            return String::from(&n[0..i]);
        }
        i = i + 1;
    }
    String::from("")
}
// does this expression perform OUTPUT? (the only side effect our value model has)
// does this body WRITE through the named binding? (`*x = v` erases to `x = v` here)
// C-PLACERET: the out-param transform appends the parameter as the function's tail value, which is only
// equivalent when the body has no EARLY return. A body that returns is refused rather than mis-transformed.
pub fn has_ret(e: &E) -> bool {
    match e {
        E::RetE(_x, _l, _h) => true,
        E::Seq(a, b, _l, _h) => has_ret(a) || has_ret(b),
        E::Let(_x, v, b, _l, _h) => has_ret(v) || has_ret(b),
        E::LetT(_x, _t, v, b, _l, _h) => has_ret(v) || has_ret(b),
        E::If(c, t, f, _l, _h) => has_ret(c) || has_ret(t) || has_ret(f),
        E::While(c, b, _l, _h) => has_ret(c) || has_ret(b),
        E::Match(s2, arms, _l, _h) => {
            if has_ret(s2) {
                return true;
            }
            let mut i = 0;
            while i < arms.len() {
                if has_ret(&arms[i].body) {
                    return true;
                }
                i = i + 1;
            }
            false
        }
        _ => false,
    }
}
// the variable a place argument names: `&mut x` or a bare `x`, and nothing else
pub fn place_name(e: &E) -> String {
    match e {
        E::Ref(x, _m, _l, _h) => match x.as_ref() {
            E::Var(n, _l2, _h2) => String::from(n.as_str()),
            _ => String::from(""),
        },
        E::Var(n, _l, _h) => String::from(n.as_str()),
        _ => String::from(""),
    }
}
// C-PLACEEXPR: a place is a NAME, a FIELD of a place, or an INDEX of one -- `mem::swap(&mut p.a, &mut p.b)`
// and `mem::take(&mut x.s)` are ordinary Rust and were refused because the desugars only knew names.
// Three questions, each asked in one place: is this a place, how is it DUPLICATED (the desugars read it
// before writing it), and how is it WRITTEN.
pub fn place_inner(e: &E) -> E {
    match e {
        E::Ref(x, _m, l, h) => clone_simple(x.as_ref(), *l, *h),
        _ => {
            let l = oast::e_lo(e);
            let h = oast::e_hi(e);
            clone_simple(e, l, h)
        }
    }
}
pub fn is_place_e(e: &E) -> bool {
    match e {
        E::Ref(x, _m, _l, _h) => is_place_e(x.as_ref()),
        E::Var(_n, _l, _h) => true,
        E::Field(o, _f, _l, _h) => is_place_e(o.as_ref()),
        E::TupIdx(o, _i, _l, _h) => is_place_e(o.as_ref()),
        E::Index(b, _i, _l, _h) => is_place_e(b.as_ref()),
        _ => false,
    }
}
pub fn assign_place(pl: &E, v: E, lo: u32, hi: u32) -> E {
    match pl {
        E::Var(n, _l, _h) => E::Assign(String::from(n.as_str()), Box::new(v), lo, hi),
        _ => E::AssignTo(Box::new(clone_simple(pl, lo, hi)), Box::new(v), lo, hi),
    }
}
pub fn is_bare_var_p(e: &E) -> bool {
    match e {
        E::Var(_n, _l, _h) => true,
        E::Seq(_a, b, _l, _h) => is_bare_var_p(b),
        E::Let(_x, _v, b, _l, _h) => is_bare_var_p(b),
        E::LetT(_x, _t, _v, b, _l, _h) => is_bare_var_p(b),
        _ => false,
    }
} // MUT-SELF-RET: `{ self.n += 1; self }` -- the TAIL is the place
pub fn tail_place_ref_p(e: &E) -> bool {
    match e {
        E::Ref(x, m, _l, _h) => {
            if *m {
                match x.as_ref() {
                    E::Index(_b, _i, _a, _b2) => true,
                    E::Field(_o, _f, _a, _b2) => true,
                    _ => false,
                }
            } else {
                false
            }
        }
        E::Seq(_a, b, _l, _h) => tail_place_ref_p(b),
        E::Let(_x, _v, b, _l, _h) => tail_place_ref_p(b),
        E::LetT(_x, _t, _v, b, _l, _h) => tail_place_ref_p(b),
        E::RetE(x, _l, _h) => tail_place_ref_p(x),
        _ => false,
    }
}
pub fn assigns_to(e: &E, n: &str) -> bool {
    match e {
        E::Assign(m, v, _l, _h) => {
            if m.as_str() == n {
                return true;
            }
            assigns_to(v, n)
        }
        E::AssignTo(pl, v, _l, _h) => {
            match pl.as_ref() {
                E::Var(m, _l2, _h2) => {
                    if m.as_str() == n {
                        return true;
                    }
                }
                _ => {}
            }
            assigns_to(pl, n) || assigns_to(v, n)
        }
        E::Seq(a, b, _l, _h) => assigns_to(a, n) || assigns_to(b, n),
        E::Let(_x, v, b, _l, _h) => assigns_to(v, n) || assigns_to(b, n),
        E::LetT(_x, _t, v, b, _l, _h) => assigns_to(v, n) || assigns_to(b, n),
        E::If(c, t, f, _l, _h) => assigns_to(c, n) || assigns_to(t, n) || assigns_to(f, n),
        E::While(c, b, _l, _h) => assigns_to(c, n) || assigns_to(b, n),
        E::Bin(_o, a, b, _l, _h) => assigns_to(a, n) || assigns_to(b, n),
        E::Match(s2, arms, _l, _h) => {
            if assigns_to(s2, n) {
                return true;
            }
            let mut i = 0;
            while i < arms.len() {
                if assigns_to(&arms[i].body, n) {
                    return true;
                }
                i = i + 1;
            }
            false
        }
        E::Call(_f, a, _l, _h) => {
            let mut i = 0;
            while i < a.len() {
                if assigns_to(&a[i], n) {
                    return true;
                }
                i = i + 1;
            }
            false
        }
        // C-PLACEMETHOD: a MUTATING METHOD is a write. The `&mut` scalar-place refusal asked only whether
        // the body ASSIGNED to the parameter, so `acc.push_str(w)` through a `&mut String` parameter slipped
        // past it and was compiled to a write of the CALLEE's own slot -- accepted, built, ran, and printed
        // the empty string where rustc prints the appended one. The mutating-method set already exists
        // (slice_mutator); this asks it instead of keeping a second list.
        E::MethodCall(r, _m, a, _l, _h) => {
            match r.as_ref() {
                E::Var(rn, _l2, _h2) => {
                    if rn.as_str() == n && slice_mutator(_m.as_str()) {
                        return true;
                    }
                }
                _ => {}
            }
            if assigns_to(r, n) {
                return true;
            }
            let mut i = 0;
            while i < a.len() {
                if assigns_to(&a[i], n) {
                    return true;
                }
                i = i + 1;
            }
            false
        }
        E::RetE(x, _l, _h) => assigns_to(x, n),
        _ => false,
    }
}
// CLOS-PREF (W1): every lifted closure slot (captures, then params) is marked by-reference, so param_drops never drops it:
// a `|x: &F|` param was dropped at the closure body end (`f(&v[1])` dropped v[1] early, then never again). A by-value
// closure param now misses its drop (safe), never doubles.
// EXT-TEMP (W1): temporary lifetime extension. In `let r = R { c: &D("c"), d: &D("d") };` each `&<temporary>` held by the
// initializer's struct/tuple literal lives as long as r and drops after it, in REVERSE creation order (they were never
// dropped). Each such temporary is hoisted into a hidden binding `$et<lo>_<k>` declared before the let; scope order then
// gives exactly rustc's order, and r's drop glue skips the `&` fields (REF-FIELD).
pub fn ext_hoist(v: E, lo: u32, ns: &mut Vec<String>, vs: &mut Vec<E>) -> E {
    if !ext_any(&v) {
        return v; // nothing to hoist: the initializer is returned untouched (never rebuilt)
    }
    match v {
        E::StructLit(s, xs, l, h) => E::StructLit(s, ext_list(xs, lo, ns, vs), l, h),
        E::TupLit(xs, l, h) => E::TupLit(ext_list(xs, lo, ns, vs), l, h),
        _ => v,
    }
}
fn ext_list(xs: Vec<E>, lo: u32, ns: &mut Vec<String>, vs: &mut Vec<E>) -> Vec<E> {
    let mut out: Vec<E> = Vec::new();
    let mut xs = xs;
    let mut rev: Vec<E> = Vec::new();
    while xs.len() > 0 {
        rev.push(xs.pop().unwrap());
    }
    while rev.len() > 0 {
        let x = rev.pop().unwrap();
        let y = match x {
            E::Ref(inner, m, l, h) => {
                let tmp = match inner.as_ref() {
                    E::StructLit(_a, _b, _c, _d) => true,
                    E::Call(_a, _b, _c, _d) => true,
                    E::EnumLit(_a, _b, _c, _d, _e) => true,
                    E::TupLit(_a, _b, _c) => true,
                    _ => false,
                };
                if tmp {
                    let mut n = String::from("$et");
                    n.push_str(&u32s2(lo));
                    n.push('_');
                    n.push_str(&u32s2(ns.len() as u32));
                    let inner2 = ext_hoist(*inner, lo, ns, vs);
                    ns.push(String::from(n.as_str()));
                    vs.push(inner2);
                    E::Ref(Box::new(E::Var(n, l, h)), m, l, h)
                } else {
                    E::Ref(inner, m, l, h)
                }
            }
            E::StructLit(s, ys, l, h) => E::StructLit(s, ext_list(ys, lo, ns, vs), l, h),
            E::TupLit(ys, l, h) => E::TupLit(ext_list(ys, lo, ns, vs), l, h),
            other => other,
        };
        out.push(y);
    }
    out
}
fn ext_tmp(e: &E) -> bool {
    match e {
        E::StructLit(_a, _b, _c, _d) => true,
        E::Call(_a, _b, _c, _d) => true,
        E::EnumLit(_a, _b, _c, _d, _e) => true,
        E::TupLit(_a, _b, _c) => true,
        _ => false,
    }
}
// EXT-TEMP: does a struct/tuple literal hold a `&<temporary>` (at any literal depth)?
fn ext_any(e: &E) -> bool {
    let xs = match e {
        E::StructLit(_s, xs, _l, _h) => xs,
        E::TupLit(xs, _l, _h) => xs,
        _ => return false,
    };
    let mut k = 0;
    while k < xs.len() {
        let hit = match &xs[k] {
            E::Ref(inner, _m, _l, _h) => ext_tmp(inner.as_ref()),
            other => ext_any(other),
        };
        if hit {
            return true;
        }
        k = k + 1;
    }
    false
}
pub fn ext_wrap(e: E, ns: Vec<String>, vs: Vec<E>, lo: u32, h: u32) -> E {
    let mut acc = e;
    let mut ns = ns;
    let mut vs = vs;
    while ns.len() > 0 {
        let n = ns.pop().unwrap();
        let v = vs.pop().unwrap();
        acc = E::Let(n, Box::new(v), Box::new(acc), lo, h);
    }
    acc
}
// CAP-DROP (W1): how a closure body uses capture n -- 0 = only through fields (collected in fs), 1 = some use of n WHOLE, 2 = a
// construct not modelled here (the caller then never drops the capture: a missed drop, never a wrong one).
pub fn cap_use(e: &E, n: &str, fs: &mut Vec<String>) -> u32 {
    match e {
        E::Var(x, _l, _h) => {
            if x.as_str() == n {
                1
            } else {
                0
            }
        }
        E::Int(_v, _l, _h) => 0,
        E::Bool(_b, _l, _h) => 0,
        E::StrLit(_s, _l, _h) => 0,
        E::Field(x, f, _l, _h) => match x.as_ref() {
            E::Var(y, _l2, _h2) => {
                if y.as_str() == n {
                    if !oast::in_list(fs, f) {
                        fs.push(String::from(f.as_str()));
                    }
                }
                0
            }
            _ => cap_use(x, n, fs),
        },
        E::TupIdx(x, i, _l, _h) => match x.as_ref() {
            E::Var(y, _l2, _h2) => {
                if y.as_str() == n {
                    let f = oast::u32s2(*i);
                    if !oast::in_list(fs, f.as_str()) {
                        fs.push(f);
                    }
                }
                0
            }
            _ => cap_use(x, n, fs),
        }, // a tuple / tuple-struct element read is a FIELD use
        E::Not(x, _l, _h) => cap_use(x, n, fs),
        E::Cast(x, _t, _l, _h) => cap_use(x, n, fs),
        E::Ref(x, _m, _l, _h) => cap_use(x, n, fs),
        E::Bin(_o, a, b, _l, _h) => cap_use2(cap_use(a, n, fs), cap_use(b, n, fs)),
        E::Index(a, b, _l, _h) => cap_use2(cap_use(a, n, fs), cap_use(b, n, fs)),
        E::Seq(a, b, _l, _h) => cap_use2(cap_use(a, n, fs), cap_use(b, n, fs)),
        E::Let(_x, a, b, _l, _h) => cap_use2(cap_use(a, n, fs), cap_use(b, n, fs)),
        E::If(c, a, b, _l, _h) => cap_use2(
            cap_use(c, n, fs),
            cap_use2(cap_use(a, n, fs), cap_use(b, n, fs)),
        ),
        E::Call(_f, a, _l, _h) => {
            let mut r = 0;
            let mut k = 0;
            while k < a.len() {
                r = cap_use2(r, cap_use(&a[k], n, fs));
                k = k + 1;
            }
            r
        }
        E::MethodCall(x, _m, a, _l, _h) => {
            let mut r = cap_use(x, n, fs);
            let mut k = 0;
            while k < a.len() {
                r = cap_use2(r, cap_use(&a[k], n, fs));
                k = k + 1;
            }
            r
        }
        _ => 2,
    }
}
fn cap_use2(a: u32, b: u32) -> u32 {
    if a == 2 || b == 2 {
        2
    } else if a == 1 || b == 1 {
        1
    } else {
        0
    }
}
// R4f CHAN: `mpsc::channel()` / `sync_channel(n)` (bare after a `use`, or by path)
pub fn is_chan_call(v: &E) -> bool {
    match v {
        E::Call(n, _a, _l, _h) => {
            n.as_str() == "channel"
                || n.as_str() == "sync_channel"
                || (n.contains("mpsc$")
                    && (n.ends_with("$channel") || n.ends_with("$sync_channel")))
        }
        _ => false,
    }
}
// R4f CHAN: `rx.recv()` -> `if rx.len() > 0 { Ok(rx.remove(0)) } else { Err(0) }` -- every sender ran to completion at its
// spawn, so an EMPTY queue is a closed one (rustc: Err(RecvError) once all senders are gone)
pub fn chan_recv(r: &str, lo: u32, hi: u32) -> E {
    let v = || E::Var(String::from(r), lo, hi);
    let len = E::MethodCall(Box::new(v()), String::from("len"), Vec::new(), lo, hi);
    let has = E::Bin(
        String::from(">"),
        Box::new(len),
        Box::new(E::Int(0, lo, hi)),
        lo,
        hi,
    );
    let mut ra: Vec<E> = Vec::new();
    ra.push(E::Int(0, lo, hi));
    let pop = E::MethodCall(Box::new(v()), String::from("remove"), ra, lo, hi);
    let mut oa: Vec<E> = Vec::new();
    oa.push(pop);
    let mut ea: Vec<E> = Vec::new();
    ea.push(E::Int(0, lo, hi));
    E::If(
        Box::new(has),
        Box::new(E::EnumLit(
            String::from("Result"),
            String::from("Ok"),
            oa,
            lo,
            hi,
        )),
        Box::new(E::EnumLit(
            String::from("Result"),
            String::from("Err"),
            ea,
            lo,
            hi,
        )),
        lo,
        hi,
    )
}
// R4f ATOMIC: a memory-ordering argument (`Ordering::SeqCst` parses as a variant of the cmp `Ordering`)
pub fn is_mem_order(e: &E) -> bool {
    match e {
        E::EnumLit(t, v, _p, _l, _h) => {
            (t.as_str() == "Ordering" || t.ends_with("$Ordering"))
                && (v.as_str() == "SeqCst"
                    || v.as_str() == "Relaxed"
                    || v.as_str() == "Acquire"
                    || v.as_str() == "Release"
                    || v.as_str() == "AcqRel")
        }
        _ => false,
    }
}
// R4f ATOMIC: an atomic method call, recognised by its trailing memory-ordering argument
pub fn atomic_op(m: &str, a: &Vec<E>) -> bool {
    if a.len() == 0 || !is_mem_order(&a[a.len() - 1]) {
        return false;
    }
    (m == "load" && a.len() == 1)
        || ((m == "store"
            || m == "fetch_add"
            || m == "fetch_sub"
            || m == "swap"
            || m == "fetch_or"
            || m == "fetch_and"
            || m == "fetch_xor")
            && a.len() == 2)
}
// R4f ATOMIC: `x.load(o)` -> x; `x.store(v, o)` -> x = v; `x.fetch_OP(v, o)` -> { let t = x; x = t OP v; t }; `x.swap(v, o)` -> { let t = x; x = v; t }
// x is a local binding ("v:<name>") or a mutable static ("s:<name>": read = $gload$, write = $gstore$). A local's write is an
// AssignTo (interior mutability: the binding is not `mut`, and E0384 is about `=` on the binding itself).
fn atomic_rd(x: &str, lo: u32, hi: u32) -> E {
    if x.starts_with("s:") {
        let mut g = String::from("$gload$");
        g.push_str(&x[2..]);
        return E::Call(g, Vec::new(), lo, hi);
    }
    E::Var(String::from(&x[2..]), lo, hi)
}
fn atomic_wr(x: &str, v: E, lo: u32, hi: u32) -> E {
    if x.starts_with("s:") {
        let mut g = String::from("$gstore$");
        g.push_str(&x[2..]);
        let mut a: Vec<E> = Vec::new();
        a.push(v);
        return E::Call(g, a, lo, hi);
    }
    E::AssignTo(
        Box::new(E::Var(String::from(&x[2..]), lo, hi)),
        Box::new(v),
        lo,
        hi,
    )
}
pub fn atomic_rewrite(m: &str, x: &str, t: &str, args: Vec<E>, lo: u32, hi: u32) -> E {
    let mut args = args;
    let _ = args.pop();
    if m == "load" {
        return atomic_rd(x, lo, hi);
    }
    let v = args.pop().unwrap();
    if m == "store" {
        return atomic_wr(x, v, lo, hi);
    }
    let nv = if m == "swap" {
        v
    } else {
        let op = if m == "fetch_add" {
            "+"
        } else if m == "fetch_sub" {
            "-"
        } else if m == "fetch_or" {
            "|"
        } else if m == "fetch_and" {
            "&"
        } else {
            "^"
        };
        E::Bin(
            String::from(op),
            Box::new(E::Var(String::from(t), lo, hi)),
            Box::new(v),
            lo,
            hi,
        )
    };
    let body = E::Seq(
        Box::new(atomic_wr(x, nv, lo, hi)),
        Box::new(E::Var(String::from(t), lo, hi)),
        lo,
        hi,
    );
    E::Let(
        String::from(t),
        Box::new(atomic_rd(x, lo, hi)),
        Box::new(body),
        lo,
        hi,
    )
}
// R4c MUTEX: the cell accessor a `.lock()` / `.write()` / `.read()` receiver stands for ("" = not a guard call)
pub fn lock_guard(e: &E) -> String {
    match e {
        E::MethodCall(_x, m, a, _l, _h) => {
            if a.len() == 0 && (m.as_str() == "lock" || m.as_str() == "write") {
                String::from("borrow_mut")
            } else if a.len() == 0 && m.as_str() == "read" {
                String::from("borrow")
            } else {
                String::new()
            }
        }
        _ => String::new(),
    }
}
pub fn clos_pref(n: usize) -> Vec<bool> {
    let mut v: Vec<bool> = Vec::new();
    let mut i = 0;
    while i < n {
        v.push(true);
        i = i + 1;
    }
    v
}
// R4f SPAWN: can this (closure) body reach an explicit panic (panic!/assert!/unreachable!)? Walks the same forms body_has_io does;
// an unmodelled form answers true (refuse rather than risk a whole-process panic where rustc's thread would die alone)
pub fn body_has_panic(e: &E) -> bool {
    match e {
        E::Call(n, a, _l, _h) => n.as_str() == "panic_exit" || any_panic(a),
        E::MethodCall(r, _m, a, _l, _h) => body_has_panic(r) || any_panic(a),
        E::Seq(a, b, _l, _h) => body_has_panic(a) || body_has_panic(b),
        E::Let(_n, v, b, _l, _h) => body_has_panic(v) || body_has_panic(b),
        E::LetT(_n, _t, v, b, _l, _h) => body_has_panic(v) || body_has_panic(b),
        E::If(c, t, f, _l, _h) => body_has_panic(c) || body_has_panic(t) || body_has_panic(f),
        E::While(c, b, _l, _h) => body_has_panic(c) || body_has_panic(b),
        E::Bin(_o, a, b, _l, _h) => body_has_panic(a) || body_has_panic(b),
        E::Assign(_n, v, _l, _h) => body_has_panic(v),
        E::AssignTo(p, v, _l, _h) => body_has_panic(p) || body_has_panic(v),
        E::Field(x, _f, _l, _h) => body_has_panic(x),
        E::Index(x, i, _l, _h) => body_has_panic(x) || body_has_panic(i),
        E::Ref(x, _m, _l, _h) => body_has_panic(x),
        E::Not(x, _l, _h) => body_has_panic(x),
        E::Cast(x, _t, _l, _h) => body_has_panic(x),
        E::TupLit(a, _l, _h) => any_panic(a),
        E::ArrLit(a, _l, _h) => any_panic(a),
        E::StructLit(_s, a, _l, _h) => any_panic(a),
        E::EnumLit(_t, _v, a, _l, _h) => any_panic(a),
        E::Var(_n, _l, _h) => false,
        E::Int(_v, _l, _h) => false,
        E::Bool(_v, _l, _h) => false,
        E::StrLit(_v, _l, _h) => false,
        E::ClosRef(_c, _p, _l, _h) => false,
        E::BreakE(_l, _h) => false,
        E::ContE(_l, _h) => false,
        _ => true,
    }
}
// R4f CHAN: does the body mention any of these names? (an unmodelled form answers true -- refuse)
pub fn body_uses_any(e: &E, ns: &Vec<String>) -> bool {
    if ns.len() == 0 {
        return false;
    }
    match e {
        E::Var(n, _l, _h) => oast::in_list(ns, n.as_str()),
        E::Call(_n, a, _l, _h) => uses_list(a, ns),
        E::MethodCall(r, _m, a, _l, _h) => body_uses_any(r, ns) || uses_list(a, ns),
        E::Seq(a, b, _l, _h) => body_uses_any(a, ns) || body_uses_any(b, ns),
        E::Let(_n, v, b, _l, _h) => body_uses_any(v, ns) || body_uses_any(b, ns),
        E::LetT(_n, _t, v, b, _l, _h) => body_uses_any(v, ns) || body_uses_any(b, ns),
        E::If(c, t, f, _l, _h) => {
            body_uses_any(c, ns) || body_uses_any(t, ns) || body_uses_any(f, ns)
        }
        E::While(c, b, _l, _h) => body_uses_any(c, ns) || body_uses_any(b, ns),
        E::Bin(_o, a, b, _l, _h) => body_uses_any(a, ns) || body_uses_any(b, ns),
        E::Assign(_n, v, _l, _h) => body_uses_any(v, ns),
        E::AssignTo(p, v, _l, _h) => body_uses_any(p, ns) || body_uses_any(v, ns),
        E::Field(x, _f, _l, _h) => body_uses_any(x, ns),
        E::Index(x, i, _l, _h) => body_uses_any(x, ns) || body_uses_any(i, ns),
        E::Ref(x, _m, _l, _h) => body_uses_any(x, ns),
        E::Not(x, _l, _h) => body_uses_any(x, ns),
        E::Cast(x, _t, _l, _h) => body_uses_any(x, ns),
        E::EnumLit(_t, _v, a, _l, _h) => uses_list(a, ns),
        E::TupLit(a, _l, _h) => uses_list(a, ns),
        E::Int(_v, _l, _h) => false,
        E::Bool(_v, _l, _h) => false,
        E::StrLit(_v, _l, _h) => false,
        E::BreakE(_l, _h) => false,
        E::ContE(_l, _h) => false,
        _ => true,
    }
}
fn uses_list(a: &Vec<E>, ns: &Vec<String>) -> bool {
    let mut k = 0;
    while k < a.len() {
        if body_uses_any(&a[k], ns) {
            return true;
        }
        k = k + 1;
    }
    false
}
fn any_panic(a: &Vec<E>) -> bool {
    let mut k = 0;
    while k < a.len() {
        if body_has_panic(&a[k]) {
            return true;
        }
        k = k + 1;
    }
    false
}
pub fn body_has_io(e: &E) -> bool {
    match e {
        E::Call(n, a, _l, _h) => {
            let io = n.as_str() == "print_any"
                || n.as_str() == "print_dbg"
                || n.as_str() == "print_raw"
                || n.as_str() == "print_int_raw"
                || n.as_str() == "print_nl"
                || n.as_str() == "print"
                || n.as_str() == "print_int"
                || n.as_str() == "panic_exit"
                || n.as_str() == "say";
            if io {
                return true;
            }
            let mut i = 0;
            while i < a.len() {
                if body_has_io(&a[i]) {
                    return true;
                }
                i = i + 1;
            }
            false
        }
        E::Seq(a, b, _l, _h) => body_has_io(a) || body_has_io(b),
        E::Let(_n, v, b, _l, _h) => body_has_io(v) || body_has_io(b),
        E::LetT(_n, _t, v, b, _l, _h) => body_has_io(v) || body_has_io(b),
        E::If(c, t, f, _l, _h) => body_has_io(c) || body_has_io(t) || body_has_io(f),
        E::While(c, b, _l, _h) => body_has_io(c) || body_has_io(b),
        E::Bin(_o, a, b, _l, _h) => body_has_io(a) || body_has_io(b),
        E::MethodCall(r, _m, a, _l, _h) => {
            if body_has_io(r) {
                return true;
            }
            let mut i = 0;
            while i < a.len() {
                if body_has_io(&a[i]) {
                    return true;
                }
                i = i + 1;
            }
            false
        }
        E::Match(s2, arms, _l, _h) => {
            if body_has_io(s2) {
                return true;
            }
            let mut i = 0;
            while i < arms.len() {
                if body_has_io(&arms[i].body) {
                    return true;
                }
                i = i + 1;
            }
            false
        }
        E::RetE(x, _l, _h) => body_has_io(x),
        E::Assign(_n, v, _l, _h) => body_has_io(v),
        _ => false,
    }
}
pub fn split_dollar(s: &str) -> Vec<String> {
    let b = s.as_bytes();
    let mut o: Vec<String> = Vec::new();
    let mut st = 0;
    let mut i = 0;
    while i <= b.len() {
        if i == b.len() || b[i] == 36 {
            o.push(String::from(&s[st..i]));
            st = i + 1;
        }
        i = i + 1;
    }
    o
}
pub fn tk_is(t: &Tk, k: &str) -> bool {
    match t {
        Tk::OpEq(s) => {
            let kb = k.as_bytes();
            kb.len() == s.len() + 1 && kb[kb.len() - 1] == 61 && &k[..s.len()] == s.as_str()
        }
        Tk::Fn => k == "fn",
        Tk::While => k == "while",
        Tk::Mut => k == "mut",
        Tk::Struct => k == "struct",
        Tk::Dot => k == ".",
        Tk::Colon => k == ":",
        Tk::Gt => k == ">",
        Tk::Le => k == "<=",
        Tk::Ge => k == ">=",
        Tk::Ne => k == "!=",
        Tk::AndAnd => k == "&&",
        Tk::OrOr => k == "||",
        Tk::Bang => k == "!",
        Tk::Enum => k == "enum",
        Tk::Impl => k == "impl",
        Tk::Trait => k == "trait",
        Tk::Ret => k == "return",
        Tk::DynT => k == "dyn",
        Tk::Bar => k == "|",
        Tk::DotDot => k == "..",
        Tk::Quest => k == "?",
        Tk::ModT => k == "mod",
        Tk::AsT => k == "as",
        Tk::UnsafeT => k == "unsafe",
        Tk::LoopT => k == "loop",
        Tk::Dollar => k == "$",
        Tk::DeriveCopy => k == "#copy",
        Tk::WhereT => k == "where",
        Tk::TypeT => k == "type",
        Tk::UseT => k == "use",
        Tk::ExternT => k == "extern",
        Tk::CrateT => k == "crate",
        Tk::Caret => k == "^",
        Tk::Shl => k == "<<",
        Tk::Shr => k == ">>",
        Tk::Amp => k == "&",
        Tk::ConstT => k == "const",
        Tk::StaticT => k == "static",
        Tk::BreakT => k == "break",
        Tk::ContT => k == "continue",
        Tk::PlusEq => k == "+=",
        Tk::MinusEq => k == "-=",
        Tk::StarEq => k == "*=",
        Tk::SlashEq => k == "/=",
        Tk::InT => k == "in",
        Tk::StrL(_x) => k == "str",
        Tk::ForT => k == "for",
        Tk::Match => k == "match",
        Tk::FatArrow => k == "=>",
        Tk::PathSep => k == "::",
        Tk::LBrack => k == "[",
        Tk::RBrack => k == "]",
        Tk::Comma => k == ",",
        Tk::Plus => k == "+",
        Tk::Minus => k == "-",
        Tk::Star => k == "*",
        Tk::Slash => k == "/",
        Tk::Percent => k == "%",
        Tk::Lt => k == "<",
        Tk::EqEq => k == "==",
        Tk::LParen => k == "(",
        Tk::RParen => k == ")",
        Tk::LBrace => k == "{",
        Tk::RBrace => k == "}",
        Tk::Let => k == "let",
        Tk::If => k == "if",
        Tk::Else => k == "else",
        Tk::Eq => k == "=",
        Tk::Semi => k == ";",
        _ => false,
    }
}

// MEM-LIT: per-variant token tests. Under SEM0 every string-literal evaluation heap-allocates (80 B, never freed), so
// `tkv_fn(t)` cost two allocations per call on the parser hot path; these compare the variant only.
pub fn tkv_fn(t: &Tk) -> bool {
    match t {
        Tk::Fn => true,
        _ => false,
    }
}
pub fn tkv_while(t: &Tk) -> bool {
    match t {
        Tk::While => true,
        _ => false,
    }
}
pub fn tkv_mut(t: &Tk) -> bool {
    match t {
        Tk::Mut => true,
        _ => false,
    }
}
pub fn tkv_struct(t: &Tk) -> bool {
    match t {
        Tk::Struct => true,
        _ => false,
    }
}
pub fn tkv_dot(t: &Tk) -> bool {
    match t {
        Tk::Dot => true,
        _ => false,
    }
}
pub fn tkv_colon(t: &Tk) -> bool {
    match t {
        Tk::Colon => true,
        _ => false,
    }
}
pub fn tkv_gt(t: &Tk) -> bool {
    match t {
        Tk::Gt => true,
        _ => false,
    }
}
pub fn tkv_le(t: &Tk) -> bool {
    match t {
        Tk::Le => true,
        _ => false,
    }
}
pub fn tkv_ge(t: &Tk) -> bool {
    match t {
        Tk::Ge => true,
        _ => false,
    }
}
pub fn tkv_ne(t: &Tk) -> bool {
    match t {
        Tk::Ne => true,
        _ => false,
    }
}
pub fn tkv_and_and(t: &Tk) -> bool {
    match t {
        Tk::AndAnd => true,
        _ => false,
    }
}
pub fn tkv_or_or(t: &Tk) -> bool {
    match t {
        Tk::OrOr => true,
        _ => false,
    }
}
pub fn tkv_bang(t: &Tk) -> bool {
    match t {
        Tk::Bang => true,
        _ => false,
    }
}
pub fn tkv_enum(t: &Tk) -> bool {
    match t {
        Tk::Enum => true,
        _ => false,
    }
}
pub fn tkv_impl(t: &Tk) -> bool {
    match t {
        Tk::Impl => true,
        _ => false,
    }
}
pub fn tkv_trait(t: &Tk) -> bool {
    match t {
        Tk::Trait => true,
        _ => false,
    }
}
pub fn tkv_ret(t: &Tk) -> bool {
    match t {
        Tk::Ret => true,
        _ => false,
    }
}
pub fn tkv_dyn_t(t: &Tk) -> bool {
    match t {
        Tk::DynT => true,
        _ => false,
    }
}
pub fn tkv_bar(t: &Tk) -> bool {
    match t {
        Tk::Bar => true,
        _ => false,
    }
}
pub fn tkv_dot_dot(t: &Tk) -> bool {
    match t {
        Tk::DotDot => true,
        _ => false,
    }
}
pub fn tkv_quest(t: &Tk) -> bool {
    match t {
        Tk::Quest => true,
        _ => false,
    }
}
pub fn tkv_mod_t(t: &Tk) -> bool {
    match t {
        Tk::ModT => true,
        _ => false,
    }
}
pub fn tkv_as_t(t: &Tk) -> bool {
    match t {
        Tk::AsT => true,
        _ => false,
    }
}
pub fn tkv_unsafe_t(t: &Tk) -> bool {
    match t {
        Tk::UnsafeT => true,
        _ => false,
    }
}
pub fn tkv_loop_t(t: &Tk) -> bool {
    match t {
        Tk::LoopT => true,
        _ => false,
    }
}
pub fn tkv_dollar(t: &Tk) -> bool {
    match t {
        Tk::Dollar => true,
        _ => false,
    }
}
pub fn tkv_derive_copy(t: &Tk) -> bool {
    match t {
        Tk::DeriveCopy => true,
        _ => false,
    }
}
pub fn tkv_where_t(t: &Tk) -> bool {
    match t {
        Tk::WhereT => true,
        _ => false,
    }
}
pub fn tkv_type_t(t: &Tk) -> bool {
    match t {
        Tk::TypeT => true,
        _ => false,
    }
}
pub fn tkv_use_t(t: &Tk) -> bool {
    match t {
        Tk::UseT => true,
        _ => false,
    }
}
pub fn tkv_extern_t(t: &Tk) -> bool {
    match t {
        Tk::ExternT => true,
        _ => false,
    }
}
pub fn tkv_crate_t(t: &Tk) -> bool {
    match t {
        Tk::CrateT => true,
        _ => false,
    }
}
pub fn tkv_caret(t: &Tk) -> bool {
    match t {
        Tk::Caret => true,
        _ => false,
    }
}
pub fn tkv_shl(t: &Tk) -> bool {
    match t {
        Tk::Shl => true,
        _ => false,
    }
}
pub fn tkv_shr(t: &Tk) -> bool {
    match t {
        Tk::Shr => true,
        _ => false,
    }
}
pub fn tkv_amp(t: &Tk) -> bool {
    match t {
        Tk::Amp => true,
        _ => false,
    }
}
pub fn tkv_const_t(t: &Tk) -> bool {
    match t {
        Tk::ConstT => true,
        _ => false,
    }
}
pub fn tkv_static_t(t: &Tk) -> bool {
    match t {
        Tk::StaticT => true,
        _ => false,
    }
}
pub fn tkv_break_t(t: &Tk) -> bool {
    match t {
        Tk::BreakT => true,
        _ => false,
    }
}
pub fn tkv_cont_t(t: &Tk) -> bool {
    match t {
        Tk::ContT => true,
        _ => false,
    }
}
pub fn tkv_plus_eq(t: &Tk) -> bool {
    match t {
        Tk::PlusEq => true,
        _ => false,
    }
}
pub fn tkv_minus_eq(t: &Tk) -> bool {
    match t {
        Tk::MinusEq => true,
        _ => false,
    }
}
pub fn tkv_star_eq(t: &Tk) -> bool {
    match t {
        Tk::StarEq => true,
        _ => false,
    }
}
pub fn tkv_slash_eq(t: &Tk) -> bool {
    match t {
        Tk::SlashEq => true,
        _ => false,
    }
}
pub fn tkv_in_t(t: &Tk) -> bool {
    match t {
        Tk::InT => true,
        _ => false,
    }
}
pub fn tkv_for_t(t: &Tk) -> bool {
    match t {
        Tk::ForT => true,
        _ => false,
    }
}
pub fn tkv_match(t: &Tk) -> bool {
    match t {
        Tk::Match => true,
        _ => false,
    }
}
pub fn tkv_fat_arrow(t: &Tk) -> bool {
    match t {
        Tk::FatArrow => true,
        _ => false,
    }
}
pub fn tkv_path_sep(t: &Tk) -> bool {
    match t {
        Tk::PathSep => true,
        _ => false,
    }
}
pub fn tkv_l_brack(t: &Tk) -> bool {
    match t {
        Tk::LBrack => true,
        _ => false,
    }
}
pub fn tkv_r_brack(t: &Tk) -> bool {
    match t {
        Tk::RBrack => true,
        _ => false,
    }
}
pub fn tkv_comma(t: &Tk) -> bool {
    match t {
        Tk::Comma => true,
        _ => false,
    }
}
pub fn tkv_plus(t: &Tk) -> bool {
    match t {
        Tk::Plus => true,
        _ => false,
    }
}
pub fn tkv_minus(t: &Tk) -> bool {
    match t {
        Tk::Minus => true,
        _ => false,
    }
}
pub fn tkv_star(t: &Tk) -> bool {
    match t {
        Tk::Star => true,
        _ => false,
    }
}
pub fn tkv_slash(t: &Tk) -> bool {
    match t {
        Tk::Slash => true,
        _ => false,
    }
}
pub fn tkv_percent(t: &Tk) -> bool {
    match t {
        Tk::Percent => true,
        _ => false,
    }
}
pub fn tkv_lt(t: &Tk) -> bool {
    match t {
        Tk::Lt => true,
        _ => false,
    }
}
pub fn tkv_eq_eq(t: &Tk) -> bool {
    match t {
        Tk::EqEq => true,
        _ => false,
    }
}
pub fn tkv_l_paren(t: &Tk) -> bool {
    match t {
        Tk::LParen => true,
        _ => false,
    }
}
pub fn tkv_r_paren(t: &Tk) -> bool {
    match t {
        Tk::RParen => true,
        _ => false,
    }
}
pub fn tkv_l_brace(t: &Tk) -> bool {
    match t {
        Tk::LBrace => true,
        _ => false,
    }
}
pub fn tkv_r_brace(t: &Tk) -> bool {
    match t {
        Tk::RBrace => true,
        _ => false,
    }
}
pub fn tkv_let(t: &Tk) -> bool {
    match t {
        Tk::Let => true,
        _ => false,
    }
}
pub fn tkv_if(t: &Tk) -> bool {
    match t {
        Tk::If => true,
        _ => false,
    }
}
pub fn tkv_else(t: &Tk) -> bool {
    match t {
        Tk::Else => true,
        _ => false,
    }
}
pub fn tkv_eq(t: &Tk) -> bool {
    match t {
        Tk::Eq => true,
        _ => false,
    }
}
pub fn tkv_semi(t: &Tk) -> bool {
    match t {
        Tk::Semi => true,
        _ => false,
    }
}
// `Vec::new()` / `String::new()` / `i32::MAX` -- the std names whose meaning our value model already
// has. A name the program itself declares always wins (a user `struct Vec` keeps its own constructor).
pub fn std_path(segs: &Vec<String>, user: bool) -> String {
    if user || segs.len() != 2 {
        return String::from("");
    }
    let a = segs[0].as_str();
    let b = segs[1].as_str();
    if a == "Vec" && (b == "new" || b == "with_capacity") {
        return String::from("vec_new");
    }
    if a == "String" && (b == "new" || b == "with_capacity") {
        return String::from("$emptystr");
    }
    if a == "String" && (b == "from" || b == "from_str") {
        return String::from("$id1");
    }
    // CHAR-FROM-U32 (R2a P4): Some(c) for a Unicode scalar value, else None -- the prelude's __char_from_u32, whose declared
    // `-> Option<char>` types the payload (an untyped Option printed the char as its code point)
    if a == "char" && b == "from_u32" {
        return String::from("__char_from_u32");
    }
    // C-MEMPLACE: mem::swap and mem::replace are PLACE operations, and places are now expressible, so
    // they desugar at the call site instead of being reported as unknown functions. mem::take is NOT here:
    // it needs the type's default value, which the parser does not know.
    // C-CELL: a Cell is interior mutability over ONE value, and a one-slot vector is exactly that in our
    // value model -- the handle is shared, so a write through it is visible to every holder, which is the
    // whole point of a Cell. new/get/set is the entire surface this needs.
    // C-SIZEOF: size_of is a fact about the TARGET -- the same number rustc answers -- and not a claim
    // about our value model, so it can be answered exactly for the types that have a fixed width.
    if a == "ptr" && (b == "null" || b == "null_mut") {
        return String::from("$ptrnull");
    } // R4b PTR: a null pointer -- 0, recorded so is_null / eq answer statically
    if a == "ptr" && (b == "from_ref" || b == "from_mut") {
        return String::from("$ptrfrom");
    }
    if a == "ptr" && (b == "eq" || b == "addr_eq") {
        return String::from("$ptreq");
    }
    if a == "iter" && b == "empty" {
        return String::from("vec_new");
    } // R4d: iterator sources are eager Vecs here -- an empty iterator is an empty Vec
    if a == "iter" && (b == "once" || b == "repeat") {
        let mut s = String::from("$iter");
        s.push_str(b);
        return s;
    } // R4d: once = a one-element Vec; repeat is ENDLESS (only `.take(n)` of it is materialised)
    if a == "IntoIterator" && b == "into_iter" {
        return String::from("$intoiter");
    }
    if a == "thread" && b == "spawn" {
        return String::from("$spawn");
    } // R4f SPAWN: run the closure NOW (a legal schedule); the handle is Ok(result)
    if a == "mem" && b == "size_of_val" {
        return String::from("$sizeofval");
    } // R4a LAYOUT: the size of a VALUE's type (an array's length is known at run time)
    if a == "mem" && b == "transmute" {
        return String::from("$transmute");
    } // R4a XMUTE: bit reinterpretation, target type from the turbofish or the let annotation (see the call site)
    if a == "mem" && (b == "zeroed" || b == "uninitialized") {
        return String::from("$zerodflt");
    } // R4a: an all-zero value of the annotated type IS the zero value (the $zeroty$T path)
    if a == "mem" && (b == "size_of" || b == "align_of") {
        return String::from("$sizeof");
    }
    if (a == "Mutex" || a == "RwLock") && b == "new" {
        return String::from("$cellnew");
    } // R4c MUTEX: a Mutex/RwLock is the one-slot cell (lock = borrow_mut, see lock_guard)
    if (a == "Cell" || a == "RefCell") && b == "new" {
        return String::from("$cellnew");
    } // R4 REFCELL: a RefCell is the same one-slot cell as a Cell (borrow / borrow_mut read it; see olower)
      // RC-IDENTITY (R4c): Rc/Arc are an identity handle over their referent, counted in a runtime side table
    if (a == "Rc" || a == "Arc") && b == "new" {
        return String::from("$rcnew");
    }
    if (a == "Rc" || a == "Arc") && b == "clone" {
        return String::from("$rcclone");
    }
    if (a == "Rc" || a == "Arc") && b == "strong_count" {
        return String::from("$rccount");
    }
    if a == "mem" && b == "swap" {
        return String::from("$memswap");
    }
    if a == "mem" && b == "replace" {
        return String::from("$memrepl");
    }
    if a == "mem" && b == "take" {
        return String::from("$memtake");
    }
    // C-DEFAULTTY: `T::default()` is the zero of a NAMED type, so the name rides in the marker and
    // lowering answers it -- the same question mem::take asks, with the type given instead of inferred.
    // the NAME is normalized the same way a declared type is (`String` is `str` in our type names), so
    // the zero rule sees one spelling per type rather than two.
    if b == "default" && a != "Default" {
        let mut m = String::from("$zeroty$");
        m.push_str(&norm_tyname(a));
        return m;
    }
    if a == "Default" && b == "default" {
        return String::from("$zerodflt");
    }
    if a == "cmp" && b == "max" {
        return String::from("imax");
    } // P-CMP-FREE: std::cmp::max/min are the free-fn form of .max()/.min()
    if a == "cmp" && b == "min" {
        return String::from("imin");
    }
    if a == "process" && b == "exit" {
        return String::from("proc_exit");
    }
    if a == "fs" && (b == "read_to_string" || b == "read") {
        return String::from("freadok");
    }
    if a == "env" && b == "args" {
        return String::from("eargs");
    } // C-ARGV   // C-READFILE: a Result, like rustc, via a prelude wrapper over the raw primitive
    if oast::is_int_width(a) && (b == "MAX" || b == "MIN") {
        return String::from("$int");
    }
    String::from("")
}
pub fn std_int_bound(segs: &Vec<String>) -> i64 {
    let a = segs[0].as_str();
    let mx = segs[1].as_str() == "MAX";
    let bits: i64 = if a == "i8" || a == "u8" {
        8
    } else if a == "i16" || a == "u16" {
        16
    } else if a == "i32" || a == "u32" {
        32
    } else {
        64
    };
    let uns = a.len() > 0 && &a[0..1] == "u";
    if uns {
        if mx {
            if bits >= 64 {
                return 0 - 1;
            }
            let mut v: i64 = 1;
            let mut i = 0;
            while i < bits {
                v = v * 2;
                i = i + 1;
            }
            return v - 1;
        }
        return 0;
    }
    if bits >= 64 {
        if mx {
            return 9223372036854775807;
        }
        return 0 - 9223372036854775807 - 1;
    }
    let mut v: i64 = 1;
    let mut i = 0;
    while i < bits - 1 {
        v = v * 2;
        i = i + 1;
    }
    if mx {
        v - 1
    } else {
        0 - v
    }
}
// A SLICE type `[T]` names no representation of its own: an array literal and a vector both reach it.
// In a PARAMETER or return position the value is a vector (`&v`), so the slice reads as `Vec$T`; in a
// `let`/`const` annotation the INITIALIZER's own shape is the truth, so the annotation is dropped.
pub fn sl_param(t: &str) -> String {
    if t.len() >= 5 && &t[0..5] == "slice" {
        let mut r = String::from("Vec");
        if t.len() > 5 {
            r.push_str(&t[5..]);
        }
        return r;
    }
    decell(t)
}
pub fn sl_const(t: &str) -> String {
    if t.len() >= 5 && &t[0..5] == "slice" {
        return String::from("arr");
    }
    String::from(t)
}
// `String` is spelled `str` in our type names (parse_tyname says so for a declared type); the zero rule
// and ety_of must see the SAME spelling whichever route the name arrived by, so the marker normalizes.
// C-FMTPREC: ONE reader for a `{..}` specifier, because the format walker exists twice (format!/write!
// and println!/print!) and a second copy of this byte matching is how the two would drift apart.
// Answers (kind, bytes): kind 0 = `{}`, 1 = `{:?}`, 2+N = `{:.N}`; kind -1 = not one we handle.
// FMT-PAD: a WIDTH specifier `{:[fill][<^>][0][width][.prec][?]}` (`{:>5}`, `{:-^9}`, `{:05}`, `{:8.3}`); answers
// (bytes, width, align, fill, prec, dbg) -- bytes 0 when it is not one (the plain forms stay with fmt_spec). align: 0 the
// type's default (numbers right, text left), 1 left, 2 right, 3 center, 4 zero-pad (sign-aware); prec -1 = none.
pub fn fmt_pad_spec(fmt: &str, k: usize) -> (i32, i32, i32, String, i32, bool) {
    let b = fmt.as_bytes();
    let none = (0, 0, 0, String::from(" "), 0 - 1, false);
    if k + 2 >= b.len() || b[k] != 123 || b[k + 1] != 58 {
        return none;
    }
    let mut j = k + 2;
    let mut al = 0;
    let mut fill = String::from(" ");
    let isal = |c: u8| c == 60 || c == 62 || c == 94;
    if j + 1 < b.len() && isal(b[j + 1]) && b[j] != 125 {
        fill = String::from(&fmt[j..(j + 1)]);
        al = if b[j + 1] == 60 {
            1
        } else if b[j + 1] == 62 {
            2
        } else {
            3
        };
        j = j + 2;
    } else if j < b.len() && isal(b[j]) {
        al = if b[j] == 60 {
            1
        } else if b[j] == 62 {
            2
        } else {
            3
        };
        j = j + 1;
    }
    if j < b.len() && b[j] == 48 && al == 0 {
        al = 4;
        fill = String::from("0");
        j = j + 1;
    }
    let mut w = 0;
    let mut anyw = false;
    while j < b.len() && b[j] >= 48 && b[j] <= 57 {
        w = w * 10 + ((b[j] - 48) as i32);
        anyw = true;
        j = j + 1;
    }
    let mut prec = 0 - 1;
    if j < b.len() && b[j] == 46 {
        j = j + 1;
        let mut p = 0;
        let mut anyp = false;
        while j < b.len() && b[j] >= 48 && b[j] <= 57 {
            p = p * 10 + ((b[j] - 48) as i32);
            anyp = true;
            j = j + 1;
        }
        if !anyp {
            return none;
        }
        prec = p;
    }
    let mut dbg = false;
    if j < b.len() && b[j] == 63 {
        dbg = true;
        j = j + 1;
    }
    if !anyw || j >= b.len() || b[j] != 125 {
        return none;
    }
    ((j + 1 - k) as i32, w, al, fill, prec, dbg)
}
pub fn fmt_spec(fmt: &str, k: usize) -> (i32, i32) {
    let b = fmt.as_bytes();
    if k >= b.len() {
        return (0 - 1, 0);
    }
    if b[k] != 123 {
        return (0 - 1, 0);
    }
    if k + 1 < b.len() && b[k + 1] == 125 {
        return (0, 2);
    }
    if k + 3 < b.len() && b[k + 1] == 58 && b[k + 2] == 63 && b[k + 3] == 125 {
        return (1, 4);
    }
    if k + 3 < b.len() && b[k + 1] == 58 && b[k + 2] == 101 && b[k + 3] == 125 {
        return (2 + 1000000, 4);
    } // FMT-SCI: `{:e}` (LowerExp) -- a precision sentinel the walkers send to f64_sci
    if k + 3 < b.len() && b[k + 1] == 58 && b[k + 2] == 46 {
        let mut j = k + 3;
        let mut n = 0;
        let mut any = false;
        while j < b.len() {
            if b[j] >= 48 && b[j] <= 57 {
                n = n * 10 + ((b[j] - 48) as i32);
                any = true;
                j = j + 1;
            } else {
                j = j + b.len() * 0;
                break;
            }
        }
        if any && j < b.len() && b[j] == 125 {
            return (2 + n, ((j + 1 - k) as i32));
        }
    }
    (0 - 1, 0)
}
// C-SIZEOF: the fixed widths, as rustc reports them; 0 means "not a type with a width we can state".
pub fn ty_width(t: &str) -> i32 {
    if t == "u8" || t == "i8" || t == "bool" {
        return 1;
    }
    if t == "u16" || t == "i16" {
        return 2;
    }
    if t == "u32" || t == "i32" || t == "f32" || t == "char" {
        return 4;
    }
    if t == "u64" || t == "i64" || t == "f64" || t == "usize" || t == "isize" || t == "int" {
        return 8;
    }
    0
}
pub fn norm_tyname(a: &str) -> String {
    if a == "String" {
        return String::from("str");
    }
    String::from(a)
}
// C-CELL: a `Cell<T>` IS a one-slot vector of T in our value model, so the type name says so and every
// layer below -- the builtin-method table, ety_of, the tag scheme -- needs to learn nothing new.
pub fn decell(t: &str) -> String {
    let b = t.as_bytes();
    let pb = "Cell$".as_bytes();
    if b.len() > pb.len() {
        let mut i = 0;
        let mut hit = true;
        while i < pb.len() {
            if b[i] != pb[i] {
                hit = false;
            }
            i = i + 1;
        }
        if hit {
            let mut r = String::from("Vec$");
            r.push_str(&t[pb.len()..]);
            return r;
        }
    }
    String::from(t)
}
pub fn sl_val(t: &str) -> String {
    if t.len() >= 5 && &t[0..5] == "slice" {
        return String::from("");
    }
    decell(t)
}
impl P {
    // V1b BOX-IDENTITY: does the program observe allocation identity -- `Box::new`, a `.clone()`, and an `as *const` cast?
    // Only then is a Box a real cell (its address is the cell handle); elsewhere a Box stays transparent.
    fn box_identity(&mut self) -> bool {
        if self.boxid < 0 {
            let mut has_box = false;
            let mut has_clone = false;
            let mut has_raw = false;
            let mut q = 0;
            while q + 2 < self.t.len() {
                if word_is(&self.t[q], "Box")
                    && tkv_path_sep(&self.t[q + 1].k)
                    && word_is(&self.t[q + 2], "new")
                {
                    has_box = true;
                }
                if tkv_dot(&self.t[q].k) && word_is(&self.t[q + 1], "clone") {
                    has_clone = true;
                }
                if tkv_as_t(&self.t[q].k)
                    && tkv_star(&self.t[q + 1].k)
                    && tkv_const_t(&self.t[q + 2].k)
                {
                    has_raw = true;
                }
                q = q + 1;
            }
            self.boxid = 0;
            if has_box && has_clone && has_raw {
                self.boxid = 1;
            }
        }
        self.boxid == 1
    }
    fn is_boxvar(&self, n: &str) -> bool {
        let mut i = 0;
        while i < self.boxvars.len() {
            if self.boxvars[i].as_str() == n {
                return true;
            }
            i = i + 1;
        }
        false
    }
    // V1b: record a let bound to a Box cell; a `.clone()` of a Box cell becomes a fresh `$boxnew` of its value
    // V1b: record a let bound to a Box cell; returns the source name when the initializer is a `.clone()` of a Box cell
    // (box_rewrap then makes it a fresh `$boxnew`: a Box clone allocates). Borrows the initializer: a SEM0 build crashed
    // on a Vec push in a method that also took an E by value.
    fn box_note(&mut self, nm: &str, v: &E) -> String {
        if self.boxid != 1 {
            return String::new();
        }
        if box_is_new(v) {
            self.boxvars.push(String::from(nm));
            return String::new();
        }
        // a MOVE keeps the allocation: `let c = a` of a Box cell is the same cell
        let mn = box_move_of(v);
        if mn.len() > 0 && self.is_boxvar(mn.as_str()) {
            self.boxvars.push(String::from(nm));
            return String::new();
        }
        let cn = box_clone_of(v);
        if cn.len() > 0 && self.is_boxvar(cn.as_str()) {
            self.boxvars.push(String::from(nm));
            return cn;
        }
        String::new()
    }
    // PAT-NEST: a tuple pattern, recursively. Called just past its `(`; consumes through the matching `)`.
    // Each binding name goes to ns with its PATH of tuple indices in ps, dotted (`(a, (b, c))` -> a "0",
    // b "1.0", c "1.1"); `_` binds nothing but keeps its slot.
    fn tup_pat(&mut self, pre: &str, ns: &mut Vec<String>, ps: &mut Vec<String>) {
        let mut k: u32 = 0;
        loop {
            if self.at_end() {
                break;
            }
            if self.is_tok(")") {
                let _ = self.adv();
                break;
            }
            if self.is_tok(",") {
                let _ = self.adv();
                k = k + 1;
                continue;
            }
            if self.is_word("ref") && self.peek_is(1, "mut") && self.perr == 0 {
                // REFMUT-TUP (W7): `(ref mut a, _)` binds a tuple element BY VALUE in our model, so `*a = v` never reached the
                // tuple (two-phase-borrows printed the old value). Refused until tuple elements are places.
                self.perr = self.lo() + 1;
                self.perrexp = String::from("a tuple-pattern binding other than `ref mut` (writes through it are not modelled)");
                self.perrgot = String::from("ref mut");
            }
            if self.is_tok("mut") || self.is_word("ref") {
                let _ = self.adv();
                continue;
            }
            if self.is_tok("..") {
                // TUP-REST-LET: a `..` with elements AFTER it shifted every later index by the rest's width, which only
                // happened to be right when the rest covered ONE element (`let (p, .., r) = (1,2,3,4)` gave r=3, rustc 4).
                // The tuple's arity is unknown here: refuse unless the rest is the group's last element.
                let _ = self.adv();
                if self.is_tok(",") && self.perr == 0 {
                    let mut j = self.i + 1;
                    while j < self.t.len() && tkv_comma(&self.t[j].k) {
                        j = j + 1;
                    }
                    if j < self.t.len() && !tkv_r_paren(&self.t[j].k) {
                        self.perr = self.t[self.i].lo + 1;
                        self.perrexp = String::from(
                            "a `..` only at the END of a tuple pattern (a leading or middle rest is not supported yet)",
                        );
                        self.perrgot = String::from("..");
                    }
                }
                continue;
            }
            let mut p = String::from(pre);
            if p.len() > 0 {
                p.push_str(".");
            }
            p.push_str(&oast::u32s2(k));
            if self.is_tok("(") {
                let _ = self.adv();
                self.tup_pat(p.as_str(), ns, ps);
                continue;
            }
            let w = tok_word(&self.t[self.i]);
            let _ = self.adv();
            if w.len() > 0 {
                ns.push(w);
                ps.push(p);
            }
        }
    }
    // FN-ITEM-RET: the declared return type of the fn a closure body DIRECTLY calls ("" otherwise) -- the FN-ITEM-VAL
    // wrapper `|__fv0| pick(__fv0)` had no return type, so `let f = pick; f("ab")` printed a String pointer as an int
    fn fret_of_call(&self, body: &E) -> String {
        if let E::Call(cn, _a, _l, _h) = body {
            let mut pre = String::from(cn.as_str());
            pre.push('=');
            let mut q = 0;
            while q < self.fretn.len() {
                if oast::str_starts(self.fretn[q].as_str(), pre.as_str()) {
                    return String::from(&self.fretn[q][pre.len()..]);
                }
                q = q + 1;
            }
        }
        String::from("")
    }
    fn variant_owner(&self, n: &str) -> String {
        let mut i = 0;
        while i < self.vnames.len() {
            if self.vnames[i].as_str() == n {
                return String::from(self.vowner[i].as_str());
            }
            i = i + 1;
        }
        String::from("")
    }
    fn enames_has(&self, n: &str) -> bool {
        let mut i = 0;
        while i < self.enames.len() {
            if self.enames[i].as_str() == n {
                return true;
            }
            i = i + 1;
        }
        false
    }
    fn mknown(&self, n: &str) -> bool {
        let mut i = 0;
        while i < self.mods.len() {
            if self.mods[i].as_str() == n {
                return true;
            }
            i = i + 1;
        }
        false
    }
    // an item inside `mod M` is named `M$item`; references use the qualified path `M::item`
    // USE-REL: is `p` an item, or a module that holds one (some fn/struct/enum named `p$..`)?
    fn prefix_known(&self, p: &str) -> bool {
        if self.fknown(p) || self.sknown(p) || self.enames_has(p) {
            return true;
        }
        let mut pre = String::from(p);
        pre.push('$');
        let mut i = 0;
        while i < self.fnames.len() {
            if oast::str_starts(self.fnames[i].as_str(), pre.as_str()) {
                return true;
            }
            i = i + 1;
        }
        let mut j = 0;
        while j < self.snames.len() {
            if oast::str_starts(self.snames[j].as_str(), pre.as_str()) {
                return true;
            }
            j = j + 1;
        }
        false
    }
    fn qual(&self, n: &str) -> String {
        if self.mpfx.len() == 0 {
            return String::from(n);
        }
        let mut r = String::from(self.mpfx.as_str());
        r.push_str(n);
        r
    }
    fn sfields_of(&self, n: &str) -> String {
        let mut i = 0;
        while i < self.snames.len() {
            if self.snames[i].as_str() == n {
                if i < self.sfnames.len() {
                    return String::from(self.sfnames[i].as_str());
                }
                return String::from("");
            }
            i = i + 1;
        }
        String::from("")
    }
    // a bare type name inside a module means THIS module's item first: structs are registered under both
    // the bare and the qualified name, so asking bare first picks the unqualified one and the emitted name
    // stops matching the definition
    fn sres(&self, n: &str) -> String {
        let r = self.tyres(n);
        if self.sknown(&r) {
            return r;
        }
        String::from(n)
    }
    fn sknown_here(&self, n: &str) -> bool {
        let r = self.tyres(n);
        self.sknown(&r)
    }
    // OPT-TAKE: does the program define ANY fn of this name (`fn take` in the token stream -- a method or a free fn)?
    fn user_meth_named(&self, m: &str) -> bool {
        let mut i = 0;
        while i + 1 < self.t.len() {
            if tkv_fn(&self.t[i].k) && word_is(&self.t[i + 1], &*m) {
                return true;
            }
            i = i + 1;
        }
        false
    }
    fn sknown(&self, n: &str) -> bool {
        let mut i = 0;
        while i < self.snames.len() {
            if self.snames[i].as_str() == n {
                return true;
            }
            i = i + 1;
        }
        false
    }
    fn at_end(&self) -> bool {
        self.i + 1 >= self.t.len()
    }
    // CONST-PAT-TY (W7): may const `n` stand as an equality pattern? Only a primitive-typed const (ints, bool, char, str, f64)
    fn const_pat_ok(&mut self, n: &str) -> bool {
        let mut pre = String::from(n);
        pre.push('=');
        let mut i = 0;
        while i < self.ctys.len() {
            if oast::str_starts(self.ctys[i].as_str(), pre.as_str()) {
                let t = String::from(&self.ctys[i][pre.len()..]);
                let t = t.as_str();
                if t == "!static" || t == "*raw" || t.len() == 0 {
                    return false;
                }
                if !(t == "int"
                    || t == "bool"
                    || t == "char"
                    || t == "str"
                    || t == "f64"
                    || oast::is_int_width(t))
                {
                    // CONST-PAT-TY widening: a struct / array / slice const is a pattern when its types are structural
                    // (derive(PartialEq)) -- checked once every item is read (cpat_check)
                    self.cpatuse.push(String::from(n));
                }
                return true;
            }
            i = i + 1;
        }
        false
    }
    fn cknown(&self, n: &str) -> bool {
        let mut i = 0;
        while i < self.consts.len() {
            if self.consts[i].as_str() == n {
                return true;
            }
            i = i + 1;
        }
        false
    }
    fn expr_no_seq(&mut self) -> E {
        self.cmp_e()
    }
    // A type-level `>` may arrive glued to the next glyph, because the lexer munches maximally and is right to:
    // `Vec<i64>=` lexes as `Vec` `<` `i64` `>=`. rustc splits these in the PARSER, not the lexer (the same
    // glyphs stay operators in expression position -- probed), so the split belongs here too. It is local: take
    // one `>` and leave the remainder in place, which needs no depth counter and so handles any nesting.
    fn at_gt(&self) -> bool {
        if self.i >= self.t.len() {
            return false;
        }
        match &self.t[self.i].k {
            Tk::Gt => true,
            Tk::Ge => true,
            Tk::Shr => true,
            Tk::OpEq(s) => s.as_str() == ">>", // GT2-EQ (R2a P5): `Vec<Box<T>>=v` -- `>>=` closes two generics, then `=`
            _ => false,
        }
    }
    fn eat_gt(&mut self) -> bool {
        if self.i >= self.t.len() {
            return false;
        }
        let k = self.t[self.i].k.clone();
        match k {
            Tk::Gt => {
                let _ = self.adv();
                true
            }
            Tk::Ge => {
                self.t[self.i].k = Tk::Eq;
                self.t[self.i].lo = self.t[self.i].lo + 1;
                true
            }
            Tk::Shr => {
                self.t[self.i].k = Tk::Gt;
                self.t[self.i].lo = self.t[self.i].lo + 1;
                true
            }
            Tk::OpEq(s) if s.as_str() == ">>" => {
                self.t[self.i].k = Tk::Ge; // `>>=` minus one `>` is `>=` (which a second eat_gt splits into `=`)
                self.t[self.i].lo = self.t[self.i].lo + 1;
                true
            }
            _ => false,
        }
    }
    // NOTE for the next rung: the bare-name branch below still has its OWN copy of this reader. Pointing
    // it here was attempted TWICE -- once by extracting its body into a method, once by replacing its span
    // with a delegating call -- and both times the file broke (a 3MB binary, every gate program failing),
    // because that branch's text runs into the structure of the enclosing match. It needs a careful manual
    // edit, not a span replacement. One rule in two places is the defect class this very rung fixed, so it
    // is worth doing properly.
    // read a struct literal's body: named fields are collected and emitted in DECLARATION order,
    // positional ones are taken as written. `{` is the current token.
    // VIS-FIELD (R2a P6, rustc E0451): a struct literal naming a PRIVATE field outside the struct's module (and its descendants);
    // `self.i` is at the literal's `{`. Real Rust only (the .src dialect has no `pub`).
    fn vis_field_check(&mut self, s: &str, lo: u32) {
        if !self.hasmain || self.privflds.len() == 0 {
            return;
        }
        let sb = s.as_bytes();
        let mut cut = 0;
        let mut q = 0;
        while q < sb.len() {
            if sb[q] == 36 {
                cut = q + 1;
            }
            q = q + 1;
        }
        let mb = self.mpfx.as_bytes();
        if mb.len() >= cut && &mb[..cut] == &sb[..cut] {
            return; // inside the struct's module
        }
        let mut d = 0;
        let mut j = self.i;
        while j < self.t.len() {
            if tok_opens(&self.t[j]) {
                d = d + 1;
            } else if tok_closes(&self.t[j]) {
                d = d - 1;
                if d == 0 {
                    return;
                }
            } else if d == 1 {
                let w = tok_word(&self.t[j]);
                let prev_ok =
                    j > 0 && (tkv_l_brace(&self.t[j - 1].k) || tkv_comma(&self.t[j - 1].k));
                if w.len() > 0 && prev_ok {
                    let mut pk = String::from(s);
                    pk.push('|');
                    pk.push_str(w.as_str());
                    if oast::in_list(&self.privflds, pk.as_str()) && self.perr == 0 {
                        self.perr = lo + 1;
                        self.perrexp = String::from(
                            "a public field (this one is private to its module: E0451)",
                        );
                        self.perrgot = String::from(w.as_str());
                        return;
                    }
                }
            }
            j = j + 1;
        }
    }
    fn slit_named(&mut self, s: &str, lo: u32, hi: u32) -> E {
        self.vis_field_check(s, lo);
        let _ = self.adv();
        let mut fv: Vec<E> = Vec::new();
        let mut sbname = String::from("");
        let mut sbval: Option<E> = None; // SLIT-BASE-TMP
        let mut sfhoist_n: Vec<String> = Vec::new(); // SLIT-ORDER: written field values hoisted ahead of the base
        let mut sbwritten: Vec<String> = Vec::new(); // SLIT-BASE-DROP: the field names the literal wrote (overriding the base)
        let mut has_var_base = false; // SLIT-BASE-DROP: `..b` with b a variable
        let mut var_base = E::Int(0, lo, hi);
        let mut sfhoist_v: Vec<E> = Vec::new();
        let named = {
            // SLIT-ONLY-BASE (W7): `Foo { ..a }` (no written field) is a struct update too; it took the POSITIONAL path, which
            // read `..a` as a range VALUE for the first field (baz printed an address; a non-generic struct was refused)
            let mut r = self.is_tok("..");
            if !r && self.i + 1 < self.t.len() {
                match self.t[self.i].k.clone() {
                    Tk::Ident(x) => {
                        r = tkv_colon(&self.t[self.i + 1].k)
                            || (self.hasmain
                                && (tkv_comma(&self.t[self.i + 1].k)
                                    || tkv_r_brace(&self.t[self.i + 1].k))
                                && oast::in_list(&split_commas(&self.sfields_of(s)), x.as_str()));
                        /* SLIT-SHORT (real Rust only: .src writes POSITIONAL `Pt { a, b }`) */
                        /* SLIT-SHORT: shorthand fields take the named path too */
                    }
                    _ => {}
                }
            }
            r
        };
        if named {
            let mut ns: Vec<String> = Vec::new();
            let mut vs: Vec<E> = Vec::new();
            // STRUCT-UPDATE: `P { x: 5, ..base }` -- every field not written takes base's value (rustc_parse
            // `StructRest::Base`; the base must come last, so it ends the field list)
            let mut base = E::Int(0, lo, hi);
            let mut has_base = false;
            loop {
                if self.is_tok("}") {
                    let _ = self.adv();
                    break;
                }
                if self.at_end() {
                    break;
                }
                if self.is_tok("..") {
                    let _ = self.adv();
                    base = self.cmp_e();
                    has_base = true;
                    if !self.is_tok("}") && self.perr == 0 {
                        // SLIT-BASE-LAST (W7): `..base` must END the literal, with no trailing comma (rustc: "cannot use a comma
                        // after the base struct"); it was skipped, and fields after it were accepted
                        self.perr = self.lo() + 1;
                        self.perrexp = String::from(
                            "`}` after the base struct (it must be last, without a trailing comma)",
                        );
                        self.perrgot = self.tok_text();
                    }
                    if self.is_tok(",") {
                        let _ = self.adv();
                    }
                    continue;
                }
                let fi = self.adv();
                let fnm = match &self.t[fi].k {
                    Tk::Ident(x) => String::from(x.as_str()),
                    _ => String::from("_"),
                };
                let v = if self.is_tok(":") {
                    let _ = self.adv();
                    self.cmp_e()
                } else {
                    E::Var(String::from(fnm.as_str()), lo, hi)
                }; // SLIT-SHORT: `P { x, n: e }` -- a field with no `:` is the variable of that name
                ns.push(fnm);
                vs.push(v);
                if self.is_tok(",") {
                    let _ = self.adv();
                }
            }
            if has_base
                && (match &base {
                    E::Var(_n, _l, _h) => true,
                    _ => false,
                })
            {
                has_var_base = true;
                var_base = clone_simple(&base, lo, hi);
            }
            // SLIT-BASE-TMP: a NON-variable base (`..a.clone()`, `..mk()`) is evaluated ONCE into a fresh local; every unwritten field reads it
            if has_base
                && !(match &base {
                    E::Var(_n, _l, _h) => true,
                    _ => false,
                })
            {
                self.nhyg = self.nhyg + 1;
                let mut tn = String::from("__sbase");
                tn.push_str(&oast::u32s2(self.nhyg));
                let real = base;
                base = E::Var(String::from(tn.as_str()), lo, hi);
                sbval = Some(real);
                sbname = tn;
                // SLIT-ORDER (heavy pass 2): rustc evaluates the WRITTEN fields (in written order) BEFORE the base -- `Foo { n1: {
                // return .. }, ..old() }` returns without ever calling old(). The base temp was evaluated first (drop/issue-21486
                // dropped a base it should never have built). Hoist the written values into temps that precede the base.
                let mut held: Vec<E> = Vec::new();
                while vs.len() > 0 {
                    held.push(vs.pop().unwrap());
                }
                while held.len() > 0 {
                    let v = held.pop().unwrap();
                    self.nhyg = self.nhyg + 1;
                    let mut fname = String::from("__sfv");
                    fname.push_str(&oast::u32s2(self.nhyg));
                    vs.push(E::Var(String::from(fname.as_str()), lo, hi));
                    sfhoist_n.push(fname);
                    sfhoist_v.push(v);
                }
            }
            let order = split_commas(&self.sfields_of(s));
            // R15.4 SLIT-SRC-ORDER: rustc evaluates a literal's fields in WRITTEN order (`S { f1: s.to_string(), f0: s }` borrows
            // s, then moves it). Emitting them in declaration order moved s first (the move checker refused a valid program, and
            // side effects ran out of order). When the written order differs, hoist every written value into a temp, in order.
            if !has_base && ns.len() > 1 && slit_out_of_order(&ns, &order) {
                let mut held: Vec<E> = Vec::new();
                while vs.len() > 0 {
                    held.push(vs.pop().unwrap());
                }
                while held.len() > 0 {
                    let v = held.pop().unwrap();
                    // a zero-argument call (`PhantomData::default()`, `Vec::new()`) stays in place: it takes its type from the
                    // field (a temp has none) and moves nothing
                    if slit_noarg_call(&v) {
                        vs.push(v);
                    } else {
                        self.nhyg = self.nhyg + 1;
                        let mut fname = String::from("__sfv");
                        fname.push_str(&oast::u32s2(self.nhyg));
                        vs.push(E::Var(String::from(fname.as_str()), lo, hi));
                        sfhoist_n.push(fname);
                        sfhoist_v.push(v);
                    }
                }
            }
            let mut oi = 0;
            while oi < order.len() {
                let mut pick = 0 - 1;
                let mut q = 0;
                while q < ns.len() {
                    if ns[q].as_str() == order[oi].as_str() {
                        pick = q as i32;
                        q = ns.len();
                    } else {
                        q = q + 1;
                    }
                }
                if pick >= 0 {
                    let pu = pick as usize;
                    let mut taken = E::Int(0, lo, hi);
                    {
                        let mut tmp: Vec<E> = Vec::new();
                        let mut r = vs.len();
                        while r > 0 {
                            r = r - 1;
                            match vs.pop() {
                                Some(x) => {
                                    if r == pu {
                                        taken = x;
                                    } else {
                                        tmp.push(x);
                                    }
                                }
                                None => {}
                            }
                        }
                        loop {
                            match tmp.pop() {
                                Some(x) => {
                                    vs.push(x);
                                }
                                None => {
                                    break;
                                }
                            }
                        }
                    }
                    fv.push(taken);
                    sbwritten.push(String::from(order[oi].as_str())); // SLIT-BASE-DROP
                    ns.remove(pu);
                } else if has_base {
                    fv.push(E::Field(
                        Box::new(clone_simple(&base, lo, hi)),
                        String::from(order[oi].as_str()),
                        lo,
                        hi,
                    ));
                }
                oi = oi + 1;
            }
        } else {
            loop {
                if self.is_tok("}") {
                    let _ = self.adv();
                    break;
                }
                if self.at_end() {
                    break;
                }
                let aprog = self.i;
                let v = self.cmp_e();
                fv.push(v);
                if self.is_tok(",") {
                    let _ = self.adv();
                }
                if self.noprog(aprog) {}
            }
        }
        let h2 = if self.i > 0 {
            self.t[self.i - 1].hi
        } else {
            hi
        };
        let lit = E::StructLit(String::from(s), fv, lo, h2);
        let mut out = match sbval {
            Some(bv) => {
                // SLIT-BASE-DROP (heavy pass 2): the base temporary is PARTIALLY moved -- only the fields the literal OVERRODE
                // remain in it, and drop with it (rustc). Its whole-value drop dropped the moved fields a second time.
                let mut body = E::Var(String::from("$slit"), lo, h2);
                let mut q = sbwritten.len();
                while q > 0 {
                    q = q - 1;
                    let mut da: Vec<E> = Vec::new();
                    da.push(E::Field(
                        Box::new(E::Var(String::from(sbname.as_str()), lo, h2)),
                        String::from(sbwritten[q].as_str()),
                        lo,
                        h2,
                    ));
                    body = E::Seq(
                        Box::new(E::Call(String::from("$dropfield"), da, lo, h2)),
                        Box::new(body),
                        lo,
                        h2,
                    );
                }
                let inner = E::Let(
                    String::from("$slit"),
                    Box::new(lit),
                    Box::new(body),
                    lo + 1,
                    h2,
                );
                E::Let(sbname, Box::new(bv), Box::new(inner), lo, h2)
            }
            None => {
                // UPD-FROM-DROP (W7, E0509): `S { a: 2, ..s0 }` MOVES s0's unwritten fields; when S implements Drop and one of them
                // is not Copy, rustc rejects it. `$updfromdrop` carries the moved fields; lowering refuses when the rule fires.
                let lit = if has_var_base {
                    let all = split_commas(&self.sfields_of(s));
                    let mut ua: Vec<E> = Vec::new();
                    let mut q = 0;
                    while q < all.len() {
                        if all[q].len() > 0 && !oast::in_list(&sbwritten, all[q].as_str()) {
                            ua.push(E::Field(
                                Box::new(clone_simple(&var_base, lo, h2)),
                                String::from(all[q].as_str()),
                                lo,
                                h2,
                            ));
                        }
                        q = q + 1;
                    }
                    if ua.len() > 0 {
                        E::Seq(
                            Box::new(E::Call(String::from("$updfromdrop"), ua, lo, h2)),
                            Box::new(lit),
                            lo,
                            h2,
                        )
                    } else {
                        lit
                    }
                } else {
                    lit
                };
                if has_var_base {
                    // SLIT-BASE-DROP: `S { f: v, ..b }` with a VARIABLE base PARTIALLY moves b -- only its overridden fields drop
                    // at b's scope end (drop flags). Not modelled: b's whole-value drop dropped the moved fields again; a
                    // `$partialmove` check refuses it in lowering when an overridden field needs drop.
                    // UPD-BASE-MOVE (R2a P5): the marker also carries b's NAME (last); lowering counts b as MOVED when an unwritten
                    // field is not Copy (it moved into the new value), so b's scope-end drop no longer drops it again (a DOUBLE drop;
                    // rustc drops it once, through the new value). All-Copy unwritten fields are copied: b still drops. Also
                    // `S { ..b }` (nothing written). The move checker skips the marker (oborrowck).
                    let mut pa: Vec<E> = Vec::new();
                    let mut q = 0;
                    while q < sbwritten.len() {
                        pa.push(E::Field(
                            Box::new(clone_simple(&var_base, lo, h2)),
                            String::from(sbwritten[q].as_str()),
                            lo,
                            h2,
                        ));
                        q = q + 1;
                    }
                    // b's NAME last (a string, so it is no value use): lowering's moves_out_cx decides whether the literal MOVED b
                    if let E::Var(bn, _bl, _bh) = &var_base {
                        pa.push(E::StrLit(String::from(bn.as_str()), lo, h2));
                    }
                    E::Seq(
                        Box::new(E::Call(String::from("$partialmove"), pa, lo, h2)),
                        Box::new(lit),
                        lo,
                        h2,
                    )
                } else {
                    lit
                }
            }
        };
        // SLIT-ORDER: the hoisted written fields wrap OUTSIDE the base (first written = outermost = evaluated first)
        while sfhoist_n.len() > 0 {
            let n = sfhoist_n.pop().unwrap();
            let v = sfhoist_v.pop().unwrap();
            out = E::Let(n, Box::new(v), Box::new(out), lo + 1, h2);
        }
        out
    } // SLIT-BASE-TMP
    fn is_tok(&self, k: &str) -> bool {
        if self.i >= self.t.len() {
            return false;
        }
        tk_is(&self.t[self.i].k, k)
    }
    fn lo(&self) -> u32 {
        self.t[self.i].lo
    }
    fn adv(&mut self) -> usize {
        let x = self.i;
        if self.i + 1 < self.t.len() {
            self.i = self.i + 1;
        }
        x
    }
    fn eat(&mut self, k: &str) {
        if self.is_tok(k) {
            let _ = self.adv();
        } else if self.perr == 0 {
            self.perr = self.lo() + 1;
            self.perrexp = String::from(k);
            self.perrgot = self.tok_text();
        }
    }
    // a printable form of the current token, for diagnostics
    fn tok_text(&self) -> String {
        if self.i >= self.t.len() {
            return String::from("<end of input>");
        }
        let w = tok_word(&self.t[self.i]);
        if w.len() > 0 {
            return w;
        }
        match self.t[self.i].k.clone() {
            Tk::Num(n) => i64s(n),
            Tk::ChL(n) => i64s(n),
            Tk::FloatL(x) => String::from(x.as_str()),
            Tk::StrL(_x) => String::from("<string>"),
            _ => {
                let ks = [
                    "(", ")", "{", "}", "[", "]", ",", ";", ":", "=", "=>", "::", "$", "+", "-",
                    "*", "/", "<", ">", "==", "!=", "<=", ">=", "&", "|", "^", "!", "?", "..",
                    "<<", ">>", ".", "fn", "let", "if", "else", "while", "for", "in", "match",
                    "impl", "trait", "struct", "enum", "mod", "use", "return", "break", "continue",
                    "loop", "as", "where", "type", "const", "static", "unsafe", "dyn", "mut",
                    "true", "false",
                ];
                let mut i2 = 0;
                while i2 < ks.len() {
                    if self.is_tok(ks[i2]) {
                        return String::from(ks[i2]);
                    }
                    i2 = i2 + 1;
                }
                String::from("<token>")
            }
        }
    }
    // LABEL: after `break`/`continue`, an optional `'l`: how many loops beyond the innermost it names (0 = the
    // innermost); -1 = no label written. An undeclared label is a parse error (and reads as 0).
    fn label_levels(&mut self, lo: u32) -> i64 {
        let bl = match self.t[self.i].k.clone() {
            Tk::Label(x) => x,
            _ => String::from(""),
        };
        if bl.len() == 0 {
            return 0 - 1;
        }
        let _ = self.adv();
        let mut lv: i64 = 0;
        let mut q = self.lbls.len();
        while q > 0 {
            q = q - 1;
            if self.lbls[q].as_str() == bl.as_str() {
                return lv;
            }
            lv = lv + 1;
        }
        self.perr = lo + 1;
        self.perrgot = bl;
        0
    }
    // LET-ELSE: `let PAT = e else { D };` (PAT refutable: `Some(x)`, `E::V { a, b }`) is rewritten IN THE TOKENS to
    // `let (b..) = match e { PAT => (b..), _ => { D } };` -- the bindings are the pattern's lowercase identifiers. Called just
    // past `let`; true when it rewrote. An initializer ending in `}` cannot precede `else` (rustc forbids it), which is what
    // keeps `let x = if c { 1 } else { 2 };` out.
    fn let_else_rewrite(&mut self) -> bool {
        let i0 = self.i;
        let n = self.t.len();
        if i0 + 1 >= n {
            return false;
        }
        let first_ident = match self.t[i0].k.clone() {
            Tk::Ident(_x) => true,
            _ => false,
        };
        if !first_ident
            || !(tkv_l_paren(&self.t[i0 + 1].k)
                || tkv_l_brace(&self.t[i0 + 1].k)
                || tkv_path_sep(&self.t[i0 + 1].k))
        {
            return false;
        }
        let mut d = 0;
        let mut eq = 0;
        let mut q = i0;
        while q < n {
            if tok_opens(&self.t[q]) {
                d = d + 1;
            } else if tok_closes(&self.t[q]) {
                d = d - 1;
            } else if d == 0 && tkv_semi(&self.t[q].k) {
                return false;
            } else if d == 0 && tkv_eq(&self.t[q].k) {
                eq = q;
                break;
            }
            q = q + 1;
        }
        if eq == 0 {
            return false;
        }
        d = 0;
        let mut el = 0;
        q = eq + 1;
        while q < n {
            if tok_opens(&self.t[q]) {
                d = d + 1;
            } else if tok_closes(&self.t[q]) {
                d = d - 1;
            } else if d == 0 && tkv_semi(&self.t[q].k) {
                return false;
            } else if d == 0 && tkv_else(&self.t[q].k) {
                el = q;
                break;
            }
            q = q + 1;
        }
        if el == 0
            || tkv_r_brace(&self.t[el - 1].k)
            || el + 1 >= n
            || !tkv_l_brace(&self.t[el + 1].k)
        {
            return false;
        }
        d = 0;
        let mut bc = el + 1;
        while bc < n {
            if tok_opens(&self.t[bc]) {
                d = d + 1;
            } else if tok_closes(&self.t[bc]) {
                d = d - 1;
                if d == 0 {
                    break;
                }
            }
            bc = bc + 1;
        }
        if bc + 1 >= n || !tkv_semi(&self.t[bc + 1].k) {
            return false;
        }
        let mut binds: Vec<String> = Vec::new();
        q = i0;
        while q < eq {
            match self.t[q].k.clone() {
                Tk::Ident(w) => {
                    let b = w.as_bytes();
                    let low =
                        b.len() > 0 && ((b[0] >= 97 && b[0] <= 122) || (b[0] == 95 && b.len() > 1));
                    let nxt_path = q + 1 < eq
                        && (tkv_l_paren(&self.t[q + 1].k)
                            || tkv_l_brace(&self.t[q + 1].k)
                            || tkv_path_sep(&self.t[q + 1].k));
                    let prv_path = q > i0 && tkv_path_sep(&self.t[q - 1].k);
                    if low && !nxt_path && !prv_path && !oast::in_list(&binds, &w) {
                        binds.push(w);
                    }
                }
                _ => {}
            }
            q = q + 1;
        }
        let lo = self.t[i0].lo;
        let hi = self.t[bc].hi;
        let mk = |k: Tk| Tok {
            k: k,
            lo: lo,
            hi: hi,
        };
        let mut tup: Vec<Tok> = Vec::new();
        if binds.len() == 1 {
            tup.push(Tok {
                k: Tk::Ident(String::from(binds[0].as_str())),
                lo: lo,
                hi: hi,
            });
        } else {
            tup.push(mk(Tk::LParen));
            let mut b = 0;
            while b < binds.len() {
                if b > 0 {
                    tup.push(mk(Tk::Comma));
                }
                tup.push(Tok {
                    k: Tk::Ident(String::from(binds[b].as_str())),
                    lo: lo,
                    hi: hi,
                });
                b = b + 1;
            }
            tup.push(mk(Tk::RParen));
        }
        let mut nt: Vec<Tok> = Vec::new();
        let mut k = 0;
        while k < i0 {
            nt.push(tok_clone(&self.t[k]));
            k = k + 1;
        }
        k = 0;
        while k < tup.len() {
            nt.push(tok_clone(&tup[k]));
            k = k + 1;
        }
        nt.push(mk(Tk::Eq));
        nt.push(mk(Tk::Match));
        k = eq + 1;
        while k < el {
            nt.push(tok_clone(&self.t[k]));
            k = k + 1;
        }
        nt.push(mk(Tk::LBrace));
        k = i0;
        while k < eq {
            nt.push(tok_clone(&self.t[k]));
            k = k + 1;
        }
        nt.push(mk(Tk::FatArrow));
        k = 0;
        while k < tup.len() {
            nt.push(tok_clone(&tup[k]));
            k = k + 1;
        }
        nt.push(mk(Tk::Comma));
        nt.push(Tok {
            k: Tk::Ident(String::from("_")),
            lo: lo,
            hi: hi,
        });
        nt.push(mk(Tk::FatArrow));
        k = el + 1;
        while k <= bc {
            nt.push(tok_clone(&self.t[k]));
            k = k + 1;
        }
        nt.push(mk(Tk::RBrace));
        k = bc + 1;
        while k < n {
            nt.push(tok_clone(&self.t[k]));
            k = k + 1;
        }
        self.t = nt;
        true
    }
    // PAT-REWRITE: ourc's match Arm is FLAT (a variant + names, or one literal), so richer arm patterns are rewritten IN THE
    // TOKENS to shapes it reads, before the arm reader runs (called just past the match body's `{`):
    //   `x @ P => B`               -> `x if matches!(x, P) => B`
    //   `V(.., P, ..) => B`        -> `V(.., __pmN, ..) if matches!(__pmN, P) => { let P = __pmN else { unreachable!() }; B }`
    //      for each non-simple sub-pattern P (nested variant, literal, or-pattern, @); the let is omitted when P binds nothing.
    // matches! and let-else expand to matches again, so nesting recurses. An existing guard is kept: `if (G) && matches!(..)`.
    fn pat_simple(&self, a: usize, b: usize) -> bool {
        // `ref x` / `mut x` / `ref mut x` / `&x` are plain bindings too (rewriting `ref v` broke real_rust_patterns/tuplestruct)
        let mut a = a;
        while a + 1 < b
            && (self.is_word_at(a, "ref") || tkv_mut(&self.t[a].k) || tkv_amp(&self.t[a].k))
        {
            a = a + 1;
        }
        {
            // LIT-SUFFIX-PAT (merge fix): a suffixed literal `0u8` is lexed as the group `( 0 as u8 )` (optionally negated);
            // it is a LITERAL, not a nested pattern -- NESTED-TUPLE-PAT rewrote it into `matches!(__pm, (0 as u8))` (a refusal)
            let mut s = a;
            if s < b && tkv_minus(&self.t[s].k) {
                s = s + 1;
            }
            if b == s + 5
                && tkv_l_paren(&self.t[s].k)
                && ilit(&self.t[s + 1]) > 0
                && (match self.t[s + 2].k {
                    Tk::AsT => true,
                    _ => false,
                })
                && tkv_r_paren(&self.t[s + 4].k)
            {
                return true;
            }
        }
        if b != a + 1 {
            return false;
        }
        match self.t[a].k.clone() {
            Tk::Ident(w) => {
                let x = w.as_bytes();
                x.len() > 0 && ((x[0] >= 97 && x[0] <= 122) || x[0] == 95)
            }
            _ => tkv_dot_dot(&self.t[a].k),
        }
    }
    // TUP-IRREF: `(a, (b, _), mut c)` -- a tuple whose leaves are all plain bindings or `_` cannot fail to match
    fn pat_irref_tuple(&self, a: usize, b: usize) -> bool {
        if b <= a + 1 || !tkv_l_paren(&self.t[a].k) || !tkv_r_paren(&self.t[b - 1].k) {
            return false;
        }
        let mut q = a;
        while q < b {
            let ok = match self.t[q].k.clone() {
                Tk::Ident(w) => {
                    let x = w.as_bytes();
                    x.len() > 0
                        && ((x[0] >= 97 && x[0] <= 122) || x[0] == 95)
                        && !(q + 1 < b
                            && (tkv_l_paren(&self.t[q + 1].k) || tkv_path_sep(&self.t[q + 1].k)))
                }
                _ => {
                    tkv_l_paren(&self.t[q].k)
                        || tkv_r_paren(&self.t[q].k)
                        || tkv_comma(&self.t[q].k)
                        || tkv_mut(&self.t[q].k)
                }
            };
            if !ok {
                return false;
            }
            q = q + 1;
        }
        true
    }
    fn pat_binds(&self, a: usize, b: usize) -> bool {
        let mut q = a;
        while q < b {
            match self.t[q].k.clone() {
                Tk::Ident(w) => {
                    let x = w.as_bytes();
                    let low =
                        x.len() > 0 && ((x[0] >= 97 && x[0] <= 122) || (x[0] == 95 && x.len() > 1));
                    let nxt = q + 1 < b
                        && (tkv_l_paren(&self.t[q + 1].k)
                            || tkv_l_brace(&self.t[q + 1].k)
                            || tkv_path_sep(&self.t[q + 1].k));
                    let prv = q > a && tkv_path_sep(&self.t[q - 1].k);
                    if low && !nxt && !prv {
                        return true;
                    }
                }
                _ => {}
            }
            q = q + 1;
        }
        false
    }
    // the names a pattern [a, b) binds (pat_binds' criterion: a lower-case/underscore-led ident that is not a path or a call)
    fn pat_names(&self, a: usize, b: usize) -> Vec<String> {
        let mut o: Vec<String> = Vec::new();
        let mut q = a;
        while q < b {
            match self.t[q].k.clone() {
                Tk::Ident(w) => {
                    let x = w.as_bytes();
                    let low =
                        x.len() > 0 && ((x[0] >= 97 && x[0] <= 122) || (x[0] == 95 && x.len() > 1));
                    let nxt = q + 1 < b
                        && (tkv_l_paren(&self.t[q + 1].k)
                            || tkv_l_brace(&self.t[q + 1].k)
                            || tkv_path_sep(&self.t[q + 1].k));
                    let prv = q > a && tkv_path_sep(&self.t[q - 1].k);
                    if low && !nxt && !prv && !self.is_word_at(q, "ref") {
                        o.push(String::from(w.as_str()));
                    }
                }
                _ => {}
            }
            q = q + 1;
        }
        o
    }
    // OR-SPLIT: `P1 | P2 [if g] => B` where the alternatives BIND names (`E::A(_, x) | E::B(x)`) becomes one arm per
    // alternative, each with the guard and a copy of B's tokens (only the matching arm runs). The flat Arm cannot carry an
    // or of binding variants, and the per-alternative positions differ. Runs before arm_rewrite_all, which then rewrites each.
    // R9 OR-NEST: a nested or-pattern (`E::A(1 | 2)`, `Some(1 | 3)`, `(0, 1 | 2)`) is DISTRIBUTED outward -- the top-level
    // alternative holding the group becomes one copy per inner alternative (`E::A(1) | E::A(2)`), so or_split_all and the
    // flat Arm see only top-level `|`. Repeats until no nested `|` remains (bounded). It was refused ("cannot apply `|`").
    // R9 OR-NEST helper: if t[o] opens a `(`/`[` group whose OWN level holds a `|` (before pe), the index of its close; else -1
    fn bar_group_close(&self, o: usize, pe: usize) -> i64 {
        if !(tkv_l_paren(&self.t[o].k) || tkv_l_brack(&self.t[o].k)) {
            return 0 - 1;
        }
        let mut q = o + 1;
        let mut d2 = 0;
        let mut bar = 0;
        while q < pe {
            if tok_opens(&self.t[q]) {
                d2 = d2 + 1;
            } else if tok_closes(&self.t[q]) {
                if d2 == 0 {
                    if bar > 0 {
                        return q as i64;
                    }
                    return 0 - 1;
                }
                d2 = d2 - 1;
            } else if d2 == 0 && tkv_bar(&self.t[q].k) {
                bar = bar + 1;
            }
            q = q + 1;
        }
        0 - 1
    }
    fn or_nest_all(&mut self) {
        let mut rounds = 0;
        while rounds < 32 {
            rounds = rounds + 1;
            let start = self.i;
            let n = self.t.len();
            // the match body: start .. its closing `}`
            let mut d = 1;
            let mut end = start;
            while end < n {
                if tok_opens(&self.t[end]) {
                    d = d + 1;
                } else if tok_closes(&self.t[end]) {
                    d = d - 1;
                    if d == 0 {
                        break;
                    }
                }
                end = end + 1;
            }
            // find one arm pattern with a nested `|`; distribute it once
            let mut done_one = false;
            let mut pos = start;
            while pos < end && !done_one {
                // pattern = pos .. (`if` at depth 0 | `=>`)
                let mut pe = pos;
                let mut dd = 0;
                while pe < end {
                    if tok_opens(&self.t[pe]) {
                        dd = dd + 1;
                    } else if tok_closes(&self.t[pe]) {
                        dd = dd - 1;
                    } else if dd == 0 && (tkv_fat_arrow(&self.t[pe].k) || tkv_if(&self.t[pe].k)) {
                        break;
                    }
                    pe = pe + 1;
                }
                if pe >= end {
                    break;
                }
                // a `(`/`[` group whose OWN level holds a `|` (a helper: an outer mut flag set in nested loops miscompiles under SEM0)
                let mut o = pos;
                let mut cpos: i64 = 0 - 1;
                while o < pe && cpos < 0 {
                    cpos = self.bar_group_close(o, pe);
                    if cpos < 0 {
                        o = o + 1;
                    }
                }
                let found = cpos >= 0;
                let cpos = if found { cpos as usize } else { pos };
                if found {
                    // the top-level alternative of this pattern that contains the group: [as, ae)
                    let mut as0 = pos;
                    let mut ae = pe;
                    {
                        let mut q = pos;
                        let mut d3 = 0;
                        while q < pe {
                            if tok_opens(&self.t[q]) {
                                d3 = d3 + 1;
                            } else if tok_closes(&self.t[q]) {
                                d3 = d3 - 1;
                            } else if d3 == 0 && tkv_bar(&self.t[q].k) {
                                if q < o {
                                    as0 = q + 1;
                                } else if q > cpos && ae == pe {
                                    ae = q;
                                }
                            }
                            q = q + 1;
                        }
                    }
                    // elements of the group (o, cpos), split at its own level; the FIRST element with `|` is expanded
                    let mut el: Vec<(usize, usize)> = Vec::new();
                    {
                        let mut s0 = o + 1;
                        let mut q = o + 1;
                        let mut d4 = 0;
                        while q <= cpos {
                            if q == cpos || (d4 == 0 && tkv_comma(&self.t[q].k)) {
                                el.push((s0, q));
                                s0 = q + 1;
                            } else if tok_opens(&self.t[q]) {
                                d4 = d4 + 1;
                            } else if tok_closes(&self.t[q]) {
                                d4 = d4 - 1;
                            }
                            q = q + 1;
                        }
                    }
                    let mut alts: Vec<(usize, usize)> = Vec::new();
                    let mut ek = 0;
                    while ek < el.len() && alts.len() == 0 {
                        let (es, ee) = el[ek];
                        let mut s0 = es;
                        let mut q = es;
                        let mut d5 = 0;
                        let mut has = false;
                        let mut tmp: Vec<(usize, usize)> = Vec::new();
                        while q <= ee {
                            if q == ee || (d5 == 0 && tkv_bar(&self.t[q].k)) {
                                if q < ee {
                                    has = true;
                                }
                                tmp.push((s0, q));
                                s0 = q + 1;
                            } else if tok_opens(&self.t[q]) {
                                d5 = d5 + 1;
                            } else if tok_closes(&self.t[q]) {
                                d5 = d5 - 1;
                            }
                            q = q + 1;
                        }
                        if has {
                            alts = tmp;
                        } else {
                            ek = ek + 1;
                        }
                    }
                    if alts.len() > 0 {
                        let (es, ee) = el[ek];
                        let bar = {
                            let (a0s, a0e) = alts[0];
                            tok_clone(&self.t[a0e]) // the `|` after the first inner alternative
                        };
                        let _ = alts[0].0;
                        let mut rep: Vec<Tok> = Vec::new();
                        let mut a = 0;
                        while a < alts.len() {
                            if a > 0 {
                                rep.push(tok_clone(&bar));
                            }
                            let mut q = as0;
                            while q < es {
                                rep.push(tok_clone(&self.t[q]));
                                q = q + 1;
                            }
                            let (xs, xe) = alts[a];
                            q = xs;
                            while q < xe {
                                rep.push(tok_clone(&self.t[q]));
                                q = q + 1;
                            }
                            q = ee;
                            while q < ae {
                                rep.push(tok_clone(&self.t[q]));
                                q = q + 1;
                            }
                            a = a + 1;
                        }
                        let mut nt: Vec<Tok> = Vec::new();
                        let mut q = 0;
                        while q < as0 {
                            nt.push(tok_clone(&self.t[q]));
                            q = q + 1;
                        }
                        while rep.len() > 0 {
                            nt.push(rep.remove(0));
                        }
                        q = ae;
                        while q < self.t.len() {
                            nt.push(tok_clone(&self.t[q]));
                            q = q + 1;
                        }
                        self.t = nt;
                        done_one = true;
                    }
                }
                if !done_one {
                    // next arm: skip the body (to the arm's `,` at depth 0 or a closing block)
                    let mut q = pe;
                    let mut d6 = 0;
                    while q < end {
                        if tok_opens(&self.t[q]) {
                            d6 = d6 + 1;
                        } else if tok_closes(&self.t[q]) {
                            d6 = d6 - 1;
                            if d6 == 0
                                && tkv_r_brace(&self.t[q].k)
                                && q + 1 < end
                                && !tkv_comma(&self.t[q + 1].k)
                            {
                                q = q + 1;
                                break;
                            }
                        } else if d6 == 0 && tkv_comma(&self.t[q].k) {
                            q = q + 1;
                            break;
                        }
                        q = q + 1;
                    }
                    pos = q;
                }
            }
            if !done_one {
                break;
            }
        }
    }
    fn or_split_all(&mut self) {
        let start = self.i;
        let n = self.t.len();
        let mut d = 1;
        let mut end = start;
        while end < n {
            if tok_opens(&self.t[end]) {
                d = d + 1;
            } else if tok_closes(&self.t[end]) {
                d = d - 1;
                if d == 0 {
                    break;
                }
            }
            end = end + 1;
        }
        let mut out: Vec<Tok> = Vec::new();
        let mut changed = false;
        let mut pos = start;
        while pos < end {
            let mut pe = pos;
            let mut gi = 0;
            let mut dd = 0;
            while pe < end {
                if tok_opens(&self.t[pe]) {
                    dd = dd + 1;
                } else if tok_closes(&self.t[pe]) {
                    dd = dd - 1;
                } else if dd == 0 && tkv_fat_arrow(&self.t[pe].k) {
                    break;
                } else if dd == 0 && gi == 0 && tkv_if(&self.t[pe].k) {
                    gi = pe;
                }
                pe = pe + 1;
            }
            if pe >= end {
                let mut k = pos;
                while k < end {
                    out.push(tok_clone(&self.t[k]));
                    k = k + 1;
                }
                break;
            }
            let pend = if gi > 0 { gi } else { pe };
            {
                let mut q = pos;
                while q < pend {
                    let w = tok_word(&self.t[q]);
                    let wb = w.as_bytes();
                    if wb.len() > 0
                        && wb[0] >= 97
                        && wb[0] <= 122
                        && !(q + 1 < pend
                            && (tkv_l_paren(&self.t[q + 1].k) || tkv_path_sep(&self.t[q + 1].k)))
                    {
                        self.lnames.push(w);
                    }
                    q = q + 1;
                }
            } // FN-ITEM-VAL: arm pattern bindings shadow fn names
            let mut be = pe + 1;
            dd = 0;
            if be < end && tkv_l_brace(&self.t[be].k) {
                while be < end {
                    if tok_opens(&self.t[be]) {
                        dd = dd + 1;
                    } else if tok_closes(&self.t[be]) {
                        dd = dd - 1;
                        if dd == 0 {
                            be = be + 1;
                            break;
                        }
                    }
                    be = be + 1;
                }
                if be < end && tkv_comma(&self.t[be].k) {
                    be = be + 1;
                }
            } else {
                while be < end {
                    if tok_opens(&self.t[be]) {
                        dd = dd + 1;
                    } else if tok_closes(&self.t[be]) {
                        dd = dd - 1;
                    } else if dd == 0 && tkv_comma(&self.t[be].k) {
                        be = be + 1;
                        break;
                    }
                    be = be + 1;
                }
            }
            // the depth-0 `|` splits of the pattern (a leading `|` is allowed and skipped)
            let mut cuts: Vec<usize> = Vec::new();
            let mut q = pos;
            dd = 0;
            while q < pend {
                if tok_opens(&self.t[q]) {
                    dd = dd + 1;
                } else if tok_closes(&self.t[q]) {
                    dd = dd - 1;
                } else if dd == 0 && tkv_bar(&self.t[q].k) && q > pos {
                    cuts.push(q);
                }
                q = q + 1;
            }
            // OR-WILD: an alternative that is a bare `_` (`0 | _`) has no literal form -- split those arms too
            let altwild = {
                let mut f = false;
                let mut b0 = pos;
                let mut ci = 0;
                while ci <= cuts.len() {
                    let b1 = if ci < cuts.len() { cuts[ci] } else { pend };
                    if b1 == b0 + 1 && word_is(&self.t[b0], "_") {
                        f = true;
                    }
                    b0 = b1 + 1;
                    ci = ci + 1;
                }
                f
            };
            // R9 OR-STRUCT: an alternative with a GROUP (`Some(1) | Some(3)`, `(0, 1) | (0, 2)`) has no flat literal form either -- split
            let altgroup = {
                let mut g = 0;
                let mut q = pos;
                while q < pend {
                    if tkv_l_paren(&self.t[q].k) || tkv_l_brack(&self.t[q].k) {
                        g = g + 1;
                    }
                    q = q + 1;
                }
                g > 0
            };
            if cuts.len() == 0 || (!self.pat_binds(pos, pend) && !altwild && !altgroup) {
                let mut k = pos;
                while k < be {
                    out.push(tok_clone(&self.t[k]));
                    k = k + 1;
                }
                pos = be;
                continue;
            }
            // E0408: every alternative must bind the same names
            {
                let mut b0 = pos;
                let mut ci = 0;
                let mut sets: Vec<Vec<String>> = Vec::new();
                while ci <= cuts.len() {
                    let b1 = if ci < cuts.len() { cuts[ci] } else { pend };
                    sets.push(self.pat_names(b0, b1));
                    b0 = b1 + 1;
                    ci = ci + 1;
                }
                let mut s = 1;
                while s < sets.len() {
                    let mut w = 0;
                    while w < 2 {
                        let (x, y) = if w == 0 {
                            (&sets[0], &sets[s])
                        } else {
                            (&sets[s], &sets[0])
                        };
                        let mut k = 0;
                        while k < x.len() {
                            let mut f = false;
                            let mut j = 0;
                            while j < y.len() {
                                if y[j].as_str() == x[k].as_str() {
                                    f = true;
                                }
                                j = j + 1;
                            }
                            if !f && self.perr == 0 {
                                self.perr = self.t[pos].lo + 1;
                                let mut m = String::from("every alternative to bind `");
                                m.push_str(x[k].as_str());
                                m.push_str("` (E0408: variable not bound in all patterns)");
                                self.perrexp = m;
                                self.perrgot = String::from("an or-pattern that does not");
                            }
                            k = k + 1;
                        }
                        w = w + 1;
                    }
                    s = s + 1;
                }
            }
            changed = true;
            let bend = if be > pe + 1 && tkv_comma(&self.t[be - 1].k) {
                be - 1
            } else {
                be
            };
            let mut a0 = pos;
            let mut ci = 0;
            while ci <= cuts.len() {
                let a1 = if ci < cuts.len() { cuts[ci] } else { pend };
                let mut k = a0;
                while k < a1 {
                    out.push(tok_clone(&self.t[k]));
                    k = k + 1;
                }
                k = pend;
                while k < pe {
                    out.push(tok_clone(&self.t[k]));
                    k = k + 1;
                } // the guard, if any
                k = pe;
                while k < bend {
                    out.push(tok_clone(&self.t[k]));
                    k = k + 1;
                } // `=>` and the body
                out.push(Tok {
                    k: Tk::Comma,
                    lo: self.t[pe].lo,
                    hi: self.t[pe].hi,
                });
                a0 = a1 + 1;
                ci = ci + 1;
            }
            pos = be;
        }
        if changed {
            let mut nt: Vec<Tok> = Vec::new();
            let mut k = 0;
            while k < start {
                nt.push(tok_clone(&self.t[k]));
                k = k + 1;
            }
            k = 0;
            while k < out.len() {
                nt.push(tok_clone(&out[k]));
                k = k + 1;
            }
            k = end;
            while k < n {
                nt.push(tok_clone(&self.t[k]));
                k = k + 1;
            }
            self.t = nt;
        }
    }
    // PAT-PATH3 (R2a P4): `m::T::V` (any depth) where `m$T` is an enum names it by its qualified enum name, `m$T::V` -- the
    // pattern grammar reads a two-segment `Enum::Variant` only, so `olex::Tk::LexErr =>` failed ("expected =>, found ::").
    // Rewrites the match body's tokens (from self.i to the closing brace) before the arm readers see them.
    fn enum_path_collapse(&mut self) {
        let start = self.i;
        let n = self.t.len();
        let mut d = 1;
        let mut end = start;
        while end < n {
            if tok_opens(&self.t[end]) {
                d = d + 1;
            } else if tok_closes(&self.t[end]) {
                d = d - 1;
                if d == 0 {
                    break;
                }
            }
            end = end + 1;
        }
        let mut nt: Vec<Tok> = Vec::new();
        let mut k = 0;
        while k < start {
            nt.push(tok_clone(&self.t[k]));
            k = k + 1;
        }
        let mut changed = false;
        while k < end {
            // a run Ident (:: Ident)+ of >= 3 segments
            let mut segs: Vec<String> = Vec::new();
            let mut q = k;
            let first_ok = match self.t[k].k.clone() {
                Tk::Ident(_x) => !(k > 0 && tkv_path_sep(&self.t[k - 1].k)),
                _ => false,
            };
            if first_ok {
                loop {
                    let w = match &self.t[q].k {
                        Tk::Ident(x) => String::from(x.as_str()),
                        _ => String::from(""),
                    };
                    if w.len() == 0 {
                        break;
                    }
                    segs.push(w);
                    if q + 2 < end && tkv_path_sep(&self.t[q + 1].k) {
                        q = q + 2;
                    } else {
                        break;
                    }
                }
            }
            let mut cut = 0;
            if segs.len() >= 3 {
                let mut m = segs.len() - 1;
                while m >= 2 {
                    let mut j = String::from(segs[0].as_str());
                    let mut z = 1;
                    while z < m {
                        j.push('$');
                        j.push_str(segs[z].as_str());
                        z = z + 1;
                    }
                    let r = self.tyres(&j);
                    if self.enames_has(&r) {
                        cut = m;
                        m = 1;
                    } else {
                        m = m - 1;
                    }
                }
            }
            if cut >= 2 {
                let mut j = String::from(segs[0].as_str());
                let mut z = 1;
                while z < cut {
                    j.push('$');
                    j.push_str(segs[z].as_str());
                    z = z + 1;
                }
                nt.push(Tok {
                    k: Tk::Ident(self.tyres(&j)),
                    lo: self.t[k].lo,
                    hi: self.t[k + 2 * (cut - 1)].hi,
                });
                k = k + 2 * (cut - 1) + 1;
                changed = true;
            } else {
                nt.push(tok_clone(&self.t[k]));
                k = k + 1;
            }
        }
        if changed {
            while k < n {
                nt.push(tok_clone(&self.t[k]));
                k = k + 1;
            }
            self.t = nt;
        }
    }
    fn arm_rewrite_all(&mut self) {
        let start = self.i;
        let n = self.t.len();
        let mut d = 1;
        let mut end = start;
        while end < n {
            if tok_opens(&self.t[end]) {
                d = d + 1;
            } else if tok_closes(&self.t[end]) {
                d = d - 1;
                if d == 0 {
                    break;
                }
            }
            end = end + 1;
        }
        let mut out: Vec<Tok> = Vec::new();
        let mut changed = false;
        let mut pos = start;
        while pos < end {
            // pattern [pos, gi|pe), guard (gi, pe), body [pe+1, be)
            let mut pe = pos;
            let mut gi = 0;
            let mut dd = 0;
            while pe < end {
                if tok_opens(&self.t[pe]) {
                    dd = dd + 1;
                } else if tok_closes(&self.t[pe]) {
                    dd = dd - 1;
                } else if dd == 0 && tkv_fat_arrow(&self.t[pe].k) {
                    break;
                } else if dd == 0 && gi == 0 && tkv_if(&self.t[pe].k) {
                    gi = pe;
                }
                pe = pe + 1;
            }
            if pe >= end {
                let mut k = pos;
                while k < end {
                    out.push(tok_clone(&self.t[k]));
                    k = k + 1;
                }
                break;
            }
            let pend = if gi > 0 { gi } else { pe };
            let mut be = pe + 1;
            dd = 0;
            if be < end && tkv_l_brace(&self.t[be].k) {
                while be < end {
                    if tok_opens(&self.t[be]) {
                        dd = dd + 1;
                    } else if tok_closes(&self.t[be]) {
                        dd = dd - 1;
                        if dd == 0 {
                            be = be + 1;
                            break;
                        }
                    }
                    be = be + 1;
                }
                if be < end && tkv_comma(&self.t[be].k) {
                    be = be + 1;
                }
            } else {
                while be < end {
                    if tok_opens(&self.t[be]) {
                        dd = dd + 1;
                    } else if tok_closes(&self.t[be]) {
                        dd = dd - 1;
                    } else if dd == 0 && tkv_comma(&self.t[be].k) {
                        be = be + 1;
                        break;
                    }
                    be = be + 1;
                }
            }
            let lo = self.t[pos].lo;
            let hi = self.t[pos].hi;
            let mk = |k: Tk| Tok {
                k: k,
                lo: lo,
                hi: hi,
            };
            let mut np: Vec<Tok> = Vec::new();
            let mut guards: Vec<Vec<Tok>> = Vec::new();
            let mut lets: Vec<Vec<Tok>> = Vec::new();
            // AT-REF: `ref y @ P` / `ref mut y @ P` -- `ref` is transparent in this value model
            pos = {
                let mut p2 = pos;
                if p2 < pend && word_is(&self.t[p2], "ref") {
                    p2 = p2 + 1;
                    if p2 < pend && tkv_mut(&self.t[p2].k) {
                        p2 = p2 + 1;
                    }
                }
                if p2 > pos
                    && p2 + 1 < pend
                    && (match self.t[p2 + 1].k {
                        Tk::At => true,
                        _ => false,
                    })
                {
                    p2
                } else {
                    pos
                }
            };
            let at_top = pos + 1 < pend
                && (match self.t[pos].k.clone() {
                    Tk::Ident(_x) => true,
                    _ => false,
                })
                && (match self.t[pos + 1].k {
                    Tk::At => true,
                    _ => false,
                });
            if at_top {
                np.push(tok_clone(&self.t[pos]));
                let mut g: Vec<Tok> = Vec::new();
                g.push(Tok {
                    k: Tk::Ident(String::from("matches")),
                    lo: lo,
                    hi: hi,
                });
                g.push(mk(Tk::Bang));
                g.push(mk(Tk::LParen));
                g.push(tok_clone(&self.t[pos]));
                g.push(mk(Tk::Comma));
                let mut k = pos + 2;
                while k < pend {
                    g.push(tok_clone(&self.t[k]));
                    k = k + 1;
                }
                g.push(mk(Tk::RParen));
                guards.push(g);
            } else {
                // a tuple-variant pattern: the first depth-0 `(` after the path
                let mut op = pos;
                while op < pend
                    && !tkv_l_paren(&self.t[op].k)
                    && (tkv_path_sep(&self.t[op].k)
                        || (match self.t[op].k.clone() {
                            Tk::Ident(_x) => true,
                            _ => false,
                        }))
                {
                    op = op + 1;
                }
                // NESTED-TUPLE-PAT (R2a P3): a bare TUPLE pattern `(Some(p), Some(q))` takes the same rewrite as a variant's
                // payload -- each nested position becomes a fresh name + matches! guard + let-else binding, so the flat tuple
                // reader only sees names (it refused nested groups: "a pattern without a nested group")
                let mut isv = (op > pos || tkv_l_paren(&self.t[pos].k))
                    && op < pend
                    && tkv_l_paren(&self.t[op].k)
                    && tkv_r_paren(&self.t[pend - 1].k);
                if isv {
                    // MID-REST-PAT (rfc-2005 tuple-struct): a `..` that is NOT the last sub-pattern (`V(Some(n), .., v)`)
                    // is not rewritten -- the rewrite kept the `..` and the flat reader then bound `v` at the wrong
                    // position (a segfault). Left as is, the pattern meets the "`..` only at the END" refusal.
                    let mut q3 = op + 1;
                    let mut d3 = 0;
                    let mut rest_seen = false;
                    while q3 + 1 < pend {
                        if tok_opens(&self.t[q3]) {
                            d3 = d3 + 1;
                        } else if tok_closes(&self.t[q3]) {
                            d3 = d3 - 1;
                        } else if d3 == 0
                            && tkv_dot_dot(&self.t[q3].k)
                            && (q3 == op + 1 || tkv_comma(&self.t[q3 - 1].k))
                            && (q3 + 2 == pend || tkv_comma(&self.t[q3 + 1].k))
                        {
                            // a WHOLE sub-pattern `..` -- not the `..` inside a range pattern (`1..=2`, `'a'..='z'`)
                            rest_seen = true;
                        } else if d3 == 0 && rest_seen && tkv_comma(&self.t[q3].k) && q3 + 2 < pend
                        {
                            isv = false;
                        }
                        q3 = q3 + 1;
                    }
                }
                if isv {
                    let mut k = pos;
                    while k <= op {
                        np.push(tok_clone(&self.t[k]));
                        k = k + 1;
                    }
                    let mut s = op + 1;
                    let mut q = op + 1;
                    let mut d2 = 0;
                    let close = pend - 1;
                    while q <= close {
                        let at_end = q == close;
                        if !at_end {
                            if tok_opens(&self.t[q]) {
                                d2 = d2 + 1;
                            } else if tok_closes(&self.t[q]) {
                                d2 = d2 - 1;
                            }
                        }
                        if at_end || (d2 == 0 && tkv_comma(&self.t[q].k)) {
                            if q > s {
                                if self.pat_simple(s, q) {
                                    let mut k2 = s;
                                    while k2 < q {
                                        np.push(tok_clone(&self.t[k2]));
                                        k2 = k2 + 1;
                                    }
                                } else {
                                    self.nhyg = self.nhyg + 1;
                                    let mut nm = String::from("__pm");
                                    nm.push_str(&oast::u32s2(self.nhyg));
                                    np.push(Tok {
                                        k: Tk::Ident(String::from(nm.as_str())),
                                        lo: lo,
                                        hi: hi,
                                    });
                                    // a sub-pattern `n @ P`: guard on P, and the binding is plain `let n = __pmN;`
                                    let subat = q > s + 2
                                        && (match self.t[s].k.clone() {
                                            Tk::Ident(_x) => true,
                                            _ => false,
                                        })
                                        && (match self.t[s + 1].k {
                                            Tk::At => true,
                                            _ => false,
                                        });
                                    let gs = if subat { s + 2 } else { s };
                                    let mut g: Vec<Tok> = Vec::new();
                                    g.push(Tok {
                                        k: Tk::Ident(String::from("matches")),
                                        lo: lo,
                                        hi: hi,
                                    });
                                    g.push(mk(Tk::Bang));
                                    g.push(mk(Tk::LParen));
                                    g.push(Tok {
                                        k: Tk::Ident(String::from(nm.as_str())),
                                        lo: lo,
                                        hi: hi,
                                    });
                                    g.push(mk(Tk::Comma));
                                    let mut k2 = gs;
                                    while k2 < q {
                                        g.push(tok_clone(&self.t[k2]));
                                        k2 = k2 + 1;
                                    }
                                    g.push(mk(Tk::RParen));
                                    guards.push(g);
                                    if subat {
                                        let mut lt: Vec<Tok> = Vec::new();
                                        lt.push(mk(Tk::Let));
                                        lt.push(tok_clone(&self.t[s]));
                                        lt.push(mk(Tk::Eq));
                                        lt.push(Tok {
                                            k: Tk::Ident(String::from(nm.as_str())),
                                            lo: lo,
                                            hi: hi,
                                        });
                                        lt.push(mk(Tk::Semi));
                                        lets.push(lt);
                                    } else if self.pat_binds(s, q) {
                                        let mut lt: Vec<Tok> = Vec::new();
                                        lt.push(mk(Tk::Let));
                                        let mut k2 = s;
                                        while k2 < q {
                                            lt.push(tok_clone(&self.t[k2]));
                                            k2 = k2 + 1;
                                        }
                                        lt.push(mk(Tk::Eq));
                                        lt.push(Tok {
                                            k: Tk::Ident(String::from(nm.as_str())),
                                            lo: lo,
                                            hi: hi,
                                        });
                                        if !self.pat_irref_tuple(s, q) {
                                            lt.push(mk(Tk::Else));
                                            lt.push(mk(Tk::LBrace));
                                            lt.push(Tok {
                                                k: Tk::Ident(String::from("unreachable")),
                                                lo: lo,
                                                hi: hi,
                                            });
                                            lt.push(mk(Tk::Bang));
                                            lt.push(mk(Tk::LParen));
                                            lt.push(mk(Tk::RParen));
                                            lt.push(mk(Tk::RBrace));
                                        }
                                        lt.push(mk(Tk::Semi)); /* TUP-IRREF: an all-bindings tuple sub-pattern is irrefutable -- a plain let, no else */
                                        lets.push(lt);
                                    }
                                }
                            }
                            if !at_end {
                                np.push(mk(Tk::Comma));
                            }
                            s = q + 1;
                        }
                        q = q + 1;
                    }
                    np.push(mk(Tk::RParen));
                }
            }
            if guards.len() == 0 {
                let mut k = pos;
                while k < be {
                    out.push(tok_clone(&self.t[k]));
                    k = k + 1;
                }
                pos = be;
                continue;
            }
            changed = true;
            let mut k = 0;
            while k < np.len() {
                out.push(tok_clone(&np[k]));
                k = k + 1;
            }
            out.push(mk(Tk::If));
            let mut g = 0;
            while g < guards.len() {
                if g > 0 {
                    out.push(mk(Tk::AndAnd));
                }
                let mut k2 = 0;
                while k2 < guards[g].len() {
                    out.push(tok_clone(&guards[g][k2]));
                    k2 = k2 + 1;
                }
                g = g + 1;
            }
            // the USER guard runs after the matches!, inside a block that first makes the rewritten bindings -- it may name them
            // (`E::A(Some(v)) if v > 10`); without the lets it read v before v existed (a wrong answer, not an error)
            if gi > 0 {
                out.push(mk(Tk::AndAnd));
                out.push(mk(Tk::LBrace));
                let mut l = 0;
                while l < lets.len() {
                    let mut k2 = 0;
                    while k2 < lets[l].len() {
                        out.push(tok_clone(&lets[l][k2]));
                        k2 = k2 + 1;
                    }
                    l = l + 1;
                }
                out.push(mk(Tk::LParen));
                let mut k2 = gi + 1;
                while k2 < pe {
                    out.push(tok_clone(&self.t[k2]));
                    k2 = k2 + 1;
                }
                out.push(mk(Tk::RParen));
                out.push(mk(Tk::RBrace));
            }
            out.push(mk(Tk::FatArrow));
            let bend = if be > pe + 1 && tkv_comma(&self.t[be - 1].k) {
                be - 1
            } else {
                be
            };
            if lets.len() > 0 {
                out.push(mk(Tk::LBrace));
                let mut l = 0;
                while l < lets.len() {
                    let mut k2 = 0;
                    while k2 < lets[l].len() {
                        out.push(tok_clone(&lets[l][k2]));
                        k2 = k2 + 1;
                    }
                    l = l + 1;
                }
            }
            let mut k2 = pe + 1;
            while k2 < bend {
                out.push(tok_clone(&self.t[k2]));
                k2 = k2 + 1;
            }
            if lets.len() > 0 {
                out.push(mk(Tk::RBrace));
            }
            out.push(mk(Tk::Comma));
            pos = be;
        }
        if changed {
            let mut nt: Vec<Tok> = Vec::new();
            let mut k = 0;
            while k < start {
                nt.push(tok_clone(&self.t[k]));
                k = k + 1;
            }
            k = 0;
            while k < out.len() {
                nt.push(tok_clone(&out[k]));
                k = k + 1;
            }
            k = end;
            while k < n {
                nt.push(tok_clone(&self.t[k]));
                k = k + 1;
            }
            self.t = nt;
        }
    }
    // CLOS-MUT: the closures lifted while parsing ONE fn body are self.clos[c0..]; each capture one of them WRITES must be a
    // shared cell in that enclosing fn too. Recorded as "@<body lo>|var" -- lowering keys the enclosing fn by its body start.
    fn mut_encl(&mut self, c0: usize, body: &E) {
        let bl = oast::e_lo(body);
        let mut k = c0;
        while k < self.clos.len() {
            let cn = String::from(self.clos[k].name.as_str());
            let mut q = 0;
            let n = self.mutcaps.len();
            while q < n {
                let e = String::from(self.mutcaps[q].as_str());
                let b = e.as_bytes();
                let mut cut = 0;
                while cut < b.len() && b[cut] != 124 {
                    cut = cut + 1;
                }
                let mut moved = String::from("!");
                moved.push_str(e.as_str());
                let is_mv = oast::in_list(&self.mutcaps, &moved);
                if cut < b.len() && &e[..cut] == cn.as_str() && !is_mv {
                    // W3 MOVE-CELL: a `move` closure's written capture is its OWN cell -- the creator's variable stays plain
                    let mut key = String::from("@");
                    key.push_str(&oast::u32s2(bl));
                    key.push('|');
                    key.push_str(&e[(cut + 1)..]);
                    self.mutcaps.push(key);
                }
                q = q + 1;
            }
            k = k + 1;
        }
        // CELL-READERS (W3): a variable the enclosing fn keeps in a shared CELL (some closure writes it) must be read THROUGH
        // that cell by every non-move closure that captures it -- a read-only `|| println!("{}", x)` beside a writer
        // `|| x += y` received the cell handle as if it were the value and printed an address
        let mut pre = String::from("@");
        pre.push_str(&oast::u32s2(bl));
        pre.push('|');
        let mut k = c0;
        while k < self.clos.len() {
            let cn = String::from(self.clos[k].name.as_str());
            let mut mv = String::from("^");
            mv.push_str(cn.as_str());
            if !oast::in_list(&self.mutcaps, &mv) {
                let nc = self.clos[k].ncap as usize;
                let mut j = 0;
                while j < nc && j < self.clos[k].params.len() {
                    let cap = String::from(self.clos[k].params[j].as_str());
                    let mut ck = String::from(pre.as_str());
                    ck.push_str(cap.as_str());
                    let mut own = String::from(cn.as_str());
                    own.push('|');
                    own.push_str(cap.as_str());
                    if oast::in_list(&self.mutcaps, &ck) && !oast::in_list(&self.mutcaps, &own) {
                        self.mutcaps.push(own);
                    }
                    j = j + 1;
                }
            }
            k = k + 1;
        }
    }
    // DEREF-PLACE (+ R4b): record a binding whose initializer borrows a fixed place mutably (`&mut y`, also through an
    // annotation `let w: &mut i32 = &mut y` or a raw-pointer cast `&mut y as *mut i32`) -- the annotated path skipped this,
    // so writes through `w` were LOST (`*w += 41` left y unchanged)
    // R4b PTR: the fixed place a pointer expression designates ("@null", a root name, or "" = unknown)
    fn ptr_root(&self, e: &E) -> String {
        match e {
            E::Ref(x, _m, _l, _h) => oast::place_root(x),
            E::Cast(x, _t, _l, _h) => self.ptr_root(x),
            E::Call(g, ga, _l, _h) if g.as_str() == "$ptrnull" && ga.len() == 0 => {
                String::from("@null")
            }
            E::Var(n, _l, _h) => {
                let mut pre = String::from(n.as_str());
                pre.push('=');
                let mut i = self.ptrinfo.len();
                while i > 0 {
                    i = i - 1;
                    if self.ptrinfo[i].starts_with(pre.as_str()) {
                        return String::from(&self.ptrinfo[i][pre.len()..]);
                    }
                }
                String::new()
            }
            _ => String::new(),
        }
    }
    fn note_ref_binding(&mut self, nm: &str, v0: &E) {
        {
            // R4b PTR: what a pointer binding points at, for is_null / ptr::eq
            let r = match v0 {
                E::Call(g, ga, _l, _h) if g.as_str() == "$ptrnull" && ga.len() == 0 => {
                    String::from("@null")
                }
                _ => self.ptr_root(v0),
            };
            let mut e = String::from(nm);
            e.push('=');
            e.push_str(r.as_str());
            self.ptrinfo.push(e); // "" root = unknown (a later lookup answers "" and refuses)
        }
        let v = match v0 {
            E::Cast(x, _t, _l, _h) => match x.as_ref() {
                E::Ref(_rx, _m, _rl, _rh) => x.as_ref(),
                _ => v0,
            },
            _ => v0,
        };
        if self.hasmain
            && (match v {
                E::Ref(_x, m, _l, _h) => *m,
                _ => false,
            })
        {
            let mut bn = String::from("!");
            bn.push_str(nm);
            self.refvars.push(bn);
            if let E::Ref(px, _pm, _pl, _ph) = v {
                // E0499 guard: a SECOND borrow of the same root (`let r1 = &mut a; let r2 = &mut a;`) is not
                // recorded -- the substitution is sound only for programs rustc accepts, and two live
                // &mut of one root is the classic rejection; keeping the refusal keeps the reject.
                let root = oast::place_root(px);
                let mut clash = false;
                let mut q = 0;
                while q < self.refpe.len() {
                    if oast::place_root(&self.refpe[q]).as_str() == root.as_str() {
                        clash = true;
                    }
                    q = q + 1;
                }
                if clash {
                    self.refpn.push(String::from(nm));
                    self.refpe.push(E::Int(0, 0, 0));
                } else if let Some(pc) = oast::place_clone(px) {
                    self.refpn.push(String::from(nm));
                    self.refpe.push(pc);
                }
            } // DEREF-PLACE: remember WHICH fixed place r borrows
        } else if self.rhs_is_ref(v) {
            self.refvars.push(String::from(nm));
        } /* DEREF-LOCAL: `let r = &mut place` -- a local has no address here, so `*r` (read or write) is REFUSED; it was a silent no-op */
    }
    fn atom(&mut self) -> E {
        // XCRATE-MAC (R2a P5): `l::twice!(21)` -- a #[macro_export] macro reached through an EXTERN crate`s path (the driver
        // marks a spliced crate with `fn __xcrate__`); the crate prefix is skipped and `twice!` expands as usual. A plain
        // module is never marked, so the single-crate form stays refused (rustc E0433)
        if self.i + 3 < self.t.len()
            && tkv_path_sep(&self.t[self.i + 1].k)
            && tkv_bang(&self.t[self.i + 3].k)
        {
            let x = tok_word(&self.t[self.i]);
            let m = tok_word(&self.t[self.i + 2]);
            if x.len() > 0 && m.len() > 0 && self.mac_index(&m) >= 0 {
                let mut mk = String::from(x.as_str());
                mk.push_str("$__xcrate__");
                if self.fknown(&mk) {
                    self.i = self.i + 2;
                }
            }
        }
        // RET-EXPR: `return` in expression position (`if (return) {}`, `f(return)`) -- type `!`
        if self.is_tok("return") {
            let lo = self.lo();
            let _ = self.adv();
            let v = if self.is_tok(")") || self.is_tok(";") || self.is_tok("}") || self.is_tok(",")
            {
                E::Int(0, lo, lo)
            } else {
                self.cmp_e()
            };
            let hi = oast::e_hi(&v);
            return E::RetE(Box::new(v), lo, hi);
        }
        // LABEL: `'outer: for ..` -- the label waits in plabel for the loop it names
        let islbl = match self.t[self.i].k.clone() {
            Tk::Label(_x) => true,
            _ => false,
        };
        if islbl && self.peek_is(1, ":") {
            let w = match self.t[self.i].k.clone() {
                Tk::Label(x) => x,
                _ => String::from(""),
            };
            self.plabel = w;
            let _ = self.adv();
            let _ = self.adv();
            return self.atom();
        }
        // every SOURCE loop pushes its label (or "") for the span of its parse, so `break 'l` can count levels
        if !self.lentry && (self.is_tok("for") || self.is_tok("while") || self.is_tok("loop")) {
            let lb = String::from(self.plabel.as_str());
            self.plabel = String::from("");
            self.lbls.push(lb);
            self.lentry = true;
            let e = self.atom();
            let _ = self.lbls.pop();
            return e;
        }
        self.lentry = false;
        // P-MC-PATH: a leading `crate::` names the crate root; one compilation unit means one root, so at
        // every path position it is a no-op prefix.
        if self.is_tok("crate") && self.peek_is(1, "::") {
            let _ = self.adv();
            let _ = self.adv();
            return self.atom();
        }
        // `move |..| ..` is the same closure as `|..| ..` in our value model (captures are lifted into
        // leading parameters either way), so the keyword is skipped
        let mut is_move = false; // W3 MOVE-CELL: a `move` closure gets a PRIVATE cell for each capture it writes
        if self.i < self.t.len() && self.i + 1 < self.t.len() {
            let w = tok_word(&self.t[self.i]);
            if w.as_str() == "move"
                && (tkv_bar(&self.t[self.i + 1].k) || tkv_or_or(&self.t[self.i + 1].k))
            {
                let _ = self.adv();
                is_move = true;
            }
        }
        // a path may be written from the crate ROOT: `::std::vec::Vec::new()`. The leading `::` adds no
        // segment, so it is skipped.
        if self.is_tok("::") {
            let _ = self.adv();
        }
        let ix = self.adv();
        let lo = self.t[ix].lo;
        let hi = self.t[ix].hi;
        match self.t[ix].k.clone() {
            Tk::StrL(v) => E::StrLit(v, lo, hi),
            Tk::ForT => {
                // the loop's synthetic names must be UNIQUE: nested `for`s sharing `$fi` made the outer
                // increment hit the inner binding's slot, so the outer loop never advanced (lowering keeps a
                // monotonically growing scope and resolves a name to its INNERMOST slot)
                let fsn = self.nfor;
                self.nfor = self.nfor + 1;
                let mut nfi = String::from("$fi");
                nfi.push_str(&oast::u32s2(fsn));
                let mut nfe = String::from("$fe");
                nfe.push_str(&oast::u32s2(fsn));
                let mut nfc = String::from("$fc");
                nfc.push_str(&oast::u32s2(fsn));
                let mut tnames: Vec<String> = Vec::new();
                let mut tsub: Vec<Vec<String>> = Vec::new(); // FOR-NEST: `for (i, (x, y)) in` -- a nested group binds a synthetic name, then its parts
                loop {
                    if self.is_tok("&") {
                        let _ = self.adv();
                        if self.is_tok("mut") {
                            let _ = self.adv();
                        }
                    } else {
                        break;
                    }
                } // P-FOR-REF-PAT: `for &x` / `for &mut x` -- no distinct reference in our value model, so bind through it
                if self.is_tok("(") {
                    let _ = self.adv();
                    loop {
                        if self.is_tok(")") {
                            let _ = self.adv();
                            break;
                        }
                        if self.at_end() {
                            break;
                        }
                        if self.is_tok(",")
                            || self.is_tok("mut")
                            || self.is_word("ref")
                            || self.is_tok("&")
                        {
                            let _ = self.adv();
                            continue;
                        }
                        if self.is_tok("(") {
                            let _ = self.adv();
                            let mut inner: Vec<String> = Vec::new();
                            loop {
                                if self.is_tok(")") {
                                    let _ = self.adv();
                                    break;
                                }
                                if self.at_end() {
                                    break;
                                }
                                if self.is_tok(",")
                                    || self.is_tok("mut")
                                    || self.is_word("ref")
                                    || self.is_tok("&")
                                {
                                    let _ = self.adv();
                                    continue;
                                }
                                let w = tok_word(&self.t[self.i]);
                                let _ = self.adv();
                                if w.len() > 0 {
                                    inner.push(w);
                                }
                            }
                            let mut pn = String::from("$fsub");
                            pn.push_str(&oast::u32s2(fsn));
                            pn.push('_');
                            pn.push_str(&oast::u32s2(tnames.len() as u32));
                            tnames.push(pn);
                            tsub.push(inner);
                            continue;
                        }
                        let w = tok_word(&self.t[self.i]);
                        if w.len() == 0 {
                            let _ = self.adv();
                            continue;
                        }
                        let _ = self.adv();
                        tnames.push(w);
                        tsub.push(Vec::new());
                    }
                }
                let vn = if tnames.len() > 0 {
                    let mut t = String::from("$ftup");
                    t.push_str(&oast::u32s2(fsn));
                    t
                } else {
                    let ni = self.adv();
                    match &self.t[ni].k {
                        Tk::Ident(x) => String::from(x.as_str()),
                        _ => String::from("_"),
                    }
                };
                if self.is_tok("in") {
                    let _ = self.adv();
                }
                let mut mutref = self.is_tok("&") && self.peek_is(1, "mut");
                let src = self.or_e();
                // `v.iter_mut()` is `&mut v`: the loop writes the binding back after the body. The call
                // lowers to the IDENTITY, so the source does not need unwrapping -- which also keeps us
                // off a `*boxed` move, a shape semc miscompiles (it produced a compiler that crashed).
                {
                    match &src {
                        E::MethodCall(_r, m, a2, _l, _h) => {
                            if m.as_str() == "iter_mut" && a2.len() == 0 {
                                mutref = true;
                            }
                        }
                        _ => {}
                    }
                }
                let charbind = {
                    match &src {
                        E::MethodCall(_r, m, _a2, _l, _h) => m.as_str() == "chars",
                        _ => false,
                    }
                }; // P-CHARS-DISPLAY: a char-iterated binding Displays as a char, not its byte code
                   // FOR-BORROW (W1): iterating BY REFERENCE (`&v`, `&mut v`, `v.iter()`, `v.iter_mut()`) binds borrows -- the
                   // element binding must never be dropped (it was: `for x in &v` dropped each element per iteration AND
                   // again with v -- a double drop). The container is named `$fcr..` so the drop schedule can tell.
                let byref = match &src {
                    E::Ref(_x, _m, _l, _h) => true,
                    E::MethodCall(_r, m, a2, _l, _h) => {
                        (m.as_str() == "iter" || m.as_str() == "iter_mut") && a2.len() == 0
                    }
                    _ => false,
                };
                let temp_src = match &src {
                    E::Ref(x, _m, _l, _h) => oast::place_clone(x).is_none(),
                    _ => false,
                }; // `for _ in &[A { .. }]` -- a TEMPORARY the loop owns: dropped after the loop (`$fct`)
                if temp_src {
                    let mut r = String::from("$fct");
                    r.push_str(&oast::u32s2(fsn));
                    nfc = r;
                } else if byref || mutref {
                    let mut r = String::from("$fcr");
                    r.push_str(&oast::u32s2(fsn));
                    nfc = r;
                }
                let mut hi2 = self.t[self.i].hi;
                if self.is_tok("..") {
                    let _ = self.adv();
                    let finc = if self.is_tok("=") {
                        let _ = self.adv();
                        true
                    } else {
                        false
                    };
                    let end = self.or_e();
                    self.eat("{");
                    let body = self.expr();
                    self.eat("}");
                    hi2 = if self.i > 0 {
                        self.t[self.i - 1].hi
                    } else {
                        hi
                    };
                    // for i in a..b  ==>  let $e = b; let $i = a; while $i < $e { let i = $i; body; $i = $i + 1 }
                    let iv = E::Var(String::from(nfi.as_str()), lo, hi2);
                    let inner = E::Let(
                        String::from(vn.as_str()),
                        Box::new(iv),
                        Box::new(body),
                        lo,
                        hi2,
                    );
                    // the step lives in the CONDITION, so `continue` cannot skip it
                    let step = E::Assign(
                        String::from(nfi.as_str()),
                        Box::new(E::Bin(
                            String::from("+"),
                            Box::new(E::Var(String::from(nfi.as_str()), lo, hi2)),
                            Box::new(E::Int(1, lo, hi2)),
                            lo,
                            hi2,
                        )),
                        lo,
                        hi2,
                    );
                    let test = E::Bin(
                        if finc {
                            String::from("<=")
                        } else {
                            String::from("<")
                        },
                        Box::new(E::Var(String::from(nfi.as_str()), lo, hi2)),
                        Box::new(E::Var(String::from(nfe.as_str()), lo, hi2)),
                        lo,
                        hi2,
                    );
                    let cond = E::Seq(Box::new(step), Box::new(test), lo, hi2);
                    let wh = E::While(Box::new(cond), Box::new(inner), lo, hi2);
                    let start1 = E::Bin(
                        String::from("-"),
                        Box::new(src),
                        Box::new(E::Int(1, lo, hi2)),
                        lo,
                        hi2,
                    );
                    let l1 = E::Let(
                        String::from(nfi.as_str()),
                        Box::new(start1),
                        Box::new(wh),
                        lo,
                        hi2,
                    );
                    return E::Let(
                        String::from(nfe.as_str()),
                        Box::new(end),
                        Box::new(l1),
                        lo,
                        hi2,
                    );
                }
                self.eat("{");
                let body = self.expr();
                self.eat("}");
                hi2 = if self.i > 0 {
                    self.t[self.i - 1].hi
                } else {
                    hi
                };
                let body = if tnames.len() > 0 {
                    let mut acc = body;
                    let mut q = tnames.len();
                    while q > 0 {
                        q = q - 1;
                        let sv = E::Var(String::from(vn.as_str()), lo, hi2);
                        let get2 = E::TupIdx(Box::new(sv), q as u32, lo, hi2);
                        if tnames[q].as_str() == "_" {
                            acc = E::Seq(Box::new(get2), Box::new(acc), lo, hi2);
                        } else {
                            let mut j = tsub[q].len();
                            while j > 0 {
                                j = j - 1;
                                if tsub[q][j].as_str() == "_" {
                                    continue;
                                }
                                let pv = E::TupIdx(
                                    Box::new(E::Var(String::from(tnames[q].as_str()), lo, hi2)),
                                    j as u32,
                                    lo,
                                    hi2,
                                );
                                acc = E::Let(
                                    String::from(tsub[q][j].as_str()),
                                    Box::new(pv),
                                    Box::new(acc),
                                    lo,
                                    hi2,
                                );
                            }
                            acc = E::Let(
                                String::from(tnames[q].as_str()),
                                Box::new(get2),
                                Box::new(acc),
                                lo,
                                hi2,
                            );
                        }
                    }
                    acc
                } else {
                    body
                };
                // for x in v  ==>  let $fc = v; let $fi = 0; while $fi < vec_len($fc) { let x = vec_get($fc,$fi); body; $fi = $fi + 1 }
                let mut ga: Vec<E> = Vec::new();
                ga.push(E::Var(String::from(nfc.as_str()), lo, hi2));
                ga.push(E::Var(String::from(nfi.as_str()), lo, hi2));
                let get = E::Call(String::from("seq_get"), ga, lo, hi2);
                let step = E::Assign(
                    String::from(nfi.as_str()),
                    Box::new(E::Bin(
                        String::from("+"),
                        Box::new(E::Var(String::from(nfi.as_str()), lo, hi2)),
                        Box::new(E::Int(1, lo, hi2)),
                        lo,
                        hi2,
                    )),
                    lo,
                    hi2,
                );
                // by-MUTABLE-REFERENCE iteration writes the binding back after the body
                // the step lives in the CONDITION (below), so `continue` cannot skip it
                let inner = if mutref {
                    let mut sa: Vec<E> = Vec::new();
                    sa.push(E::Var(String::from(nfc.as_str()), lo, hi2));
                    sa.push(E::Var(String::from(nfi.as_str()), lo, hi2));
                    sa.push(E::Var(String::from(vn.as_str()), lo, hi2));
                    let back = E::Call(String::from("seq_set"), sa, lo, hi2);
                    E::Let(
                        String::from(vn.as_str()),
                        Box::new(get),
                        Box::new(E::Seq(Box::new(body), Box::new(back), lo, hi2)),
                        lo,
                        hi2,
                    )
                } else if charbind {
                    E::LetT(
                        String::from(vn.as_str()),
                        String::from("char"),
                        Box::new(get),
                        Box::new(body),
                        lo,
                        hi2,
                    )
                } else {
                    E::Let(
                        String::from(vn.as_str()),
                        Box::new(get),
                        Box::new(body),
                        lo,
                        hi2,
                    )
                };
                let mut la: Vec<E> = Vec::new();
                la.push(E::Var(String::from(nfc.as_str()), lo, hi2));
                let test = E::Bin(
                    String::from("<"),
                    Box::new(E::Var(String::from(nfi.as_str()), lo, hi2)),
                    Box::new(E::Call(String::from("seq_len"), la, lo, hi2)),
                    lo,
                    hi2,
                );
                let cond = E::Seq(Box::new(step), Box::new(test), lo, hi2);
                let wh = E::While(Box::new(cond), Box::new(inner), lo, hi2);
                let l1 = E::Let(
                    String::from(nfi.as_str()),
                    Box::new(E::Int(0 - 1, lo, hi2)),
                    Box::new(wh),
                    lo,
                    hi2,
                );
                E::Let(
                    String::from(nfc.as_str()),
                    Box::new(src),
                    Box::new(l1),
                    lo,
                    hi2,
                )
            }
            Tk::Bar => {
                let mut ps: Vec<String> = Vec::new();
                let mut pts: Vec<String> = Vec::new();
                {
                    let mut q = self.i;
                    let mut d = 0;
                    while q < self.t.len() {
                        if d == 0 && tkv_bar(&self.t[q].k) {
                            break;
                        }
                        if tok_opens(&self.t[q]) {
                            d = d + 1;
                        } else if tok_closes(&self.t[q]) {
                            if d == 0 {
                                break;
                            }
                            d = d - 1;
                        }
                        let w = tok_word(&self.t[q]);
                        let wb = w.as_bytes();
                        if wb.len() > 0 && wb[0] >= 97 && wb[0] <= 122 {
                            self.lnames.push(w);
                        }
                        q = q + 1;
                    }
                } // FN-ITEM-VAL: closure params shadow fn names
                let mut destr: Vec<String> = Vec::new();
                let mut destr_of = String::from("");
                let mut destrf: Vec<String> = Vec::new(); // CLOS-STRUCT-PAT: field name per binding ("" = tuple position)
                loop {
                    if self.is_tok("|") {
                        let _ = self.adv();
                        break;
                    }
                    if self.at_end() {
                        break;
                    }
                    if self.is_tok("&")
                        && (self.peek_is(1, "(")
                            || (self.peek_is(1, "mut") && self.peek_is(2, "(")))
                    {
                        let _ = self.adv();
                        if self.is_tok("mut") {
                            let _ = self.adv();
                        }
                    } // CLOS-REF-TUP: `|&(a, b)|` -- a reference pattern is transparent in this value model (as `for &x`)
                    if self.hasmain
                        && destr.len() == 0
                        && (match self.t[self.i].k.clone() {
                            Tk::Ident(_) => true,
                            _ => false,
                        })
                        && self.peek_is(1, "{")
                    {
                        // CLOS-STRUCT-PAT: `|Foo { x: a, y: _b }: Foo|`
                        let sni = self.adv();
                        let spty = tok_word(&self.t[sni]);
                        let _ = self.adv();
                        loop {
                            if self.is_tok("}") {
                                let _ = self.adv();
                                break;
                            }
                            if self.at_end() {
                                break;
                            }
                            if self.is_tok("..") {
                                let _ = self.adv();
                                continue;
                            }
                            let fi = self.adv();
                            let fname = tok_word(&self.t[fi]);
                            let mut bname = String::from(fname.as_str());
                            if self.is_tok(":") {
                                let _ = self.adv();
                                let bi = self.adv();
                                bname = tok_word(&self.t[bi]);
                            }
                            destr.push(bname);
                            destrf.push(fname);
                            if self.is_tok(",") {
                                let _ = self.adv();
                            }
                        }
                        destr_of = String::from("$cp");
                        ps.push(String::from("$cp"));
                        if self.is_tok(":") {
                            let _ = self.adv();
                            let _ = self.parse_tyname();
                        }
                        pts.push(spty);
                        if self.is_tok(",") {
                            let _ = self.adv();
                        }
                        continue;
                    }
                    if self.is_tok("(") {
                        // a tuple PATTERN parameter: one value, destructured in the body
                        let _ = self.adv();
                        let synth = String::from("$cp");
                        loop {
                            if self.is_tok(")") {
                                let _ = self.adv();
                                break;
                            }
                            if self.at_end() {
                                break;
                            }
                            let bi = self.adv();
                            match self.t[bi].k.clone() {
                                Tk::Ident(x) => {
                                    destr.push(String::from(x.as_str()));
                                    destrf.push(String::from(""));
                                }
                                _ => {}
                            }
                            if self.is_tok(",") {
                                let _ = self.adv();
                            }
                        }
                        destr_of = String::from(synth.as_str());
                        ps.push(synth);
                        if self.is_tok(":") {
                            let _ = self.adv();
                            let _ = self.parse_tyname();
                        }
                        pts.push(String::from(""));
                        if self.is_tok(",") {
                            let _ = self.adv();
                        }
                        continue;
                    }
                    // C-CLOSPTY, re-landed in rung 77. Reverted in rung 64 because the linked binary
                    // segfaulted; rung 68 showed that crash was the cross-module struct-return defect, which
                    // rungs 71, 72 and 74 fixed. The alignment bug rung 67 found in the first version is
                    // kept fixed here: `pts` grows exactly when `ps` does.
                    let pi = self.adv();
                    let mut got = false;
                    match self.t[pi].k.clone() {
                        Tk::Ident(x) => {
                            ps.push(String::from(x.as_str()));
                            got = true;
                        }
                        _ => {}
                    }
                    // C-CLOSPTY is REVERTED (rung 64): recording a closure parameter's declared type made
                    // `want_sinst`'s closure call MISCOMPILE -- the callee received a null vector handle and
                    // a string where the index belongs, and the linked binary segfaulted. A refusal beats a
                    // silent miscompile, so the declared type is read and discarded again and the `$push`
                    // site goes back to being refused. See the rung-64 record for the reproducer.
                    if self.is_tok(":") {
                        let _ = self.adv();
                        let pt = sl_param(&self.parse_tyname());
                        if got {
                            pts.push(pt);
                        }
                    } else {
                        if got {
                            pts.push(String::from(""));
                        }
                    }
                    if self.is_tok(",") {
                        let _ = self.adv();
                    }
                }
                // C-CLOSRET: a closure may declare its return type between the params and the body
                // (`|ch:char| -> i64 { .. }`). Probed from rustc: every arity accepts it, and the body is
                // then REQUIRED to be a block. Our closures are lifted with inferred returns, so the
                // declared type is read and discarded -- what matters is not derailing the parse.
                let mut cret = String::from("");
                if self.is_tok("-") && self.peek_is(1, ">") {
                    let _ = self.adv();
                    let _ = self.adv();
                    cret = self.parse_tyname();
                }
                let _ = &cret;
                let body0 = if self.is_tok("{") {
                    let _ = self.adv();
                    let b = self.expr();
                    self.eat("}");
                    b
                } else {
                    self.cmp_e()
                };
                // wrap the destructured names around the body: `|(a, b)| ..` binds a = $cp.0, b = $cp.1
                let body = if destr.len() > 0 {
                    let mut bb = body0;
                    let mut q = destr.len();
                    while q > 0 {
                        q = q - 1;
                        let src = if q < destrf.len() && destrf[q].len() > 0 {
                            E::Field(
                                Box::new(E::Var(String::from(destr_of.as_str()), lo, hi)),
                                String::from(destrf[q].as_str()),
                                lo,
                                hi,
                            )
                        } else {
                            E::TupIdx(
                                Box::new(E::Var(String::from(destr_of.as_str()), lo, hi)),
                                q as u32,
                                lo,
                                hi,
                            )
                        };
                        bb = E::Let(
                            String::from(destr[q].as_str()),
                            Box::new(src),
                            Box::new(bb),
                            lo,
                            hi,
                        );
                    }
                    bb
                } else {
                    body0
                };
                let mut caps: Vec<String> = Vec::new();
                oast::collect_free(&body, &ps, &mut caps);
                {
                    // CLOS-CAP-CLOS (R2a P4): a CALL of a `let`-bound closure is a capture too -- collect_free sees only Var
                    // reads, so `let h = || g(1)` never captured g ("cannot find function g")
                    let mut cn: Vec<String> = Vec::new();
                    oast::collect_calls(&body, &mut cn);
                    let mut z = 0;
                    while z < self.closlets.len() {
                        let w = self.closlets[z].as_str();
                        if oast::in_list(&cn, w)
                            && !oast::in_list(&ps, w)
                            && !oast::in_list(&caps, w)
                        {
                            caps.push(String::from(w));
                        }
                        z = z + 1;
                    }
                }
                let mut nm = String::from("$clos");
                nm.push_str(&oast::u32s2(self.clos.len() as u32));
                let mut allp: Vec<String> = Vec::new();
                let mut allt: Vec<String> = Vec::new();
                let mut k = 0;
                while k < caps.len() {
                    allp.push(String::from(caps[k].as_str()));
                    allt.push(String::from(""));
                    k = k + 1;
                }
                let mut j = 0;
                while j < ps.len() {
                    allp.push(String::from(ps[j].as_str()));
                    let dt = if j < pts.len() {
                        String::from(pts[j].as_str())
                    } else {
                        String::from("")
                    };
                    allt.push(dt);
                    j = j + 1;
                }
                // C-CAPWRITE: a lifted closure receives its captures BY VALUE, so a body that WRITES one
                // writes the copy and the enclosing variable never changes -- `let mut t=0; |x| { t=t+x; }`
                // answered 0 where rustc answers 5, silently. The mutating-method forms (a captured Vec
                // pushed, a captured String grown) only failed later, as an unresolved internal marker.
                // One rule decides all three, and it asks the write test that already knows both shapes.
                {
                    let mut cz = 0;
                    while cz < caps.len() {
                        if assigns_to(&body, caps[cz].as_str()) {
                            let mut key = String::from(nm.as_str());
                            if body_has_io(&body) {
                                key.push_str("$io"); // CLOS-IO-KEY (W7): the closure is renamed `$closN$io` below; its cell key must match
                            }
                            key.push('|');
                            key.push_str(caps[cz].as_str());
                            if is_move {
                                let mut mk = String::from("!");
                                mk.push_str(key.as_str());
                                self.mutcaps.push(mk);
                            } // W3 MOVE-CELL: the closure owns this cell; the creator's variable is not shared
                            self.mutcaps.push(key);
                        } // CLOS-MUT: a WRITTEN capture is recorded (it becomes a shared cell in lowering) instead of refused
                        cz = cz + 1;
                    }
                }
                let io = body_has_io(&body);
                let nm = if io {
                    let mut t = String::from(nm.as_str());
                    t.push_str("$io");
                    t
                } else {
                    nm
                };
                if !is_move {
                    // CLOS-BORROW (W1): a non-move closure that reads a capture only through its FIELDS borrows it -- the creator's
                    // binding still drops at its scope end (`let t = (Foo(0), 0); || { t.1 }` never dropped t)
                    let mut cz = 0;
                    while cz < caps.len() {
                        let mut fs: Vec<String> = Vec::new();
                        if cap_use(&body, caps[cz].as_str(), &mut fs) == 0
                            && !assigns_to(&body, caps[cz].as_str())
                        {
                            let mut bk = String::from("%");
                            bk.push_str(nm.as_str());
                            bk.push('|');
                            bk.push_str(caps[cz].as_str());
                            self.mutcaps.push(bk);
                        }
                        cz = cz + 1;
                    }
                }
                if is_move {
                    let mut mk = String::from("^");
                    mk.push_str(nm.as_str());
                    self.mutcaps.push(mk);
                    let mut cz = 0;
                    while cz < caps.len() {
                        let mut fs: Vec<String> = Vec::new();
                        let u = cap_use(&body, caps[cz].as_str(), &mut fs);
                        if u != 2 {
                            let mut uk = String::from("~");
                            uk.push_str(nm.as_str());
                            uk.push('|');
                            uk.push_str(caps[cz].as_str());
                            uk.push('=');
                            if u == 1 {
                                uk.push('*');
                            } else {
                                let mut q = 0;
                                while q < fs.len() {
                                    if q > 0 {
                                        uk.push(',');
                                    }
                                    uk.push_str(fs[q].as_str());
                                    q = q + 1;
                                }
                            }
                            self.mutcaps.push(uk);
                        }
                        cz = cz + 1;
                    }
                } // CAP-DROP (W1): a `move` closure OWNS its captures; they drop with the closure value
                self.clos.push(Fun {
                    gsub: Vec::new(),
                    pref: clos_pref(allp.len()), // CLOS-PREF (W1): a closure param never drops (its `&`-ness is not recorded)
                    ncap: caps.len() as i32,
                    gbounds: Vec::new(),
                    dflt: 0 - 1,
                    name: String::from(nm.as_str()),
                    params: allp,
                    ptys: allt,
                    ret: self.fret_of_call(&body), // FN-ITEM-RET: a wrapper `|..| f(..)` answers f's declared return
                    gparams: Vec::new(),
                    owner: String::from(""),
                    body: body,
                });
                let h2 = if self.i > 0 {
                    self.t[self.i - 1].hi
                } else {
                    hi
                };
                E::ClosRef(nm, caps, lo, h2)
            }
            Tk::OrOr => {
                // P-CLOSURE-EMPTY: `|| body` -- an empty-parameter closure; `||` lexes as one OrOr
                let ps: Vec<String> = Vec::new();
                let mut cret = String::from("");
                if self.is_tok("-") && self.peek_is(1, ">") {
                    let _ = self.adv();
                    let _ = self.adv();
                    cret = self.parse_tyname();
                }
                let _ = &cret;
                let body = if self.is_tok("{") {
                    let _ = self.adv();
                    let b = self.expr();
                    self.eat("}");
                    b
                } else {
                    self.cmp_e()
                };
                let mut caps: Vec<String> = Vec::new();
                oast::collect_free(&body, &ps, &mut caps);
                {
                    // CLOS-CAP-CLOS (R2a P4): a CALL of a `let`-bound closure is a capture too -- collect_free sees only Var
                    // reads, so `let h = || g(1)` never captured g ("cannot find function g")
                    let mut cn: Vec<String> = Vec::new();
                    oast::collect_calls(&body, &mut cn);
                    let mut z = 0;
                    while z < self.closlets.len() {
                        let w = self.closlets[z].as_str();
                        if oast::in_list(&cn, w)
                            && !oast::in_list(&ps, w)
                            && !oast::in_list(&caps, w)
                        {
                            caps.push(String::from(w));
                        }
                        z = z + 1;
                    }
                }
                let mut nm = String::from("$clos");
                nm.push_str(&oast::u32s2(self.clos.len() as u32));
                let mut allp: Vec<String> = Vec::new();
                let mut allt: Vec<String> = Vec::new();
                let mut k = 0;
                while k < caps.len() {
                    allp.push(String::from(caps[k].as_str()));
                    allt.push(String::from(""));
                    k = k + 1;
                }
                {
                    let mut cz = 0;
                    while cz < caps.len() {
                        if assigns_to(&body, caps[cz].as_str()) {
                            let mut key = String::from(nm.as_str());
                            if body_has_io(&body) {
                                key.push_str("$io"); // CLOS-IO-KEY (W7): the closure is renamed `$closN$io` below; its cell key must match
                            }
                            key.push('|');
                            key.push_str(caps[cz].as_str());
                            if is_move {
                                let mut mk = String::from("!");
                                mk.push_str(key.as_str());
                                self.mutcaps.push(mk);
                            } // W3 MOVE-CELL: the closure owns this cell; the creator's variable is not shared
                            self.mutcaps.push(key);
                        } // CLOS-MUT: a WRITTEN capture is recorded (it becomes a shared cell in lowering) instead of refused
                        cz = cz + 1;
                    }
                }
                let io = body_has_io(&body);
                let nm = if io {
                    let mut t = String::from(nm.as_str());
                    t.push_str("$io");
                    t
                } else {
                    nm
                };
                if !is_move {
                    // CLOS-BORROW (W1): a non-move closure that reads a capture only through its FIELDS borrows it -- the creator's
                    // binding still drops at its scope end (`let t = (Foo(0), 0); || { t.1 }` never dropped t)
                    let mut cz = 0;
                    while cz < caps.len() {
                        let mut fs: Vec<String> = Vec::new();
                        if cap_use(&body, caps[cz].as_str(), &mut fs) == 0
                            && !assigns_to(&body, caps[cz].as_str())
                        {
                            let mut bk = String::from("%");
                            bk.push_str(nm.as_str());
                            bk.push('|');
                            bk.push_str(caps[cz].as_str());
                            self.mutcaps.push(bk);
                        }
                        cz = cz + 1;
                    }
                }
                if is_move {
                    let mut mk = String::from("^");
                    mk.push_str(nm.as_str());
                    self.mutcaps.push(mk);
                    let mut cz = 0;
                    while cz < caps.len() {
                        let mut fs: Vec<String> = Vec::new();
                        let u = cap_use(&body, caps[cz].as_str(), &mut fs);
                        if u != 2 {
                            let mut uk = String::from("~");
                            uk.push_str(nm.as_str());
                            uk.push('|');
                            uk.push_str(caps[cz].as_str());
                            uk.push('=');
                            if u == 1 {
                                uk.push('*');
                            } else {
                                let mut q = 0;
                                while q < fs.len() {
                                    if q > 0 {
                                        uk.push(',');
                                    }
                                    uk.push_str(fs[q].as_str());
                                    q = q + 1;
                                }
                            }
                            self.mutcaps.push(uk);
                        }
                        cz = cz + 1;
                    }
                } // CAP-DROP (W1): a `move` closure OWNS its captures; they drop with the closure value
                self.clos.push(Fun {
                    gsub: Vec::new(),
                    pref: clos_pref(allp.len()), // CLOS-PREF (W1): a closure param never drops (its `&`-ness is not recorded)
                    ncap: caps.len() as i32,
                    gbounds: Vec::new(),
                    dflt: 0 - 1,
                    name: String::from(nm.as_str()),
                    params: allp,
                    ptys: allt,
                    ret: self.fret_of_call(&body), // FN-ITEM-RET: a wrapper `|..| f(..)` answers f's declared return
                    gparams: Vec::new(),
                    owner: String::from(""),
                    body: body,
                });
                let h2 = if self.i > 0 {
                    self.t[self.i - 1].hi
                } else {
                    hi
                };
                E::ClosRef(nm, caps, lo, h2)
            }
            Tk::Amp => {
                let mut mt = false;
                if self.is_tok("mut") {
                    let _ = self.adv();
                    mt = true;
                }
                let x = self.unary();
                let h = oast::e_hi(&x);
                E::Ref(Box::new(x), mt, lo, h)
            }
            Tk::Star => self.unary(),
            Tk::BreakT => {
                // LABEL: `break 'l` leaves every loop up to and including the one labeled 'l: BreakN(levels) with
                // levels = loops crossed beyond the innermost (0 = plain break)
                let lv = self.label_levels(lo);
                // LABEL-BREAK-VALUE: `break 'o v` -- assign the labeled loop's result slot, then leave lv levels
                // R1.8 LABEL-BREAK-COND: a `{` after `break 'l` is the enclosing while's BODY (`'l: while break 'l {}`), not a
                // value -- rustc forbids a block-like value in that position
                if lv >= 0
                    && !self.is_tok(";")
                    && !self.is_tok("}")
                    && !self.is_tok(",")
                    && !self.is_tok(")")
                    && !self.is_tok("{")
                {
                    let q = self.lbls.len() as i64 - 1 - lv;
                    let mut slot = String::from("");
                    if q >= 0 {
                        let mut pre = String::from(self.lbls[q as usize].as_str());
                        pre.push('=');
                        let mut z = 0;
                        while z < self.lvlab.len() {
                            if oast::str_starts(self.lvlab[z].as_str(), pre.as_str()) {
                                slot = String::from(&self.lvlab[z][pre.len()..]);
                            }
                            z = z + 1;
                        }
                    }
                    if slot.len() == 0 {
                        if self.perr == 0 {
                            self.perr = lo + 1;
                            self.perrexp =
                                String::from("`break 'label value` out of a labeled `loop`");
                            self.perrgot = self.tok_text();
                        }
                    }
                    let v = self.cmp_e();
                    let h2 = oast::e_hi(&v);
                    self.lvhit.push(String::from(slot.as_str()));
                    let asg = E::Assign(slot, Box::new(v), lo, h2);
                    let brk = if lv > 0 {
                        E::BreakN(lv as u32, lo, h2)
                    } else {
                        E::BreakE(lo, h2)
                    };
                    return E::Seq(Box::new(asg), Box::new(brk), lo, h2);
                }
                if lv > 0 {
                    return E::BreakN(lv as u32, lo, hi);
                }
                if lv == 0 {
                    return E::BreakE(lo, hi);
                }
                // `break v` inside a `loop` assigns the loop's result before leaving
                let hasv = !self.is_tok(";") && !self.is_tok("}") && self.loopv.len() > 0;
                if hasv {
                    let lv = String::from(self.loopv[self.loopv.len() - 1].as_str());
                    self.lvhit.push(String::from(lv.as_str()));
                    let v = self.cmp_e();
                    let h2 = oast::e_hi(&v);
                    let asg = E::Assign(lv, Box::new(v), lo, h2);
                    E::Seq(Box::new(asg), Box::new(E::BreakE(lo, h2)), lo, h2)
                } else {
                    E::BreakE(lo, hi)
                }
            }
            Tk::ContT => {
                let lv = self.label_levels(lo);
                if lv > 0 {
                    E::ContN(lv as u32, lo, hi)
                } else {
                    E::ContE(lo, hi)
                }
            }
            // `<Type as Trait>::ITEM` / `<Type>::ITEM` -- a qualified path. A prefix `<` is unambiguous
            // here because `<` is otherwise only an infix comparison.
            Tk::Lt => {
                let ty = self.parse_tyname();
                if self.is_tok("as") {
                    let _ = self.adv();
                    let _tr = self.parse_tyname();
                }
                if self.at_gt() {
                    let _ = self.eat_gt();
                }
                if self.is_tok("::") {
                    let _ = self.adv();
                }
                let ii = self.adv();
                let item = match &self.t[ii].k {
                    Tk::Ident(x) => String::from(x.as_str()),
                    _ => String::from("_"),
                };
                let mut nm = String::from(ty.as_str());
                nm.push('$');
                nm.push_str(&item);
                let mut ps: Vec<E> = Vec::new();
                if self.is_tok("(") {
                    let _ = self.adv();
                    loop {
                        if self.is_tok(")") {
                            let _ = self.adv();
                            break;
                        }
                        if self.at_end() {
                            break;
                        }
                        let aprog = self.i;
                        ps.push(self.cmp_e());
                        if self.is_tok(",") {
                            let _ = self.adv();
                        }
                        if self.noprog(aprog) {}
                    }
                }
                let h2 = if self.i > 0 {
                    self.t[self.i - 1].hi
                } else {
                    hi
                };
                E::Call(nm, ps, lo, h2)
            }
            Tk::LoopT => {
                // a `loop` is a VALUE: a result slot is bound around it, `break v` assigns the slot, and
                // the loop's own value is the slot afterwards
                let lsn = self.nfor;
                self.nfor = self.nfor + 1;
                let mut lv = String::from("$lv");
                lv.push_str(&oast::u32s2(lsn));
                self.loopv.push(String::from(lv.as_str()));
                // LABEL-BREAK-VALUE: remember which result slot this loop's LABEL names (`'o: loop` -> "o=$lvN"), for `break 'o v`
                if self.lbls.len() > 0 && self.lbls[self.lbls.len() - 1].len() > 0 {
                    let mut k = String::from(self.lbls[self.lbls.len() - 1].as_str());
                    k.push('=');
                    k.push_str(lv.as_str());
                    self.lvlab.push(k);
                }
                self.eat("{");
                let bd = self.expr();
                self.eat("}");
                let _ = self.loopv.pop();
                let h2 = if self.i > 0 {
                    self.t[self.i - 1].hi
                } else {
                    hi
                };
                let wh = E::While(Box::new(E::Bool(true, lo, h2)), Box::new(bd), lo, h2);
                let used = {
                    let mut r = false;
                    let mut i2 = 0;
                    while i2 < self.lvhit.len() {
                        if self.lvhit[i2].as_str() == lv.as_str() {
                            r = true;
                        }
                        i2 = i2 + 1;
                    }
                    r
                };
                if !used {
                    return wh;
                }
                let tail = E::Var(String::from(lv.as_str()), lo, h2);
                E::Let(
                    lv,
                    Box::new(E::Int(0, lo, h2)),
                    Box::new(E::Seq(Box::new(wh), Box::new(tail), lo, h2)),
                    lo,
                    h2,
                )
            }
            Tk::UnsafeT => {
                if self.is_tok("{") {
                    let _ = self.adv();
                    let b = self.expr();
                    self.eat("}");
                    b
                } else {
                    self.atom()
                }
            }
            Tk::LBrace => {
                // LNAMES-SCOPE (W7): a block's let bindings stop shadowing at its `}` (an outer `baz()` after `{ let baz = .. }`
                // must reach the import / item again; it was refused as unknown)
                let ln0 = self.lnames.len();
                let b = self.expr();
                self.eat("}");
                self.lnames.truncate(ln0);
                b
            }
            Tk::Num(n) => E::Int(n, lo, hi),
            // F-1 (probed from RC): the literal's DECIMAL TEXT is the durable form; the exact
            // decimal->binary64 conversion is strtod's, in the emitted program
            Tk::FloatL(t) => {
                let mut a: Vec<E> = Vec::new();
                a.push(E::StrLit(String::from(t.as_str()), lo, hi));
                E::Call(String::from("f64_parse"), a, lo, hi)
            }
            // P-NOM: a char literal is an integer that KNOWS it is a char, so `{}` can print the
            // character rather than its code point. The cast to a 64-bit name is identity in codegen.
            Tk::ChL(n) => E::Cast(Box::new(E::Int(n, lo, hi)), String::from("char"), lo, hi),
            Tk::TrueT => E::Bool(true, lo, hi),
            Tk::FalseT => E::Bool(false, lo, hi),
            Tk::LBrack => {
                let mut xs: Vec<E> = Vec::new();
                loop {
                    if self.is_tok("]") {
                        let _ = self.adv();
                        break;
                    }
                    if self.at_end() {
                        break;
                    }
                    let x = self.cmp_e();
                    if self.is_tok(";") {
                        // the REPEAT form `[e; n]`
                        let _ = self.adv();
                        let mut cnt: i64 = 0;
                        let mut cexp = E::Int(0, lo, hi);
                        let mut lit_cnt = false;
                        if self.i < self.t.len() {
                            if ilit(&self.t[self.i]) > 0
                                && self.i + 1 < self.t.len()
                                && tkv_r_brack(&self.t[self.i + 1].k)
                            {
                                cnt = ilit_val(&self.t[self.i]);
                                lit_cnt = true;
                                let _ = self.adv();
                            } else {
                                cexp = self.cmp_e();
                            }
                        }
                        loop {
                            if self.is_tok("]") {
                                let _ = self.adv();
                                break;
                            }
                            if self.at_end() {
                                break;
                            }
                            let _ = self.adv();
                        }
                        // ARR-REP-LOOP (C1): a large or NON-literal count is built by a loop, as `vec![x; n]` is -- unrolling a
                        // million copies was the IR-BLOAT root (`[7u8; 1_000_000]`: llc never finished), and a non-literal count
                        // (`[7; N]`, `[1; 2 + 1]`) was parsed and DROPPED, building an EMPTY array (rustc `4 3 8`, ourc panicked)
                        let et = lit_ty(&x);
                        let scalar = et.as_str() == "int"
                            || et.as_str() == "bool"
                            || et.as_str() == "char"
                            || et.as_str() == "f64"
                            || et.as_str() == "str"
                            || oast::is_int_width(et.as_str());
                        if (!lit_cnt || cnt > 64) && scalar {
                            if lit_cnt {
                                cexp = E::Int(cnt, lo, hi);
                            }
                            let h2 = if self.i > 0 {
                                self.t[self.i - 1].hi
                            } else {
                                hi
                            };
                            let mut fa: Vec<E> = Vec::new();
                            fa.push(cexp);
                            fa.push(x);
                            return E::Call(String::from("arr_fill"), fa, lo, h2);
                        }
                        if !lit_cnt || cnt > 4096 {
                            // an AGGREGATE fill (`[[0; 3]; n]`) needs a COPY per slot -- one shared handle would alias every
                            // row; unrolled copies are bounded, so a large or unknown count is refused, never aliased
                            let h2 = if self.i > 0 {
                                self.t[self.i - 1].hi
                            } else {
                                hi
                            };
                            return E::Call(
                                String::from(
                                    "$array_repeat_of_an_aggregate_with_a_large_or_unknown_count",
                                ),
                                Vec::new(),
                                lo,
                                h2,
                            );
                        }
                        let mut q = 0;
                        while q < cnt {
                            xs.push(clone_simple(&x, lo, hi));
                            q = q + 1;
                        }
                        if cnt > 0 {
                            let _ = xs.pop();
                            xs.push(x);
                        }
                        break;
                    }
                    xs.push(x);
                    if self.is_tok(",") {
                        let _ = self.adv();
                    }
                }
                let h2 = if self.i > 0 {
                    self.t[self.i - 1].hi
                } else {
                    hi
                };
                E::ArrLit(xs, lo, h2)
            }
            Tk::Match => {
                let sc = self.cmp_e();
                self.eat("{");
                self.enum_path_collapse();
                self.or_nest_all(); // R9 OR-NEST (after the path collapse: distribute over normalized paths)
                self.or_split_all();
                self.arm_rewrite_all();
                let mut arms: Vec<Arm> = Vec::new(); // PAT-REWRITE
                                                     // P-PAT: the scrutinee is bound ONCE to a synthetic name, so a structural arm pattern can
                                                     // desugar into reads of it (a tuple/struct/slice pattern becomes bindings around the body,
                                                     // a range pattern becomes a guard) without the pattern grammar reaching the lowering
                let msn = self.nfor;
                self.nfor = self.nfor + 1;
                let mut msv = String::from("$ms");
                msv.push_str(&oast::u32s2(msn));
                loop {
                    if self.is_tok("}") {
                        let _ = self.adv();
                        break;
                    }
                    if self.at_end() {
                        break;
                    }
                    if self.is_tok("(") && self.perr == 0 {
                        // REFMUT-TUP (W7): a TUPLE arm pattern with `ref mut` binds the element by value here; `*left = v` in the
                        // arm never reached the scrutinee (two-phase-borrows was WRONG). Refused until tuple elements are places.
                        let mut q = self.i;
                        let mut d = 0;
                        while q + 1 < self.t.len() {
                            if tkv_l_paren(&self.t[q].k)
                                || tkv_l_brack(&self.t[q].k)
                                || tkv_l_brace(&self.t[q].k)
                            {
                                d = d + 1;
                            } else if tkv_r_paren(&self.t[q].k)
                                || tkv_r_brack(&self.t[q].k)
                                || tkv_r_brace(&self.t[q].k)
                            {
                                d = d - 1;
                            }
                            if d <= 0 && tkv_fat_arrow(&self.t[q].k) {
                                break;
                            }
                            if d > 0 && word_is(&self.t[q], "ref") && tkv_mut(&self.t[q + 1].k) {
                                self.perr = self.t[q].lo + 1;
                                self.perrexp = String::from("a tuple-pattern binding other than `ref mut` (writes through it are not modelled)");
                                self.perrgot = String::from("ref mut");
                                break;
                            }
                            q = q + 1;
                        }
                    }
                    // a pattern may be prefixed by `&` / `&mut` (matching through a reference); our value
                    // model has no distinct reference, so the prefix is skipped
                    loop {
                        if self.is_tok("&") {
                            let _ = self.adv();
                            if self.is_tok("mut") {
                                let _ = self.adv();
                            }
                        } else {
                            break;
                        }
                    }
                    // a structural pattern: `(a, b)`, `[a, b]`, `P { x, y }`. Each position is a NAME
                    // (bound from a component read), a LITERAL (an equality test folded into the guard),
                    // or `..` (a rest, which relaxes the length test). A literal AFTER a rest cannot be
                    // addressed positionally, so such a pattern is refused rather than silently matched.
                    // PAT-PAREN: `(P)` with no top-level comma is a GROUPED pattern, not a 1-tuple -- drop the parens
                    if self.is_tok("(") && !self.peek_is(1, ")") && !self.cast_lit_at(self.i) {
                        let mut q = self.i + 1;
                        let mut d = 0;
                        let mut comma = false;
                        let mut close = 0;
                        while q < self.t.len() {
                            if tok_opens(&self.t[q]) {
                                d = d + 1;
                            } else if tok_closes(&self.t[q]) {
                                if d == 0 {
                                    close = q;
                                    break;
                                }
                                d = d - 1;
                            } else if d == 0 && tkv_comma(&self.t[q].k) {
                                comma = true;
                            }
                            q = q + 1;
                        }
                        if !comma && close > 0 {
                            let _ = self.t.remove(close);
                            let _ = self.t.remove(self.i);
                        }
                    }
                    {
                        let dk = if self.cast_lit_at(self.i) {
                            0
                        } else if self.is_tok("(") {
                            1
                        } else if self.is_tok("[") {
                            2
                        } else {
                            let mut r = 0;
                            if self.i + 1 < self.t.len() {
                                let w = tok_word(&self.t[self.i]);
                                if w.len() > 0 && self.sknown(&w) {
                                    if tkv_l_brace(&self.t[self.i + 1].k) {
                                        r = 3;
                                    } else if tkv_l_paren(&self.t[self.i + 1].k) {
                                        r = 4;
                                    }
                                }
                            }
                            r
                        };
                        if dk > 0 {
                            // TUP-REST-ARITY: a tuple-struct arm knows its arity here (the declared field count), so a `..` with
                            // elements after it binds from the END exactly; a plain tuple's arity is only known later (still refused)
                            let dknf: usize = if dk == 4 {
                                split_commas(&self.sfields_of(&tok_word(&self.t[self.i]))).len()
                            } else {
                                0
                            };
                            if dk == 3 || dk == 4 {
                                let _ = self.adv();
                            }
                            let _ = self.adv();
                            let mut ns: Vec<String> = Vec::new();
                            let mut kinds: Vec<i32> = Vec::new();
                            let mut lits: Vec<i64> = Vec::new();
                            let mut slits: Vec<String> = Vec::new(); /* TUP-STR-LIT: a string literal per position ("" = none) */
                            let mut rest = false;
                            let mut bad = false;
                            let mut restat: i32 = 0 - 1;
                            let mut restname = String::from("");
                            loop {
                                if self.is_tok(")") || self.is_tok("]") || self.is_tok("}") {
                                    let _ = self.adv();
                                    break;
                                }
                                if self.at_end() {
                                    break;
                                }
                                // a NESTED group -- `(Some(f), Some(b))`, `[Foo { .. }, _]` -- needs a tag
                                // test per position, which the flat position reader cannot express. Refuse
                                // it: mis-binding it silently answered the wrong arm.
                                if self.cast_lit_at(self.i) {
                                    let nv = ilit_val(&self.t[self.i + 1]);
                                    self.i = self.i + 5;
                                    ns.push(String::from("_"));
                                    kinds.push(1);
                                    lits.push(nv);
                                    continue;
                                } /* PAT-LIT-SUFFIX in a tuple */
                                if self.is_tok("(") || self.is_tok("[") || self.is_tok("{") {
                                    if self.perr == 0 {
                                        self.perr = lo + 1;
                                        self.perrexp =
                                            String::from("a pattern without a nested group");
                                        self.perrgot = self.tok_text();
                                    }
                                    let mut d3 = 0;
                                    loop {
                                        if self.at_end() {
                                            break;
                                        }
                                        if self.is_tok("(") || self.is_tok("[") || self.is_tok("{")
                                        {
                                            d3 = d3 + 1;
                                        } else if self.is_tok(")")
                                            || self.is_tok("]")
                                            || self.is_tok("}")
                                        {
                                            d3 = d3 - 1;
                                            if d3 <= 0 {
                                                let _ = self.adv();
                                                break;
                                            }
                                        }
                                        let _ = self.adv();
                                    }
                                    continue;
                                }
                                // LEX-AT: a `name @ pat` binding is not supported in this flat pattern reader -- refuse it (it was skipped, then miscompiled)
                                // SLICE-REST: `name @ ..` -- the binding just read is the REST of the slice, not an element
                                if (match self.t[self.i].k {
                                    Tk::At => true,
                                    _ => false,
                                }) && self.i + 1 < self.t.len()
                                    && tkv_dot_dot(&self.t[self.i + 1].k)
                                    && ns.len() > 0
                                    && kinds[kinds.len() - 1] == 0
                                {
                                    restname = match ns.pop() {
                                        Some(x) => x,
                                        None => String::from(""),
                                    };
                                    let _ = kinds.pop();
                                    let _ = lits.pop();
                                    let _ = self.adv();
                                    let _ = self.adv();
                                    rest = true;
                                    restat = kinds.len() as i32;
                                    continue;
                                }
                                if (match self.t[self.i].k {
                                    Tk::At => true,
                                    _ => false,
                                }) {
                                    if self.perr == 0 {
                                        self.perr = self.t[self.i].lo + 1;
                                        self.perrexp = String::from(
                                            "a pattern without `@` bindings (not supported yet)",
                                        );
                                        self.perrgot = String::from("@");
                                    }
                                    let _ = self.adv();
                                    continue;
                                }
                                if self.is_tok(",")
                                    || self.is_tok("mut")
                                    || self.is_word("ref")
                                    || self.is_tok("&")
                                {
                                    let _ = self.adv();
                                    continue;
                                }
                                if self.is_tok("..") {
                                    let _ = self.adv();
                                    rest = true;
                                    restat = kinds.len() as i32;
                                    continue;
                                }
                                // TUP-BOOL-LIT: `true`/`false` in a tuple pattern is a LITERAL test (it was read as a binding named `true`, matching anything);
                                // a string literal here has no slot reader -- REFUSE it (it was skipped, shifting every later position)
                                if (match self.t[self.i].k {
                                    Tk::TrueT => true,
                                    Tk::FalseT => true,
                                    _ => false,
                                }) {
                                    let bv = match self.t[self.i].k {
                                        Tk::TrueT => 1,
                                        _ => 0,
                                    };
                                    let _ = self.adv();
                                    ns.push(String::from("_"));
                                    kinds.push(1);
                                    lits.push(bv);
                                    continue;
                                }
                                if (match self.t[self.i].k {
                                    Tk::StrL(_) => true,
                                    _ => false,
                                }) {
                                    let mut sv0 = String::from("S");
                                    match self.t[self.i].k.clone() {
                                        Tk::StrL(x) => sv0.push_str(x.as_str()),
                                        _ => {}
                                    };
                                    let _ = self.adv();
                                    while slits.len() < kinds.len() {
                                        slits.push(String::from(""));
                                    }
                                    ns.push(String::from("_"));
                                    kinds.push(1);
                                    lits.push(0);
                                    slits.push(sv0);
                                    continue;
                                } /* TUP-STR-LIT: compared by content */
                                {
                                    let isn = ilit(&self.t[self.i]) > 0;
                                    if isn {
                                        let nv = ilit_val(&self.t[self.i]);
                                        let _ = self.adv();
                                        ns.push(String::from("_"));
                                        kinds.push(1);
                                        lits.push(nv);
                                        continue;
                                    }
                                }
                                let w = tok_word(&self.t[self.i]);
                                if w.len() == 0 {
                                    let _ = self.adv();
                                    continue;
                                }
                                let _ = self.adv();
                                if self.is_tok(":") {
                                    let _ = self.adv();
                                    {
                                        let isn2 = ilit(&self.t[self.i]) > 0;
                                        if isn2 {
                                            let nv = ilit_val(&self.t[self.i]);
                                            let _ = self.adv();
                                            ns.push(w);
                                            kinds.push(1);
                                            lits.push(nv);
                                            continue;
                                        }
                                    }
                                    let w2 = tok_word(&self.t[self.i]);
                                    if w2.len() > 0 {
                                        let _ = self.adv();
                                        ns.push(w2);
                                    } else {
                                        ns.push(w);
                                    }
                                    kinds.push(0);
                                    lits.push(0);
                                    continue;
                                }
                                ns.push(w);
                                kinds.push(0);
                                lits.push(0);
                            }
                            // TUP-REST-MID: a tuple / tuple-struct arm reads slot q directly, so a `..` with elements AFTER it
                            // bound them from the front (`(.., b, c)` on (1,2,3) gave 1 2, rustc 2 3) -- the arity is unknown
                            // here for a plain tuple: its slots after the rest are marked from-the-END (TUP-REST-END, resolved where the arity is
                            // known); tuple structs use the declared arity (TUP-REST-ARITY), refused only when that is unreadable
                            if (dk == 4 && dknf < kinds.len())
                                && restat >= 0
                                && (restat as usize) < kinds.len()
                                && self.perr == 0
                            {
                                self.perr = lo + 1;
                                self.perrexp = String::from(
                                    "a `..` only at the END of a tuple pattern (a leading or middle rest is not supported yet)",
                                );
                                self.perrgot = String::from("..");
                            }
                            let _ = &bad;
                            let mut hg2 = false;
                            let mut gd2 = E::Bool(true, lo, lo);
                            // positional literal tests, plus a LENGTH test for a slice pattern
                            {
                                let mut q = 0;
                                while q < kinds.len() {
                                    if kinds[q] == 1 {
                                        let sv = E::Var(String::from(msv.as_str()), lo, hi);
                                        let ixe = if restat >= 0 && (q as i32) >= restat {
                                            let mut la2: Vec<E> = Vec::new();
                                            la2.push(E::Var(String::from(msv.as_str()), lo, hi));
                                            let ln2 = E::Call(String::from("seq_len"), la2, lo, hi);
                                            E::Bin(
                                                String::from("-"),
                                                Box::new(ln2),
                                                Box::new(E::Int((kinds.len() - q) as i64, lo, hi)),
                                                lo,
                                                hi,
                                            )
                                        } else {
                                            E::Int(q as i64, lo, hi)
                                        };
                                        let get = if dk == 1 {
                                            E::TupIdx(
                                                Box::new(sv),
                                                tup_slot(q, restat, kinds.len()),
                                                lo,
                                                hi,
                                            )
                                        } else if dk == 2 {
                                            E::Index(Box::new(sv), Box::new(ixe), lo, hi)
                                        } else if dk == 4 {
                                            E::Field(
                                                Box::new(sv),
                                                oast::u32s2(
                                                    rest_slot(q, restat, kinds.len(), dknf) as u32,
                                                ),
                                                lo,
                                                hi,
                                            )
                                        } else {
                                            E::Field(
                                                Box::new(sv),
                                                String::from(ns[q].as_str()),
                                                lo,
                                                hi,
                                            )
                                        };
                                        let rhs = if q < slits.len() && slits[q].len() > 0 {
                                            E::StrLit(String::from(&slits[q][1..]), lo, hi)
                                        } else {
                                            E::Int(lits[q], lo, hi)
                                        };
                                        let eqt = E::Bin(
                                            String::from("=="),
                                            Box::new(get),
                                            Box::new(rhs),
                                            lo,
                                            hi,
                                        );
                                        gd2 = E::Bin(
                                            String::from("&&"),
                                            Box::new(gd2),
                                            Box::new(eqt),
                                            lo,
                                            hi,
                                        );
                                        hg2 = true;
                                    }
                                    q = q + 1;
                                }
                                if dk == 2 {
                                    let mut la: Vec<E> = Vec::new();
                                    la.push(E::Var(String::from(msv.as_str()), lo, hi));
                                    let ln = E::Call(String::from("seq_len"), la, lo, hi);
                                    let opl = if rest {
                                        String::from(">=")
                                    } else {
                                        String::from("==")
                                    };
                                    let lt = E::Bin(
                                        opl,
                                        Box::new(ln),
                                        Box::new(E::Int(kinds.len() as i64, lo, hi)),
                                        lo,
                                        hi,
                                    );
                                    gd2 = E::Bin(
                                        String::from("&&"),
                                        Box::new(gd2),
                                        Box::new(lt),
                                        lo,
                                        hi,
                                    );
                                    hg2 = true;
                                }
                            }
                            if self.is_tok("if") {
                                let _ = self.adv();
                                let g2 = self.or_e();
                                // GUARD-TUPBIND: the guard sees the pattern's bindings (`(x, y) if x + y == 0`) -- they were bound
                                // around the BODY only, so the guard read unbound slots (a wrong arm: `pos` vs rustc `zero`).
                                // Safe to read here: the positional/length tests sit left of this `&&`.
                                let mut gw = g2;
                                let mut q = ns.len();
                                while q > 0 {
                                    q = q - 1;
                                    if kinds[q] != 0 || ns[q].as_str() == "_" {
                                        continue;
                                    }
                                    let sv = E::Var(String::from(msv.as_str()), lo, hi);
                                    let ixe = if restat >= 0 && (q as i32) >= restat {
                                        let mut la3: Vec<E> = Vec::new();
                                        la3.push(E::Var(String::from(msv.as_str()), lo, hi));
                                        let ln3 = E::Call(String::from("seq_len"), la3, lo, hi);
                                        E::Bin(
                                            String::from("-"),
                                            Box::new(ln3),
                                            Box::new(E::Int((kinds.len() - q) as i64, lo, hi)),
                                            lo,
                                            hi,
                                        )
                                    } else {
                                        E::Int(q as i64, lo, hi)
                                    };
                                    let get = if dk == 1 {
                                        E::TupIdx(
                                            Box::new(sv),
                                            tup_slot(q, restat, kinds.len()),
                                            lo,
                                            hi,
                                        )
                                    } else if dk == 2 {
                                        E::Index(Box::new(sv), Box::new(ixe), lo, hi)
                                    } else if dk == 4 {
                                        E::Field(
                                            Box::new(sv),
                                            oast::u32s2(
                                                rest_slot(q, restat, kinds.len(), dknf) as u32
                                            ),
                                            lo,
                                            hi,
                                        )
                                    } else {
                                        E::Field(Box::new(sv), String::from(ns[q].as_str()), lo, hi)
                                    };
                                    gw = E::Let(
                                        String::from(ns[q].as_str()),
                                        Box::new(get),
                                        Box::new(gw),
                                        lo,
                                        hi,
                                    );
                                }
                                gd2 =
                                    E::Bin(String::from("&&"), Box::new(gd2), Box::new(gw), lo, hi);
                                hg2 = true;
                            }
                            self.eat("=>");
                            let b0 = self.cmp_e();
                            if self.is_tok(",") {
                                let _ = self.adv();
                            }
                            let hb = oast::e_hi(&b0);
                            let mut acc = b0;
                            let mut q = ns.len();
                            while q > 0 {
                                q = q - 1;
                                if kinds[q] != 0 {
                                    continue;
                                }
                                let nm2 = String::from(ns[q].as_str());
                                let sv = E::Var(String::from(msv.as_str()), lo, hb);
                                let ixe2 = if restat >= 0 && (q as i32) >= restat {
                                    let mut la3: Vec<E> = Vec::new();
                                    la3.push(E::Var(String::from(msv.as_str()), lo, hb));
                                    let ln3 = E::Call(String::from("seq_len"), la3, lo, hb);
                                    E::Bin(
                                        String::from("-"),
                                        Box::new(ln3),
                                        Box::new(E::Int((kinds.len() - q) as i64, lo, hb)),
                                        lo,
                                        hb,
                                    )
                                } else {
                                    E::Int(q as i64, lo, hb)
                                };
                                let get = if dk == 1 {
                                    E::TupIdx(
                                        Box::new(sv),
                                        tup_slot(q, restat, kinds.len()),
                                        lo,
                                        hb,
                                    )
                                } else if dk == 2 {
                                    E::Index(Box::new(sv), Box::new(ixe2), lo, hb)
                                } else if dk == 4 {
                                    E::Field(
                                        Box::new(sv),
                                        oast::u32s2(rest_slot(q, restat, kinds.len(), dknf) as u32),
                                        lo,
                                        hb,
                                    )
                                } else {
                                    E::Field(Box::new(sv), String::from(ns[q].as_str()), lo, hb)
                                };
                                if nm2.as_str() == "_" {
                                    acc = E::Seq(Box::new(get), Box::new(acc), lo, hb);
                                } else {
                                    acc = E::Let(nm2, Box::new(get), Box::new(acc), lo, hb);
                                }
                            }
                            // SLICE-REST: bind the rest name to scrutinee[restat .. len - (elements after the rest)]
                            if dk == 2 && restname.len() > 0 && restname.as_str() != "_" {
                                let mut la4: Vec<E> = Vec::new();
                                la4.push(E::Var(String::from(msv.as_str()), lo, hb));
                                let ln4 = E::Call(String::from("seq_len"), la4, lo, hb);
                                let endx = E::Bin(
                                    String::from("-"),
                                    Box::new(ln4),
                                    Box::new(E::Int(
                                        (kinds.len() as i64) - (restat as i64),
                                        lo,
                                        hb,
                                    )),
                                    lo,
                                    hb,
                                );
                                let mut sa4: Vec<E> = Vec::new();
                                sa4.push(E::Var(String::from(msv.as_str()), lo, hb));
                                sa4.push(E::Int(restat as i64, lo, hb));
                                sa4.push(endx);
                                acc = E::Let(
                                    String::from(restname.as_str()),
                                    Box::new(E::Call(String::from("seq_slice"), sa4, lo, hb)),
                                    Box::new(acc),
                                    lo,
                                    hb,
                                );
                            }
                            arms.push(Arm {
                                slit: String::from(""),
                                has_slit: false,
                                lit: 0,
                                has_lit: false,
                                has_guard: hg2,
                                guard: gd2,
                                variant: String::from("_"),
                                binds: Vec::new(),
                                body: acc,
                                wild: true,
                            });
                            continue;
                        }
                    }
                    // a RANGE pattern `lo..=hi` / `lo..hi` becomes a guarded wildcard
                    {
                        let ew = if self.cast_lit_at(self.i) {
                            5
                        } else if self.i + 1 < self.t.len()
                            && tkv_minus(&self.t[self.i].k)
                            && self.cast_lit_at(self.i + 1)
                        {
                            6
                        } else if self.i < self.t.len() && tkv_minus(&self.t[self.i].k) {
                            2
                        } else if self.i + 1 < self.t.len() && tkv_path_sep(&self.t[self.i + 1].k) {
                            3
                        } else {
                            1
                        }; // PAT-RANGE-NEG/CONST: the endpoint spans 1-3 tokens
                        let isnum = if self.i < self.t.len() {
                            if ew > 1 {
                                true
                            } else if ilit(&self.t[self.i]) > 0 {
                                true
                            } else {
                                match self.t[self.i].k.clone() {
                                    Tk::Ident(w) => self.cknown(&w),
                                    _ => false,
                                }
                            }
                        } else {
                            false
                        };
                        let rng = isnum
                            && self.i + ew < self.t.len()
                            && tkv_dot_dot(&self.t[self.i + ew].k);
                        if rng {
                            // an endpoint is a literal OR a const name (`s..=e`), so both ends are EXPRESSIONS
                            let e0 = self.pat_endpoint(lo, hi);
                            let _ = self.adv(); // endpoint, then `..`
                            let incl = if self.is_tok("=") {
                                let _ = self.adv();
                                true
                            } else {
                                false
                            };
                            let e1 = self.pat_endpoint(lo, hi);
                            let opc = if incl {
                                String::from("<=")
                            } else {
                                String::from("<")
                            };
                            let gl = E::Bin(
                                String::from(">="),
                                Box::new(E::Var(String::from(msv.as_str()), lo, hi)),
                                Box::new(e0),
                                lo,
                                hi,
                            );
                            let gh = E::Bin(
                                opc,
                                Box::new(E::Var(String::from(msv.as_str()), lo, hi)),
                                Box::new(e1),
                                lo,
                                hi,
                            );
                            let mut gd3 =
                                E::Bin(String::from("&&"), Box::new(gl), Box::new(gh), lo, hi);
                            if self.is_tok("if") {
                                let _ = self.adv();
                                let g2 = self.or_e();
                                gd3 =
                                    E::Bin(String::from("&&"), Box::new(gd3), Box::new(g2), lo, hi);
                            }
                            self.eat("=>");
                            let b2 = self.cmp_e();
                            if self.is_tok(",") {
                                let _ = self.adv();
                            }
                            arms.push(Arm {
                                slit: String::from(""),
                                has_slit: false,
                                lit: 0,
                                has_lit: false,
                                has_guard: true,
                                guard: gd3,
                                variant: String::from("_"),
                                binds: Vec::new(),
                                body: b2,
                                wild: true,
                            });
                            continue;
                        }
                    }
                    // an OR pattern over literals (`1 | 2 => ..`) becomes a guarded wildcard: duplicating
                    // the body is impossible (our `E` has no Clone), and a disjunction of tests is exact
                    {
                        let isn3 = if self.i < self.t.len() {
                            ilit(&self.t[self.i]) > 0
                        } else {
                            false
                        };
                        if isn3 && self.i + 1 < self.t.len() && tkv_bar(&self.t[self.i + 1].k) {
                            let mut gd4 = E::Bool(false, lo, lo);
                            loop {
                                let isn4 = if self.i < self.t.len() {
                                    ilit(&self.t[self.i]) > 0
                                } else {
                                    false
                                };
                                if !isn4 {
                                    break;
                                }
                                let nv = ilit_val(&self.t[self.i]);
                                let _ = self.adv();
                                let eqt = E::Bin(
                                    String::from("=="),
                                    Box::new(E::Var(String::from(msv.as_str()), lo, hi)),
                                    Box::new(E::Int(nv, lo, hi)),
                                    lo,
                                    hi,
                                );
                                gd4 = E::Bin(
                                    String::from("||"),
                                    Box::new(gd4),
                                    Box::new(eqt),
                                    lo,
                                    hi,
                                );
                                if self.is_tok("|") {
                                    let _ = self.adv();
                                } else {
                                    break;
                                }
                            }
                            if self.is_tok("if") {
                                let _ = self.adv();
                                let g5 = self.or_e();
                                gd4 =
                                    E::Bin(String::from("&&"), Box::new(gd4), Box::new(g5), lo, hi);
                            }
                            self.eat("=>");
                            let b5 = self.cmp_e();
                            if self.is_tok(",") {
                                let _ = self.adv();
                            }
                            arms.push(Arm {
                                slit: String::from(""),
                                has_slit: false,
                                lit: 0,
                                has_lit: false,
                                has_guard: true,
                                guard: gd4,
                                variant: String::from("_"),
                                binds: Vec::new(),
                                body: b5,
                                wild: true,
                            });
                            continue;
                        }
                    }
                    // `E::C { x, y } =>` -- a struct-variant PATTERN: the names bind in the variant's
                    // declaration order, so the payload slots line up with the construction above
                    {
                        let w0 = tok_word(&self.t[self.i]);
                        if w0.len() > 0 {
                            let mut vnm = String::from(w0.as_str());
                            let mut look = self.i + 1;
                            if look < self.t.len() && tkv_path_sep(&self.t[look].k) {
                                if look + 1 < self.t.len() {
                                    vnm = tok_word(&self.t[look + 1]);
                                    look = look + 2;
                                }
                            }
                            let vf = self.variant_fields(&vnm);
                            if vf.len() > 0 && look < self.t.len() && tkv_l_brace(&self.t[look].k) {
                                while self.i < look {
                                    let _ = self.adv();
                                }
                                let _ = self.adv();
                                let mut given: Vec<String> = Vec::new();
                                let mut gbind: Vec<String> = Vec::new();
                                let mut litg: Vec<String> = Vec::new();
                                let mut litv: Vec<i64> = Vec::new(); // SV-PAT-FIELD: `f: lit` / `f: name`
                                loop {
                                    if self.is_tok("}") {
                                        let _ = self.adv();
                                        break;
                                    }
                                    if self.at_end() {
                                        break;
                                    }
                                    if self.is_tok(",")
                                        || self.is_tok("..")
                                        || self.is_tok("mut")
                                        || self.is_word("ref")
                                    {
                                        let _ = self.adv();
                                        continue;
                                    }
                                    let w = tok_word(&self.t[self.i]);
                                    if w.len() == 0 {
                                        let _ = self.adv();
                                        continue;
                                    }
                                    let _ = self.adv();
                                    let mut bn = String::from(w.as_str());
                                    if self.is_tok(":") {
                                        let _ = self.adv();
                                        let isn = ilit(&self.t[self.i]) > 0;
                                        let isb = (match self.t[self.i].k {
                                            Tk::TrueT => true,
                                            Tk::FalseT => true,
                                            _ => false,
                                        });
                                        if isn || isb {
                                            let lv = if isn {
                                                ilit_val(&self.t[self.i])
                                            } else {
                                                match self.t[self.i].k {
                                                    Tk::TrueT => 1,
                                                    _ => 0,
                                                }
                                            };
                                            let _ = self.adv();
                                            self.nhyg = self.nhyg + 1;
                                            let mut hn = String::from("__svf");
                                            hn.push_str(&oast::u32s2(self.nhyg));
                                            litg.push(String::from(hn.as_str()));
                                            litv.push(lv);
                                            bn = hn;
                                        } else {
                                            let w2 = tok_word(&self.t[self.i]);
                                            if w2.len() > 0
                                                && !(self.i + 1 < self.t.len()
                                                    && (tkv_l_paren(&self.t[self.i + 1].k)
                                                        || tkv_l_brace(&self.t[self.i + 1].k)
                                                        || tkv_path_sep(&self.t[self.i + 1].k)))
                                            {
                                                let _ = self.adv();
                                                bn = w2;
                                            } else if self.perr == 0 {
                                                self.perr = self.t[self.i].lo + 1;
                                                self.perrexp=String::from("a struct-variant field pattern that is a binding or a literal (nested patterns not supported yet)");
                                                self.perrgot = self.tok_text();
                                            }
                                        }
                                    }
                                    given.push(w);
                                    gbind.push(bn);
                                }
                                let order = split_commas(&vf);
                                let mut binds: Vec<String> = Vec::new();
                                {
                                    let mut oi = 0;
                                    while oi < order.len() {
                                        let mut hit = String::from("$ignore");
                                        let mut q = 0;
                                        while q < given.len() {
                                            if given[q].as_str() == order[oi].as_str() {
                                                hit = String::from(gbind[q].as_str());
                                                q = given.len();
                                            } else {
                                                q = q + 1;
                                            }
                                        }
                                        binds.push(hit);
                                        oi = oi + 1;
                                    }
                                }
                                let mut hg5 = false;
                                let mut gd5 = E::Int(0, lo, lo);
                                if self.is_tok("if") {
                                    let _ = self.adv();
                                    gd5 = self.or_e();
                                    hg5 = true;
                                }
                                {
                                    let mut k = 0;
                                    while k < litg.len() {
                                        let eqt = E::Bin(
                                            String::from("=="),
                                            Box::new(E::Var(
                                                String::from(litg[k].as_str()),
                                                lo,
                                                lo,
                                            )),
                                            Box::new(E::Int(litv[k], lo, lo)),
                                            lo,
                                            lo,
                                        );
                                        gd5 = if hg5 {
                                            E::Bin(
                                                String::from("&&"),
                                                Box::new(eqt),
                                                Box::new(gd5),
                                                lo,
                                                lo,
                                            )
                                        } else {
                                            eqt
                                        };
                                        hg5 = true;
                                        k = k + 1;
                                    }
                                } // SV-PAT-FIELD: literal field tests run before the user guard
                                self.eat("=>");
                                let b5 = self.cmp_e();
                                if self.is_tok(",") {
                                    let _ = self.adv();
                                }
                                arms.push(Arm {
                                    slit: String::from(""),
                                    has_slit: false,
                                    lit: 0,
                                    has_lit: false,
                                    has_guard: hg5,
                                    guard: gd5,
                                    variant: vnm,
                                    binds: binds,
                                    body: b5,
                                    wild: false,
                                });
                                continue;
                            }
                        }
                    }
                    if self.is_word("ref") {
                        let _ = self.adv();
                    }
                    let castl = self.cast_lit_at(self.i);
                    if castl {
                        self.i = self.i + 1;
                    } // PAT-LIT-SUFFIX: `0u8 =>` lexes as `(0 as u8)`
                    let pi = self.adv();
                    if castl {
                        self.i = self.i + 3;
                    }
                    let mut plit: i64 = 0;
                    let mut phas = false;
                    let mut pslit = String::from("");
                    let mut pshas = false;
                    let (vn, wild) = match self.t[pi].k.clone() {
                        Tk::Ident(x) => {
                            if x.as_str() == "_" {
                                (String::from("_"), true)
                            } else {
                                (String::from(x.as_str()), false)
                            }
                        }
                        Tk::Num(nv) => {
                            plit = nv;
                            phas = true;
                            (String::from("_"), true)
                        }
                        Tk::ChL(nv) => {
                            plit = nv;
                            phas = true;
                            (String::from("_"), true)
                        }
                        Tk::StrL(x) => {
                            pslit = String::from(x.as_str());
                            pshas = true;
                            (String::from("_"), true)
                        }
                        Tk::TrueT => {
                            plit = 1;
                            phas = true;
                            (String::from("_"), true)
                        }
                        Tk::FalseT => {
                            plit = 0;
                            phas = true;
                            (String::from("_"), true)
                        }
                        _ => (String::from("_"), true),
                    };
                    // optional `Enum::Variant`
                    if self.is_tok("::") && self.peek_is(1, "<") {
                        let _ = self.adv();
                        self.skip_generics();
                    } // PAT-TURBOFISH: `foo::<T>::arm(..)`
                    let mut vmod = String::new();
                    let mut vpath = String::new(); // CONST-PAT-PATH: `or::X` keeps its module path for a const pattern
                    let mut vname = vn;
                    if self.is_tok("::") {
                        let _ = self.adv();
                        let v2 = self.adv();
                        if let Tk::Ident(x) = self.t[v2].k.clone() {
                            vmod = String::from(vname.as_str());
                            vpath = String::from(vname.as_str());
                            vpath.push('$');
                            vpath.push_str(x.as_str());
                        }
                        vname = match &self.t[v2].k {
                            Tk::Ident(x) => String::from(x.as_str()),
                            _ => vname,
                        };
                    }
                    if self.is_tok("::") && self.peek_is(1, "<") {
                        let _ = self.adv();
                        self.skip_generics();
                    } // PAT-TURBOFISH: `foo::arm::<T>(..)`
                    let mut binds: Vec<String> = Vec::new();
                    if self.is_tok("(") {
                        let _ = self.adv();
                        loop {
                            if self.is_tok(")") {
                                let _ = self.adv();
                                break;
                            }
                            if self.at_end() {
                                break;
                            }
                            if self.is_tok("..")
                                || self.is_word("ref")
                                || self.is_tok("mut")
                                || self.is_tok("&")
                            {
                                let _ = self.adv();
                                continue;
                            }
                            let bi = self.adv();
                            match self.t[bi].k.clone() {
                                Tk::Ident(x) => {
                                    let nmp = String::from(x.as_str());
                                    if nmp.as_str() == "_" {
                                        binds.push(String::from("$ignore"));
                                    } else {
                                        binds.push(nmp);
                                    }
                                }
                                _ => {}
                            }
                            if self.is_tok(",") {
                                let _ = self.adv();
                            }
                        }
                    }
                    let mut hg = false;
                    let mut gd = E::Int(0, lo, lo);
                    if self.is_tok("if") {
                        let _ = self.adv();
                        gd = self.or_e();
                        hg = true;
                    }
                    self.eat("=>");
                    let body = self.cmp_e();
                    if self.is_tok(",") {
                        let _ = self.adv();
                    }
                    let bindall = !wild
                        && binds.len() == 0
                        && !phas
                        && !pshas
                        && self.variant_owner(&vname).len() == 0
                        && !self.cknown(&vname)
                        && !self.sknown(&vname)
                        && !self.enames_has(&vname);
                    if bindall {
                        let bodyb = E::Let(
                            String::from(vname.as_str()),
                            Box::new(E::Var(String::from(msv.as_str()), lo, lo)),
                            Box::new(body),
                            lo,
                            lo,
                        );
                        // GUARD-BINDALL: the guard sees the binding too (`y if y > 1`, `y @ P` -> `y if matches!(y, P)`); it read an unbound slot
                        let gdb = if hg {
                            E::Let(
                                String::from(vname.as_str()),
                                Box::new(E::Var(String::from(msv.as_str()), lo, lo)),
                                Box::new(gd),
                                lo,
                                lo,
                            )
                        } else {
                            gd
                        };
                        arms.push(Arm {
                            slit: String::from(""),
                            has_slit: false,
                            lit: 0,
                            has_lit: false,
                            has_guard: hg,
                            guard: gdb,
                            variant: String::from("_"),
                            binds: Vec::new(),
                            body: bodyb,
                            wild: true,
                        });
                    } else if !wild
                        && binds.len() == 0
                        && !phas
                        && !pshas
                        && self.cknown(&vname)
                        && self.variant_owner(&vname).len() == 0
                    {
                        // CONST-PAT (W7): a const NAME as a pattern is an EQUALITY test (`X => ..` matches only X's value). It was
                        // an arm on an unknown variant, which matched EVERYTHING: `match 1 { X => 10, _ => 1 }` gave 10 (WRONG)
                        if !self.const_pat_ok(&vname) && self.perr == 0 {
                            // CONST-PAT-TY (W7): a static (E0530) or a non-primitive const (a raw pointer, a type without structural
                            // PartialEq) is not an equality pattern here -- refused (it was accepted)
                            self.perr = lo + 1;
                            self.perrexp = String::from("a pattern const of a primitive type (a static, or a const of this type, is not a pattern here)");
                            self.perrgot = String::from(vname.as_str());
                        }
                        let eq = E::Bin(
                            String::from("=="),
                            Box::new(E::Var(String::from(msv.as_str()), lo, lo)),
                            Box::new(E::Call(
                                if vpath.len() > 0 && self.mknown(&vmod) {
                                    String::from(vpath.as_str())
                                } else {
                                    String::from(vname.as_str())
                                },
                                Vec::new(),
                                lo,
                                lo,
                            )),
                            lo,
                            lo,
                        );
                        let g2 = if hg {
                            E::Bin(String::from("&&"), Box::new(eq), Box::new(gd), lo, lo)
                        } else {
                            eq
                        };
                        arms.push(Arm {
                            slit: String::from(""),
                            has_slit: false,
                            lit: 0,
                            has_lit: false,
                            has_guard: true,
                            guard: g2,
                            variant: String::from("_"),
                            binds: Vec::new(),
                            body: body,
                            wild: true,
                        });
                    } else {
                        arms.push(Arm {
                            slit: pslit,
                            has_slit: pshas,
                            lit: plit,
                            has_lit: phas,
                            has_guard: hg,
                            guard: gd,
                            variant: vname,
                            binds: binds,
                            body: body,
                            wild: wild,
                        });
                    }
                }
                let h2 = if self.i > 0 {
                    self.t[self.i - 1].hi
                } else {
                    hi
                };
                let m = E::Match(
                    Box::new(E::Var(String::from(msv.as_str()), lo, h2)),
                    arms,
                    lo,
                    h2,
                );
                E::Let(msv, Box::new(sc), Box::new(m), lo, h2)
            }
            // `assert!(c)` / `assert_eq!(a, b)` -> a real check with a PANIC (exit 101, as rustc does), so an
            // asserting run-pass test genuinely verifies behavior instead of being ignored
            Tk::Ident(s) if self.is_tok("!") && self.mac_index(&s) >= 0 => {
                // a USER macro: expand it in place and parse the expansion
                let mi = self.mac_index(&s) as usize;
                self.i = self.i - 1; // back up onto the macro name
                                     // MR-STMT: a call that STARTS a statement (`..; mk!(z, 41); z`) expands to STATEMENTS -- its `let`s bind the rest
                                     // of the block. The rule body's outer `{ }` and the call's `;` are dropped so the expansion is read in place.
                let stmt = self.i == 0
                    || tkv_semi(&self.t[self.i - 1].k)
                    || tkv_l_brace(&self.t[self.i - 1].k)
                    || tkv_r_brace(&self.t[self.i - 1].k);
                if self.mac_expand(mi) {
                    if stmt && self.is_tok("{") {
                        let o = self.i;
                        let mut d = 0;
                        let mut c = o;
                        while c < self.t.len() {
                            if tok_opens(&self.t[c]) {
                                d = d + 1;
                            } else if tok_closes(&self.t[c]) {
                                d = d - 1;
                                if d == 0 {
                                    break;
                                }
                            }
                            c = c + 1;
                        }
                        if c + 1 < self.t.len() && tkv_semi(&self.t[c + 1].k) && {
                            let mut hl = false;
                            let mut q = o + 1;
                            let mut dd = 0;
                            while q < c {
                                if tok_opens(&self.t[q]) {
                                    dd = dd + 1;
                                } else if tok_closes(&self.t[q]) {
                                    dd = dd - 1;
                                } else if dd == 0 && tkv_let(&self.t[q].k) {
                                    hl = true;
                                }
                                q = q + 1;
                            }
                            hl
                        } {
                            let mut nt: Vec<Tok> = Vec::new();
                            let mut k = 0;
                            while k < self.t.len() {
                                if k != o && k != c && k != c + 1 {
                                    nt.push(tok_clone(&self.t[k]));
                                }
                                k = k + 1;
                            }
                            self.t = nt;
                            return self.expr();
                        }
                    }
                    // the rule body may already be unbraced: an expansion that BEGINS with `let` binds the rest of the block
                    if stmt && self.is_tok("let") {
                        // the expansion's own `let ..;` is followed by the CALL's `;` -- drop that second one
                        let mut q = self.i;
                        let mut d = 0;
                        while q < self.t.len() {
                            if tok_opens(&self.t[q]) {
                                d = d + 1;
                            } else if tok_closes(&self.t[q]) {
                                d = d - 1;
                            } else if d == 0 && tkv_semi(&self.t[q].k) {
                                break;
                            }
                            q = q + 1;
                        }
                        if q + 1 < self.t.len() && tkv_semi(&self.t[q + 1].k) {
                            let mut nt: Vec<Tok> = Vec::new();
                            let mut k = 0;
                            while k < self.t.len() {
                                if k != q + 1 {
                                    nt.push(tok_clone(&self.t[k]));
                                }
                                k = k + 1;
                            }
                            self.t = nt;
                        }
                        return self.expr();
                    }
                    return self.cmp_e();
                }
                let _ = self.adv();
                let _ = self.adv();
                if self.perr == 0 {
                    self.perr = self.lo() + 1;
                }
                E::Int(0, lo, hi)
            }
            // BI-CONCAT / BI-STRINGIFY (rustc_builtin_macros): concat! joins its LITERALS' text into one &'static str;
            // stringify! renders its tokens as source text (spaces between tokens, none before , ) . ; or after ( .)
            Tk::Ident(s)
                if self.is_tok("!") && (s.as_str() == "concat" || s.as_str() == "stringify") =>
            {
                let isc = s.as_str() == "concat";
                let _ = self.adv();
                let _ = self.adv(); // `!` and the opening delimiter
                let mut out = String::from("");
                let mut d = 0;
                let mut neg = false;
                let mut prevtxt = String::from("");
                let mut prevhi: u32 = 0; // STRINGIFY-SPACING: the previous token's source end (0 = none yet)
                loop {
                    if self.at_end() {
                        break;
                    }
                    if tok_opens(&self.t[self.i]) {
                        d = d + 1;
                    } else if tok_closes(&self.t[self.i]) {
                        if d == 0 {
                            let _ = self.adv();
                            break;
                        }
                        d = d - 1;
                    }
                    // CONCAT-NESTED (W2): a concat! argument that is itself a macro invocation (`concat!(..)`, a user macro
                    // expanding to a literal) is EXPANDED as an expression first; its string literal joins the result
                    // (it was read as raw tokens: `concat!(concat!("hel", "lo"), "world")` lost the inner call)
                    if isc && d == 0 && is_ident(&self.t[self.i]) && self.peek_is(1, "!") {
                        let ne = self.cmp_e();
                        match ne {
                            E::StrLit(x, _l2, _h2) => out.push_str(x.as_str()),
                            _ => {
                                if self.perr == 0 {
                                    self.perr = lo + 1;
                                    self.perrexp = String::from(
                                        "a concat! argument that expands to a string literal",
                                    );
                                    self.perrgot =
                                        String::from("a macro expanding to another value");
                                }
                            }
                        }
                        continue;
                    }
                    let txt = match self.t[self.i].k.clone() {
                        Tk::StrL(x) => {
                            if isc {
                                x
                            } else {
                                let mut q = String::from("\"");
                                q.push_str(x.as_str());
                                q.push('"');
                                q
                            }
                        }
                        Tk::ChL(n) => {
                            if isc {
                                let mut c = String::from("");
                                c.push((n as u8) as char);
                                c
                            } else {
                                let mut c = String::from("'");
                                c.push((n as u8) as char);
                                c.push('\'');
                                c
                            }
                        }
                        Tk::TrueT => String::from("true"),
                        Tk::FalseT => String::from("false"),
                        _ => self.tok_text(),
                    };
                    if isc {
                        if d == 0 && self.is_tok(",") {
                        } else if self.is_tok("-") {
                            neg = true;
                        } else {
                            if neg {
                                out.push('-');
                                neg = false;
                            }
                            out.push_str(txt.as_str());
                        }
                    } else {
                        let nosp_before = txt.as_str() == ","
                            || txt.as_str() == ")"
                            || txt.as_str() == "."
                            || txt.as_str() == ";"
                            || txt.as_str() == "]";
                        let nosp_after_prev = prevtxt.as_str() == "("
                            || prevtxt.as_str() == "."
                            || prevtxt.as_str() == "["
                            || prevtxt.as_str() == "!";
                        let call_or_index = (txt.as_str() == "(" || txt.as_str() == "[")
                            && prevtxt.len() > 0
                            && (is_ident(&self.t[self.i - 1])
                                || prevtxt.as_str() == ")"
                                || prevtxt.as_str() == "]");
                        // STRINGIFY-SPACING (W2): rustc keeps SOURCE spacing -- tokens adjacent in the source (`std::option`,
                        // `Vec<i64>`) print with no space; a source gap prints one (`a + b`)
                        let adjacent = prevhi > 0 && self.t[self.i].lo == prevhi;
                        if out.len() > 0
                            && !adjacent
                            && !nosp_before
                            && !nosp_after_prev
                            && !call_or_index
                        {
                            out.push(' ');
                        }
                        out.push_str(txt.as_str());
                        prevtxt = txt;
                        prevhi = self.t[self.i].hi;
                    }
                    let _ = self.adv();
                }
                let h2 = if self.i > 0 {
                    self.t[self.i - 1].hi
                } else {
                    hi
                };
                E::StrLit(out, lo, h2)
            }
            Tk::Ident(s)
                if self.is_tok("!")
                    && (s.as_str() == "format"
                        || s.as_str() == "write"
                        || s.as_str() == "writeln") =>
            {
                let _ = self.adv();
                self.eat("(");
                if s.as_str() == "format" {
                    self.strvals.push(lo);
                } // BCK-STRMOVE: format! makes a String
                  // `write!`/`writeln!` take a sink first; we ignore the sink and build the string
                if s.as_str() != "format" {
                    let _ = self.cmp_e();
                    if self.is_tok(",") {
                        let _ = self.adv();
                    }
                }
                let mut fmt = String::from("");
                let mut args: Vec<E> = Vec::new();
                if !self.is_tok(")") {
                    let fi = self.adv();
                    match self.t[fi].k.clone() {
                        Tk::StrL(x) => {
                            fmt = String::from(x.as_str());
                        }
                        _ => {}
                    }
                    loop {
                        if self.is_tok(",") {
                            let _ = self.adv();
                            if self.is_tok(")") {
                                break;
                            }
                            args.push(self.cmp_e());
                        } else {
                            break;
                        }
                    }
                }
                {
                    let (f2, a2) = fmt_named(&fmt, args, lo, hi);
                    fmt = f2;
                    args = a2;
                } // FMT-NAMED
                if self.is_tok(")") {
                    let _ = self.adv();
                }
                let h2 = if self.i > 0 {
                    self.t[self.i - 1].hi
                } else {
                    hi
                };
                let fb = fmt.as_bytes();
                let mut parts: Vec<String> = Vec::new();
                let mut cur = String::from("");
                let mut k = 0;
                let mut prec_at: Vec<usize> = Vec::new();
                let mut prec_of: Vec<i32> = Vec::new();
                let mut pad_at: Vec<usize> = Vec::new();
                let mut pad_w: Vec<i32> = Vec::new();
                let mut pad_al: Vec<i32> = Vec::new();
                let mut pad_fill: Vec<String> = Vec::new();
                let mut pad_prec: Vec<i32> = Vec::new();
                let mut fdbg_at: Vec<usize> = Vec::new(); /* FMT-PAD + FMT-DBG-STR in format! */
                while k < fb.len() {
                    if k + 1 < fb.len()
                        && ((fb[k] == 123 && fb[k + 1] == 123)
                            || (fb[k] == 125 && fb[k + 1] == 125))
                    {
                        cur.push_str(&fmt[k..(k + 1)]);
                        k = k + 2;
                        continue;
                    } /* FMT-ESC: `{{` / `}}` are a literal brace */
                    let (kd, sk) = fmt_spec(&fmt, k);
                    if kd >= 0 {
                        parts.push(String::from(cur.as_str()));
                        cur = String::from("");
                        k = k + (sk as usize);
                        if kd == 1 {
                            fdbg_at.push(parts.len() - 1);
                        }
                        if kd >= 2 {
                            prec_at.push(parts.len() - 1);
                            prec_of.push(kd - 2);
                        }
                    } else {
                        let (pb, pw, pal, pfill, pprec, _pdbg) = fmt_pad_spec(&fmt, k);
                        if pb > 0 {
                            parts.push(String::from(cur.as_str()));
                            cur = String::from("");
                            k = k + (pb as usize);
                            pad_at.push(parts.len() - 1);
                            pad_w.push(pw);
                            pad_al.push(pal);
                            pad_fill.push(pfill);
                            pad_prec.push(pprec);
                        } else {
                            if fb[k] == 123 && self.badfmt == 0 {
                                self.badfmt = lo + 1;
                            }
                            cur.push_str(&fmt[k..(k + 1)]);
                            k = k + 1;
                        }
                    }
                }
                parts.push(cur);
                let mut argsr: Vec<E> = Vec::new();
                loop {
                    match args.pop() {
                        Some(a) => {
                            argsr.push(a);
                        }
                        None => {
                            break;
                        }
                    }
                }
                let mut acc = E::StrLit(String::from(""), lo, h2);
                let mut pi2 = 0;
                while pi2 < parts.len() {
                    if parts[pi2].len() > 0 {
                        let mut c1: Vec<E> = Vec::new();
                        c1.push(acc);
                        c1.push(E::StrLit(String::from(parts[pi2].as_str()), lo, h2));
                        acc = E::Call(String::from("str_cat"), c1, lo, h2);
                    }
                    match argsr.pop() {
                        Some(a) => {
                            let mut sa: Vec<E> = Vec::new();
                            // C-FMTPREC: `{:.N}` asks the prelude's fixed-precision float printer, which
                            // is what the shortest-repr printer already calls under the hood
                            let pz = {
                                let mut r = 0 - 1;
                                let mut z = 0;
                                while z < prec_at.len() {
                                    if prec_at[z] == pi2 {
                                        r = prec_of[z];
                                    }
                                    z = z + 1;
                                }
                                r
                            };
                            let pdi = {
                                let mut r = 0 - 1;
                                let mut z = 0;
                                while z < pad_at.len() {
                                    if pad_at[z] == pi2 {
                                        r = z as i32;
                                    }
                                    z = z + 1;
                                }
                                r
                            };
                            let pz = if pdi >= 0 { pad_prec[pdi as usize] } else { pz };
                            let mut fdbg_hit = false;
                            {
                                let mut z = 0;
                                while z < fdbg_at.len() {
                                    if fdbg_at[z] == pi2 {
                                        fdbg_hit = true;
                                    }
                                    z = z + 1;
                                }
                            } // FMT-DBG-STR (a flag: the block-as-condition shape mis-evaluated under SEM0 -- see ledger)
                            let conv = if pz == 1000000 {
                                sa.push(a);
                                E::Call(String::from("f64_sci"), sa, lo, h2)
                            } else if pz >= 0 {
                                sa.push(E::Int(pz as i64, lo, h2));
                                sa.push(a);
                                E::Call(String::from("f64_snprintf"), sa, lo, h2)
                            } else if fdbg_hit {
                                sa.push(a);
                                E::Call(String::from("str_dbg"), sa, lo, h2)
                            }
                            // FMT-DBG-STR: {:?} inside format!, resolved by type in lowering
                            else {
                                sa.push(a);
                                E::Call(String::from("str_of"), sa, lo, h2)
                            };
                            let conv = if pdi >= 0 {
                                let mut pa: Vec<E> = Vec::new();
                                pa.push(conv);
                                pa.push(E::Int(pad_w[pdi as usize] as i64, lo, h2));
                                pa.push(E::Int(pad_al[pdi as usize] as i64, lo, h2));
                                pa.push(E::StrLit(
                                    String::from(pad_fill[pdi as usize].as_str()),
                                    lo,
                                    h2,
                                ));
                                E::Call(String::from("fmtpad_call"), pa, lo, h2)
                            } else {
                                conv
                            };
                            let mut c2: Vec<E> = Vec::new();
                            c2.push(acc);
                            c2.push(conv);
                            acc = E::Call(String::from("str_cat"), c2, lo, h2);
                        }
                        None => {}
                    }
                    pi2 = pi2 + 1;
                }
                return acc;
            }
            Tk::Ident(s) if self.is_tok("!") && s.as_str() == "vec" => {
                // `vec![a, b, c]` == `{ let $v = vec_new(); $v.push(a); ..; $v }`
                let _ = self.adv();
                let close = if self.is_tok("[") {
                    String::from("]")
                } else {
                    String::from(")")
                };
                let _ = self.adv();
                let mut els: Vec<E> = Vec::new();
                let mut rep = E::Int(0, lo, hi);
                let mut isrep = false;
                loop {
                    if self.is_tok(close.as_str()) {
                        let _ = self.adv();
                        break;
                    }
                    if self.at_end() {
                        break;
                    }
                    let aprog = self.i;
                    let e1 = self.cmp_e();
                    if self.is_tok(";") {
                        // `vec![x; n]` -- the REPEAT form
                        let _ = self.adv();
                        rep = self.cmp_e();
                        isrep = true;
                        els.push(e1);
                        loop {
                            if self.is_tok(close.as_str()) {
                                let _ = self.adv();
                                break;
                            }
                            if self.at_end() {
                                break;
                            }
                            let _ = self.adv();
                        }
                        break;
                    }
                    els.push(e1);
                    if self.is_tok(",") {
                        let _ = self.adv();
                    }
                    if self.noprog(aprog) {}
                }
                let h2 = if self.i > 0 {
                    self.t[self.i - 1].hi
                } else {
                    hi
                };
                let vty = {
                    let mut t = String::from("Vec");
                    // C-VECLITANN: the DECLARED element wins over the first element's own type.
                    // `let v: Vec<Box<dyn T>> = vec![Box::new(A), Box::new(B)]` typed the temp
                    // vector from A and then refused B -- the annotation was right there, in the
                    // channel rung 94 opened for Default::default().
                    {
                        let ae = {
                            let b = self.annty.as_bytes();
                            let mut r = String::from("");
                            if b.len() > 4 && b[0] == 86 && b[1] == 101 && b[2] == 99 && b[3] == 36
                            {
                                r = String::from(&self.annty[4..]);
                            }
                            r
                        };
                        if ae.len() > 0 {
                            t.push('$');
                            t.push_str(ae.as_str());
                        } else if els.len() > 0 {
                            let e0 = lit_ty(&els[0]);
                            if e0.len() > 0 {
                                t.push('$');
                                t.push_str(e0.as_str());
                            }
                        }
                    }
                    t
                };
                let vn = String::from("$vecl");
                let mut tail = E::Var(String::from(vn.as_str()), lo, h2);
                if isrep {
                    // `let $v = vec_new(); for $i in 0..n { $v.push(x); } $v`
                    let mut ar: Vec<E> = Vec::new();
                    ar.push(E::Var(String::from(vn.as_str()), lo, h2));
                    match els.pop() {
                        Some(x) => {
                            ar.push(x);
                        }
                        None => {
                            ar.push(E::Int(0, lo, h2));
                        }
                    }
                    let push = E::Call(String::from("vec_push"), ar, lo, h2);
                    let iv = String::from("$vri");
                    let step = E::Assign(
                        String::from(iv.as_str()),
                        Box::new(E::Bin(
                            String::from("+"),
                            Box::new(E::Var(String::from(iv.as_str()), lo, h2)),
                            Box::new(E::Int(1, lo, h2)),
                            lo,
                            h2,
                        )),
                        lo,
                        h2,
                    );
                    let cond = E::Bin(
                        String::from("<"),
                        Box::new(E::Var(String::from(iv.as_str()), lo, h2)),
                        Box::new(rep),
                        lo,
                        h2,
                    );
                    let wh = E::While(
                        Box::new(cond),
                        Box::new(E::Seq(Box::new(push), Box::new(step), lo, h2)),
                        lo,
                        h2,
                    );
                    let body = E::Let(
                        iv,
                        Box::new(E::Int(0, lo, h2)),
                        Box::new(E::Seq(Box::new(wh), Box::new(tail), lo, h2)),
                        lo,
                        h2,
                    );
                    let mk2: Vec<E> = Vec::new();
                    return E::LetT(
                        vn,
                        vty,
                        Box::new(E::Call(String::from("vec_new"), mk2, lo, h2)),
                        Box::new(body),
                        lo,
                        h2,
                    );
                }
                let mut q = els.len();
                while q > 0 {
                    q = q - 1;
                    match els.pop() {
                        Some(x) => {
                            let mut ar: Vec<E> = Vec::new();
                            ar.push(E::Var(String::from(vn.as_str()), lo, h2));
                            ar.push(x);
                            let push = E::Call(String::from("vec_push"), ar, lo, h2);
                            tail = E::Seq(Box::new(push), Box::new(tail), lo, h2);
                        }
                        None => {}
                    }
                }
                let mk: Vec<E> = Vec::new();
                return E::LetT(
                    vn,
                    vty,
                    Box::new(E::Call(String::from("vec_new"), mk, lo, h2)),
                    Box::new(tail),
                    lo,
                    h2,
                );
            }
            Tk::Ident(s)
                if self.is_tok("!")
                    && (s.as_str() == "cfg"
                        || s.as_str() == "stringify"
                        || s.as_str() == "line"
                        || s.as_str() == "file"
                        || s.as_str() == "column") =>
            {
                // `cfg!(..)` with no configuration flags is false; the others are placeholders
                let _ = self.adv();
                let argstart = self.i; // BI-POS-REFUSE: remember the argument tokens
                if self.is_tok("(") {
                    let mut d = 0;
                    loop {
                        if self.at_end() {
                            break;
                        }
                        if self.is_tok("(") {
                            d = d + 1;
                        }
                        if self.is_tok(")") {
                            d = d - 1;
                            let _ = self.adv();
                            if d == 0 {
                                break;
                            }
                            continue;
                        }
                        let _ = self.adv();
                    }
                }
                let h2 = if self.i > 0 {
                    self.t[self.i - 1].hi
                } else {
                    hi
                };
                // BI-POS-REFUSE: these were PLACEHOLDERS -- line!/column!/file! answered 0 (silent wrong answers); cfg!(..) answered false, which
                // is right for an unknown flag but WRONG for flags rustc's default build sets (debug_assertions, unix, target_*, ...).
                {
                    let mut k = argstart;
                    let mut maytrue = false;
                    while k < self.i {
                        let w = tok_word(&self.t[k]);
                        if w.as_str() == "debug_assertions"
                            || w.as_str() == "unix"
                            || w.as_str() == "windows"
                            || w.as_str() == "overflow_checks"
                            || w.as_str() == "panic"
                            || oast::str_starts(w.as_str(), "target_")
                            || w.as_str() == "all"
                            || w.as_str() == "any"
                            || w.as_str() == "not"
                        {
                            maytrue = true;
                        }
                        k = k + 1;
                    }
                    if (s.as_str() != "cfg" || maytrue) && self.perr == 0 {
                        self.perr = lo + 1;
                        let mut m = String::from("a macro other than `");
                        m.push_str(s.as_str());
                        m.push_str("!` here (line!/column!/file! and cfg! of a build-set flag are not implemented)");
                        self.perrexp = m;
                        self.perrgot = String::from(s.as_str());
                    }
                }
                if s.as_str() == "cfg" {
                    E::Bool(false, lo, h2)
                } else {
                    E::Int(0, lo, h2)
                }
            }
            Tk::Ident(s)
                if self.is_tok("!")
                    && (s.as_str() == "panic"
                        || s.as_str() == "unreachable"
                        || s.as_str() == "todo"
                        || s.as_str() == "unimplemented") =>
            {
                let _ = self.adv();
                if self.is_tok("(") {
                    let mut d = 0;
                    loop {
                        if self.at_end() {
                            break;
                        }
                        if self.is_tok("(") {
                            d = d + 1;
                        }
                        if self.is_tok(")") {
                            d = d - 1;
                            let _ = self.adv();
                            if d == 0 {
                                break;
                            }
                            continue;
                        }
                        let _ = self.adv();
                    }
                }
                let h2 = if self.i > 0 {
                    self.t[self.i - 1].hi
                } else {
                    hi
                };
                let pa: Vec<E> = Vec::new();
                E::Call(String::from("panic_exit"), pa, lo, h2)
            }
            Tk::Ident(s)
                if self.is_tok("!")
                    && (s.as_str() == "assert"
                        || s.as_str() == "assert_eq"
                        || s.as_str() == "assert_ne") =>
            {
                let kind = String::from(s.as_str());
                let _ = self.adv();
                self.eat("(");
                let a = self.cmp_e();
                let mut b = E::Int(0, lo, lo);
                let mut two = false;
                if self.is_tok(",") {
                    let _ = self.adv();
                    if !self.is_tok(")") {
                        b = self.cmp_e();
                        two = true;
                    }
                }
                loop {
                    if self.is_tok(")") {
                        let _ = self.adv();
                        break;
                    }
                    if self.at_end() {
                        break;
                    }
                    let _ = self.adv();
                }
                let h2 = if self.i > 0 {
                    self.t[self.i - 1].hi
                } else {
                    hi
                };
                let cond = if two {
                    let op = if kind.as_str() == "assert_ne" {
                        String::from("==")
                    } else {
                        String::from("!=")
                    };
                    E::Bin(op, Box::new(a), Box::new(b), lo, h2)
                } else {
                    E::Not(Box::new(a), lo, h2)
                };
                let pa: Vec<E> = Vec::new();
                let thn = E::Call(String::from("panic_exit"), pa, lo, h2);
                return E::If(
                    Box::new(cond),
                    Box::new(thn),
                    Box::new(E::Int(0, lo, h2)),
                    lo,
                    h2,
                );
            }
            Tk::Ident(s)
                if self.is_tok("!") && (s.as_str() == "eprintln" || s.as_str() == "eprint") =>
            {
                // EPRINT (R2a P4): the print! / println! reader, with stdout pointed at stderr around it (stdio_swap)
                let pn = if s.as_str() == "eprintln" {
                    "println"
                } else {
                    "print"
                };
                self.t[self.i - 1].k = Tk::Ident(String::from(pn));
                self.i = self.i - 1;
                let inner = self.atom();
                let mut a1: Vec<E> = Vec::new();
                a1.push(E::Int(1, lo, hi));
                let mut a0: Vec<E> = Vec::new();
                a0.push(E::Int(0, lo, hi));
                let on = E::Call(String::from("stdio_swap"), a1, lo, hi);
                let off = E::Call(String::from("stdio_swap"), a0, lo, hi);
                E::Seq(
                    Box::new(on),
                    Box::new(E::Seq(Box::new(inner), Box::new(off), lo, hi)),
                    lo,
                    hi,
                )
            }
            Tk::Ident(s)
                if self.is_tok("!") && (s.as_str() == "println" || s.as_str() == "print") =>
            {
                let nl = s.as_str() == "println";
                let _ = self.adv();
                self.eat("(");
                let mut fmt = String::from("");
                let mut args: Vec<E> = Vec::new();
                if !self.is_tok(")") {
                    let fi = self.adv();
                    match self.t[fi].k.clone() {
                        Tk::StrL(x) => {
                            fmt = String::from(x.as_str());
                        }
                        _ => {}
                    }
                    loop {
                        if self.is_tok(",") {
                            let _ = self.adv();
                            if self.is_tok(")") {
                                break;
                            }
                            args.push(self.cmp_e());
                        } else {
                            break;
                        }
                    }
                }
                {
                    let (f2, a2) = fmt_named(&fmt, args, lo, hi);
                    fmt = f2;
                    args = a2;
                } // FMT-NAMED
                if self.is_tok(")") {
                    let _ = self.adv();
                }
                let h2 = if self.i > 0 {
                    self.t[self.i - 1].hi
                } else {
                    hi
                };
                // split the format string on `{}`
                let fb = fmt.as_bytes();
                let mut parts: Vec<String> = Vec::new();
                let mut cur = String::from("");
                let mut k = 0;
                let mut dbg_at: Vec<usize> = Vec::new();
                let mut prec_at: Vec<usize> = Vec::new();
                let mut prec_of: Vec<i32> = Vec::new();
                let mut pad_at: Vec<usize> = Vec::new();
                let mut pad_w: Vec<i32> = Vec::new();
                let mut pad_al: Vec<i32> = Vec::new();
                let mut pad_fill: Vec<String> = Vec::new();
                let mut pad_prec: Vec<i32> = Vec::new();
                while k < fb.len() {
                    if k + 1 < fb.len()
                        && ((fb[k] == 123 && fb[k + 1] == 123)
                            || (fb[k] == 125 && fb[k + 1] == 125))
                    {
                        cur.push_str(&fmt[k..(k + 1)]);
                        k = k + 2;
                        continue;
                    } /* FMT-ESC: `{{` / `}}` are a literal brace */
                    let (kd, sk) = fmt_spec(&fmt, k);
                    if kd >= 0 {
                        parts.push(String::from(cur.as_str()));
                        cur = String::from("");
                        k = k + (sk as usize);
                        // `{:?}` -- for an int or a str this is the same output as `{}`; a user type needs
                        // its derived Debug, which print_dbg walks
                        if kd == 1 {
                            dbg_at.push(parts.len() - 1);
                        }
                        if kd >= 2 {
                            prec_at.push(parts.len() - 1);
                            prec_of.push(kd - 2);
                        }
                    } else {
                        let (pb, pw, pal, pfill, pprec, pdbg) = fmt_pad_spec(&fmt, k);
                        if pb > 0 {
                            parts.push(String::from(cur.as_str()));
                            cur = String::from("");
                            k = k + (pb as usize); // FMT-PAD
                            pad_at.push(parts.len() - 1);
                            pad_w.push(pw);
                            pad_al.push(pal);
                            pad_fill.push(pfill);
                            pad_prec.push(pprec);
                            let _ = pdbg;
                        } else {
                            if fb[k] == 123 && self.badfmt == 0 {
                                self.badfmt = lo + 1;
                            }
                            cur.push_str(&fmt[k..(k + 1)]);
                            k = k + 1;
                        }
                    }
                }
                parts.push(cur);
                // the arguments are CONSUMED in order (E has no Clone): reverse once, then pop
                let mut argsr: Vec<E> = Vec::new();
                loop {
                    match args.pop() {
                        Some(a) => {
                            argsr.push(a);
                        }
                        None => {
                            break;
                        }
                    }
                }
                // F-ARGS: evaluate the non-place arguments FIRST, into temporaries, and leave the places
                // to be read when they are printed -- which is where rustc reads them
                let mut pre: Vec<E> = Vec::new();
                let mut prenames: Vec<String> = Vec::new();
                {
                    let mut argsr2: Vec<E> = Vec::new();
                    let mut idx = 0;
                    loop {
                        match argsr.pop() {
                            Some(a) => {
                                if is_place_expr(&a) {
                                    argsr2.push(a);
                                } else {
                                    // a TUPLE LITERAL keeps its shape -- the Debug printer reads its element types
                                    // from it -- so its ELEMENTS are what get bound, not the tuple
                                    let istup = match &a {
                                        E::TupLit(p2, _l2, _h2) => p2.len() > 0,
                                        _ => false,
                                    };
                                    if istup {
                                        match a {
                                            E::TupLit(mut p2, l2, h3) => {
                                                let mut np2: Vec<E> = Vec::new();
                                                let mut rev: Vec<E> = Vec::new();
                                                loop {
                                                    match p2.pop() {
                                                        Some(x) => {
                                                            rev.push(x);
                                                        }
                                                        None => {
                                                            break;
                                                        }
                                                    }
                                                }
                                                loop {
                                                    match rev.pop() {
                                                        Some(x) => {
                                                            if is_place_expr(&x) {
                                                                np2.push(x);
                                                            } else {
                                                                let mut nm = String::from("$pa");
                                                                nm.push_str(&oast::u32s2(
                                                                    self.nfor,
                                                                ));
                                                                nm.push('_');
                                                                nm.push_str(&oast::u32s2(idx));
                                                                np2.push(E::Var(
                                                                    String::from(nm.as_str()),
                                                                    l2,
                                                                    h3,
                                                                ));
                                                                prenames.push(String::from(
                                                                    nm.as_str(),
                                                                ));
                                                                pre.push(x);
                                                                idx = idx + 1;
                                                            }
                                                        }
                                                        None => {
                                                            break;
                                                        }
                                                    }
                                                }
                                                argsr2.push(E::TupLit(np2, l2, h3));
                                            }
                                            _ => {}
                                        }
                                    } else {
                                        let mut nm = String::from("$pa");
                                        nm.push_str(&oast::u32s2(self.nfor));
                                        nm.push('_');
                                        nm.push_str(&oast::u32s2(idx));
                                        argsr2.push(E::Var(String::from(nm.as_str()), lo, h2));
                                        prenames.push(String::from(nm.as_str()));
                                        pre.push(a);
                                    }
                                }
                                idx = idx + 1;
                            }
                            None => {
                                break;
                            }
                        }
                    }
                    self.nfor = self.nfor + 1;
                    // argsr is popped from the END below, so restore that order
                    loop {
                        match argsr2.pop() {
                            Some(a) => {
                                argsr.push(a);
                            }
                            None => {
                                break;
                            }
                        }
                    }
                }
                let mut seq: Vec<E> = Vec::new();
                let mut pi2 = 0;
                while pi2 < parts.len() {
                    if parts[pi2].len() > 0 {
                        let mut a1: Vec<E> = Vec::new();
                        a1.push(E::StrLit(String::from(parts[pi2].as_str()), lo, h2));
                        seq.push(E::Call(String::from("print_raw"), a1, lo, h2));
                    }
                    match argsr.pop() {
                        Some(a) => {
                            // FMT-PAD: a width spec formats the argument to text, pads it, and prints it raw
                            let pdi = {
                                let mut r = 0 - 1;
                                let mut z = 0;
                                while z < pad_at.len() {
                                    if pad_at[z] == pi2 {
                                        r = z as i32;
                                    }
                                    z = z + 1;
                                }
                                r
                            };
                            if pdi >= 0 {
                                let pz2 = pad_prec[pdi as usize];
                                let sx = if pz2 >= 0 {
                                    let mut fa: Vec<E> = Vec::new();
                                    fa.push(E::Int(pz2 as i64, lo, h2));
                                    fa.push(a);
                                    E::Call(String::from("f64_snprintf"), fa, lo, h2)
                                } else {
                                    let mut sa: Vec<E> = Vec::new();
                                    sa.push(a);
                                    E::Call(String::from("str_of"), sa, lo, h2)
                                };
                                let mut pa: Vec<E> = Vec::new();
                                pa.push(sx);
                                pa.push(E::Int(pad_w[pdi as usize] as i64, lo, h2));
                                pa.push(E::Int(pad_al[pdi as usize] as i64, lo, h2));
                                pa.push(E::StrLit(
                                    String::from(pad_fill[pdi as usize].as_str()),
                                    lo,
                                    h2,
                                ));
                                let mut ra: Vec<E> = Vec::new();
                                ra.push(E::Call(String::from("fmtpad_call"), pa, lo, h2));
                                seq.push(E::Call(String::from("print_raw"), ra, lo, h2));
                                pi2 = pi2 + 1;
                                continue;
                            }
                            let pz = {
                                let mut r = 0 - 1;
                                let mut z = 0;
                                while z < prec_at.len() {
                                    if prec_at[z] == pi2 {
                                        r = prec_of[z];
                                    }
                                    z = z + 1;
                                }
                                r
                            };
                            if pz == 1000000 {
                                let mut fa: Vec<E> = Vec::new();
                                fa.push(a);
                                let mut ra: Vec<E> = Vec::new();
                                ra.push(E::Call(String::from("f64_sci"), fa, lo, h2));
                                seq.push(E::Call(String::from("print_raw"), ra, lo, h2));
                            }
                            /* FMT-SCI */
                            else if pz >= 0 {
                                let mut fa: Vec<E> = Vec::new();
                                fa.push(E::Int(pz as i64, lo, h2));
                                fa.push(a);
                                let st = E::Call(String::from("f64_snprintf"), fa, lo, h2);
                                let mut ra: Vec<E> = Vec::new();
                                ra.push(st);
                                seq.push(E::Call(String::from("print_raw"), ra, lo, h2));
                            } else {
                                let mut a2: Vec<E> = Vec::new();
                                a2.push(a);
                                let isdbg = {
                                    let mut r = false;
                                    let mut z = 0;
                                    while z < dbg_at.len() {
                                        if dbg_at[z] == pi2 {
                                            r = true;
                                        }
                                        z = z + 1;
                                    }
                                    r
                                };
                                let prim = if isdbg {
                                    String::from("print_dbg")
                                } else {
                                    String::from("print_any")
                                };
                                seq.push(E::Call(prim, a2, lo, h2));
                            }
                        }
                        None => {}
                    }
                    pi2 = pi2 + 1;
                }
                if nl {
                    let a3: Vec<E> = Vec::new();
                    seq.push(E::Call(String::from("print_nl"), a3, lo, h2));
                }
                let mut out = E::Int(0, lo, h2);
                let mut q = seq.len();
                while q > 0 {
                    q = q - 1;
                    match seq.pop() {
                        Some(x) => {
                            out = E::Seq(Box::new(x), Box::new(out), lo, h2);
                        }
                        None => {}
                    }
                }
                // wrap the temporaries around the prints, innermost LAST so they evaluate left to right
                {
                    let mut w = pre.len();
                    while w > 0 {
                        w = w - 1;
                        match pre.pop() {
                            Some(v) => {
                                let nm = String::from(prenames[w].as_str());
                                out = E::Let(nm, Box::new(v), Box::new(out), lo, h2);
                            }
                            None => {}
                        }
                    }
                }
                return out;
            }
            Tk::Ident(s)
                if self.sknown_here(&s)
                    && self.sfields_of(&self.sres(&s)).len() == 0
                    && !self.is_tok("::")
                    && !self.is_tok("(")
                    && !self.is_tok("{") =>
            {
                // a unit struct used as a value
                let fv: Vec<E> = Vec::new();
                E::StructLit(self.sres(&s), fv, lo, hi)
            }
            // P-MC-VARIANT (REFUTED attempt, kept as a comment so the next rung does not retry it): adding
            // `&& !self.sknown_here(&s)` here took the composed unit from 3 errors to 94. sknown_here
            // consults GLOBS, and in the composed unit every module globs seventeen others, so the guard
            // fires for any variant whose name matches any struct anywhere. The real fix is to scope the
            // VARIANT table by module (register variants under the declaring module prefix) rather than to
            // add a preference here.
            Tk::Ident(s)
                if self.variant_owner(&s).len() > 0
                    && !self.is_tok("::")
                    && !(self.hasmain
                        && !self.is_tok("(")
                        && (oast::in_list(&self.lnames, s.as_str())
                            || oast::in_list(&self.cparams, s.as_str()))) =>
            {
                /* VAR-SHADOWS-VARIANT: a local `a` is not the variant `a` */
                // a bare VARIANT constructor: `Some(x)` / `None`, the way Rust's prelude exposes them
                let own = self.variant_owner(&s);
                let mut ps: Vec<E> = Vec::new();
                if self.is_tok("(") {
                    let _ = self.adv();
                    loop {
                        if self.is_tok(")") {
                            let _ = self.adv();
                            break;
                        }
                        if self.at_end() {
                            break;
                        }
                        let aprog = self.i;
                        ps.push(self.cmp_e());
                        if self.is_tok(",") {
                            let _ = self.adv();
                        }
                        if self.noprog(aprog) {}
                    }
                }
                let h2 = if self.i > 0 {
                    self.t[self.i - 1].hi
                } else {
                    hi
                };
                E::EnumLit(own, String::from(s.as_str()), ps, lo, h2)
            }
            // a read of a MUTABLE STATIC loads the global
            Tk::Ident(s) if self.is_smut(&s) && !self.is_tok("::") && !self.is_tok("(") => {
                let mut gn = String::from("$gload$");
                gn.push_str(s.as_str());
                let a: Vec<E> = Vec::new();
                E::Call(gn, a, lo, hi)
            }
            Tk::Ident(s) if self.use_alias(&s).len() > 0 && !self.is_tok("::") => {
                // a name a `use` imported: `use m::fifteen;` then `fifteen()` is `m$fifteen()`
                let full = self.use_alias(&s);
                // PATH-STRUCT-LIT (R2a P2): `use a::S; S { .. }` -- an imported STRUCT before `{` is a literal; this arm
                // made it the call `a$S()` and left `{ .. }` as stray input (glob imports and `a::S {` already took slit_named)
                if self.is_tok("{") && self.sknown(&full) {
                    return self.slit_named(&full, lo, hi);
                }
                let mut ps: Vec<E> = Vec::new();
                if self.is_tok("(") {
                    let _ = self.adv();
                    loop {
                        if self.is_tok(")") {
                            let _ = self.adv();
                            break;
                        }
                        if self.at_end() {
                            break;
                        }
                        let aprog = self.i;
                        ps.push(self.cmp_e());
                        if self.is_tok(",") {
                            let _ = self.adv();
                        }
                        if self.noprog(aprog) {}
                    }
                }
                let h2 = if self.i > 0 {
                    self.t[self.i - 1].hi
                } else {
                    hi
                };
                if self.enames_has(&full) {
                    E::Call(full, ps, lo, h2)
                } else {
                    E::Call(full, ps, lo, h2)
                }
            }
            Tk::Ident(s)
                if self.cbind.len() > 0 && !self.is_tok("::") && !self.is_tok("(") && {
                    let mut pre = String::from(s.as_str());
                    pre.push('=');
                    let mut hit = false;
                    let mut q = 0;
                    while q < self.cbind.len() {
                        if oast::str_starts(self.cbind[q].as_str(), pre.as_str()) {
                            hit = true;
                        }
                        q = q + 1;
                    }
                    hit && !oast::in_list(&self.lnames, s.as_str())
                } =>
            {
                // CONST-GEN-LEN (R3): a const param bound to an array param's length reads that length -- `0..N` in
                // `fn sum<const N: usize>(a: [i64; N])` is `0..a.len()` (exact: an array carries its length)
                let mut pre = String::from(s.as_str());
                pre.push('=');
                let mut pn = String::new();
                let mut q = 0;
                while q < self.cbind.len() {
                    if oast::str_starts(self.cbind[q].as_str(), pre.as_str()) {
                        pn = String::from(&self.cbind[q][pre.len()..]);
                    }
                    q = q + 1;
                }
                let h2 = if self.i > 0 {
                    self.t[self.i - 1].hi
                } else {
                    hi
                };
                E::MethodCall(
                    Box::new(E::Var(pn, lo, h2)),
                    String::from("len"),
                    Vec::new(),
                    lo,
                    h2,
                )
            }
            Tk::Ident(s) => {
                // FN-ITEM-VAL: a fn NAME used as a VALUE (`ap(dbl, 4)`, `let f = dbl;`, `S { f: dbl }`) becomes the closure
                // `|__fv0, ..| dbl(__fv0, ..)` (closure values dispatch through the lifted-closure table, which has no entry
                // for a plain fn). Not when the name is a parameter or a let-bound local (which shadow it).
                let fv_ctx = self.i >= 2
                    && (tkv_l_paren(&self.t[self.i - 2].k)
                        || tkv_comma(&self.t[self.i - 2].k)
                        || tkv_eq(&self.t[self.i - 2].k)
                        || tkv_colon(&self.t[self.i - 2].k))
                    && (self.is_tok(",")
                        || self.is_tok(")")
                        || self.is_tok(";")
                        || self.is_tok("}")); // a VALUE-passing position only (an arm body `=> v` or `x + v` is never rewritten)
                if fv_ctx
                    && !self.namesonly
                    && !self.is_tok("(")
                    && !self.is_tok("!")
                    && !self.is_tok("::")
                    && !oast::in_list(&self.cparams, s.as_str())
                    && !oast::in_list(&self.lnames, s.as_str())
                {
                    let mut ar: i32 = if self.hasmain && s.as_str() == "drop" {
                        1
                    } else {
                        0 - 1
                    };
                    {
                        /* FN-ITEM-VAL: the builtin `drop` as a value (`.map(drop)`) */
                        let mut pre = String::from(s.as_str());
                        pre.push('=');
                        let mut q = 0;
                        while q < self.farity.len() {
                            if oast::str_starts(self.farity[q].as_str(), pre.as_str()) {
                                ar = oast::parse_i32(&self.farity[q][pre.len()..]);
                            }
                            q = q + 1;
                        }
                    }
                    if ar >= 0 {
                        let at = self.i - 1;
                        let mk = |k: Tk| Tok {
                            k: k,
                            lo: lo,
                            hi: hi,
                        };
                        let mut nt: Vec<Tok> = Vec::new();
                        let mut k = 0;
                        while k < at {
                            nt.push(tok_clone(&self.t[k]));
                            k = k + 1;
                        }
                        nt.push(mk(Tk::Bar));
                        let mut a = 0;
                        while a < ar {
                            if a > 0 {
                                nt.push(mk(Tk::Comma));
                            }
                            let mut an = String::from("__fv");
                            an.push_str(&oast::u32s2(a as u32));
                            nt.push(mk(Tk::Ident(an)));
                            a = a + 1;
                        }
                        nt.push(mk(Tk::Bar));
                        nt.push(mk(Tk::Ident(String::from(s.as_str()))));
                        nt.push(mk(Tk::LParen));
                        a = 0;
                        while a < ar {
                            if a > 0 {
                                nt.push(mk(Tk::Comma));
                            }
                            let mut an = String::from("__fv");
                            an.push_str(&oast::u32s2(a as u32));
                            nt.push(mk(Tk::Ident(an)));
                            a = a + 1;
                        }
                        nt.push(mk(Tk::RParen));
                        k = at + 1;
                        while k < self.t.len() {
                            nt.push(tok_clone(&self.t[k]));
                            k = k + 1;
                        }
                        self.t = nt;
                        self.i = at;
                        return self.atom();
                    }
                }
                // a const name used as a VALUE. No `{` guard: `for i in 0..L {` legitimately has a brace after the
                // name, and a struct literal is recognised by its own name table anyway
                if self.cknown(&s)
                    && !self.is_tok("(")
                    && !self.is_tok("::")
                    && !(oast::in_list(&self.lnames, s.as_str())
                        || oast::in_list(&self.cparams, s.as_str()))
                {
                    /* VAR-SHADOWS-CONST: a local `i` is not the global `i` */
                    let ps: Vec<E> = Vec::new();
                    return E::Call(String::from(s.as_str()), ps, lo, hi);
                }
                if self.is_tok("::") {
                    // P-PATH: collect the WHOLE path, then resolve it once (the old code matched
                    // `Ident::Ident` as a shape, so `a::b::c` and `Type::assoc::<T>` derailed)
                    let mut segs: Vec<String> = Vec::new();
                    segs.push(String::from(s.as_str()));
                    self.tfish = String::new(); // G1 G-P2: this path's own turbofish only
                    self.tfish2 = String::new();
                    self.tfishtx = String::new();
                    self.read_path_tail(&mut segs);
                    let tfp = String::from(self.tfish.as_str());
                    let tfp2 = String::from(self.tfish2.as_str());
                    let tfraw = String::from(self.tfishtx.as_str()); // lossless text of the FIRST turbofish type
                                                                     // USE-REL: `geo::sq(..)` where `pub use self::area::sq;` inside geo bound the alias `geo$sq`
                    if segs.len() >= 2 && self.is_tok("(") {
                        let mut joined = String::from(segs[0].as_str());
                        let mut q = 1;
                        while q < segs.len() {
                            joined.push('$');
                            joined.push_str(segs[q].as_str());
                            q = q + 1;
                        }
                        if !self.fknown(&joined) {
                            let mut j = 0;
                            while j < self.usen.len() {
                                if self.usen[j].as_str() == joined.as_str() {
                                    let t = String::from(self.uset[j].as_str());
                                    segs = Vec::new();
                                    segs.push(t);
                                    break;
                                }
                                j = j + 1;
                            }
                        }
                    }
                    // F64-CONSTS: `f64::NAN` / INFINITY / NEG_INFINITY / MAX / MIN / EPSILON / MIN_POSITIVE are float literals
                    // (a float literal is f64_parse of its text; strtod reads "nan"/"inf")
                    if segs.len() == 2 && segs[0].as_str() == "f64" && !self.is_tok("(") {
                        let txt = match segs[1].as_str() {
                            "NAN" => "nan",
                            "INFINITY" => "inf",
                            "NEG_INFINITY" => "-inf",
                            "MAX" => "1.7976931348623157e308",
                            "MIN" => "-1.7976931348623157e308",
                            "EPSILON" => "2.220446049250313e-16",
                            "MIN_POSITIVE" => "2.2250738585072014e-308",
                            _ => "",
                        };
                        if txt.len() > 0 {
                            let h2 = if self.i > 0 {
                                self.t[self.i - 1].hi
                            } else {
                                hi
                            };
                            let mut a: Vec<E> = Vec::new();
                            a.push(E::StrLit(String::from(txt), lo, h2));
                            return E::Call(String::from("f64_parse"), a, lo, h2);
                        }
                    }
                    // DFLT-ENUM: `E::default()` on an enum with a `#[default]` variant is that (unit) variant
                    if segs.len() == 2
                        && segs[1].as_str() == "default"
                        && self.is_tok("(")
                        && self.peek_is(1, ")")
                    {
                        let mut pre = String::from(segs[0].as_str());
                        pre.push('=');
                        let mut dv = String::from("");
                        let mut q = 0;
                        while q < self.edflt.len() {
                            if oast::str_starts(self.edflt[q].as_str(), pre.as_str()) {
                                dv = String::from(&self.edflt[q][pre.len()..]);
                            }
                            q = q + 1;
                        }
                        if dv.len() > 0 {
                            let _ = self.adv();
                            let _ = self.adv();
                            let h2 = if self.i > 0 {
                                self.t[self.i - 1].hi
                            } else {
                                hi
                            };
                            return E::EnumLit(
                                String::from(segs[0].as_str()),
                                dv,
                                Vec::new(),
                                lo,
                                h2,
                            );
                        }
                    }
                    let mut ps: Vec<E> = Vec::new();
                    let pcall = self.is_tok("("); // MOD-STATIC-MUT: was this path CALLED (vs read as a value)?
                    if self.is_tok("(") {
                        let _ = self.adv();
                        loop {
                            if self.is_tok(")") {
                                let _ = self.adv();
                                break;
                            }
                            if self.at_end() {
                                break;
                            }
                            let aprog = self.i;
                            ps.push(self.cmp_e());
                            if self.is_tok(",") {
                                let _ = self.adv();
                            }
                            if self.noprog(aprog) {}
                        }
                    }
                    let h2 = if self.i > 0 {
                        self.t[self.i - 1].hi
                    } else {
                        hi
                    };
                    // UFCS-TRAIT: `Tr::m(&x, a..)` / `Tr::<T>::m(..)` is the method call `x.m(a..)` (the receiver's `&` is auto-ref)
                    if self.hasmain
                        && segs.len() == 2
                        && ps.len() >= 1
                        && oast::in_list(&self.trnames, segs[0].as_str())
                    {
                        let r0 = ps.remove(0);
                        let recv = match r0 {
                            E::Ref(x, _m, _a, _b) => *x,
                            other => other,
                        };
                        return E::MethodCall(
                            Box::new(recv),
                            String::from(segs[1].as_str()),
                            ps,
                            lo,
                            h2,
                        );
                    }
                    if segs.len() == 1 {
                        // a pure TURBOFISH (`None::<String>`): the path is one segment, so this is the
                        // bare-variant/const/ident case the arms above would have taken without the `::`
                        if self.variant_owner(&s).len() > 0 {
                            return E::EnumLit(
                                self.variant_owner(&s),
                                String::from(s.as_str()),
                                ps,
                                lo,
                                h2,
                            );
                        }
                        let mut ps = ps;
                        let mut tc: Vec<E> = Vec::new();
                        while self.tfconst.len() > 0 {
                            tc.push(self.tfconst.remove(0));
                        }
                        self.cg_append(segs[0].as_str(), tc, &mut ps); // CONST-GEN-ARG: `f::<3>()` passes 3 as the hidden const param
                                                                       // G1 G-P2: `sz::<u64>()` -- a one-segment turbofish call records its types for the mono demand
                        if tfp.len() > 0 && self.hasmain && self.fknown(&segs[0]) {
                            let mut e = oast::u32s2(lo);
                            e.push('|');
                            e.push_str(tfp.as_str());
                            e.push('|');
                            e.push_str(tfp2.as_str());
                            e.push('|');
                            e.push_str(tfraw.as_str());
                            self.tfcalls.push(e);
                        }
                        return E::Call(String::from(segs[0].as_str()), ps, lo, h2);
                    }
                    let last = String::from(segs[segs.len() - 1].as_str());
                    // a smart-pointer constructor is identity in our value model
                    if segs.len() == 2
                        && (segs[0].as_str() == "Box"
                            || segs[0].as_str() == "Rc"
                            || segs[0].as_str() == "Arc"
                            || (segs[0].starts_with("Atomic") && !self.sknown(&segs[0])))
                        && last.as_str() == "new"
                        && ps.len() == 1
                        && !self.sknown(&segs[0])
                    {
                        match ps.pop() {
                            Some(x) => {
                                if segs[0].as_str() == "Box" && self.box_identity() {
                                    let mut ba: Vec<E> = Vec::new();
                                    ba.push(x);
                                    return E::Call(String::from("$boxnew"), ba, lo, h2);
                                }
                                if segs[0].as_str() == "Box" || segs[0].starts_with("Atomic") {
                                    return x;
                                }
                                // RC-IDENTITY (R4c): Rc/Arc::new registers the referent in the strong-count table
                                let mut ra: Vec<E> = Vec::new();
                                ra.push(x);
                                return E::Call(String::from("$rcnew"), ra, lo, h2);
                            }
                            None => {}
                        }
                    }
                    // `E::C { x: 2 }` -- a STRUCT VARIANT construction: order the named values by the
                    // variant's own declaration, the way a struct literal already does
                    {
                        let last2 = String::from(segs[segs.len() - 1].as_str());
                        let vf = self.variant_fields(&last2);
                        if vf.len() > 0
                            && ps.len() == 0
                            && self.is_tok("{")
                            && self.variant_owner(&last2).len() > 0
                        {
                            let _ = self.adv();
                            let mut ns: Vec<String> = Vec::new();
                            let mut vs2: Vec<E> = Vec::new();
                            loop {
                                if self.is_tok("}") {
                                    let _ = self.adv();
                                    break;
                                }
                                if self.at_end() {
                                    break;
                                }
                                if self.is_tok(",") {
                                    let _ = self.adv();
                                    continue;
                                }
                                let fi3 = self.adv();
                                let fn3 = match &self.t[fi3].k {
                                    Tk::Ident(x) => String::from(x.as_str()),
                                    _ => String::from("_"),
                                };
                                if self.is_tok(":") {
                                    let _ = self.adv();
                                }
                                let v2 = self.cmp_e();
                                ns.push(fn3);
                                vs2.push(v2);
                            }
                            let order = split_commas(&vf);
                            let mut pay: Vec<E> = Vec::new();
                            let mut oi = 0;
                            while oi < order.len() {
                                let mut pick = 0 - 1;
                                let mut q = 0;
                                while q < ns.len() {
                                    if ns[q].as_str() == order[oi].as_str() {
                                        pick = q as i32;
                                        q = ns.len();
                                    } else {
                                        q = q + 1;
                                    }
                                }
                                if pick >= 0 {
                                    let pu = pick as usize;
                                    let mut taken = E::Int(0, lo, h2);
                                    {
                                        let mut tmp: Vec<E> = Vec::new();
                                        let mut r = vs2.len();
                                        while r > 0 {
                                            r = r - 1;
                                            match vs2.pop() {
                                                Some(x) => {
                                                    if r == pu {
                                                        taken = x;
                                                    } else {
                                                        tmp.push(x);
                                                    }
                                                }
                                                None => {}
                                            }
                                        }
                                        loop {
                                            match tmp.pop() {
                                                Some(x) => {
                                                    vs2.push(x);
                                                }
                                                None => {
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                    pay.push(taken);
                                    ns.remove(pu);
                                } else {
                                    pay.push(E::Int(0, lo, h2));
                                }
                                oi = oi + 1;
                            }
                            let h4 = if self.i > 0 {
                                self.t[self.i - 1].hi
                            } else {
                                h2
                            };
                            return E::EnumLit(self.variant_owner(&last2), last2, pay, lo, h4);
                        }
                    }
                    // an ENUM variant, possibly reached through a module path
                    {
                        let mut en = String::from(segs[0].as_str());
                        let mut q = 1;
                        while q + 1 < segs.len() {
                            en.push('$');
                            en.push_str(segs[q].as_str());
                            q = q + 1;
                        }
                        {
                            let er = self.tyres(&en);
                            if self.enames_has(&er) {
                                return E::EnumLit(er, last, ps, lo, h2);
                            }
                        }
                    }
                    // a path rooted in the standard library is TRANSPARENT down to its last two
                    // segments: `::std::vec::Vec::new` is `Vec::new`, which P-STD then resolves. A user
                    // module of the same name wins, exactly as with the Box/Rc transparency.
                    {
                        let r = String::from(segs[0].as_str());
                        if (r.as_str() == "std" || r.as_str() == "core" || r.as_str() == "alloc")
                            && !self.mod_known(&r)
                        {
                            while segs.len() > 2 {
                                segs.remove(0);
                            }
                        }
                    }
                    // P-STD: a small std surface that is ALREADY expressible in our value model -- a
                    // vector constructor, an empty string, and the integer bounds. Anything whose
                    // meaning we cannot produce stays unresolved and is reported, not guessed.
                    {
                        // ALIAS-PATH (mir/issue-77002): `M::default()` with `type M = [i64; 2]` names the ALIAS in an
                        // expression path, which the type reader's alias substitution never sees -- `$zeroty$M` answered
                        // the integer 0 for an array. The alias's target stands in for it here.
                        if segs.len() == 2 {
                            let mut ai = 0;
                            while ai < self.aliasn.len() {
                                if self.aliasn[ai].as_str() == segs[0].as_str() {
                                    segs[0] = String::from(self.aliast[ai].as_str());
                                    ai = self.aliasn.len();
                                } else {
                                    ai = ai + 1;
                                }
                            }
                        }
                        let sp =
                            std_path(&segs, self.sknown(&segs[0]) || self.enames_has(&segs[0]));
                        if sp.len() > 0 {
                            if sp.as_str() == "$emptystr" || sp.as_str() == "$id1" {
                                self.strvals.push(lo);
                            } // BCK-STRMOVE: a String VALUE starts here
                            if sp.as_str() == "vec_new" && ps.len() > 0 {
                                let _ = ps.pop();
                            } // WITH-CAP: Vec::with_capacity(n) is an empty Vec (capacity is not observable here)
                            if sp.as_str() == "$int" {
                                let iv = E::Int(std_int_bound(&segs), lo, h2);
                                if segs.len() > 0 && oast::is_int_width(segs[0].as_str()) {
                                    return E::Cast(
                                        Box::new(iv),
                                        String::from(segs[0].as_str()),
                                        lo,
                                        h2,
                                    );
                                } /* INT-CONST-TY: every width`s MIN/MAX keeps its type (overflowing_* needs the width) */
                                return iv;
                            } // UINT-CONST: `u64::MAX` keeps its unsigned width (it printed -1)
                            if sp.as_str() == "$emptystr" {
                                return E::StrLit(String::from(""), lo, h2);
                            }
                            // C-COPYISCOPY: String::from(x) must COPY. It used to answer its argument,
                            // which is sound only while every string is immutable -- and push_str now grows
                            // in place, so an identity copy made the "copy" an alias of the growing buffer.
                            if sp.as_str() == "$id1" {
                                if ps.len() == 1 {
                                    match ps.pop() {
                                        Some(x) => {
                                            match &x {
                                                E::StrLit(_sv, l9, _h9) => {
                                                    self.strvals.push(*l9);
                                                    return x;
                                                }
                                                _ => {}
                                            }
                                            let a9 = oast::e_lo(&x);
                                            let b9 = oast::e_hi(&x);
                                            let mut ca: Vec<E> = Vec::new();
                                            ca.push(x);
                                            return E::Call(String::from("str_copy"), ca, a9, b9);
                                        }
                                        None => {}
                                    }
                                }
                            }
                            if sp.as_str() == "$memswap" && ps.len() == 2 {
                                if is_place_e(&ps[0]) && is_place_e(&ps[1]) {
                                    let p1 = place_inner(&ps[0]);
                                    let p2 = place_inner(&ps[1]);
                                    let mut tn = String::from("$sw");
                                    tn.push_str(&oast::u32s2(self.nhyg));
                                    self.nhyg = self.nhyg + 1;
                                    let zero = E::Int(0, lo, h2);
                                    let b2 = assign_place(
                                        &p2,
                                        E::Var(String::from(tn.as_str()), lo, h2),
                                        lo,
                                        h2,
                                    );
                                    let a2 = assign_place(&p1, clone_simple(&p2, lo, h2), lo, h2);
                                    let body = E::Seq(
                                        Box::new(a2),
                                        Box::new(E::Seq(Box::new(b2), Box::new(zero), lo, h2)),
                                        lo,
                                        h2,
                                    );
                                    return E::Let(tn, Box::new(p1), Box::new(body), lo, h2);
                                }
                            }
                            if sp.as_str() == "$memswap" && ps.len() == 2 {
                                let n1 = place_name(&ps[0]);
                                let n2 = place_name(&ps[1]);
                                if n1.len() > 0 && n2.len() > 0 {
                                    // let $t = a; a = b; b = $t
                                    let mut tn = String::from("$sw");
                                    tn.push_str(&oast::u32s2(self.nhyg));
                                    self.nhyg = self.nhyg + 1;
                                    let zero = E::Int(0, lo, h2);
                                    let b2 = E::Assign(
                                        String::from(n2.as_str()),
                                        Box::new(E::Var(String::from(tn.as_str()), lo, h2)),
                                        lo,
                                        h2,
                                    );
                                    let a2 = E::Assign(
                                        String::from(n1.as_str()),
                                        Box::new(E::Var(String::from(n2.as_str()), lo, h2)),
                                        lo,
                                        h2,
                                    );
                                    let body = E::Seq(
                                        Box::new(a2),
                                        Box::new(E::Seq(Box::new(b2), Box::new(zero), lo, h2)),
                                        lo,
                                        h2,
                                    );
                                    return E::Let(
                                        tn,
                                        Box::new(E::Var(n1, lo, h2)),
                                        Box::new(body),
                                        lo,
                                        h2,
                                    );
                                }
                            }
                            if sp.as_str() == "$sizeof" {
                                let w = ty_width(self.tfish.as_str());
                                if w > 0 && !self.tfishtx.contains(' ') {
                                    return E::Int(w as i64, lo, h2);
                                }
                                if self.tfishtx.len() > 0 {
                                    // R4a LAYOUT: an aggregate / reference / Option -- lowering computes rustc's layout from the raw text
                                    let mut m = String::from(
                                        if segs[segs.len() - 1].as_str() == "align_of" {
                                            "$alignof$"
                                        } else {
                                            "$sizeof$"
                                        },
                                    );
                                    m.push_str(self.tfishtx.as_str());
                                    return E::Call(m, Vec::new(), lo, h2);
                                }
                            }
                            if sp.as_str() == "$ptrfrom" && ps.len() == 1 {
                                return ps.pop().unwrap(); // R4b PTR: `ptr::from_ref(&x)` is the reference itself
                            }
                            if sp.as_str() == "$ptreq" && ps.len() == 2 {
                                // R4b PTR: pointer identity, answered STATICALLY from the places both sides designate (a value model
                                // has no addresses to compare); anything unresolved is refused
                                let rb = self.ptr_root(&ps[1]);
                                let ra = self.ptr_root(&ps[0]);
                                if ra.len() > 0 && rb.len() > 0 {
                                    return E::Bool(ra.as_str() == rb.as_str(), lo, h2);
                                }
                                return E::Call(
                                    String::from("$ptr_eq_of_unresolved_pointers_is_not_modelled"),
                                    Vec::new(),
                                    lo,
                                    h2,
                                );
                            }
                            if sp.as_str() == "$iteronce" && ps.len() == 1 {
                                // the `vec![x]` form: `{ let $ov = vec_new(); vec_push($ov, x); $ov }`
                                let mut vn = String::from("$ov");
                                vn.push_str(&oast::u32s2(self.nhyg));
                                self.nhyg = self.nhyg + 1;
                                let mut pa: Vec<E> = Vec::new();
                                pa.push(E::Var(String::from(vn.as_str()), lo, h2));
                                pa.push(ps.pop().unwrap());
                                let push = E::Call(String::from("vec_push"), pa, lo, h2);
                                let body = E::Seq(
                                    Box::new(push),
                                    Box::new(E::Var(String::from(vn.as_str()), lo, h2)),
                                    lo,
                                    h2,
                                );
                                return E::Let(
                                    vn,
                                    Box::new(E::Call(String::from("vec_new"), Vec::new(), lo, h2)),
                                    Box::new(body),
                                    lo + 1,
                                    h2,
                                );
                            } // R4d: `iter::once(x)` -> [x]
                            if sp.as_str() == "$intoiter" && ps.len() == 1 {
                                let x = ps.pop().unwrap();
                                return E::MethodCall(
                                    Box::new(x),
                                    String::from("into_iter"),
                                    Vec::new(),
                                    lo,
                                    h2,
                                );
                            } // R4d: `IntoIterator::into_iter(v)` is `v.into_iter()`
                            if sp.as_str() == "$spawn" && ps.len() == 1 {
                                // R4f SPAWN (single-threaded schedule): `thread::spawn(c)` runs c to completion HERE -- one of the
                                // interleavings the spec permits -- and its JoinHandle is `Ok(c())`; a zero-arg `.join()` is that handle.
                                // A closure that can PANIC is refused: rustc's thread dies and join() answers Err; here the process would.
                                let c = ps.pop().unwrap();
                                let panics = match &c {
                                    E::ClosRef(cn, _caps, _cl, _ch) => {
                                        let mut hit = true;
                                        let mut k = 0;
                                        while k < self.clos.len() {
                                            if self.clos[k].name.as_str() == cn.as_str() {
                                                hit = body_has_panic(&self.clos[k].body);
                                            }
                                            k = k + 1;
                                        }
                                        hit
                                    }
                                    _ => true,
                                };
                                // R4f CHAN: a spawned thread that holds a channel RECEIVER would run before main sends --
                                // not a legal sequential schedule (rustc blocks it until data arrives)
                                let blocks = match &c {
                                    E::ClosRef(_cn, caps, _cl, _ch) => {
                                        let mut hit = false;
                                        let mut k = 0;
                                        while k < caps.len() {
                                            if oast::in_list(&self.chanrx, caps[k].as_str()) {
                                                hit = true;
                                            }
                                            k = k + 1;
                                        }
                                        hit
                                    }
                                    _ => true,
                                };
                                if let E::ClosRef(scn, _scaps, _scl, _sch) = &c {
                                    self.spawncl.push(String::from(scn.as_str()));
                                    // SPAWN-PANIC: checked at the end of the parse
                                }
                                if blocks {
                                    return E::Call(
                                        String::from(
                                            "$spawn_of_a_receiving_thread_is_not_modelled",
                                        ),
                                        Vec::new(),
                                        lo,
                                        h2,
                                    );
                                }
                                if panics {
                                    return E::Call(
                                        String::from(
                                            "$spawn_of_a_closure_that_may_panic_is_not_modelled",
                                        ),
                                        Vec::new(),
                                        lo,
                                        h2,
                                    );
                                }
                                let mut tn = String::from("$sp");
                                tn.push_str(&oast::u32s2(self.nhyg));
                                self.nhyg = self.nhyg + 1;
                                let run = E::Call(String::from(tn.as_str()), Vec::new(), lo, h2);
                                return E::Let(tn, Box::new(c), Box::new(run), lo, h2);
                                // the handle IS the result; join() wraps it Ok
                            }
                            if sp.as_str() == "$transmute" && ps.len() == 1 {
                                // R4a XMUTE: `transmute::<A, B>(x)` or `let b: B = transmute(x)` -> `$xmute$B(x)`; no target -> refused
                                let dst = if self.tfish2.len() > 0 {
                                    String::from(self.tfish2.as_str())
                                } else if self.annty.as_str() == "arr" && self.annel.len() > 0 {
                                    let mut d = String::from("arr$");
                                    d.push_str(self.annel.as_str());
                                    d
                                } else {
                                    String::from(self.annty.as_str())
                                };
                                if dst.len() > 0 {
                                    let mut m = String::from("$xmute$");
                                    m.push_str(dst.as_str());
                                    let x = ps.pop().unwrap();
                                    let mut xa: Vec<E> = Vec::new();
                                    xa.push(x);
                                    return E::Call(m, xa, lo, h2);
                                }
                            }
                            if sp.as_str() == "$zerodflt" {
                                if self.annty.len() > 0 {
                                    let mut m = String::from("$zeroty$");
                                    m.push_str(self.annty.as_str());
                                    let z: Vec<E> = Vec::new();
                                    return E::Call(m, z, lo, h2);
                                }
                            }
                            if sp.as_str() == "$memtake" && ps.len() == 1 {
                                if is_place_e(&ps[0]) {
                                    let p1 = place_inner(&ps[0]);
                                    let mut tn = String::from("$tk");
                                    tn.push_str(&oast::u32s2(self.nhyg));
                                    self.nhyg = self.nhyg + 1;
                                    let mut za: Vec<E> = Vec::new();
                                    za.push(clone_simple(&p1, lo, h2));
                                    let zero = E::Call(String::from("$zeroof"), za, lo, h2);
                                    let asg = assign_place(&p1, zero, lo, h2);
                                    let body = E::Seq(
                                        Box::new(asg),
                                        Box::new(E::Var(String::from(tn.as_str()), lo, h2)),
                                        lo,
                                        h2,
                                    );
                                    return E::Let(tn, Box::new(p1), Box::new(body), lo, h2);
                                }
                            }
                            if sp.as_str() == "$memtake" && ps.len() == 1 {
                                let n1 = place_name(&ps[0]);
                                if n1.len() > 0 {
                                    // C-MEMTAKE: `mem::take(&mut a)` is replace with the TYPE's default. The
                                    // parser does not know types, so it writes `$zeroof(a)` and lowering --
                                    // which does know -- turns that into 0, "" or an empty vector.
                                    let mut tn = String::from("$tk");
                                    tn.push_str(&oast::u32s2(self.nhyg));
                                    self.nhyg = self.nhyg + 1;
                                    let mut za: Vec<E> = Vec::new();
                                    za.push(E::Var(String::from(n1.as_str()), lo, h2));
                                    let zero = E::Call(String::from("$zeroof"), za, lo, h2);
                                    let asg = E::Assign(
                                        String::from(n1.as_str()),
                                        Box::new(zero),
                                        lo,
                                        h2,
                                    );
                                    let body = E::Seq(
                                        Box::new(asg),
                                        Box::new(E::Var(String::from(tn.as_str()), lo, h2)),
                                        lo,
                                        h2,
                                    );
                                    return E::Let(
                                        tn,
                                        Box::new(E::Var(n1, lo, h2)),
                                        Box::new(body),
                                        lo,
                                        h2,
                                    );
                                }
                            }
                            if sp.as_str() == "$memrepl" && ps.len() == 2 {
                                if is_place_e(&ps[0]) {
                                    let p1 = place_inner(&ps[0]);
                                    let mut tn = String::from("$rp");
                                    tn.push_str(&oast::u32s2(self.nhyg));
                                    self.nhyg = self.nhyg + 1;
                                    let mut ps2 = ps;
                                    let nv = match ps2.pop() {
                                        Some(x) => x,
                                        None => E::Int(0, lo, h2),
                                    };
                                    let asg = assign_place(&p1, nv, lo, h2);
                                    let body = E::Seq(
                                        Box::new(asg),
                                        Box::new(E::Var(String::from(tn.as_str()), lo, h2)),
                                        lo,
                                        h2,
                                    );
                                    return E::Let(tn, Box::new(p1), Box::new(body), lo, h2);
                                }
                            }
                            if sp.as_str() == "$memrepl" && ps.len() == 2 {
                                let n1 = place_name(&ps[0]);
                                if n1.len() > 0 {
                                    // let $t = a; a = v; $t   -- an EXPRESSION, whose value is the OLD one
                                    let mut tn = String::from("$rp");
                                    tn.push_str(&oast::u32s2(self.nhyg));
                                    self.nhyg = self.nhyg + 1;
                                    let mut ps2 = ps;
                                    let nv = match ps2.pop() {
                                        Some(x) => x,
                                        None => E::Int(0, lo, h2),
                                    };
                                    let asg =
                                        E::Assign(String::from(n1.as_str()), Box::new(nv), lo, h2);
                                    let body = E::Seq(
                                        Box::new(asg),
                                        Box::new(E::Var(String::from(tn.as_str()), lo, h2)),
                                        lo,
                                        h2,
                                    );
                                    return E::Let(
                                        tn,
                                        Box::new(E::Var(n1, lo, h2)),
                                        Box::new(body),
                                        lo,
                                        h2,
                                    );
                                }
                            }
                            return E::Call(sp, ps, lo, h2);
                        }
                    }
                    // C-ZEROAGG: `T::default()` where T is one of OUR types never reaches std_path, which
                    // steps aside for a user type by design -- so the zero marker is built here instead,
                    // and lowering answers it with a struct of per-field zeros (or refuses, for an enum).
                    // MOD-DEFAULT (R2a P5): a module-qualified type too (`l::P::default()`, `crate::l::P::default()`) -- the
                    // type is the path minus its last segment; it was made the call `l$P$default()` (not found)
                    if last.as_str() == "default" && ps.len() == 0 && segs.len() >= 2 {
                        let t0 = if segs.len() == 2 {
                            self.tyres(&segs[0])
                        } else {
                            let mut tp: Vec<String> = Vec::new();
                            let mut k = 0;
                            while k + 1 < segs.len() {
                                tp.push(String::from(segs[k].as_str()));
                                k = k + 1;
                            }
                            self.resolve_path(&tp)
                        };
                        if self.sknown(&t0) || self.enames_has(&t0) {
                            let mut m = String::from("$zeroty$");
                            m.push_str(t0.as_str());
                            let z: Vec<E> = Vec::new();
                            return E::Call(m, z, lo, h2);
                        }
                    }
                    let joined = self.resolve_path(&segs);
                    // `M::S { .. }` -- a struct literal through a path
                    if ps.len() == 0 && self.is_tok("{") && self.sknown(&joined) {
                        return self.slit_named(&joined, lo, hi);
                    }
                    // VIS-PRIV (rustc_privacy E0603): a private fn `m$x` is visible only inside module `m` and its
                    // descendants -- i.e. when the caller's module prefix starts with `m$`
                    // real Rust only (a root `fn main`): ourc's .src dialect has no `pub`, every module item is open
                    if self.hasmain {
                        let mut k = 0;
                        while k < self.privfns.len() {
                            if self.privfns[k].as_str() == joined.as_str() {
                                let jb = joined.as_bytes();
                                let mut cut = 0;
                                let mut q = 0;
                                while q < jb.len() {
                                    if jb[q] == 36 {
                                        cut = q + 1;
                                    }
                                    q = q + 1;
                                }
                                let mb = self.mpfx.as_bytes();
                                let vis = mb.len() >= cut && &mb[..cut] == &jb[..cut];
                                if !vis && self.perr == 0 {
                                    self.perr = lo + 1;
                                    self.perrexp = String::from(
                                        "a public function (this one is private to its module)",
                                    );
                                    self.perrgot = String::from(joined.as_str());
                                }
                            }
                            k = k + 1;
                        }
                    }
                    // MOD-STATIC-MUT (R2a P5): `l::N` naming a `static mut` (or an atomic static) declared in module l is the
                    // global, as the bare `N` already is inside l -- the path was made the call `l$N()` ("cannot find function")
                    if !pcall && segs.len() >= 2 && self.is_smut(&segs[segs.len() - 1]) {
                        let mut gn = String::from("$gload$");
                        gn.push_str(segs[segs.len() - 1].as_str());
                        let a: Vec<E> = Vec::new();
                        return E::Call(gn, a, lo, h2);
                    }
                    // PUB-CRATE (R2a P6, E0603): a pub(crate) fn of an extern crate called from outside that crate
                    if self.hasmain && self.perr == 0 {
                        let mut k = 0;
                        while k < self.pcfns.len() {
                            let e = self.pcfns[k].as_str();
                            let eb = e.as_bytes();
                            let mut c = 0;
                            while c < eb.len() && eb[c] != 124 {
                                c = c + 1;
                            }
                            if c < eb.len() && &e[..c] == joined.as_str() {
                                let root = &e[c + 1..];
                                let mb = self.mpfx.as_bytes();
                                let rb = root.as_bytes();
                                let inside = mb.len() >= rb.len() && &mb[..rb.len()] == rb;
                                if !inside {
                                    self.perr = lo + 1;
                                    self.perrexp = String::from(
                                        "a public function (this one is `pub(crate)` in another crate: E0603)",
                                    );
                                    self.perrgot = String::from(joined.as_str());
                                }
                            }
                            k = k + 1;
                        }
                    }
                    // G1 G-P2 MONO-ALL: a turbofish-only instantiation (`sz::<u64>()` -- no argument names T) reaches the mono
                    // demand through an `@tf:` fact keyed by the call's position (lowering decides whether the callee is generic)
                    if tfp.len() > 0 && self.hasmain && self.fknown(&joined) {
                        let mut e = oast::u32s2(lo);
                        e.push('|');
                        e.push_str(tfp.as_str());
                        e.push('|');
                        e.push_str(tfp2.as_str());
                        e.push('|');
                        e.push_str(tfraw.as_str());
                        self.tfcalls.push(e);
                    }
                    return E::Call(joined, ps, lo, h2);
                }
                if self.is_tok("{") && self.sknown_here(&s) {
                    let s = self.sres(&s);
                    // STRUCT-LIT: ONE named/positional/update reader (slit_named) -- this site kept its own copy, which missed `..base`
                    self.slit_named(&s, lo, hi)
                } else if self.is_tok("=") {
                    let _ = self.adv();
                    let v = self.cmp_e();
                    let h2 = oast::e_hi(&v);
                    E::Assign(String::from(s.as_str()), Box::new(v), lo, h2)
                } else if self.is_tok("(") {
                    let _ = self.adv();
                    let mut args: Vec<E> = Vec::new();
                    loop {
                        if self.is_tok(")") {
                            let _ = self.adv();
                            break;
                        }
                        let a = self.cmp_e();
                        args.push(a);
                        if self.is_tok(",") {
                            let _ = self.adv();
                        }
                    }
                    let h2 = if self.i > 0 {
                        self.t[self.i - 1].hi
                    } else {
                        hi
                    };
                    // MOD-TUPLE-CTOR (R2a P5): inside module l, `Sq(3)` constructs l's tuple struct `l$Sq` (registered qualified);
                    // the bare call `Sq` was "cannot find function". A same-named fn still wins (fknown first).
                    let mut cn = String::from(s.as_str());
                    if self.mpfx.len() > 0 && !self.fknown(&cn) {
                        let q = self.qual(&cn);
                        if self.sknown(&q) {
                            cn = q;
                        }
                    }
                    E::Call(cn, args, lo, h2)
                } else {
                    E::Var(String::from(s.as_str()), lo, hi)
                }
            }
            Tk::LParen => {
                if self.is_tok(")") {
                    let _ = self.adv();
                    let h2 = if self.i > 0 {
                        self.t[self.i - 1].hi
                    } else {
                        hi
                    };
                    let z: Vec<E> = Vec::new();
                    return E::TupLit(z, lo, h2);
                }
                let e = self.cmp_e();
                if self.is_tok(",") {
                    let mut ps: Vec<E> = Vec::new();
                    ps.push(e);
                    loop {
                        if self.is_tok(",") {
                            let _ = self.adv();
                        }
                        if self.is_tok(")") {
                            let _ = self.adv();
                            break;
                        }
                        if self.at_end() {
                            break;
                        }
                        ps.push(self.cmp_e());
                    }
                    let b = if self.i > 0 {
                        self.t[self.i - 1].hi
                    } else {
                        lo
                    };
                    E::TupLit(ps, lo, b)
                } else if self.is_tok("..") {
                    let _ = self.adv();
                    let pinc = if self.is_tok("=") {
                        let _ = self.adv();
                        true
                    } else {
                        false
                    };
                    if self.is_tok(")") && !pinc {
                        // R4d: `(a..)` is ENDLESS -- `$rangefrom(a)` is materialised only under `.take(n)` (the method-call
                        // builder); any other use stays unresolved, i.e. refused
                        let _ = self.adv();
                        let mut ra: Vec<E> = Vec::new();
                        ra.push(e);
                        return E::Call(String::from("$rangefrom"), ra, lo, hi);
                    }
                    if self.is_tok(")") {
                        let _ = self.adv();
                        if self.perr == 0 {
                            self.perr = lo + 1;
                            self.perrexp = String::from(
                                "a bounded range (an endless one cannot be materialised)",
                            );
                            self.perrgot = self.tok_text();
                        }
                        return e;
                    }
                    let hi2r = self.cmp_e();
                    self.eat(")");
                    let h3 = if self.i > 0 {
                        self.t[self.i - 1].hi
                    } else {
                        hi
                    };
                    let hi2 = if pinc {
                        E::Bin(
                            String::from("+"),
                            Box::new(hi2r),
                            Box::new(E::Int(1, lo, h3)),
                            lo,
                            h3,
                        )
                    } else {
                        hi2r
                    };
                    let mut ra: Vec<E> = Vec::new();
                    ra.push(e);
                    ra.push(hi2);
                    E::Call(String::from("range2"), ra, lo, h3)
                } else {
                    self.eat(")");
                    e
                }
            }
            Tk::If => {
                // R9 IFLET-OR: `if let P = e { A } [else { B }]` whose pattern P holds a `|` (`Some(1 | 3)`) is spliced into
                // `match e { P => { A } _ => { B } }`, so the match or-pattern rules (OR-NEST, OR-STRUCT) apply. `else if` stays.
                if self.is_tok("let") {
                    let ifp = self.i - 1; // the `if` token
                    let mut q = self.i + 1;
                    let mut d = 0;
                    let mut bar = 0;
                    let mut eq = 0;
                    while q < self.t.len() {
                        if tok_opens(&self.t[q]) {
                            d = d + 1;
                        } else if tok_closes(&self.t[q]) {
                            if d == 0 {
                                break;
                            }
                            d = d - 1;
                        } else if d == 0 && tkv_eq(&self.t[q].k) {
                            eq = q;
                            break;
                        } else if tkv_bar(&self.t[q].k) {
                            bar = bar + 1;
                        }
                        q = q + 1;
                    }
                    // the then-block: the first `{` at depth 0 after `=`, and its matching `}`
                    let mut bo = 0;
                    let mut bc = 0;
                    if bar > 0 && eq > 0 {
                        let mut r = eq + 1;
                        let mut d2 = 0;
                        while r < self.t.len() {
                            if d2 == 0 && tkv_l_brace(&self.t[r].k) {
                                bo = r;
                                break;
                            }
                            if tok_opens(&self.t[r]) {
                                d2 = d2 + 1;
                            } else if tok_closes(&self.t[r]) {
                                d2 = d2 - 1;
                            }
                            r = r + 1;
                        }
                        if bo > 0 {
                            let mut d3 = 0;
                            let mut r2 = bo;
                            while r2 < self.t.len() {
                                if tok_opens(&self.t[r2]) {
                                    d3 = d3 + 1;
                                } else if tok_closes(&self.t[r2]) {
                                    d3 = d3 - 1;
                                    if d3 == 0 {
                                        bc = r2;
                                        break;
                                    }
                                }
                                r2 = r2 + 1;
                            }
                        }
                    }
                    // optional `else { .. }` (an `else if` chain is not spliced)
                    let mut eo = 0;
                    let mut ec = 0;
                    let mut elseif = false;
                    if bc > 0 && bc + 1 < self.t.len() && tkv_else(&self.t[bc + 1].k) {
                        if bc + 2 < self.t.len() && tkv_l_brace(&self.t[bc + 2].k) {
                            eo = bc + 2;
                            let mut d4 = 0;
                            let mut r3 = eo;
                            while r3 < self.t.len() {
                                if tok_opens(&self.t[r3]) {
                                    d4 = d4 + 1;
                                } else if tok_closes(&self.t[r3]) {
                                    d4 = d4 - 1;
                                    if d4 == 0 {
                                        ec = r3;
                                        break;
                                    }
                                }
                                r3 = r3 + 1;
                            }
                        } else {
                            elseif = true;
                        }
                    }
                    if bar > 0 && eq > 0 && bc > 0 && !elseif && (eo == 0 || ec > 0) {
                        let lo0 = self.t[ifp].lo;
                        let mut nt: Vec<Tok> = Vec::new();
                        let mut k = 0;
                        while k < ifp {
                            nt.push(tok_clone(&self.t[k]));
                            k = k + 1;
                        }
                        nt.push(Tok {
                            k: Tk::Match,
                            lo: lo0,
                            hi: lo0,
                        });
                        k = eq + 1;
                        while k < bo {
                            nt.push(tok_clone(&self.t[k])); // the scrutinee
                            k = k + 1;
                        }
                        nt.push(Tok {
                            k: Tk::LBrace,
                            lo: lo0,
                            hi: lo0,
                        });
                        k = self.i + 1;
                        while k < eq {
                            nt.push(tok_clone(&self.t[k])); // the pattern
                            k = k + 1;
                        }
                        nt.push(Tok {
                            k: Tk::FatArrow,
                            lo: lo0,
                            hi: lo0,
                        });
                        k = bo;
                        while k <= bc {
                            nt.push(tok_clone(&self.t[k])); // the then-block
                            k = k + 1;
                        }
                        nt.push(Tok {
                            k: Tk::Ident(String::from("_")),
                            lo: lo0,
                            hi: lo0,
                        });
                        nt.push(Tok {
                            k: Tk::FatArrow,
                            lo: lo0,
                            hi: lo0,
                        });
                        if eo > 0 {
                            k = eo;
                            while k <= ec {
                                nt.push(tok_clone(&self.t[k])); // the else-block
                                k = k + 1;
                            }
                        } else {
                            nt.push(Tok {
                                k: Tk::LBrace,
                                lo: lo0,
                                hi: lo0,
                            });
                            nt.push(Tok {
                                k: Tk::RBrace,
                                lo: lo0,
                                hi: lo0,
                            });
                        }
                        nt.push(Tok {
                            k: Tk::RBrace,
                            lo: lo0,
                            hi: lo0,
                        });
                        k = if ec > 0 { ec + 1 } else { bc + 1 };
                        while k < self.t.len() {
                            nt.push(tok_clone(&self.t[k]));
                            k = k + 1;
                        }
                        self.t = nt;
                        self.i = ifp;
                        return self.atom();
                    }
                }
                // `if let V(binds) = e { body } else { alt }` desugars to a two-arm match -- the variant arm
                // and a wildcard -- which the if-chain lowering already handles (test + payload binds)
                if self.is_tok("let") {
                    let _ = self.adv();
                    {
                        let mut q = self.i;
                        let mut d = 0;
                        while q < self.t.len() {
                            if tok_opens(&self.t[q]) {
                                d = d + 1;
                            } else if tok_closes(&self.t[q]) {
                                if d == 0 {
                                    break;
                                }
                                d = d - 1;
                            } else if d == 0 && tkv_eq(&self.t[q].k) {
                                break;
                            }
                            let w = tok_word(&self.t[q]);
                            let wb = w.as_bytes();
                            if wb.len() > 0
                                && wb[0] >= 97
                                && wb[0] <= 122
                                && !(q + 1 < self.t.len()
                                    && (tkv_l_paren(&self.t[q + 1].k)
                                        || tkv_path_sep(&self.t[q + 1].k)))
                            {
                                self.lnames.push(w);
                            }
                            q = q + 1;
                        }
                    } // FN-ITEM-VAL: if-let bindings shadow fn names
                      // IFLET-RANGE: `if let lo..=hi = e { a } else { b }` is a bounds test on e, evaluated once
                    {
                        let ew = if self.cast_lit_at(self.i) {
                            5
                        } else if self.i + 1 < self.t.len()
                            && tkv_minus(&self.t[self.i].k)
                            && self.cast_lit_at(self.i + 1)
                        {
                            6
                        } else if self.i < self.t.len() && tkv_minus(&self.t[self.i].k) {
                            2
                        } else {
                            1
                        };
                        let isnum = self.i < self.t.len() && (ew > 1 || ilit(&self.t[self.i]) > 0);
                        if isnum
                            && self.i + ew < self.t.len()
                            && tkv_dot_dot(&self.t[self.i + ew].k)
                        {
                            let e0 = self.pat_endpoint(lo, hi);
                            let _ = self.adv();
                            let incl = if self.is_tok("=") {
                                let _ = self.adv();
                                true
                            } else {
                                false
                            };
                            let e1 = self.pat_endpoint(lo, hi);
                            self.eat("=");
                            let sc = self.or_e();
                            self.eat("{");
                            let th = self.expr();
                            self.eat("}");
                            let el = if self.is_tok("else") {
                                let _ = self.adv();
                                if self.is_tok("if") {
                                    self.atom()
                                } else {
                                    self.eat("{");
                                    let e2 = self.expr();
                                    self.eat("}");
                                    e2
                                }
                            } else {
                                let zh = if self.i > 0 {
                                    self.t[self.i - 1].hi
                                } else {
                                    hi
                                };
                                E::Int(0, zh, zh)
                            };
                            let h3 = if self.i > 0 {
                                self.t[self.i - 1].hi
                            } else {
                                hi
                            };
                            let mut tv = String::from("$ilr");
                            tv.push_str(&oast::u32s2(lo));
                            let c0 = E::Bin(
                                String::from(">="),
                                Box::new(E::Var(String::from(tv.as_str()), lo, h3)),
                                Box::new(e0),
                                lo,
                                h3,
                            );
                            let c1 = E::Bin(
                                String::from(if incl { "<=" } else { "<" }),
                                Box::new(E::Var(String::from(tv.as_str()), lo, h3)),
                                Box::new(e1),
                                lo,
                                h3,
                            );
                            let c = E::Bin(String::from("&&"), Box::new(c0), Box::new(c1), lo, h3);
                            return E::Let(
                                tv,
                                Box::new(sc),
                                Box::new(E::If(Box::new(c), Box::new(th), Box::new(el), lo, h3)),
                                lo,
                                h3,
                            );
                        }
                    }
                    let pi2 = self.adv();
                    let mut pv = match &self.t[pi2].k {
                        Tk::Ident(x) => String::from(x.as_str()),
                        _ => String::from("_"),
                    };
                    if self.is_tok("::") {
                        let _ = self.adv();
                        let p2 = self.adv();
                        pv = match &self.t[p2].k {
                            Tk::Ident(x) => String::from(x.as_str()),
                            _ => pv,
                        };
                    }
                    let mut pb: Vec<String> = Vec::new();
                    if self.is_tok("(") {
                        let _ = self.adv();
                        loop {
                            if self.is_tok(")") {
                                let _ = self.adv();
                                break;
                            }
                            if self.at_end() {
                                break;
                            }
                            let bi = self.adv();
                            match self.t[bi].k.clone() {
                                Tk::Ident(x) => pb.push(String::from(x.as_str())),
                                _ => {}
                            }
                            if self.is_tok(",") {
                                let _ = self.adv();
                            }
                        }
                    }
                    self.eat("=");
                    let sc = self.or_e();
                    self.eat("{");
                    let th = self.expr();
                    self.eat("}");
                    let el = if self.is_tok("else") {
                        let _ = self.adv();
                        self.eat("{");
                        let e2 = self.expr();
                        self.eat("}");
                        e2
                    } else {
                        let zh = if self.i > 0 {
                            self.t[self.i - 1].hi
                        } else {
                            hi
                        };
                        E::Int(0, zh, zh)
                    };
                    let h3 = if self.i > 0 {
                        self.t[self.i - 1].hi
                    } else {
                        hi
                    };
                    let mut arms: Vec<Arm> = Vec::new();
                    arms.push(Arm {
                        slit: String::from(""),
                        has_slit: false,
                        lit: 0,
                        has_lit: false,
                        has_guard: false,
                        guard: E::Int(0, lo, lo),
                        variant: pv,
                        binds: pb,
                        body: th,
                        wild: false,
                    });
                    arms.push(Arm {
                        slit: String::from(""),
                        has_slit: false,
                        lit: 0,
                        has_lit: false,
                        has_guard: false,
                        guard: E::Int(0, lo, lo),
                        variant: String::from("_"),
                        binds: Vec::new(),
                        body: el,
                        wild: true,
                    });
                    return E::Match(Box::new(sc), arms, lo, h3);
                }
                let c = self.cmp_e();
                self.eat("{");
                let th = self.expr();
                self.eat("}");
                let el = if self.is_tok("else") {
                    let _ = self.adv();
                    if self.is_tok("if") {
                        self.atom()
                    } else {
                        self.eat("{");
                        let e2 = self.expr();
                        self.eat("}");
                        e2
                    }
                } else {
                    let zh = if self.i > 0 {
                        self.t[self.i - 1].hi
                    } else {
                        hi
                    };
                    E::Int(0, zh, zh)
                };
                let h = oast::e_hi(&el);
                E::If(Box::new(c), Box::new(th), Box::new(el), lo, h)
            }
            Tk::While => {
                // `while let V(b) = e { body }` is `loop { match e { V(b) => body, _ => break } }` -- the
                // scrutinee is re-evaluated each iteration, which is what an iterator loop needs
                // WHILE-LET-NEST: a pattern with a NESTED group (`Some((a, b))`, `Some(E::V(x))`) is rewritten in the tokens to
                // `loop { match E { PAT => { BODY } _ => break } }`, so the match machinery (PAT-REWRITE) handles the nesting.
                if self.is_tok("let") {
                    let ws = self.i - 1;
                    let mut q = self.i + 1;
                    let mut d = 0;
                    let mut nested = false;
                    while q < self.t.len() {
                        if tok_opens(&self.t[q]) {
                            d = d + 1;
                            if d > 1 {
                                nested = true;
                            }
                        } else if tok_closes(&self.t[q]) {
                            d = d - 1;
                        } else if d == 0 && tkv_eq(&self.t[q].k) {
                            break;
                        }
                        q = q + 1;
                    }
                    let pe = q;
                    let mut sb = pe + 1;
                    d = 0;
                    while sb < self.t.len() {
                        if d == 0 && tkv_l_brace(&self.t[sb].k) {
                            break;
                        }
                        if tok_opens(&self.t[sb]) {
                            d = d + 1;
                        } else if tok_closes(&self.t[sb]) {
                            d = d - 1;
                        }
                        sb = sb + 1;
                    }
                    let mut be = sb;
                    d = 0;
                    while be < self.t.len() {
                        if tok_opens(&self.t[be]) {
                            d = d + 1;
                        } else if tok_closes(&self.t[be]) {
                            d = d - 1;
                            if d == 0 {
                                break;
                            }
                        }
                        be = be + 1;
                    }
                    if nested && be < self.t.len() {
                        let lo2 = self.t[ws].lo;
                        let hi2 = self.t[ws].hi;
                        let mk = |k: Tk| Tok {
                            k: k,
                            lo: lo2,
                            hi: hi2,
                        };
                        let mut nt: Vec<Tok> = Vec::new();
                        let mut k = 0;
                        while k < ws {
                            nt.push(tok_clone(&self.t[k]));
                            k = k + 1;
                        }
                        nt.push(mk(Tk::LoopT));
                        nt.push(mk(Tk::LBrace));
                        nt.push(mk(Tk::Match));
                        k = pe + 1;
                        while k < sb {
                            nt.push(tok_clone(&self.t[k]));
                            k = k + 1;
                        }
                        nt.push(mk(Tk::LBrace));
                        k = self.i + 1;
                        while k < pe {
                            nt.push(tok_clone(&self.t[k]));
                            k = k + 1;
                        }
                        nt.push(mk(Tk::FatArrow));
                        k = sb;
                        while k <= be {
                            nt.push(tok_clone(&self.t[k]));
                            k = k + 1;
                        }
                        nt.push(Tok {
                            k: Tk::Ident(String::from("_")),
                            lo: lo2,
                            hi: hi2,
                        });
                        nt.push(mk(Tk::FatArrow));
                        nt.push(mk(Tk::BreakT));
                        nt.push(mk(Tk::RBrace));
                        nt.push(mk(Tk::RBrace));
                        k = be + 1;
                        while k < self.t.len() {
                            nt.push(tok_clone(&self.t[k]));
                            k = k + 1;
                        }
                        self.t = nt;
                        self.i = ws;
                        return self.atom();
                    }
                }
                if self.is_tok("let") {
                    let _ = self.adv();
                    let pi2 = self.adv();
                    let mut pv = match &self.t[pi2].k {
                        Tk::Ident(x) => String::from(x.as_str()),
                        _ => String::from("_"),
                    };
                    if self.is_tok("::") {
                        let _ = self.adv();
                        let p2 = self.adv();
                        pv = match &self.t[p2].k {
                            Tk::Ident(x) => String::from(x.as_str()),
                            _ => pv,
                        };
                    }
                    let mut pb: Vec<String> = Vec::new();
                    if self.is_tok("(") {
                        let _ = self.adv();
                        loop {
                            if self.is_tok(")") {
                                let _ = self.adv();
                                break;
                            }
                            if self.at_end() {
                                break;
                            }
                            if self.is_tok("..")
                                || self.is_word("ref")
                                || self.is_tok("mut")
                                || self.is_tok("&")
                            {
                                let _ = self.adv();
                                continue;
                            }
                            let bi = self.adv();
                            match self.t[bi].k.clone() {
                                Tk::Ident(x) => pb.push(String::from(x.as_str())),
                                _ => {}
                            }
                            if self.is_tok(",") {
                                let _ = self.adv();
                            }
                        }
                    }
                    self.eat("=");
                    let sc = self.or_e();
                    self.eat("{");
                    let bd = self.expr();
                    self.eat("}");
                    let h4 = if self.i > 0 {
                        self.t[self.i - 1].hi
                    } else {
                        hi
                    };
                    let mut arms: Vec<Arm> = Vec::new();
                    arms.push(Arm {
                        slit: String::from(""),
                        has_slit: false,
                        lit: 0,
                        has_lit: false,
                        has_guard: false,
                        guard: E::Int(0, lo, lo),
                        variant: pv,
                        binds: pb,
                        body: bd,
                        wild: false,
                    });
                    arms.push(Arm {
                        slit: String::from(""),
                        has_slit: false,
                        lit: 0,
                        has_lit: false,
                        has_guard: false,
                        guard: E::Int(0, lo, lo),
                        variant: String::from("_"),
                        binds: Vec::new(),
                        body: E::BreakE(lo, h4),
                        wild: true,
                    });
                    let m = E::Match(Box::new(sc), arms, lo, h4);
                    return E::While(Box::new(E::Bool(true, lo, lo)), Box::new(m), lo, h4);
                }
                let c = self.cmp_e();
                self.eat("{");
                let bd = self.expr();
                self.eat("}");
                let h2 = if self.i > 0 {
                    self.t[self.i - 1].hi
                } else {
                    hi
                };
                E::While(Box::new(c), Box::new(bd), lo, h2)
            }
            Tk::Let => {
                let _ = self.let_else_rewrite(); // LET-ELSE
                {
                    let mut q = self.i;
                    let mut d = 0;
                    while q < self.t.len() {
                        if tok_opens(&self.t[q]) {
                            d = d + 1;
                        } else if tok_closes(&self.t[q]) {
                            if d == 0 {
                                break;
                            }
                            d = d - 1;
                        } else if d == 0
                            && (tkv_eq(&self.t[q].k)
                                || tkv_colon(&self.t[q].k)
                                || tkv_semi(&self.t[q].k))
                        {
                            break;
                        }
                        let w = tok_word(&self.t[q]);
                        let wb = w.as_bytes();
                        if wb.len() > 0
                            && wb[0] >= 97
                            && wb[0] <= 122
                            && w.as_str() != "mut"
                            && w.as_str() != "ref"
                        {
                            self.lnames.push(w);
                        }
                        q = q + 1;
                    }
                } // FN-ITEM-VAL: let-bound names shadow fn names
                  // LET-DEFER (A8 prerequisite): `let r;` / `let r: T;` has NO initializer -- its first assignment IS the
                  // initialization, so it is not an immutable binding for BCK-MUT (`let r; r = 5;` was refused as "assign twice")
                let deferred = {
                    let mut q = self.i;
                    let mut d = 0;
                    let mut r = false;
                    while q < self.t.len() {
                        if tkv_lt(&self.t[q].k)
                            || tkv_l_paren(&self.t[q].k)
                            || tkv_l_brack(&self.t[q].k)
                        {
                            d = d + 1;
                        } else if tkv_gt(&self.t[q].k)
                            || tkv_r_paren(&self.t[q].k)
                            || tkv_r_brack(&self.t[q].k)
                        {
                            d = d - 1;
                        } else if tkv_shr(&self.t[q].k) {
                            d = d - 2; // REGRESS-E: `Vec<Box<T>> = ..` -- `>>` closes two generics
                        } else if d <= 0 && tkv_eq(&self.t[q].k) {
                            break;
                        } else if d == 1 && tkv_ge(&self.t[q].k) {
                            // `let x:Option<i32>=v` lexes `>=` as ONE token: it closes the generic AND is the `=`
                            break;
                        } else if tkv_shr(&self.t[q].k) {
                            // GT2-DEFER (R2a P4): `Vec<Vec<T>>` lexes its closers as ONE `>>` -- two levels; counted as none, the
                            // depth never returned to 0, the `= init` was skipped and the let read as DEFERRED (`let x;`): the
                            // definite-assignment walk then refused ourc's own `let mut lets: Vec<Vec<Tok>> = Vec::new();` (E0381)
                            d = d - 2;
                        } else if d == 2 && tk_is(&self.t[q].k, ">>=") {
                            break;
                        } else if d <= 0 && tkv_semi(&self.t[q].k) {
                            r = true;
                            break;
                        }
                        q = q + 1;
                    }
                    r
                };
                if deferred {
                    // LET-FLOW (REGRESS-D): the borrow checker runs a definite-assignment walk over each deferred binding
                    let mut e = String::from(if self.is_tok("mut") {
                        "@deferm:"
                    } else {
                        "@defer:"
                    });
                    e.push_str(lo.to_string().as_str());
                    self.fpays.push(e);
                    // A2 DEFER-NOANN: `let x;` with NO type annotation -- if nothing ever assigns it, its type is unknown (E0282)
                    let nm_at = if self.is_tok("mut") {
                        self.i + 1
                    } else {
                        self.i
                    };
                    if nm_at + 1 < self.t.len() && tkv_semi(&self.t[nm_at + 1].k) {
                        let mut u = String::from("@deferu:");
                        u.push_str(lo.to_string().as_str());
                        self.fpays.push(u);
                    }
                }
                if self.is_tok("mut") {
                    let _ = self.adv();
                } else if !deferred && !pat_has_mut(&self.t, self.i) {
                    self.immlets.push(lo);
                } // BCK-MUT: a `let` without `mut` (keyed by its start)
                  // R15.4 PAT-MUT: `let (a, mut b) = ..` -- a destructuring pattern's own `mut` makes its binding assignable; the
                  // desugared bindings share the let's key, so a pattern holding any `mut` is not marked immutable (a missed E0384
                  // on an immutable sibling is a refusal lost, never a wrong answer)
                  // P-PAT: a DESTRUCTURING `let` -- `let (a, b) = e`, `let [a, b] = e`, `let P { x, y } = e`
                  // -- desugars to one binding for the scrutinee plus one binding per name, each reading its
                  // own component. The pattern is a shape the `let` reader owns; nothing below it changes.
                {
                    let dk = if self.cast_lit_at(self.i) {
                        0
                    } else if self.is_tok("(") {
                        1
                    } else if self.is_tok("[") {
                        2
                    } else {
                        let mut r = 0;
                        if self.i + 1 < self.t.len() {
                            let w = tok_word(&self.t[self.i]);
                            if w.len() > 0 && self.sknown(&w) {
                                if tkv_l_brace(&self.t[self.i + 1].k) {
                                    r = 3;
                                } else if tkv_l_paren(&self.t[self.i + 1].k) {
                                    r = 4;
                                }
                            }
                        }
                        r
                    };
                    if dk > 0 {
                        let mut sn = String::from("");
                        if dk == 3 || dk == 4 {
                            sn = tok_word(&self.t[self.i]);
                            let _ = self.adv();
                        }
                        let _ = self.adv(); // `(` / `[` / `{`
                        let mut ns: Vec<String> = Vec::new();
                        let mut ps: Vec<String> = Vec::new();
                        let mut fsn: Vec<String> = Vec::new();
                        let mut lrest: i32 = 0 - 1; // TUP-REST-LET: where a `..` sits among the bound slots
                        if dk == 1 {
                            self.tup_pat("", &mut ns, &mut ps);
                        } else {
                            loop {
                                if self.is_tok(")") || self.is_tok("]") || self.is_tok("}") {
                                    let _ = self.adv();
                                    break;
                                }
                                if self.at_end() {
                                    break;
                                }
                                if self.is_tok("..") {
                                    lrest = ns.len() as i32;
                                    let _ = self.adv();
                                    continue;
                                }
                                if self.is_tok(",") || self.is_tok("mut") || self.is_word("ref") {
                                    let _ = self.adv();
                                    continue;
                                }
                                let w = tok_word(&self.t[self.i]);
                                if w.len() == 0 {
                                    let _ = self.adv();
                                    continue;
                                }
                                let _ = self.adv();
                                // `P { x: a }` renames the field; the BINDING is the name after the colon
                                if self.is_tok(":") {
                                    let _ = self.adv();
                                    let w2 = tok_word(&self.t[self.i]);
                                    if w2.len() > 0 {
                                        let _ = self.adv();
                                        ns.push(w2);
                                    } else {
                                        ns.push(String::from(w.as_str()));
                                    }
                                    fsn.push(w); // PAT-RENAME: the FIELD read is w, the binding w2
                                    continue;
                                }
                                fsn.push(String::from(w.as_str()));
                                ns.push(w);
                            }
                        }
                        if self.is_tok(":") {
                            let _ = self.adv();
                            let _ = self.parse_tyname();
                        }
                        self.eat("=");
                        let v = self.cmp_e();
                        self.eat(";");
                        let ischan = dk == 1
                            && ns.len() == 2
                            && is_chan_call(&v)
                            && ns[0].as_str() != "_"
                            && ns[1].as_str() != "_";
                        if ischan {
                            self.chantx.push(String::from(ns[0].as_str()));
                            self.chanrx.push(String::from(ns[1].as_str()));
                        } // R4f CHAN: registered BEFORE the body parses, so its send/recv rewrite
                        let body = self.expr();
                        let h = oast::e_hi(&body);
                        if ischan {
                            // R4f CHAN (single-threaded schedule): a channel is ONE shared queue -- the sender is a Vec handle
                            // (send = push, typed by VEC-INFER from its pushes), the receiver borrows the same handle
                            let mut ra: Vec<E> = Vec::new();
                            ra.push(E::Ref(
                                Box::new(E::Var(String::from(ns[0].as_str()), lo, h)),
                                false,
                                lo,
                                h,
                            ));
                            let rx = E::Call(String::from("$alias"), ra, lo, h);
                            let inner = E::Let(
                                String::from(ns[1].as_str()),
                                Box::new(rx),
                                Box::new(body),
                                lo + 1, // not the `let` token: these bindings are MUTABLE (a channel mutates through a shared handle)
                                h,
                            );
                            // the queue is DECLARED Vec<int>: integer channels type everywhere (a captured sender too); a channel of
                            // any other element type fails typeck at its first send -- refused, never printed as an address
                            return E::LetT(
                                String::from(ns[0].as_str()),
                                String::from("Vec$int"),
                                Box::new(E::Call(String::from("vec_new"), Vec::new(), lo, h)),
                                Box::new(inner),
                                lo + 1,
                                h,
                            );
                        }
                        let dsn = self.nfor;
                        self.nfor = self.nfor + 1;
                        let mut dn = String::from("$dp");
                        dn.push_str(&oast::u32s2(dsn));
                        // innermost-out, so each component reads the scrutinee binding
                        let mut acc = body;
                        let mut q = ns.len();
                        while q > 0 {
                            q = q - 1;
                            let nm2 = String::from(ns[q].as_str());
                            let sv = E::Var(String::from(dn.as_str()), lo, h);
                            let get = if dk == 1 {
                                let pb = ps[q].as_bytes();
                                let mut g = sv;
                                let mut n: u32 = 0;
                                let mut j = 0;
                                while j <= pb.len() {
                                    if j == pb.len() || pb[j] == 46 {
                                        g = E::TupIdx(Box::new(g), n, lo, h);
                                        n = 0;
                                    } else {
                                        n = n * 10 + ((pb[j] - 48) as u32);
                                    }
                                    j = j + 1;
                                }
                                g
                            } else if dk == 2 {
                                // TUP-REST-LET: a slot after `..` counts from the END (`let [a, .., b] = [1,2,3,4]` read b = 2)
                                let ix = if lrest >= 0 && (q as i32) >= lrest {
                                    let mut la: Vec<E> = Vec::new();
                                    la.push(E::Var(String::from(dn.as_str()), lo, h));
                                    E::Bin(
                                        String::from("-"),
                                        Box::new(E::Call(String::from("seq_len"), la, lo, h)),
                                        Box::new(E::Int((ns.len() - q) as i64, lo, h)),
                                        lo,
                                        h,
                                    )
                                } else {
                                    E::Int(q as i64, lo, h)
                                };
                                E::Index(Box::new(sv), Box::new(ix), lo, h)
                            } else if dk == 4 {
                                // TUP-REST-LET: `let Foo(.., d) = Foo(1,2,3,4)` read d = .0 (rustc 4): count from the declared arity
                                let nf = split_commas(&self.sfields_of(sn.as_str())).len();
                                E::Field(
                                    Box::new(sv),
                                    oast::u32s2(rest_slot(q, lrest, ns.len(), nf) as u32),
                                    lo,
                                    h,
                                )
                            } else {
                                let fnm = if q < fsn.len() {
                                    String::from(fsn[q].as_str())
                                } else {
                                    String::from(ns[q].as_str())
                                };
                                E::Field(Box::new(sv), fnm, lo, h)
                            };
                            if nm2.as_str() == "_" {
                                acc = E::Seq(Box::new(get), Box::new(acc), lo, h);
                            } else {
                                acc = E::Let(nm2, Box::new(get), Box::new(acc), lo, h);
                            }
                        }
                        let _ = &sn;
                        return E::Let(dn, Box::new(v), Box::new(acc), lo, h);
                    }
                }
                let ni = self.adv();
                let nm = match &self.t[ni].k {
                    Tk::Ident(s) => String::from(s.as_str()),
                    _ => String::from("_"),
                };
                let mut ann = String::from("");
                if self.is_tok(":") {
                    let _ = self.adv();
                    ann = sl_val(&self.parse_tyname());
                    if (ann.as_str() == "Option" || ann.as_str() == "Result")
                        && self.lastpay.len() > 0
                    {
                        let pb = self.lastpay.as_bytes();
                        let mut t = String::from(ann.as_str());
                        t.push('$');
                        let mut z = 0;
                        while z < pb.len() {
                            if pb[z] == 44 {
                                t.push('$');
                            } else {
                                t.push(pb[z] as char);
                            }
                            z = z + 1;
                        }
                        let tb = t.as_bytes();
                        let mut segs: Vec<String> = Vec::new();
                        let mut cur = String::from("");
                        let mut z2 = 0;
                        while z2 < tb.len() {
                            if tb[z2] == 36 {
                                segs.push(cur);
                                cur = String::from("");
                            } else {
                                cur.push(tb[z2] as char);
                            }
                            z2 = z2 + 1;
                        }
                        segs.push(cur);
                        let mut t2 = String::new();
                        let mut s3 = 0;
                        while s3 < segs.len() {
                            if s3 > 0 {
                                t2.push('$');
                            }
                            let sg = segs[s3].as_str();
                            if sg == "String" || sg == "&str" || sg == "&String" {
                                t2.push_str("str");
                            } else {
                                t2.push_str(sg);
                            }
                            s3 = s3 + 1;
                        }
                        ann = t2;
                    } /* LET-PAYLOAD: `let o: Option<String>` records `Option$str` (payload spellings normalised) -- the bare name made {:?} print the payload as a pointer */
                }
                // `let x: T;` -- a declaration with no initializer (Rust allows it before first assignment)
                if self.is_tok(";") {
                    let _ = self.adv();
                    let b = self.expr();
                    let h = oast::e_hi(&b);
                    return E::Let(nm, Box::new(E::Int(0, lo, lo)), Box::new(b), lo, h);
                }
                // C-DEFAULTTY: `let x: T = Default::default()` names T right here and the initializer is
                // parsed next, so the annotation is in scope for it. Cleared straight after, so a bare
                // Default::default() with no annotation still has no type and is reported.
                self.eat("=");
                self.annty = String::from(ann.as_str());
                self.annel = if ann.as_str() == "arr" {
                    String::from(self.lastarrel.as_str())
                } else {
                    String::new()
                };
                // R8 RANGE-TO: a RangeTo/RangeToInclusive VALUE (`let x = ..4`) is its bound, wrapped `rangeto(b)` / `rangetoinc(b)`
                // so `{:?}` prints `..b` / `..=b` (lowering records the binding; the value itself is the int bound)
                let mut rto = String::new();
                if self.is_tok("..") && self.perr == 0 {
                    let _ = self.adv();
                    rto = String::from("rangeto");
                    if self.is_tok("=") {
                        let _ = self.adv();
                        rto = String::from("rangetoinc");
                    }
                }
                let v = self.cmp_e();
                let v = if rto.len() > 0 {
                    let rl = oast::e_lo(&v);
                    let rh = oast::e_hi(&v);
                    let mut ra: Vec<E> = Vec::new();
                    ra.push(v);
                    E::Call(rto, ra, rl, rh)
                } else {
                    v
                };
                let bcn = self.box_note(nm.as_str(), &v);
                let v = box_rewrap(bcn, v);
                // R8 RANGE-LET: `let r = a..b;` / `a..=b` -- the bounded range VALUE, as `(a..b)` already builds it (range2)
                let v = if self.is_tok("..") {
                    let rlo = oast::e_lo(&v);
                    let _ = self.adv();
                    let inc = if self.is_tok("=") {
                        let _ = self.adv();
                        true
                    } else {
                        false
                    };
                    if self.is_tok(";") {
                        if self.perr == 0 {
                            self.perr = self.lo() + 1;
                            self.perrexp = String::from(
                                "a bounded range (an endless one cannot be materialised)",
                            );
                            self.perrgot = self.tok_text();
                        }
                        v
                    } else {
                        let h0 = self.cmp_e();
                        let rh = oast::e_hi(&h0);
                        let h1 = if inc {
                            E::Bin(
                                String::from("+"),
                                Box::new(h0),
                                Box::new(E::Int(1, rlo, rh)),
                                rlo,
                                rh,
                            )
                        } else {
                            h0
                        };
                        let mut ra: Vec<E> = Vec::new();
                        ra.push(v);
                        ra.push(h1);
                        E::Call(String::from("range2"), ra, rlo, rh)
                    }
                } else {
                    v
                };
                self.annty = String::from("");
                if ann.len() > 0 {
                    self.eat(";");
                    self.note_ref_binding(nm.as_str(), &v); // DEREF-PLACE on the ANNOTATED let too
                    if let E::ClosRef(_cn, _cc, _cl, _ch) = &v {
                        self.closlets.push(String::from(nm.as_str()));
                    }
                    let b = self.expr();
                    let h = oast::e_hi(&b);
                    let mut etn: Vec<String> = Vec::new();
                    let mut etv: Vec<E> = Vec::new();
                    let v = ext_hoist(v, lo, &mut etn, &mut etv);
                    return ext_wrap(
                        E::LetT(nm, ann, Box::new(v), Box::new(b), lo, h),
                        etn,
                        etv,
                        lo,
                        h,
                    );
                }
                self.eat(";");
                self.note_ref_binding(nm.as_str(), &v);
                if let E::ClosRef(_cn, _cc, _cl, _ch) = &v {
                    self.closlets.push(String::from(nm.as_str())); // CLOS-CAP-CLOS: before the rest is parsed (its closures capture it)
                }
                let b = self.expr();
                let h = oast::e_hi(&b);
                let mut etn: Vec<String> = Vec::new();
                let mut etv: Vec<E> = Vec::new();
                let v = ext_hoist(v, lo, &mut etn, &mut etv);
                ext_wrap(E::Let(nm, Box::new(v), Box::new(b), lo, h), etn, etv, lo, h)
            }
            _ => E::Int(0, lo, hi),
        }
    }
    fn postfix(&mut self) -> E {
        let a0 = self.atom();
        // C-PLACERET, the CALL side: every call is built by atom, so this is the ONE point all five
        // call-construction paths pass through, in every position -- statement or value.
        let mut e = self.place_call(a0);
        loop {
            if self.is_tok("as") {
                break;
            }
            // PREC-AS: `as` binds LOOSER than unary (`-3i64 as u8` = `(-3i64) as u8`) -- cast_e owns it
            // CALL-VALUE: `(s.f)(x)` / `(move || x + y)()` -- a call on a FIELD or a CLOSURE value binds the callee to a fresh local and
            // calls that (the callable-variable path); only those two shapes, so a block-like statement followed by a tuple stays two statements
            else if self.is_tok("(")
                && (match &e {
                    E::Field(_b, _f, _l, _h) => true,
                    E::ClosRef(_c, _v, _l, _h) => true,
                    // CALL-RESULT (R2a P5): a call's RESULT called again -- `adder(5)(6)`, `l::adder(5)(6)`, `o.get()(x)` (an
                    // `impl Fn` / closure return); `f(x)` directly followed by `(` is always a call in Rust, never two statements
                    E::Call(_n, _a, _l, _h) => true,
                    E::MethodCall(_r, _m, _a, _l, _h) => true,
                    _ => false,
                })
            {
                let _ = self.adv();
                let mut args: Vec<E> = Vec::new();
                loop {
                    if self.is_tok(")") {
                        let _ = self.adv();
                        break;
                    }
                    if self.at_end() {
                        break;
                    }
                    args.push(self.cmp_e());
                    if self.is_tok(",") {
                        let _ = self.adv();
                    }
                }
                let a = oast::e_lo(&e);
                let b = if self.i > 0 { self.t[self.i - 1].hi } else { a };
                self.nhyg = self.nhyg + 1;
                let mut cv = String::from("__callee");
                cv.push_str(&oast::u32s2(self.nhyg));
                let call = E::Call(String::from(cv.as_str()), args, a, b);
                e = E::Let(cv, Box::new(e), Box::new(call), a, b);
            } else if self.is_tok("?") {
                let _ = self.adv();
                let a = oast::e_lo(&e);
                let b = if self.i > 0 { self.t[self.i - 1].hi } else { a };
                e = E::Try(Box::new(e), a, b);
            } else if self.is_tok("[") {
                let _ = self.adv();
                let a = oast::e_lo(&e);
                // a RANGE index `v[a..b]` / `v[a..]` / `v[..b]` / `v[..]` is a slice: it COPIES into a
                // fresh vector. A copy is only honest in a value position, so a slice used as a method
                // RECEIVER (`self[..n].clone_from_slice(..)`, which mutates through it) is refused.
                let open = self.is_tok("..");
                let ix = if open { E::Int(0, a, a) } else { self.cmp_e() };
                if self.is_tok("..") {
                    let _ = self.adv();
                    let sincl = if self.is_tok("=") {
                        let _ = self.adv();
                        true
                    } else {
                        false
                    }; // P-SLICE-INCL-RANGE
                    let endopen = self.is_tok("]");
                    let endx0 = if endopen {
                        let mut la: Vec<E> = Vec::new();
                        la.push(clone_simple(&e, a, a));
                        E::Call(String::from("seq_len"), la, a, a)
                    } else {
                        self.cmp_e()
                    };
                    let endx = if sincl && !endopen {
                        E::Bin(
                            String::from("+"),
                            Box::new(endx0),
                            Box::new(E::Int(1, a, a)),
                            a,
                            a,
                        )
                    } else {
                        endx0
                    };
                    self.eat("]");
                    let b = if self.i > 0 { self.t[self.i - 1].hi } else { a };
                    // C-SLICEREAD: the refusal below is about MUTATION -- our seq_slice answers a COPY, so a
                    // method that writes through the slice would silently do nothing. A method that READS is
                    // a different matter and rustc accepts it everywhere: probed, s[1..4].len(),
                    // s[1..4].to_string(), src[i..i+1].chars() and v[1..3].to_vec() all compile and run. So
                    // the refusal is narrowed to the mutators, and a read is admitted.
                    if self.is_tok(".")
                        && self.i + 1 < self.t.len()
                        && slice_mutator(&tok_word(&self.t[self.i + 1]))
                    {
                        if self.perr == 0 {
                            self.perr = a + 1;
                            self.perrexp=String::from("a mutating method on a slice in a value position (our slice is a COPY, so the write would be lost)");
                            self.perrgot = self.tok_text();
                        }
                    }
                    let mut sa: Vec<E> = Vec::new();
                    sa.push(e);
                    sa.push(ix);
                    sa.push(endx);
                    e = E::Call(String::from("seq_slice"), sa, a, b);
                } else {
                    self.eat("]");
                    let b = if self.i > 0 { self.t[self.i - 1].hi } else { a };
                    e = E::Index(Box::new(e), Box::new(ix), a, b);
                }
            } else if self.is_tok(".") {
                let _ = self.adv();
                let fi = self.adv();
                match self.t[fi].k.clone() {
                    Tk::Num(n) => {
                        let a0 = oast::e_lo(&e);
                        let b0 = self.t[fi].hi;
                        e = E::TupIdx(Box::new(e), n as u32, a0, b0);
                        continue;
                    }
                    _ => {}
                }
                let mut fname = match &self.t[fi].k {
                    Tk::Ident(x) => String::from(x.as_str()),
                    _ => String::from("_"),
                };
                let a = oast::e_lo(&e);
                let b = self.t[fi].hi;
                // a METHOD may carry a turbofish: `.sum::<i32>()`, `.collect::<Vec<_>>()`
                let mut tostr = false;
                if self.is_tok("::") && self.peek_is(1, "<") {
                    tostr = self.i + 3 < self.t.len()
                        && word_is(&self.t[self.i + 2], "String")
                        && tkv_gt(&self.t[self.i + 3].k);
                    let _ = self.adv();
                    self.skip_generics();
                }
                if tostr && fname.as_str() == "collect" {
                    fname = String::from("collect_string");
                } // COLLECT-TURBO: `.collect::<String>()` concatenates (chars or strs) into a String
                if self.is_tok("(") {
                    let _ = self.adv();
                    let mut args: Vec<E> = Vec::new();
                    loop {
                        if self.is_tok(")") {
                            let _ = self.adv();
                            break;
                        }
                        if self.at_end() {
                            break;
                        }
                        let aprog = self.i;
                        let ag = self.cmp_e();
                        args.push(ag);
                        if self.is_tok(",") {
                            let _ = self.adv();
                        }
                        if self.noprog(aprog) {}
                    }
                    let b2 = if self.i > 0 { self.t[self.i - 1].hi } else { b };
                    let mut args = args;
                    if fname.as_str() == "take"
                        && args.len() == 0
                        && is_place_e(&e)
                        && !self.user_meth_named("take")
                    {
                        // OPT-TAKE (R2a P4): `p.take()` on an Option place is `mem::replace(&mut p, None)` -- `let $t = p; p = None;
                        // $t` (the $memrepl desugar); a USER `fn take(&mut self)` keeps its own method
                        let mut tn = String::from("$rp");
                        tn.push_str(&oast::u32s2(self.nhyg));
                        self.nhyg = self.nhyg + 1;
                        let pl = clone_simple(&e, a, b2);
                        let none = E::EnumLit(
                            String::from("Option"),
                            String::from("None"),
                            Vec::new(),
                            a,
                            b2,
                        );
                        let asg = assign_place(&pl, none, a, b2);
                        let body = E::Seq(
                            Box::new(asg),
                            Box::new(E::Var(String::from(tn.as_str()), a, b2)),
                            a,
                            b2,
                        );
                        e = E::Let(tn, Box::new(e), Box::new(body), a, b2);
                        continue;
                    }
                    if (fname.as_str() == "eq" || fname.as_str() == "ne") && args.len() == 1 {
                        let a1 = match args.pop() {
                            Some(x) => x,
                            None => E::Int(0, a, b2),
                        };
                        let op = if fname.as_str() == "eq" { "==" } else { "!=" };
                        e = E::Bin(String::from(op), Box::new(e), Box::new(a1), a, b2);
                        continue;
                    } /* EQ-METHOD: a.eq(&b) is a == b (which dispatches a user PartialEq::eq) */
                    if (fname.as_str() == "overflowing_add"
                        || fname.as_str() == "overflowing_sub"
                        || fname.as_str() == "overflowing_mul")
                        && args.len() == 1
                        && is_simple_operand(&e)
                        && is_simple_operand(&args[0])
                    {
                        let op = String::from(&fname[12..]);
                        let mut w = String::from("wrapping_");
                        w.push_str(&op);
                        let mut c = String::from("checked_");
                        c.push_str(&op);
                        let mut a1: Vec<E> = Vec::new();
                        a1.push(clone_simple(&args[0], a, b2));
                        let mut a2: Vec<E> = Vec::new();
                        a2.push(clone_simple(&args[0], a, b2));
                        let wr = E::MethodCall(Box::new(clone_simple(&e, a, b2)), w, a1, a, b2);
                        let ck = E::MethodCall(
                            Box::new(E::MethodCall(
                                Box::new(clone_simple(&e, a, b2)),
                                c,
                                a2,
                                a,
                                b2,
                            )),
                            String::from("is_none"),
                            Vec::new(),
                            a,
                            b2,
                        );
                        let mut tp: Vec<E> = Vec::new();
                        tp.push(wr);
                        tp.push(ck);
                        e = E::TupLit(tp, a, b2);
                        continue;
                    } /* OVF-METHOD: overflowing_X = (wrapping_X, checked_X.is_none()) for SIMPLE operands */
                    if (fname.as_str() == "overflowing_shl" || fname.as_str() == "overflowing_shr")
                        && args.len() == 1
                        && is_simple_operand(&e)
                        && is_simple_operand(&args[0])
                    {
                        let mut w = String::from("wrapping_");
                        w.push_str(&fname[12..]);
                        let mut a1: Vec<E> = Vec::new();
                        a1.push(clone_simple(&args[0], a, b2));
                        let wr = E::MethodCall(Box::new(clone_simple(&e, a, b2)), w, a1, a, b2);
                        let mut a2: Vec<E> = Vec::new();
                        a2.push(clone_simple(&e, a, b2));
                        a2.push(clone_simple(&args[0], a, b2));
                        let ck = E::Call(String::from("$shovf"), a2, a, b2);
                        let mut tp: Vec<E> = Vec::new();
                        tp.push(wr);
                        tp.push(ck);
                        e = E::TupLit(tp, a, b2);
                        continue;
                    } /* OVF-SHIFT: (wrapping_shX(n), n >= BITS) -- BITS resolved by the receiver's width in lowering */
                    if (fname.as_str() == "overflowing_neg"
                        || fname.as_str() == "overflowing_abs"
                        || fname.as_str() == "wrapping_abs")
                        && args.len() == 0
                        && is_simple_operand(&e)
                    {
                        let isneg = fname.as_str() == "overflowing_neg";
                        let wv = if isneg {
                            E::MethodCall(
                                Box::new(clone_simple(&e, a, b2)),
                                String::from("wrapping_neg"),
                                Vec::new(),
                                a,
                                b2,
                            )
                        } else {
                            E::If(
                                Box::new(E::Bin(
                                    String::from("<"),
                                    Box::new(clone_simple(&e, a, b2)),
                                    Box::new(E::Int(0, a, b2)),
                                    a,
                                    b2,
                                )),
                                Box::new(E::MethodCall(
                                    Box::new(clone_simple(&e, a, b2)),
                                    String::from("wrapping_neg"),
                                    Vec::new(),
                                    a,
                                    b2,
                                )),
                                Box::new(clone_simple(&e, a, b2)),
                                a,
                                b2,
                            )
                        };
                        if fname.as_str() == "wrapping_abs" {
                            e = wv;
                            continue;
                        }
                        let mut ca: Vec<E> = Vec::new();
                        ca.push(E::Int(if isneg { 0 } else { 1 }, a, b2));
                        ca.push(clone_simple(&e, a, b2));
                        let ck = E::Call(String::from("$ovf1"), ca, a, b2);
                        let mut tp: Vec<E> = Vec::new();
                        tp.push(wv);
                        tp.push(ck);
                        e = E::TupLit(tp, a, b2);
                        continue;
                    } /* OVF-NEGABS: wrapping_abs = if x<0 {x.wrapping_neg()} else {x}; the overflow flag $ovf1 is resolved by width/signedness in lowering */
                    e = if (fname.as_str() == "map"
                        || fname.as_str() == "rev"
                        || fname.as_str() == "collect"
                        || fname.as_str() == "collect_string"
                        || fname.as_str() == "count"
                        || fname.as_str() == "filter")
                        && {
                            match &e {
                                E::MethodCall(_x, cm, ca, _cl, _ch) => {
                                    (cm.as_str() == "chars" || cm.as_str() == "bytes")
                                        && ca.len() == 0
                                }
                                _ => false,
                            }
                        } {
                        match e {
                            E::MethodCall(x, cm2, _ca, cl, ch) => {
                                let mut sa: Vec<E> = Vec::new();
                                sa.push(*x);
                                let vf = if cm2.as_str() == "bytes" {
                                    "str_bytes_vec"
                                } else {
                                    "str_chars_vec"
                                };
                                E::MethodCall(
                                    Box::new(E::Call(String::from(vf), sa, cl, ch)),
                                    fname,
                                    args,
                                    a,
                                    b2,
                                )
                            }
                            other => E::MethodCall(Box::new(other), fname, args, a, b2),
                        }
                    } else if fname.as_str() == "send"
                        && args.len() == 1
                        && (match &e {
                            E::Var(n, _l, _h) => oast::in_list(&self.chantx, n.as_str()),
                            _ => false,
                        })
                    {
                        // R4f CHAN: `tx.send(v)` pushes onto the shared queue and answers Ok(())
                        let mut args = args;
                        let v = args.pop().unwrap();
                        let mut ca: Vec<E> = Vec::new();
                        ca.push(v);
                        let mut pa: Vec<E> = Vec::new();
                        pa.push(E::Call(String::from("$chanint"), ca, a, b2)); // the queue is Vec<int>: a non-integer send is refused
                        let push = E::MethodCall(Box::new(e), String::from("push"), pa, a, b2);
                        let mut oa: Vec<E> = Vec::new();
                        oa.push(E::Int(0, a, b2));
                        let ok = E::EnumLit(String::from("Result"), String::from("Ok"), oa, a, b2);
                        E::Seq(Box::new(push), Box::new(ok), a, b2)
                    } else if ((fname.as_str() == "recv" || fname.as_str() == "try_recv")
                        && args.len() == 0
                        || fname.as_str() == "recv_timeout" && args.len() == 1)
                        && (match &e {
                            E::Var(n, _l, _h) => oast::in_list(&self.chanrx, n.as_str()),
                            _ => false,
                        })
                    {
                        let rn = match &e {
                            E::Var(n, _l, _h) => String::from(n.as_str()),
                            _ => String::new(),
                        };
                        chan_recv(rn.as_str(), a, b2)
                    } else if fname.as_str() == "take"
                        && args.len() == 1
                        && (match &e {
                            E::Call(g, ga, _l, _h) => g.as_str() == "$iterrepeat" && ga.len() == 1,
                            _ => false,
                        })
                    {
                        // R4d: `iter::repeat(x).take(n)` -- the endless source materialised under its bound: n copies of x
                        let x = match e {
                            E::Call(_g, ga, _l, _h) => {
                                let mut ga = ga;
                                ga.pop().unwrap()
                            }
                            other => other,
                        };
                        let mut args = args;
                        let n = args.pop().unwrap();
                        let k = oast::u32s2(self.nhyg);
                        self.nhyg = self.nhyg + 1;
                        let mut vn = String::from("$rv");
                        vn.push_str(&k);
                        let mut xn = String::from("$rx");
                        xn.push_str(&k);
                        let mut iname = String::from("$ri");
                        iname.push_str(&k);
                        let var = |s: &str| E::Var(String::from(s), a, b2);
                        let mut pa: Vec<E> = Vec::new();
                        pa.push(var(vn.as_str()));
                        pa.push(var(xn.as_str()));
                        let push = E::Call(String::from("vec_push"), pa, a, b2); // the `vec![x; n]` push form
                        let step = E::Assign(
                            String::from(iname.as_str()),
                            Box::new(E::Bin(
                                String::from("+"),
                                Box::new(var(iname.as_str())),
                                Box::new(E::Int(1, a, b2)),
                                a,
                                b2,
                            )),
                            a,
                            b2,
                        );
                        let lp = E::While(
                            Box::new(E::Bin(
                                String::from("<"),
                                Box::new(var(iname.as_str())),
                                Box::new(n),
                                a,
                                b2,
                            )),
                            Box::new(E::Seq(Box::new(push), Box::new(step), a, b2)),
                            a,
                            b2,
                        );
                        let body = E::Seq(Box::new(lp), Box::new(var(vn.as_str())), a, b2);
                        let b3 =
                            E::Let(iname, Box::new(E::Int(0, a, b2)), Box::new(body), a + 1, b2);
                        let b4 = E::Let(
                            vn,
                            Box::new(E::Call(String::from("vec_new"), Vec::new(), a, b2)),
                            Box::new(b3),
                            a + 1,
                            b2,
                        );
                        E::Let(xn, Box::new(x), Box::new(b4), a + 1, b2)
                    } else if fname.as_str() == "take"
                        && args.len() == 1
                        && (match &e {
                            E::Call(g, ga, _l, _h) => g.as_str() == "$rangefrom" && ga.len() == 1,
                            _ => false,
                        })
                    {
                        // R4d: `(a..).take(n)` is `a..a + n` (a bound once)
                        let x = match e {
                            E::Call(_g, ga, _l, _h) => {
                                let mut ga = ga;
                                ga.pop().unwrap()
                            }
                            other => other,
                        };
                        let mut args = args;
                        let n = args.pop().unwrap();
                        let mut an = String::from("$rf");
                        an.push_str(&oast::u32s2(self.nhyg));
                        self.nhyg = self.nhyg + 1;
                        let mut ra: Vec<E> = Vec::new();
                        ra.push(E::Var(String::from(an.as_str()), a, b2));
                        ra.push(E::Bin(
                            String::from("+"),
                            Box::new(E::Var(String::from(an.as_str()), a, b2)),
                            Box::new(n),
                            a,
                            b2,
                        ));
                        E::Let(
                            an,
                            Box::new(x),
                            Box::new(E::Call(String::from("range2"), ra, a, b2)),
                            a + 1,
                            b2,
                        )
                    } else if fname.as_str() == "is_null"
                        && args.len() == 0
                        && (match &e {
                            E::Var(_n, _l, _h) => self.ptr_root(&e).len() > 0,
                            _ => false,
                        })
                    {
                        // R4b PTR: `p.is_null()` from what p was bound to (null() vs a fixed place) -- statically
                        E::Bool(self.ptr_root(&e).as_str() == "@null", a, b2)
                    } else if fname.as_str() == "join" && args.len() == 0 {
                        // R4f SPAWN: a zero-arg `.join()` is only JoinHandle's (slice/Path join take an argument); the
                        // thread already ran at spawn, so the handle IS its `Ok(result)`
                        let mut oa: Vec<E> = Vec::new();
                        oa.push(e);
                        E::EnumLit(String::from("Result"), String::from("Ok"), oa, a, b2)
                    } else if atomic_op(fname.as_str(), &args)
                        && (match &e {
                            E::Var(_n, _l, _h) => true,
                            E::Call(g, ga, _l, _h) => g.starts_with("$gload$") && ga.len() == 0,
                            _ => false,
                        })
                    {
                        // R4f ATOMIC: single-threaded schedule -- an atomic IS its integer; the Ordering argument has no meaning
                        let an = match &e {
                            E::Var(n, _l, _h) => {
                                let mut s = String::from("v:");
                                s.push_str(n.as_str());
                                s
                            }
                            E::Call(g, _ga, _l, _h) => {
                                let mut s = String::from("s:");
                                s.push_str(&g[7..]);
                                s
                            }
                            _ => String::new(),
                        };
                        let mut tn = String::from("$ao");
                        tn.push_str(&oast::u32s2(self.nhyg));
                        self.nhyg = self.nhyg + 1;
                        atomic_rewrite(fname.as_str(), an.as_str(), tn.as_str(), args, a, b2)
                    } else if fname.as_str() == "unwrap"
                        && args.len() == 0
                        && lock_guard(&e).len() > 0
                    {
                        // R4c MUTEX: single-threaded schedule -- `m.lock().unwrap()` / `m.write().unwrap()` is the cell's
                        // borrow_mut, `m.read().unwrap()` its borrow (Mutex/RwLock are the RefCell cell)
                        let bm = lock_guard(&e);
                        match e {
                            E::MethodCall(x, _lm, _la, cl, ch) => {
                                E::MethodCall(x, bm, Vec::new(), cl, ch)
                            }
                            other => other,
                        }
                    } else {
                        E::MethodCall(Box::new(e), fname, args, a, b2)
                    }; /* CHARS-VEC: `s.chars().map/rev/collect` iterates a Vec<char> */
                } else {
                    e = E::Field(Box::new(e), fname, a, b);
                }
            } else {
                break;
            }
        }
        e
    }
    fn postfix_old(&mut self) -> E {
        let mut e = self.atom();
        while self.is_tok(".") {
            let _ = self.adv();
            let fi = self.adv();
            let fname = match &self.t[fi].k {
                Tk::Ident(x) => String::from(x.as_str()),
                _ => String::from("_"),
            };
            let a = oast::e_lo(&e);
            let b = self.t[fi].hi;
            e = E::Field(Box::new(e), fname, a, b);
        }
        e
    }
    fn unary(&mut self) -> E {
        // prefix DEREFERENCE: `*x` / `**x` is `x` here -- C2 says a reference is the value. Without this
        // a closure body like `**x % 2 == 1` read as a multiplication.
        if self.is_tok("*") {
            let _ = self.adv();
            let inner = self.unary();
            // DEREF-PLACE (R6): r = &mut <fixed place> -- `*r` IS that place while r is live (rustc's borrow checker forbids
            // any other use of the place meanwhile), so a read is the place and `*r = v` / `*r op= v` writes it.
            // It was the `$deref_local_unsupported` refusal. Compound forms arrive as Assign(r, Bin(op, Var r, rhs)).
            let dkind = match &inner {
                E::Var(n, _l, _h) => {
                    let mut bn = String::from("!");
                    bn.push_str(n.as_str());
                    if self.is_refvar(bn.as_str()) && self.refplace(n.as_str()).is_some() {
                        1
                    } else {
                        0
                    }
                }
                E::Assign(an, av, _l, _h) => {
                    let mut bn = String::from("!");
                    bn.push_str(an.as_str());
                    if !self.is_refvar(bn.as_str()) || self.refplace(an.as_str()).is_none() {
                        0
                    } else {
                        match av.as_ref() {
                            E::Bin(_o, x, y, _bl, _bh)
                                if (match x.as_ref() {
                                    E::Var(xn, _a, _b) => xn.as_str() == an.as_str(),
                                    _ => false,
                                }) && !oast::e_mentions(y, an.as_str()) =>
                            {
                                2
                            }
                            _ => {
                                if oast::e_mentions(av, an.as_str()) {
                                    0
                                } else {
                                    3
                                }
                            }
                        }
                    }
                }
                _ => 0,
            };
            if dkind == 1 {
                if let E::Var(n, _l, _h) = &inner {
                    if let Some(pl) = self.refplace(n.as_str()) {
                        return pl;
                    }
                }
            }
            if dkind == 2 || dkind == 3 {
                if let E::Assign(an, av, al, ah) = inner {
                    let pl = match self.refplace(an.as_str()) {
                        Some(p) => p,
                        None => E::Var(String::from(an.as_str()), al, ah),
                    };
                    let nv = if dkind == 2 {
                        match *av {
                            E::Bin(op, _x, y, bl, bh) => {
                                let p2 = match self.refplace(an.as_str()) {
                                    Some(p) => p,
                                    None => E::Var(String::from(an.as_str()), al, ah),
                                };
                                E::Bin(op, Box::new(p2), y, bl, bh)
                            }
                            other => other,
                        }
                    } else {
                        *av
                    };
                    return E::AssignTo(Box::new(pl), Box::new(nv), al, ah);
                }
                return E::Int(0, 0, 0);
            }
            // C2-R3: `*p` where p is a tracked REFERENCE local loads through the address; every other `*`
            // stays identity (C2: a reference IS the value), so only reference locals are diverted.
            match &inner {
                E::Assign(an, _av, al, ah) => {
                    let mut bn = String::from("!");
                    bn.push_str(an.as_str());
                    if self.is_refvar(bn.as_str()) {
                        let a0: Vec<E> = Vec::new();
                        return E::Call(String::from("$deref_local_unsupported"), a0, *al, *ah);
                    }
                    return inner;
                } /* DEREF-LOCAL: atom's Ident arm swallows `= v`, so `*r = v` arrives here as Assign(r, v) -- the deref was silently dropped (r reassigned, the referent untouched) */
                E::Var(n, il, ih) => {
                    {
                        let mut bn = String::from("!");
                        bn.push_str(n.as_str());
                        if self.is_refvar(bn.as_str()) {
                            let a0: Vec<E> = Vec::new();
                            return E::Call(String::from("$deref_local_unsupported"), a0, *il, *ih);
                        }
                    }
                    if self.is_refvar(n.as_str()) {
                        let mut la: Vec<E> = Vec::new();
                        la.push(E::Var(String::from(n.as_str()), *il, *ih));
                        return E::Call(String::from("load_at"), la, *il, *ih);
                    }
                    return inner;
                }
                _ => {
                    return inner;
                }
            }
        }
        // unary MINUS: `-5`, `-x`, `-0xBEEF` -- read as `0 - operand`, which is what our int model is
        if self.is_tok("-") {
            let lo = self.lo();
            let _ = self.adv();
            let x = self.unary();
            let h = oast::e_hi(&x);
            return E::Bin(
                String::from("-"),
                Box::new(E::Int(0, lo, lo)),
                Box::new(x),
                lo,
                h,
            );
        }
        if self.is_tok("!") {
            let lo = self.lo();
            let _ = self.adv();
            let x = self.unary();
            let h = oast::e_hi(&x);
            return E::Not(Box::new(x), lo, h);
        }
        self.postfix()
    }
    // PREC-AS (rustc_parse AssocOp::As): unary > `as` > `*` `/` `%`. Chains left to right: `x as u8 as i64`.
    fn cast_e(&mut self) -> E {
        let mut e = self.unary();
        while self.is_tok("as") {
            let _ = self.adv();
            // V1b BOX-IDENTITY: `&b as *const T` (or `&*b`) of a Box cell is the cell handle: the allocation's address
            if self.boxid == 1 && self.is_tok("*") {
                let mut cell = String::new();
                if let E::Ref(rx, _m, _rl, _rh) = &e {
                    if let E::Var(rn, _vl, _vh) = rx.as_ref() {
                        if self.is_boxvar(rn.as_str()) {
                            cell = String::from(rn.as_str());
                        }
                    }
                }
                if cell.len() > 0 {
                    let a0 = oast::e_lo(&e);
                    let b0 = oast::e_hi(&e);
                    let mut ca: Vec<E> = Vec::new();
                    ca.push(E::Var(cell, a0, b0));
                    e = E::Call(String::from("$cellhandle"), ca, a0, b0);
                }
            }
            let t = self.parse_tyname();
            let a = oast::e_lo(&e);
            let b = if self.i > 0 { self.t[self.i - 1].hi } else { a };
            e = E::Cast(Box::new(e), t, a, b);
        }
        e
    }
    fn mul(&mut self) -> E {
        let mut l = self.cast_e();
        loop {
            if self.is_tok("*") && block_form(&l) {
                break;
            }
            if self.is_tok("*") {
                let _ = self.adv();
                let r = self.cast_e();
                let a = oast::e_lo(&l);
                let b = oast::e_hi(&r);
                l = E::Bin(String::from("*"), Box::new(l), Box::new(r), a, b);
            } else if self.is_tok("/") {
                let _ = self.adv();
                let r = self.cast_e();
                let a = oast::e_lo(&l);
                let b = oast::e_hi(&r);
                l = E::Bin(String::from("/"), Box::new(l), Box::new(r), a, b);
            }
            // K3 (probed from RC): `%` is multiplicative, and on negatives it truncates toward zero --
            // the remainder takes the DIVIDEND's sign (`-7 % 2 == -1`). Our own prelude had been writing
            // `v - v / 10 * 10` for want of it, which is a constraint we had accumulated without stating.
            else if self.is_tok("%") {
                let _ = self.adv();
                let r = self.cast_e();
                let a = oast::e_lo(&l);
                let b = oast::e_hi(&r);
                l = E::Bin(String::from("%"), Box::new(l), Box::new(r), a, b);
            } else {
                break;
            }
        }
        l
    }
    // the BITWISE layers, in Rust's order: `|` binds loosest, then `^`, then `&`, then the shifts
    fn bor_e(&mut self) -> E {
        let mut l = self.bxor_e();
        while self.is_tok("|") {
            let _ = self.adv();
            let r = self.bxor_e();
            let a = oast::e_lo(&l);
            let b = oast::e_hi(&r);
            l = E::Bin(String::from("|"), Box::new(l), Box::new(r), a, b);
        }
        l
    }
    fn bxor_e(&mut self) -> E {
        let mut l = self.band_e();
        while self.is_tok("^") {
            let _ = self.adv();
            let r = self.band_e();
            let a = oast::e_lo(&l);
            let b = oast::e_hi(&r);
            l = E::Bin(String::from("^"), Box::new(l), Box::new(r), a, b);
        }
        l
    }
    fn band_e(&mut self) -> E {
        let mut l = self.shift_e();
        while self.is_tok("&") {
            let _ = self.adv();
            let r = self.shift_e();
            let a = oast::e_lo(&l);
            let b = oast::e_hi(&r);
            l = E::Bin(String::from("&"), Box::new(l), Box::new(r), a, b);
        }
        l
    }
    fn shift_e(&mut self) -> E {
        let mut l = self.add();
        loop {
            if self.is_tok("<<") {
                let _ = self.adv();
                let r = self.add();
                let a = oast::e_lo(&l);
                let b = oast::e_hi(&r);
                l = E::Bin(String::from("<<"), Box::new(l), Box::new(r), a, b);
            } else if self.is_tok(">>") {
                let _ = self.adv();
                let r = self.add();
                let a = oast::e_lo(&l);
                let b = oast::e_hi(&r);
                l = E::Bin(String::from(">>"), Box::new(l), Box::new(r), a, b);
            } else {
                break;
            }
        }
        l
    }
    fn add(&mut self) -> E {
        let mut l = self.mul();
        loop {
            if self.is_tok("+") {
                let _ = self.adv();
                let r = self.mul();
                let a = oast::e_lo(&l);
                let b = oast::e_hi(&r);
                l = E::Bin(String::from("+"), Box::new(l), Box::new(r), a, b);
            } else if self.is_tok("-") {
                let _ = self.adv();
                let r = self.mul();
                let a = oast::e_lo(&l);
                let b = oast::e_hi(&r);
                l = E::Bin(String::from("-"), Box::new(l), Box::new(r), a, b);
            } else {
                break;
            }
        }
        l
    }
    // Sequencing. Rust lets a BLOCK-FORM statement (`while .. {}`, `if .. {}`) stand without a `;`, so after
    // parsing one element continue the sequence when the element ended in a block and something other than `}`
    // follows. Without this the remainder of the block was silently DROPPED (a tokenizer written in this
    // language lost every statement after an inner `while`, and the compiler reported ok).
    // A type NAME as a string: `int`, `Sq`, `dyn Area`, `Vec<dyn Area>`, `Vec<int>`. The layers below speak
    // type names, so parsing a nested type to a name (`Vec$dyn$Area`) keeps one representation everywhere.
    // step over a generic parameter list we do not need to bind (`impl<T, 'a>`, `trait Foo<T>`)
    // P-GEN: the one generic-signature reader. A parameter may carry type ARGUMENTS, associated-type
    // constraints, `+`-composed bounds and `?Sized`; the FIRST bound's name is what monomorphization keys
    // on. Every item kind calls this, so a bound form learned once is learned everywhere.
    // P-PATH: read a path's remaining `::seg` segments (and any turbofish) after the first identifier.
    // The old shape matched `Ident::Ident` as a two-segment special case, so `a::b::c`,
    // `Type::assoc::<T>` and `m::T::item` all derailed.
    // a range-pattern endpoint: an integer literal, a char literal (already a Num in our lexer), or a
    // CONST name used as a value
    // PAT-LIT-SUFFIX: the lexer turns `5i8` into `(5 as i8)`; in a pattern it is just the literal
    fn cast_lit_at(&self, q: usize) -> bool {
        q + 4 < self.t.len()
            && tkv_l_paren(&self.t[q].k)
            && ilit(&self.t[q + 1]) > 0
            && tkv_as_t(&self.t[q + 2].k)
            && tkv_r_paren(&self.t[q + 4].k)
    }
    fn pat_endpoint(&mut self, lo: u32, hi: u32) -> E {
        if self.i >= self.t.len() {
            return E::Int(0, lo, hi);
        }
        if self.cast_lit_at(self.i) {
            let v = ilit_val(&self.t[self.i + 1]);
            self.i = self.i + 5;
            return E::Int(v, lo, hi);
        }
        if tkv_minus(&self.t[self.i].k) && self.cast_lit_at(self.i + 1) {
            let v = ilit_val(&self.t[self.i + 2]);
            self.i = self.i + 6;
            return E::Int(0 - v, lo, hi);
        }
        match self.t[self.i].k.clone() {
            Tk::Minus => {
                let _ = self.adv();
                match self.t[self.i].k.clone() {
                    Tk::Num(n) => {
                        let _ = self.adv();
                        E::Int(0 - n, lo, hi)
                    }
                    _ => E::Int(0, lo, hi),
                }
            } // PAT-RANGE-NEG
            Tk::Num(n) => {
                let _ = self.adv();
                E::Int(n, lo, hi)
            }
            Tk::ChL(n) => {
                let _ = self.adv();
                E::Int(n, lo, hi)
            }
            Tk::Ident(w) => {
                let _ = self.adv();
                if self.is_tok("::") {
                    let _ = self.adv();
                    let ci = self.adv();
                    let c = tok_word(&self.t[ci]);
                    return E::Int(int_bound(w.as_str(), c.as_str()), lo, hi);
                } // PAT-RANGE-CONST: `i64::MIN..=-1`
                let ps: Vec<E> = Vec::new();
                E::Call(String::from(w.as_str()), ps, lo, hi)
            }
            _ => E::Int(0, lo, hi),
        }
    }
    // CONST-GEN-ARG (R3): the LITERAL const arguments of the turbofish at self.i (`::<i32, 3, true, { 4 }, -2>` -> 3, true, 4,
    // -2), in order; type arguments are skipped. Reads only -- the cursor is not moved.
    // CONST-GEN-ARG: append the turbofish consts that fill fn's HIDDEN const params (an array-bound one is read from its
    // array, so its turbofish value is not an argument). A count mismatch or unknown fn appends nothing (arity refusal).
    fn cg_append(&self, f: &str, tc: Vec<E>, ps: &mut Vec<E>) {
        let mut pre = String::from(f);
        pre.push('=');
        let mut ent = String::new();
        let mut q = 0;
        while q < self.cghid.len() {
            if oast::str_starts(self.cghid[q].as_str(), pre.as_str()) {
                ent = String::from(&self.cghid[q][pre.len()..]);
            }
            q = q + 1;
        }
        let eb = ent.as_bytes();
        let mut c = 0;
        while c < eb.len() && eb[c] != 124 {
            c = c + 1;
        }
        if c >= eb.len() {
            return;
        }
        let total = cg_num(&ent[..c]);
        if total != tc.len() as i64 {
            return;
        }
        let mut tc = tc;
        let mut idx: Vec<usize> = Vec::new();
        let mut cur = String::new();
        let rest = String::from(&ent[c + 1..]);
        let rb = rest.as_bytes();
        let mut k = 0;
        while k <= rb.len() {
            if k == rb.len() || rb[k] == 44 {
                if cur.len() > 0 {
                    idx.push(cg_num(cur.as_str()) as usize);
                }
                cur = String::new();
            } else {
                cur.push(rb[k] as char);
            }
            k = k + 1;
        }
        let mut j = tc.len();
        let mut keep: Vec<E> = Vec::new();
        while j > 0 {
            j = j - 1;
            let e = tc.remove(j);
            if cg_has(&idx, j) {
                keep.insert(0, e);
            }
        }
        while keep.len() > 0 {
            ps.push(keep.remove(0));
        }
    }
    fn tfish_consts(&self) -> Vec<E> {
        let mut out: Vec<E> = Vec::new();
        let mut q = self.i + 1;
        let mut d = 0;
        let mut arg: Vec<usize> = Vec::new();
        while q < self.t.len() {
            let k = &self.t[q].k;
            let closer = match k {
                Tk::Gt => true,
                Tk::Shr => true,
                Tk::Ge => true,
                _ => false,
            };
            let end = d == 0 && (tkv_comma(k) || closer);
            if end {
                let n = arg.len();
                let lo = if n > 0 { self.t[arg[0]].lo } else { 0 };
                let hi = if n > 0 { self.t[arg[n - 1]].hi } else { 0 };
                let one = if n == 1 {
                    arg[0]
                } else if n == 3 && tkv_l_brace(&self.t[arg[0]].k) && tkv_r_brace(&self.t[arg[2]].k)
                {
                    arg[1]
                } else {
                    0
                };
                if one > 0 {
                    match self.t[one].k.clone() {
                        Tk::Num(v) => out.push(E::Int(v, lo, hi)),
                        Tk::TrueT => out.push(E::Bool(true, lo, hi)),
                        Tk::FalseT => out.push(E::Bool(false, lo, hi)),
                        _ => {}
                    }
                } else if n == 2 && tkv_minus(&self.t[arg[0]].k) {
                    if let Tk::Num(v) = self.t[arg[1]].k.clone() {
                        out.push(E::Int(0 - v, lo, hi));
                    }
                }
                if !tkv_comma(k) {
                    break;
                }
                arg = Vec::new();
            } else {
                if tkv_lt(k) || tkv_l_paren(k) || tkv_l_brack(k) || tkv_l_brace(k) {
                    d = d + 1;
                } else if tkv_gt(k) || tkv_r_paren(k) || tkv_r_brack(k) || tkv_r_brace(k) {
                    d = d - 1;
                } else if closer {
                    d = d - 2;
                }
                arg.push(q);
            }
            q = q + 1;
        }
        out
    }
    fn read_path_tail(&mut self, segs: &mut Vec<String>) {
        loop {
            if !self.is_tok("::") {
                break;
            }
            if self.peek_is(1, "<") {
                let _ = self.adv();
                // C-SIZEOF: a turbofish type is MEANING for size_of, so it is read before being skipped.
                // Reading then rewinding keeps the skip as the single consumer of the token run.
                {
                    let save = self.i;
                    let _ = self.adv();
                    let t0 = self.i;
                    // TFISH-GT: parse_tyname's eat_gt SPLITS a `>>` / `>=` token IN PLACE; reading-then-rewinding left the
                    // mutated token for skip_generics, whose depth count then ran past the call (`size_of::<Option<&u8>>()`
                    // was a parse error). Snapshot the window and restore it on rewind.
                    let mut snk: Vec<Tk> = Vec::new();
                    let mut snl: Vec<u32> = Vec::new();
                    {
                        let mut q = save;
                        while q < self.t.len() && q < save + 256 {
                            snk.push(self.t[q].k.clone());
                            snl.push(self.t[q].lo);
                            q = q + 1;
                        }
                    }
                    {
                        // R4a LAYOUT: the FIRST type argument as raw token text, scanned from the ORIGINAL tokens
                        let mut tx = String::new();
                        let mut d: i32 = 0;
                        let mut pd: i32 = 0; // a `,` inside `( .. )` / `[ .. ]` is not the turbofish separator
                        let mut q = t0;
                        while q < self.t.len() {
                            self.i = q;
                            let w = self.tok_text();
                            if w.as_str() == "(" || w.as_str() == "[" {
                                pd = pd + 1;
                            }
                            if w.as_str() == ")" || w.as_str() == "]" {
                                pd = pd - 1;
                            }
                            if d == 0
                                && pd == 0
                                && (w.as_str() == ","
                                    || w.as_str() == ">"
                                    || w.as_str() == ">>"
                                    || w.as_str() == ">=")
                            {
                                break;
                            }
                            if w.as_str() == "<" {
                                d = d + 1;
                            }
                            if w.as_str() == ">" {
                                d = d - 1;
                            }
                            if w.as_str() == ">>" {
                                if d == 1 {
                                    if tx.len() > 0 {
                                        tx.push(' ');
                                    }
                                    tx.push('>');
                                    break;
                                }
                                d = d - 2;
                            }
                            if tx.len() > 0 {
                                tx.push(' ');
                            }
                            tx.push_str(w.as_str());
                            q = q + 1;
                        }
                        self.i = t0;
                        self.tfishtx = tx;
                    }
                    let t = self.parse_tyname();
                    self.tfish = t;
                    self.tfish2 = if self.is_tok(",") {
                        let _ = self.adv();
                        self.parse_tyname()
                    } else {
                        String::new()
                    };
                    self.i = save;
                    {
                        let mut q = 0;
                        while q < snk.len() {
                            self.t[save + q].k = snk[q].clone();
                            self.t[save + q].lo = snl[q];
                            q = q + 1;
                        }
                    } // TFISH-GT: the skip sees the original tokens
                }
                self.tfconst = self.tfish_consts();
                self.skip_generics();
                continue;
            } // turbofish
            let _ = self.adv();
            if self.i >= self.t.len() {
                break;
            }
            let w = tok_word(&self.t[self.i]);
            if w.len() == 0 {
                break;
            }
            segs.push(w);
            let _ = self.adv();
        }
    }
    // resolve a collected path to a callable/constructible name
    fn mod_known(&self, n: &str) -> bool {
        let mut i = 0;
        while i < self.mods.len() {
            if self.mods[i].as_str() == n {
                return true;
            }
            i = i + 1;
        }
        false
    }
    // the field names of a struct VARIANT, in declaration order ("" when it is positional)
    fn place_call(&mut self, e: E) -> E {
        let ixs = match &e {
            E::Call(f, _a, _l, _h) => self.place_idxs(f.as_str()),
            _ => Vec::new(),
        };
        if ixs.len() == 0 {
            return e;
        }
        let ofs = match &e {
            E::Call(f, _a, _l, _h) => self.place_ofs(f.as_str()),
            _ => 0,
        };
        {
            let mut ok = true;
            let mut z = 0;
            while z < ixs.len() {
                let kz = ixs[z] as usize;
                if self.place_argname(&e, kz).len() == 0 {
                    ok = false;
                }
                z = z + 1;
            }
            if !ok {
                if self.perr == 0 {
                    self.perr = self.lo() + 1;
                    self.perrexp=String::from("a `&mut` scalar argument that names a VARIABLE (the place is handed back by return, so the call site must have somewhere to put it)");
                    self.perrgot = self.tok_text();
                }
                return e;
            }
        }
        let lo = oast::e_lo(&e);
        let hi = oast::e_hi(&e);
        // the simple case keeps its simple shape: one place, nothing else to answer, so the call IS the
        // new value of the variable and the statement is an assignment.
        if ixs.len() == 1 && ofs == 0 {
            let vn = self.place_argname(&e, ixs[0] as usize);
            return E::Assign(vn, Box::new(e), lo, hi);
        }
        // otherwise the answer is a TUPLE: slot 0 is the declared return when there is one, then the
        //   places. `let $pl = f(..); a = $pl.N; ..; $pl.0` -- an EXPRESSION, so a call whose value is
        //   used reads the declared return and a call in statement position just drops it.
        let mut tn9 = String::from("$pl");
        tn9.push_str(&oast::u32s2(self.nhyg));
        self.nhyg = self.nhyg + 1;
        let mut acc = if ofs == 1 {
            E::TupIdx(
                Box::new(E::Var(String::from(tn9.as_str()), lo, hi)),
                0,
                lo,
                hi,
            )
        } else {
            E::Int(0, lo, hi)
        };
        let mut q9 = ixs.len();
        while q9 > 0 {
            q9 = q9 - 1;
            let vk = self.place_argname(&e, ixs[q9] as usize);
            let slot = (q9 as i32) + ofs;
            let rd = E::TupIdx(
                Box::new(E::Var(String::from(tn9.as_str()), lo, hi)),
                slot as u32,
                lo,
                hi,
            );
            let asg = E::Assign(vk, Box::new(rd), lo, hi);
            acc = E::Seq(Box::new(asg), Box::new(acc), lo, hi);
        }
        E::Let(tn9, Box::new(e), Box::new(acc), lo, hi)
    }
    fn place_argname(&self, e: &E, k: usize) -> String {
        match e {
            E::Call(_f, a, _l, _h) => {
                if k < a.len() {
                    match &a[k] {
                        E::Ref(x, _m, _l2, _h2) => match x.as_ref() {
                            E::Var(n, _l3, _h3) => String::from(n.as_str()),
                            _ => String::from(""),
                        },
                        E::Var(n, _l2, _h2) => String::from(n.as_str()),
                        _ => String::from(""),
                    }
                } else {
                    String::from("")
                }
            }
            _ => String::from(""),
        }
    }
    // DEREF-PLACE: a fresh copy of the fixed place `r` borrows (latest binding wins: shadowing)
    fn refplace(&self, r: &str) -> Option<E> {
        let mut i = self.refpn.len();
        while i > 0 {
            i = i - 1;
            if self.refpn[i].as_str() == r {
                return oast::place_clone(&self.refpe[i]);
            }
        }
        None
    }
    fn is_refvar(&self, n: &str) -> bool {
        let mut i = 0;
        while i < self.refvars.len() {
            if self.refvars[i].as_str() == n {
                return true;
            }
            i = i + 1;
        }
        false
    }
    fn refret_has(&self, n: &str) -> bool {
        let mut i = 0;
        while i < self.refret.len() {
            if self.refret[i].as_str() == n {
                return true;
            }
            i = i + 1;
        }
        false
    }
    fn rhs_is_ref(&self, v: &E) -> bool {
        match v {
            E::Ref(_x, m, _l, _h) => *m,
            E::Call(f, _a, _l, _h) => self.refret_has(f.as_str()),
            _ => false,
        }
    }
    fn place_known(&self, n: &str) -> bool {
        let mut i = 0;
        while i < self.plname.len() {
            if self.plname[i].as_str() == n {
                return true;
            }
            i = i + 1;
        }
        false
    }
    fn place_ofs(&self, n: &str) -> i32 {
        let mut i = 0;
        while i < self.plname.len() {
            if self.plname[i].as_str() == n {
                return self.plofs[i];
            }
            i = i + 1;
        }
        0
    }
    fn place_idxs(&self, n: &str) -> Vec<i32> {
        let mut o: Vec<i32> = Vec::new();
        let mut i = 0;
        while i < self.plname.len() {
            if self.plname[i].as_str() == n {
                o.push(self.plidx[i]);
            }
            i = i + 1;
        }
        o
    }
    fn is_smut(&self, n: &str) -> bool {
        let mut i = 0;
        while i < self.smut.len() {
            if self.smut[i].as_str() == n {
                return true;
            }
            i = i + 1;
        }
        false
    }
    fn variant_fields(&self, v: &str) -> String {
        let mut i = 0;
        while i < self.vnames.len() {
            if self.vnames[i].as_str() == v && i < self.vfn.len() {
                return String::from(self.vfn[i].as_str());
            }
            i = i + 1;
        }
        String::from("")
    }
    fn assoc_ty(&self, ty: &str, name: &str) -> String {
        let mut key = String::from(ty);
        key.push('$');
        key.push_str(name);
        let mut i = 0;
        while i < self.atn.len() {
            if self.atn[i].as_str() == key.as_str() {
                return String::from(self.att[i].as_str());
            }
            i = i + 1;
        }
        String::from("")
    }
    fn fknown(&self, n: &str) -> bool {
        let mut i = 0;
        while i < self.fnames.len() {
            if self.fnames[i].as_str() == n {
                return true;
            }
            i = i + 1;
        }
        false
    }
    // a glob is in effect only inside the module that declared it; entries are `<declaring mpfx>|<prefix>`
    fn gres(&self, n: &str) -> String {
        let mut i = 0;
        while i < self.globs.len() {
            // V5.1 GLOB-NOCOPY: the declaring module is compared as a String (no copy); only a glob of THIS module reads its
            // prefix bytes (as_bytes copies under ourc's runtime -- 340 globs x every name lookup was the self-compile's main cost)
            if i < self.globm.len() && self.globm[i].as_str() == self.mpfx.as_str() {
                let b = self.globp[i].as_bytes();
                if has_joined(&self.snames, b, 0, n)
                    || has_joined(&self.enames, b, 0, n)
                    || has_joined(&self.fnames, b, 0, n)
                {
                    let mut q = String::from(self.globp[i].as_str());
                    q.push('$');
                    q.push_str(n);
                    return q;
                }
            }
            i = i + 1;
        }
        String::from("")
    }
    // the TYPES-only glob resolver: a struct or enum name a glob brings into this module
    // the one resolver for a bare type name: exact, then this module's own, then an explicit `use` alias,
    // then a glob import. Asked by the struct-literal test, the enum-literal path and parse_tyname.
    fn tyres(&self, n: &str) -> String {
        if self.mpfx.len() > 0 {
            let q = self.qual(n);
            if self.sknown(&q) || self.enames_has(&q) {
                return q;
            }
        }
        {
            let a = self.use_alias_ty(n);
            if a.len() > 0 {
                return a;
            }
        }
        {
            let g = self.gres_ty(n);
            if g.len() > 0 {
                return g;
            }
        }
        if self.sknown(n) || self.enames_has(n) {
            return String::from(n);
        }
        String::from(n)
    }
    // the explicit-`use` half, types only (use_alias itself is the call-position resolver)
    fn use_alias_ty(&self, n: &str) -> String {
        let mut i = 0;
        while i < self.usen.len() {
            if self.usen[i].as_str() == n {
                let t = String::from(self.uset[i].as_str());
                if self.sknown(&t) || self.enames_has(&t) {
                    return t;
                }
            }
            i = i + 1;
        }
        String::from("")
    }
    fn gres_ty(&self, n: &str) -> String {
        let mut i = 0;
        while i < self.globs.len() {
            // V5.1 GLOB-NOCOPY: the declaring module is compared as a String (no copy); only a glob of THIS module reads its
            // prefix bytes (as_bytes copies under ourc's runtime -- 340 globs x every name lookup was the self-compile's main cost)
            if i < self.globm.len() && self.globm[i].as_str() == self.mpfx.as_str() {
                let b = self.globp[i].as_bytes();
                if has_joined(&self.snames, b, 0, n) || has_joined(&self.enames, b, 0, n) {
                    let mut q = String::from(self.globp[i].as_str());
                    q.push('$');
                    q.push_str(n);
                    return q;
                }
            }
            i = i + 1;
        }
        String::from("")
    }
    fn use_alias(&self, n: &str) -> String {
        // RES-SHADOW (rustc_resolve ribs): a LOCAL binding shadows an import -- the prelude's `fn map(v, f)` calls its
        // PARAMETER `f`, which `use m::f;` was rewriting to `m$f`. Parameters only; let/closure bindings: open.
        {
            let mut c = 0;
            while c < self.cparams.len() {
                if self.cparams[c].as_str() == n {
                    return String::from("");
                }
                c = c + 1;
            }
            // LET-SHADOWS-USE (W7): a `let` binding shadows a block-local import too (`let foo = || false; use bar::foo;
            // foo()` calls the closure: rustc's ribs put the local first). It called bar::foo -- a WRONG answer
            if oast::in_list(&self.lnames, n) {
                return String::from("");
            }
        }
        let mut i = 0;
        while i < self.usen.len() {
            if self.usen[i].as_str() == n {
                return String::from(self.uset[i].as_str());
            }
            i = i + 1;
        }
        if self.mpfx.len() > 0 {
            // USE-REL: a `use` inside this module was bound under the module prefix (`geo$sq`)
            let q = self.qual(n);
            let mut j = 0;
            while j < self.usen.len() {
                if self.usen[j].as_str() == q.as_str() {
                    return String::from(self.uset[j].as_str());
                }
                j = j + 1;
            }
        }
        // P-MC-MODFN: a bare call to a SIBLING function inside the same module. Functions in a module are
        // registered under the module prefix (`h$a`), so a bare `a()` in that module's own code did not
        // resolve -- the same rule P-MC-MODTYPE fixed for TYPES, missing for functions. Call position only,
        // so a local binding that shares a sibling's name is untouched.
        if self.mpfx.len() > 0 && !self.fknown(n) && self.peek_is(0, "(") {
            let q = self.qual(n);
            if self.fknown(&q) {
                return q;
            }
        }
        // then a glob import -- but a LOCAL item wins, as in rustc, and this runs only when a glob is in scope
        if self.globs.len() > 0 && !self.fknown(n) && self.peek_is(0, "(") {
            let mut local = false;
            if self.mpfx.len() > 0 {
                let q = self.qual(n);
                if self.fknown(&q) {
                    local = true;
                }
            }
            if !local {
                let g = self.gres(n);
                if g.len() > 0 && self.fknown(&g) {
                    return g;
                }
            }
        }
        String::from("")
    }
    fn resolve_path(&self, segs: &Vec<String>) -> String {
        if segs.len() == 0 {
            return String::from("");
        }
        if segs.len() > 1 && segs[0].as_str() == "crate" {
            let mut rest: Vec<String> = Vec::new();
            let mut k = 1;
            while k < segs.len() {
                rest.push(String::from(segs[k].as_str()));
                k = k + 1;
            }
            return self.resolve_path(&rest);
        }
        // RES-SELF-SUPER: `self::x` is x in THIS module, `super::x` in the parent (one `super` per level). mpfx is the
        // current module as `a$b$` ("" at the root), so both rewrite to an absolute `$` path.
        if segs.len() > 1 && (segs[0].as_str() == "self" || segs[0].as_str() == "super") {
            let mut parts: Vec<String> = Vec::new();
            {
                let b = self.mpfx.as_bytes();
                let mut cur = String::from("");
                let mut k = 0;
                while k < b.len() {
                    if b[k] == 36 {
                        if cur.len() > 0 {
                            parts.push(cur);
                        }
                        cur = String::from("");
                    } else {
                        cur.push(b[k] as char);
                    }
                    k = k + 1;
                }
            }
            let mut k = 0;
            while k < segs.len() && (segs[k].as_str() == "self" || segs[k].as_str() == "super") {
                if segs[k].as_str() == "super" && parts.len() > 0 {
                    let _ = parts.pop();
                }
                k = k + 1;
            }
            while k < segs.len() {
                parts.push(String::from(segs[k].as_str()));
                k = k + 1;
            }
            return self.resolve_path(&parts);
        }
        if segs.len() == 1 {
            return String::from(segs[0].as_str());
        }
        // `a::b::c..` -- join with `$`, which is how modules and associated items are named
        let mut joined = String::from(segs[0].as_str());
        let mut i = 1;
        while i < segs.len() {
            joined.push('$');
            joined.push_str(segs[i].as_str());
            i = i + 1;
        }
        // a MODULE-qualified item may itself be a two-segment name inside the module
        joined
    }
    fn read_generics(&mut self, gps: &mut Vec<String>, gbs: &mut Vec<String>) {
        if !self.is_tok("<") {
            return;
        }
        let _ = self.adv();
        loop {
            if self.at_gt() {
                let _ = self.eat_gt();
                break;
            }

            if self.at_end() {
                break;
            }
            if self.is_tok(",") {
                let _ = self.adv();
                continue;
            }
            if self.is_tok("const") && self.gen_fn {
                // CONST-GEN-LEN (R3): `fn f<const N: usize>(a: [T; N])` -- N is recorded; it must be bound to an array
                // parameter's length below (else refused), and reads that parameter's length in the body
                let _ = self.adv();
                let ni = self.adv();
                let cn = tok_word(&self.t[ni]);
                let mut ct = String::from("int");
                if self.is_tok(":") {
                    let _ = self.adv();
                    ct = self.parse_tyname();
                }
                if cn.len() > 0 {
                    self.cgen.push(cn);
                    self.cgty.push(ct);
                }
                continue;
            }
            if self.is_tok("const") && self.perr == 0 {
                self.perr = self.lo() + 1;
                self.perrexp = String::from(
                    "a type parameter (const generic parameters are not supported yet)",
                );
                self.perrgot = self.tok_text();
            } // CONST-GEN-REFUSE: the turbofish value was DROPPED (a wrong answer)
            let gi = self.adv();
            match self.t[gi].k.clone() {
                Tk::Ident(x) => {
                    gps.push(String::from(x.as_str()));
                    if self.is_tok(":") {
                        let _ = self.adv();
                        if self.is_tok("?") {
                            let _ = self.adv();
                        }
                        if self.is_tok("for") {
                            let _ = self.adv();
                            self.skip_generics();
                        }
                        let mut bn = String::from("");
                        if !self.at_gt() && !self.is_tok(",") {
                            let bi = self.adv();
                            match self.t[bi].k.clone() {
                                Tk::Ident(y) => {
                                    bn = String::from(y.as_str());
                                }
                                _ => {}
                            }
                            // BOUND-PATH: the bound's trait is the path's LAST segment (`std::ops::Add` names Add)
                            while self.is_tok("::") && self.i + 1 < self.t.len() {
                                let _ = self.adv();
                                let bj = self.adv();
                                if let Tk::Ident(y) = self.t[bj].k.clone() {
                                    bn = String::from(y.as_str());
                                }
                            }
                        }
                        self.skip_bound_tail();
                        loop {
                            if self.is_tok("+") {
                                let _ = self.adv();
                                if self.is_tok("?") {
                                    let _ = self.adv();
                                }
                                if !self.at_gt() && !self.is_tok(",") {
                                    let _ = self.adv();
                                }
                                self.skip_bound_tail();
                            } else {
                                break;
                            }
                        }
                        gbs.push(bn);
                    } else {
                        gbs.push(String::from(""));
                    }
                }
                _ => {}
            }
        }
    }
    // after a bound NAME: `Fn(A, B) -> R` carries a parenthesized list and a return type
    fn skip_bound_tail(&mut self) {
        // BOUND-PATH (R2a P5): a bound written as a PATH (`std::ops::Add<Output = T>`) -- the remaining `::seg` segments
        // come first; only the first segment had been read, so `<Output = T>`'s `>` closed the parameter list
        while self.is_tok("::") {
            let _ = self.adv();
            let _ = self.adv();
        }
        self.skip_generics();
        if self.is_tok("(") {
            let mut d = 0;
            loop {
                if self.at_end() {
                    break;
                }
                if self.is_tok("(") {
                    d = d + 1;
                } else if self.is_tok(")") {
                    d = d - 1;
                    let _ = self.adv();
                    if d == 0 {
                        break;
                    }
                    continue;
                }
                let _ = self.adv();
            }
        }
        if self.is_tok("-") && self.peek_is(1, ">") {
            let _ = self.adv();
            let _ = self.adv();
            let _ = self.parse_tyname();
        }
    }
    fn skip_generics(&mut self) {
        if !self.is_tok("<") {
            return;
        }
        let mut d = 0;
        loop {
            if self.at_end() {
                break;
            }
            if word_is(&self.t[self.i], "f32") && self.perr == 0 {
                self.perr = self.t[self.i].lo + 1;
                self.perrexp =
                    String::from("a type other than `f32` (f32 is not implemented; f64 is)");
                self.perrgot = String::from("f32");
            } // F32-TYPE: a skipped generic list (`.parse::<f32>()`) must not erase f32 into f64
            if self.is_tok("<") {
                d = d + 1;
            } else if self.at_gt() {
                d = d - 1;
                let _ = self.eat_gt();
                if d == 0 {
                    break;
                }
                continue;
            } else if self.is_tok(">>") {
                d = d - 2;
                let _ = self.adv();
                if d <= 0 {
                    break;
                }
                continue;
            }
            let _ = self.adv();
        }
    }
    // a `where` clause carries no meaning here: skip to the body or the `;`
    // consume a balanced token run up to (not including) the matching `}` -- the opening brace has
    // already been eaten by the caller
    fn skip_body(&mut self) -> E {
        let lo = self.lo();
        let mut d = 0;
        loop {
            if self.at_end() {
                break;
            }
            if self.is_tok("}") {
                if d == 0 {
                    break;
                }
                d = d - 1;
            } else if self.is_tok("{") {
                d = d + 1;
            }
            let _ = self.adv();
        }
        E::Int(0, lo, lo)
    }
    fn skip_to_semi(&mut self) -> E {
        let lo = self.lo();
        let mut d = 0;
        loop {
            if self.at_end() {
                break;
            }
            if self.is_tok(";") && d == 0 {
                break;
            }
            if self.is_tok("{") || self.is_tok("(") || self.is_tok("[") {
                d = d + 1;
            } else if self.is_tok("}") || self.is_tok(")") || self.is_tok("]") {
                d = d - 1;
            }
            let _ = self.adv();
        }
        E::Int(0, lo, lo)
    }
    fn skip_where(&mut self) {
        let g: Vec<String> = Vec::new();
        let g2: Vec<String> = Vec::new();
        let mut b: Vec<String> = Vec::new();
        self.read_where(&g, &mut b, &g2);
    }
    // `where T: Mark` carries the SAME bound a `<T: Mark>` list does -- discarding it made a bounded
    // generic look unbounded, so no monomorphization ran and the method call emitted an undefined name
    fn read_where(&mut self, gps: &Vec<String>, gbs: &mut Vec<String>, _unused: &Vec<String>) {
        if !self.is_tok("where") {
            return;
        }
        let _ = self.adv();
        loop {
            if self.at_end() {
                break;
            }
            if self.is_tok("{") || self.is_tok(";") {
                break;
            }
            if self.is_tok(",") {
                let _ = self.adv();
                continue;
            }
            let w = tok_word(&self.t[self.i]);
            if w.len() == 0 {
                let _ = self.adv();
                continue;
            }
            let _ = self.adv();
            if !self.is_tok(":") {
                continue;
            }
            let _ = self.adv();
            if self.is_tok("?") {
                let _ = self.adv();
            }
            // a HIGHER-RANKED bound: `for<'a> Fn(&'a u8) -> u8`. The quantifier binds lifetimes, which we
            // do not have, so it is skipped -- and the bound NAME after it is what must be recorded.
            if self.is_tok("for") {
                let _ = self.adv();
                self.skip_generics();
            }
            let mut bn = String::from("");
            if !self.is_tok("{") && !self.is_tok(",") && !self.is_tok(";") {
                let bw = tok_word(&self.t[self.i]);
                if bw.len() > 0 {
                    bn = bw;
                    let _ = self.adv();
                } else {
                    let _ = self.adv();
                }
            }
            if (bn.as_str() == "Fn" || bn.as_str() == "FnMut" || bn.as_str() == "FnOnce")
                && self.is_tok("(")
            {
                // FN-BOUND-RET (W7): `where F: FnOnce(String) -> String` -- the bound's RETURN type, so `f(..)` is typed
                let mut k = self.i;
                let mut d = 0;
                while k < self.t.len() {
                    if tkv_l_paren(&self.t[k].k) {
                        d = d + 1;
                    } else if tkv_r_paren(&self.t[k].k) {
                        d = d - 1;
                        if d == 0 {
                            break;
                        }
                    }
                    k = k + 1;
                }
                {
                    // FN-BOUND-ARGS (W7): the bound's ARG types `(A, B)` (one word each; `&` / `mut` skipped; String -> str)
                    let mut args = String::new();
                    let mut a = self.i + 1;
                    let mut cur = String::new();
                    let mut d2 = 0;
                    let mut ok = 1;
                    while a <= k {
                        let atend = a == k;
                        if !atend
                            && (tkv_l_paren(&self.t[a].k)
                                || tkv_lt(&self.t[a].k)
                                || tkv_l_brack(&self.t[a].k))
                        {
                            d2 = d2 + 1;
                            ok = 0;
                        } else if !atend
                            && (tkv_r_paren(&self.t[a].k)
                                || tkv_gt(&self.t[a].k)
                                || tkv_r_brack(&self.t[a].k))
                        {
                            d2 = d2 - 1;
                        } else if atend || (d2 == 0 && tkv_comma(&self.t[a].k)) {
                            if cur.len() > 0 {
                                if args.len() > 0 {
                                    args.push(',');
                                }
                                args.push_str(if cur.as_str() == "String" {
                                    "str"
                                } else {
                                    cur.as_str()
                                });
                            }
                            cur = String::new();
                        } else {
                            let w2 = tok_word(&self.t[a]);
                            if w2.len() > 0 && w2.as_str() != "mut" && cur.len() == 0 {
                                cur = w2;
                            }
                        }
                        a = a + 1;
                    }
                    if ok == 1 && args.len() > 0 {
                        let mut e = String::from(w.as_str());
                        e.push('=');
                        e.push_str(args.as_str());
                        self.fnargsv.push(e);
                    }
                }
                if k + 3 < self.t.len() && tkv_minus(&self.t[k + 1].k) && tkv_gt(&self.t[k + 2].k) {
                    let rw = tok_word(&self.t[k + 3]);
                    let rw = if rw.as_str() == "String" {
                        String::from("str")
                    } else {
                        rw
                    };
                    if rw.len() > 0 && !(k + 4 < self.t.len() && tkv_lt(&self.t[k + 4].k)) {
                        let mut e = String::from(w.as_str());
                        e.push('=');
                        e.push_str(rw.as_str());
                        self.fnrets.push(e);
                    }
                }
            }
            self.skip_bound_tail();
            loop {
                if self.is_tok("+") {
                    let _ = self.adv();
                    if self.is_tok("?") {
                        let _ = self.adv();
                    }
                    if !self.is_tok("{") && !self.is_tok(",") && !self.is_tok(";") {
                        let _ = self.adv();
                    }
                    self.skip_bound_tail();
                } else {
                    break;
                }
            }
            // record it against the generic parameter of that name, if the caller gave us the list
            if bn.len() > 0 {
                let mut q = 0;
                while q < gps.len() {
                    if gps[q].as_str() == w.as_str() && q < gbs.len() && gbs[q].len() == 0 {
                        gbs[q] = String::from(bn.as_str());
                    }
                    q = q + 1;
                }
            }
        }
    }
    // a loop iteration that consumed nothing: report it and step over one token. Used by every inner loop
    // that can meet a token it does not expect -- without it the parser spins forever.
    // is the current token the identifier `w`? (keywords go through tk_is; this is for words like
    // `macro_rules` that the lexer has no token for)
    // ---- macro_rules EXPANSION ----
    // Match an invocation's tokens against a rule's pattern, binding each `$name:frag` to a token run.
    // A metavariable consumes tokens until the pattern's NEXT literal token appears at depth 0 (or the
    // arguments run out), which covers the shapes real macros use without a full fragment parser.
    fn mac_match(
        &self,
        pat: &Vec<Tok>,
        args: &Vec<Tok>,
        ns: &mut Vec<String>,
        vs: &mut Vec<Vec<Tok>>,
    ) -> bool {
        let mut pi = 0;
        let mut ai = 0;
        while pi < pat.len() {
            // MR-TT-REP: a repetition INSIDE a pattern, `$( $t:frag )*` / `+` (no separator, as in `($h:tt $($t:tt)*)`): binds t to
            // ALL the tokens up to the pattern's next literal (or the end) -- possibly none for `*`
            if tkv_dollar(&pat[pi].k) && pi + 1 < pat.len() && tkv_l_paren(&pat[pi + 1].k) {
                let mut d = 0;
                let mut rc = pi + 1;
                while rc < pat.len() {
                    if tok_opens(&pat[rc]) {
                        d = d + 1;
                    } else if tok_closes(&pat[rc]) {
                        d = d - 1;
                        if d == 0 {
                            break;
                        }
                    }
                    rc = rc + 1;
                }
                let mut nm = String::from("");
                {
                    let mut q = pi + 2;
                    while q < rc {
                        if tkv_dollar(&pat[q].k) && q + 1 < rc {
                            nm = tok_word(&pat[q + 1]);
                            break;
                        }
                        q = q + 1;
                    }
                }
                let mut pj = rc + 1;
                // MR-REP-SEP: an in-pattern `$( .. ),*` / `,+` -- the run (commas and all) is the capture, which the
                // transcriber splits on top-level commas; any other separator is not modelled, so the rule does not match
                if pj + 1 < pat.len() && (tkv_star(&pat[pj + 1].k) || tkv_plus(&pat[pj + 1].k)) {
                    if !tkv_comma(&pat[pj].k) {
                        return false;
                    }
                    pj = pj + 1;
                }
                let star = pj < pat.len() && tkv_star(&pat[pj].k);
                if pj < pat.len() && (tkv_star(&pat[pj].k) || tkv_plus(&pat[pj].k)) {
                    pj = pj + 1;
                }
                let mut cap: Vec<Tok> = Vec::new();
                let mut dd = 0;
                while ai < args.len() {
                    if dd == 0 && pj < pat.len() && tok_same(&args[ai], &pat[pj]) {
                        break;
                    }
                    if tok_opens(&args[ai]) {
                        dd = dd + 1;
                    }
                    if tok_closes(&args[ai]) {
                        if dd == 0 {
                            break;
                        }
                        dd = dd - 1;
                    }
                    cap.push(tok_clone(&args[ai]));
                    ai = ai + 1;
                }
                if cap.len() == 0 && !star {
                    return false;
                }
                // R5 MR-REP-PREFIX: `$h:expr $(, $t:expr)*` writes the separator INSIDE the group, before `$t` -- the run
                // captured `, 2, 3`, whose leading `,` split into an EMPTY first element. Strip a literal prefix token that
                // the group starts with, so the capture has the `$($t),*` shape (`2, 3`).
                if pi + 2 < rc
                    && !tkv_dollar(&pat[pi + 2].k)
                    && cap.len() > 0
                    && tok_same(&cap[0], &pat[pi + 2])
                {
                    let mut c2: Vec<Tok> = Vec::new();
                    let mut q = 1;
                    while q < cap.len() {
                        c2.push(tok_clone(&cap[q]));
                        q = q + 1;
                    }
                    cap = c2;
                }
                {
                    // MR-TT-ELEMS (heavy pass 2): a repetition of `$t:tt` (`$($code:tt)*`) has ONE ELEMENT PER TOKEN TREE; the
                    // transcriber split runs on top-level commas, so `$(+ $t)*` over `1 2 3` was ONE element and
                    // `{ $($code)* }` of statements expanded to garbage/nothing (a silent WRONG). Mark the tree boundaries
                    // with `__ttsep__` (literal commas stay tokens); the transcriber splits on it when present.
                    let mut istt = false;
                    let mut q = pi + 2;
                    while q + 2 < rc {
                        if tkv_dollar(&pat[q].k)
                            && tkv_colon(&pat[q + 2].k)
                            && q + 3 < rc
                            && word_is(&pat[q + 3], "tt")
                        {
                            istt = true;
                        }
                        q = q + 1;
                    }
                    if istt && cap.len() > 0 {
                        let mut c3: Vec<Tok> = Vec::new();
                        let mut dd = 0;
                        let mut k = 0;
                        while k < cap.len() {
                            if dd == 0 && c3.len() > 0 {
                                c3.push(Tok {
                                    k: Tk::Ident(String::from("__ttsep__")),
                                    lo: cap[k].lo,
                                    hi: cap[k].lo,
                                });
                            }
                            if tok_opens(&cap[k]) {
                                dd = dd + 1;
                            } else if tok_closes(&cap[k]) {
                                dd = dd - 1;
                            }
                            c3.push(tok_clone(&cap[k]));
                            k = k + 1;
                        }
                        cap = c3;
                    }
                }
                ns.push(nm);
                vs.push(cap);
                pi = pj;
                continue;
            }
            // MR-TT-REP: `$x:tt` is exactly ONE token tree -- one token, or a balanced ( ) [ ] { } group
            if tkv_dollar(&pat[pi].k)
                && pi + 3 < pat.len()
                && tkv_colon(&pat[pi + 2].k)
                && word_is(&pat[pi + 3], "tt")
            {
                if ai >= args.len() {
                    return false;
                }
                let mut cap: Vec<Tok> = Vec::new();
                if tok_opens(&args[ai]) {
                    let mut dd = 0;
                    while ai < args.len() {
                        if tok_opens(&args[ai]) {
                            dd = dd + 1;
                        }
                        if tok_closes(&args[ai]) {
                            dd = dd - 1;
                        }
                        cap.push(tok_clone(&args[ai]));
                        ai = ai + 1;
                        if dd == 0 {
                            break;
                        }
                    }
                } else {
                    if tok_closes(&args[ai]) {
                        return false;
                    }
                    cap.push(tok_clone(&args[ai]));
                    ai = ai + 1;
                }
                ns.push(tok_word(&pat[pi + 1]));
                vs.push(cap);
                pi = pi + 4;
                continue;
            }
            if tkv_dollar(&pat[pi].k) && pi + 1 < pat.len() {
                let nm = tok_word(&pat[pi + 1]);
                let mut pj = pi + 2;
                let mut isexpr = false;
                if pj < pat.len() && tkv_colon(&pat[pj].k) {
                    if pj + 1 < pat.len() {
                        isexpr = word_is(&pat[pj + 1], "expr");
                    } // a `:meta` fragment is refused by mac_expand before the rule is tried (META-FRAG-REFUSE)

                    pj = pj + 2;
                } /* META-FRAG-REFUSE */
                // skip `:frag`
                // the delimiter that ends this capture
                let mut cap: Vec<Tok> = Vec::new();
                let mut d = 0;
                while ai < args.len() {
                    if d == 0 && pj < pat.len() && tok_same(&args[ai], &pat[pj]) {
                        break;
                    }
                    if isexpr && d == 0 && (tkv_comma(&args[ai].k) || tkv_semi(&args[ai].k)) {
                        break;
                    } // MR-EXPR-GROUP: an expr never spans a top-level , or ;
                    if tok_opens(&args[ai]) {
                        d = d + 1;
                    }
                    if tok_closes(&args[ai]) {
                        if d == 0 {
                            break;
                        }
                        d = d - 1;
                    }
                    cap.push(tok_clone(&args[ai]));
                    ai = ai + 1;
                }
                if cap.len() == 0 {
                    return false;
                }
                // MR-EXPR-GROUP (rustc_expand): an `$x:expr` capture is ONE expression node -- `dbl!(1 + 2)` with body `$x * 2`
                // is 6, not 5. Captured tokens were pasted raw; a multi-token expr capture is now parenthesized here.
                if isexpr && cap.len() > 1 {
                    let l0 = cap[0].lo;
                    let h0 = cap[cap.len() - 1].hi;
                    let mut g: Vec<Tok> = Vec::new();
                    g.push(Tok {
                        k: Tk::LParen,
                        lo: l0,
                        hi: l0,
                    });
                    let mut q = 0;
                    while q < cap.len() {
                        g.push(tok_clone(&cap[q]));
                        q = q + 1;
                    }
                    g.push(Tok {
                        k: Tk::RParen,
                        lo: h0,
                        hi: h0,
                    });
                    cap = g;
                }
                ns.push(nm);
                vs.push(cap);
                pi = pj;
                continue;
            }
            if ai >= args.len() {
                return false;
            }
            if !tok_same(&pat[pi], &args[ai]) {
                return false;
            }
            pi = pi + 1;
            ai = ai + 1;
        }
        ai >= args.len()
    }
    // Substitute the bound captures into a rule body.
    // the identifiers a macro body BINDS with `let`
    fn mac_body_lets(&self, body: &Vec<Tok>) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        let mut i = 0;
        while i < body.len() {
            if tkv_let(&body[i].k) {
                let mut j = i + 1;
                if j < body.len() && tkv_mut(&body[j].k) {
                    j = j + 1;
                }
                if j < body.len() {
                    let w = tok_word(&body[j]);
                    if w.len() > 0 && !oast::in_list(&out, &w) {
                        out.push(w);
                    }
                }
            }
            // MAC-HYG-LABEL (W2): a label the transcriber DEFINES (`'x: loop { $e }`) is hygienic -- a caller's `break 'x` passed
            // in through $e must target the CALLER's loop (it bound the macro's: rustc `done`, ourc ran the outer loop on).
            if let Tk::Label(lx) = &body[i].k {
                if i + 1 < body.len() && tkv_colon(&body[i + 1].k) {
                    let mut key = String::from("'");
                    key.push_str(lx.as_str());
                    if !oast::in_list(&out, &key) {
                        out.push(key);
                    }
                }
            }
            // MAC-HYG-PAT: every other binding form a transcriber can introduce is hygienic too -- a match-arm binding
            // (`_x =>`, `x @ ..`), a payload binding in an arm pattern (`Some(x) =>`), `for x in`, closure params `|a, b|`.
            // Only `let` was renamed, so `match 9 { _x => $e }` captured the caller's `_x` (rustc 8, ourc 9).
            if is_bind_word(&body[i]) {
                let prev_path = i > 0 && tkv_path_sep(&body[i - 1].k);
                let nx = i + 1 < body.len();
                let mut binds = false;
                if !prev_path && nx && (tkv_fat_arrow(&body[i + 1].k) || tk_is(&body[i + 1].k, "@"))
                {
                    binds = true;
                }
                if i > 0 && tkv_for_t(&body[i - 1].k) {
                    binds = true;
                }
                // payload in an arm pattern: `( .. x )` or `( x, ..` closed by `) =>`
                if !prev_path && nx && (tkv_r_paren(&body[i + 1].k) || tkv_comma(&body[i + 1].k)) {
                    let mut d = 0;
                    let mut k = i + 1;
                    while k < body.len() {
                        if tok_opens(&body[k]) {
                            d = d + 1;
                        } else if tok_closes(&body[k]) {
                            if d == 0 {
                                if k + 1 < body.len() && tkv_fat_arrow(&body[k + 1].k) {
                                    binds = true;
                                }
                                break;
                            }
                            d = d - 1;
                        } else if d == 0 && (tkv_semi(&body[k].k) || tkv_fat_arrow(&body[k].k)) {
                            break;
                        }
                        k = k + 1;
                    }
                }
                // closure params: between a `|` that opens and the next `|`
                if i > 0 && (tkv_bar(&body[i - 1].k) || tkv_comma(&body[i - 1].k)) {
                    let mut k = i;
                    let mut ok = false;
                    while k > 0 {
                        k = k - 1;
                        let bar = tkv_bar(&body[k].k);
                        if bar {
                            ok = k == 0
                                || !is_bind_word(&body[k - 1]) && !tkv_r_paren(&body[k - 1].k);
                            break;
                        }
                        if !(tkv_comma(&body[k].k) || is_bind_word(&body[k])) {
                            break;
                        }
                    }
                    if ok && nx && (tkv_bar(&body[i + 1].k) || tkv_comma(&body[i + 1].k)) {
                        binds = true;
                    }
                }
                let w = tok_word(&body[i]);
                if binds && w.len() > 0 && !oast::in_list(&out, &w) {
                    out.push(w);
                }
            }
            i = i + 1;
        }
        out
    }
    fn mac_subst(&self, body: &Vec<Tok>, ns: &Vec<String>, vs: &Vec<Vec<Tok>>) -> Vec<Tok> {
        self.mac_subst_h(body, ns, vs, 0)
    }
    fn mac_subst_h(
        &self,
        body: &Vec<Tok>,
        ns: &Vec<String>,
        vs: &Vec<Vec<Tok>>,
        hyg: u32,
    ) -> Vec<Tok> {
        let lets = self.mac_body_lets(body);
        let mut out: Vec<Tok> = Vec::new();
        let mut i = 0;
        while i < body.len() {
            // MR-TT-REP: `$( $t )*` in a body pastes the tokens a pattern repetition bound to t
            if tkv_dollar(&body[i].k) && i + 1 < body.len() && tkv_l_paren(&body[i + 1].k) {
                let mut d = 0;
                let mut rc = i + 1;
                while rc < body.len() {
                    if tok_opens(&body[rc]) {
                        d = d + 1;
                    } else if tok_closes(&body[rc]) {
                        d = d - 1;
                        if d == 0 {
                            break;
                        }
                    }
                    rc = rc + 1;
                }
                // R5 MR-REP-TEMPLATE: `$( inner ) [sep] *|+` -- inner is a TEMPLATE expanded once per element of the bound
                // repetition (its capture split on top-level commas), with `sep` between; `$($t)*` of a comma-free run is one
                // element (the old paste). Only `$($t)*` exactly was handled: `$($t),*` left `, *` behind and `$(+ $t)*` was not
                // expanded at all.
                let mut nm = String::new();
                let mut hit = 0 - 1;
                {
                    let mut q2 = i + 2;
                    while q2 + 1 < rc && hit < 0 {
                        if tkv_dollar(&body[q2].k) {
                            let w = tok_word(&body[q2 + 1]);
                            let mut q = 0;
                            while q < ns.len() {
                                if ns[q].as_str() == w.as_str() {
                                    hit = q as i32;
                                    nm = String::from(w.as_str());
                                }
                                q = q + 1;
                            }
                        }
                        q2 = q2 + 1;
                    }
                }
                if hit >= 0 {
                    let mut j = rc + 1;
                    let mut sep: Vec<Tok> = Vec::new();
                    if j < body.len()
                        && !(tkv_star(&body[j].k) || tkv_plus(&body[j].k) || tkv_quest(&body[j].k))
                    {
                        sep.push(tok_clone(&body[j]));
                        j = j + 1;
                    }
                    if j < body.len()
                        && (tkv_star(&body[j].k) || tkv_plus(&body[j].k) || tkv_quest(&body[j].k))
                    {
                        j = j + 1;
                    }
                    let mut inner: Vec<Tok> = Vec::new();
                    let mut k = i + 2;
                    while k < rc {
                        inner.push(tok_clone(&body[k]));
                        k = k + 1;
                    }
                    let hv = &vs[hit as usize];
                    let mut elems: Vec<Vec<Tok>> = Vec::new();
                    let mut cur: Vec<Tok> = Vec::new();
                    let mut dd = 0;
                    k = 0;
                    while k < hv.len() {
                        if tok_opens(&hv[k]) {
                            dd = dd + 1;
                        } else if tok_closes(&hv[k]) {
                            dd = dd - 1;
                        }
                        let ttm = hv.len() > 0 && has_ttsep(hv); // MR-TT-ELEMS: a tt repetition splits on its tree markers
                        if (ttm && word_is(&hv[k], "__ttsep__"))
                            || (!ttm && dd == 0 && tkv_comma(&hv[k].k))
                        {
                            elems.push(cur);
                            cur = Vec::new();
                        } else {
                            cur.push(tok_clone(&hv[k]));
                        }
                        k = k + 1;
                    }
                    if cur.len() > 0 {
                        elems.push(cur);
                    }
                    let mut e = 0;
                    while e < elems.len() {
                        if e > 0 {
                            let mut s = 0;
                            while s < sep.len() {
                                out.push(tok_clone(&sep[s]));
                                s = s + 1;
                            }
                        }
                        let mut ns2: Vec<String> = Vec::new();
                        let mut vs2: Vec<Vec<Tok>> = Vec::new();
                        let mut q = 0;
                        while q < ns.len() {
                            ns2.push(String::from(ns[q].as_str()));
                            let mut c: Vec<Tok> = Vec::new();
                            let mut z = 0;
                            while z < vs[q].len() {
                                c.push(tok_clone(&vs[q][z]));
                                z = z + 1;
                            }
                            vs2.push(c);
                            q = q + 1;
                        }
                        ns2.push(String::from(nm.as_str()));
                        let mut c: Vec<Tok> = Vec::new();
                        let mut z = 0;
                        while z < elems[e].len() {
                            c.push(tok_clone(&elems[e][z]));
                            z = z + 1;
                        }
                        vs2.push(c);
                        let piece = self.mac_subst_h(&inner, &ns2, &vs2, hyg);
                        let mut z2 = 0;
                        while z2 < piece.len() {
                            out.push(tok_clone(&piece[z2]));
                            z2 = z2 + 1;
                        }
                        e = e + 1;
                    }
                    i = j;
                    continue;
                }
            }
            if tkv_dollar(&body[i].k) && i + 1 < body.len() {
                let nm = tok_word(&body[i + 1]);
                let mut hit = 0 - 1;
                let mut q = 0;
                while q < ns.len() {
                    if ns[q].as_str() == nm.as_str() {
                        hit = q as i32;
                    }
                    q = q + 1;
                }
                if hit >= 0 {
                    let hv = &vs[hit as usize];
                    let mut k = 0;
                    while k < hv.len() {
                        if !word_is(&hv[k], "__ttsep__") {
                            out.push(tok_clone(&hv[k])); // MR-TT-ELEMS markers never reach the output
                        }
                        k = k + 1;
                    }
                    i = i + 2;
                    continue;
                }
                // MR-UNBOUND: a `$name` the matched pattern never bound means part of the pattern was not modelled (the lexer drops
                // `#[..]` attributes and doc comments, so `#[$y:meta]` vanished and the rule matched without it) -- REFUSE via an
                // unresolvable name instead of pasting `$y` text (stringify!($y) became "" and the assert failed: a wrong answer)
                if nm.len() > 0 && nm.as_str() != "crate" {
                    out.push(Tok {
                        k: Tk::Ident(String::from("macro_metavariable_unbound_by_its_pattern")),
                        lo: body[i].lo,
                        hi: body[i].hi,
                    });
                    i = i + 2;
                    continue;
                }
            }
            if let Tk::Label(lx) = &body[i].k {
                let mut key = String::from("'");
                key.push_str(lx.as_str());
                if oast::in_list(&lets, &key) {
                    let mut nl = String::from(lx.as_str());
                    nl.push_str("$h");
                    nl.push_str(&u32s2(hyg));
                    out.push(Tok {
                        k: Tk::Label(nl),
                        lo: body[i].lo,
                        hi: body[i].hi,
                    });
                    i = i + 1;
                    continue;
                }
            } // MAC-HYG-LABEL: the body's own label definition and uses get the hygienic name
              // rename a body-introduced binding (and its uses) so it cannot capture the caller's name
            {
                let w = tok_word(&body[i]);
                if w.len() > 0 && oast::in_list(&lets, &w) {
                    let mut nw = String::from(w.as_str());
                    nw.push_str("$h");
                    nw.push_str(&u32s2(hyg));
                    out.push(Tok {
                        k: Tk::Ident(nw),
                        lo: body[i].lo,
                        hi: body[i].hi,
                    });
                    i = i + 1;
                    continue;
                }
            }
            out.push(tok_clone(&body[i]));
            i = i + 1;
        }
        out
    }
    // is this pattern a single repetition group? returns (inner, separator-is-comma) via out-params
    fn mac_rep_inner(&self, pat: &Vec<Tok>, inner: &mut Vec<Tok>) -> bool {
        if pat.len() < 4 {
            return false;
        }
        if !tkv_dollar(&pat[0].k) {
            return false;
        }
        if !tok_opens(&pat[1]) {
            return false;
        }
        // find the matching close
        let mut d = 0;
        let mut i = 1;
        let mut close = 0;
        while i < pat.len() {
            if tok_opens(&pat[i]) {
                d = d + 1;
            } else if tok_closes(&pat[i]) {
                d = d - 1;
                if d == 0 {
                    close = i;
                    i = pat.len();
                    continue;
                }
            }
            i = i + 1;
        }
        if close == 0 {
            return false;
        }
        // what follows must be `,*` / `;*` / `*` / `+` (optionally with a trailing `,`)
        let mut j = close + 1;
        if j < pat.len() && (tkv_comma(&pat[j].k) || tkv_semi(&pat[j].k)) {
            j = j + 1;
        }
        if j >= pat.len() {
            return false;
        }
        if !(tkv_star(&pat[j].k) || tkv_plus(&pat[j].k)) {
            return false;
        }
        let mut k = 2;
        while k < close {
            inner.push(tok_clone(&pat[k]));
            k = k + 1;
        }
        true
    }
    // split a token run on top-level commas
    // MR-TT-ELEMS: the groups of a `$($t:tt)*` repetition -- ONE PER TOKEN TREE (a token, or a balanced group)
    fn tt_split(&self, args: &Vec<Tok>) -> Vec<Vec<Tok>> {
        let mut out: Vec<Vec<Tok>> = Vec::new();
        let mut cur: Vec<Tok> = Vec::new();
        let mut d = 0;
        let mut k = 0;
        while k < args.len() {
            if tok_opens(&args[k]) {
                d = d + 1;
            } else if tok_closes(&args[k]) {
                d = d - 1;
            }
            cur.push(tok_clone(&args[k]));
            if d == 0 {
                out.push(cur);
                cur = Vec::new();
            }
            k = k + 1;
        }
        if cur.len() > 0 {
            out.push(cur);
        }
        out
    }
    fn mac_split(&self, args: &Vec<Tok>) -> Vec<Vec<Tok>> {
        let mut out: Vec<Vec<Tok>> = Vec::new();
        let mut cur: Vec<Tok> = Vec::new();
        let mut d = 0;
        let mut i = 0;
        while i < args.len() {
            if tok_opens(&args[i]) {
                d = d + 1;
            } else if tok_closes(&args[i]) {
                d = d - 1;
            }
            if d == 0 && tkv_comma(&args[i].k) {
                if cur.len() > 0 {
                    out.push(toks_clone(&cur));
                    cur = Vec::new();
                }
            } else {
                cur.push(tok_clone(&args[i]));
            }
            i = i + 1;
        }
        if cur.len() > 0 {
            out.push(cur);
        }
        out
    }
    // MR-REP-SEP: is this whole-pattern repetition separated by `;` (`$( .. );*`)?
    fn mac_rep_semi(&self, pat: &Vec<Tok>) -> bool {
        let mut d = 0;
        let mut i = 1;
        while i < pat.len() {
            if tok_opens(&pat[i]) {
                d = d + 1;
            } else if tok_closes(&pat[i]) {
                d = d - 1;
                if d == 0 {
                    return i + 1 < pat.len() && tkv_semi(&pat[i + 1].k);
                }
            }
            i = i + 1;
        }
        false
    }
    // split a token run on top-level semicolons
    fn mac_split_semi(&self, args: &Vec<Tok>) -> Vec<Vec<Tok>> {
        let mut out: Vec<Vec<Tok>> = Vec::new();
        let mut cur: Vec<Tok> = Vec::new();
        let mut d = 0;
        let mut i = 0;
        while i < args.len() {
            if tok_opens(&args[i]) {
                d = d + 1;
            } else if tok_closes(&args[i]) {
                d = d - 1;
            }
            if d == 0 && tkv_semi(&args[i].k) {
                if cur.len() > 0 {
                    out.push(toks_clone(&cur));
                    cur = Vec::new();
                }
            } else {
                cur.push(tok_clone(&args[i]));
            }
            i = i + 1;
        }
        if cur.len() > 0 {
            out.push(cur);
        }
        out
    }
    // expand a body containing `$( inner ),*` once per captured group
    fn mac_subst_rep(&self, body: &Vec<Tok>, groups: &Vec<Vec<Tok>>, inner: &Vec<Tok>) -> Vec<Tok> {
        let mut out: Vec<Tok> = Vec::new();
        let mut i = 0;
        while i < body.len() {
            if tkv_dollar(&body[i].k) && i + 1 < body.len() && tok_opens(&body[i + 1]) {
                // the repeated body, its close, and the separator
                let mut d = 0;
                let mut j = i + 1;
                let mut close = 0;
                while j < body.len() {
                    if tok_opens(&body[j]) {
                        d = d + 1;
                    } else if tok_closes(&body[j]) {
                        d = d - 1;
                        if d == 0 {
                            close = j;
                            j = body.len();
                            continue;
                        }
                    }
                    j = j + 1;
                }
                if close > 0 {
                    let mut rb: Vec<Tok> = Vec::new();
                    let mut k = i + 2;
                    while k < close {
                        rb.push(tok_clone(&body[k]));
                        k = k + 1;
                    }
                    let mut sep: Vec<Tok> = Vec::new();
                    let mut n2 = close + 1;
                    if n2 < body.len() && (tkv_comma(&body[n2].k) || tkv_semi(&body[n2].k)) {
                        sep.push(tok_clone(&body[n2]));
                        n2 = n2 + 1;
                    }
                    if n2 < body.len() && (tkv_star(&body[n2].k) || tkv_plus(&body[n2].k)) {
                        n2 = n2 + 1;
                    }
                    let mut g = 0;
                    while g < groups.len() {
                        if g > 0 {
                            let mut q = 0;
                            while q < sep.len() {
                                out.push(tok_clone(&sep[q]));
                                q = q + 1;
                            }
                        }
                        let mut ns: Vec<String> = Vec::new();
                        let mut vs: Vec<Vec<Tok>> = Vec::new();
                        if self.mac_match(inner, &groups[g], &mut ns, &mut vs) {
                            let piece = self.mac_subst(&rb, &ns, &vs);
                            let mut q2 = 0;
                            while q2 < piece.len() {
                                out.push(tok_clone(&piece[q2]));
                                q2 = q2 + 1;
                            }
                        }
                        g = g + 1;
                    }
                    i = n2;
                    continue;
                }
            }
            out.push(tok_clone(&body[i]));
            i = i + 1;
        }
        out
    }
    fn mac_index(&self, n: &str) -> i32 {
        let mut i = 0;
        while i < self.macs.len() {
            if self.macs[i].name.as_str() == n {
                return i as i32;
            }
            i = i + 1;
        }
        0 - 1
    }
    // Expand `name!(args)` at the cursor: collect the argument tokens, match a rule, splice the expansion
    // into the token stream in place of the invocation, and keep parsing.
    fn mac_expand(&mut self, mi: usize) -> bool {
        let start = self.i;
        let _ = self.adv(); // the macro name
        if self.is_tok("!") {
            let _ = self.adv();
        } else {
            self.i = start;
            return false;
        }
        if !(self.is_tok("(") || self.is_tok("[") || self.is_tok("{")) {
            self.i = start;
            return false;
        }
        let _ = self.adv();
        let mut args: Vec<Tok> = Vec::new();
        let mut d = 1;
        while self.i < self.t.len() {
            if tok_opens(&self.t[self.i]) {
                d = d + 1;
            } else if tok_closes(&self.t[self.i]) {
                d = d - 1;
                if d == 0 {
                    let _ = self.adv();
                    break;
                }
            }
            args.push(tok_clone(&self.t[self.i]));
            let _ = self.adv();
        }
        let after = self.i;
        // try each rule in order
        let mut r = 0;
        while r < self.macs[mi].rules.len() {
            // META-FRAG-REFUSE: a `$x:meta` fragment is not modelled -- refuse before trying the rule
            if self.perr == 0 {
                let mut meta = 0;
                {
                    let pat = &self.macs[mi].rules[r].pat;
                    let mut q = 0;
                    while q + 3 < pat.len() && meta == 0 {
                        if tkv_dollar(&pat[q].k) && tkv_colon(&pat[q + 2].k) && word_is(&pat[q + 3], "meta") {
                            meta = pat[q + 3].lo + 1;
                        }
                        q = q + 1;
                    }
                }
                if meta > 0 {
                    self.perr = meta;
                    self.perrexp = String::from("a macro fragment other than :meta (not supported yet)");
                    self.perrgot = String::from("meta");
                }
            }
            let mut ns: Vec<String> = Vec::new();
            let mut vs: Vec<Vec<Tok>> = Vec::new();
            // a REPETITION rule first: `$( .. ),*` binds a list of groups
            {
                let mut inner: Vec<Tok> = Vec::new();
                let isrep = {
                    let pat = &self.macs[mi].rules[r].pat;
                    self.mac_rep_inner(pat, &mut inner)
                };
                if isrep {
                    let istt = {
                        let mut t = false;
                        let mut q = 0;
                        while q + 2 < inner.len() {
                            if tkv_dollar(&inner[q].k)
                                && tkv_colon(&inner[q + 2].k)
                                && q + 3 < inner.len()
                                && word_is(&inner[q + 3], "tt")
                            {
                                t = true;
                            }
                            q = q + 1;
                        }
                        t
                    };
                    // MR-TT-ELEMS: a tt repetition has one group per TOKEN TREE (mac_split splits on commas: `run!(1 + 2)` was one
                    // group and expanded wrong; `{ $($code)* }` of statements expanded to nothing)
                    // MR-REP-SEP: split on the rule's own separator -- `$( .. );*` split on commas cut `x: 1, 2; y: 3` mid-group,
                    // and the pieces that then failed to match were dropped: the call expanded to nothing (a silent WRONG)
                    let semi = {
                        let pat = &self.macs[mi].rules[r].pat;
                        self.mac_rep_semi(pat)
                    };
                    let groups = if istt {
                        self.tt_split(&args)
                    } else if semi {
                        self.mac_split_semi(&args)
                    } else {
                        self.mac_split(&args)
                    };
                    // every group must match the repeated pattern, or this rule does not match (rustc then tries the next)
                    let mut allm = true;
                    {
                        let mut g = 0;
                        while g < groups.len() && allm {
                            let mut ns0: Vec<String> = Vec::new();
                            let mut vs0: Vec<Vec<Tok>> = Vec::new();
                            allm = self.mac_match(&inner, &groups[g], &mut ns0, &mut vs0);
                            g = g + 1;
                        }
                    }
                    if !allm {
                        r = r + 1;
                        continue;
                    }
                    self.nhyg = self.nhyg + 1;
                    let exp = {
                        let body = &self.macs[mi].rules[r].body;
                        self.mac_subst_rep(body, &groups, &inner)
                    };
                    let mut nt: Vec<Tok> = Vec::new();
                    let mut k = 0;
                    while k < start {
                        nt.push(tok_clone(&self.t[k]));
                        k = k + 1;
                    }
                    let mut e = 0;
                    while e < exp.len() {
                        nt.push(tok_clone(&exp[e]));
                        e = e + 1;
                    }
                    let mut z = after;
                    while z < self.t.len() {
                        nt.push(tok_clone(&self.t[z]));
                        z = z + 1;
                    }
                    self.t = nt;
                    self.i = start;
                    return true;
                }
            }
            let okm = {
                let pat = &self.macs[mi].rules[r].pat;
                self.mac_match(pat, &args, &mut ns, &mut vs)
            };
            if okm {
                self.nhyg = self.nhyg + 1;
                let hyg = self.nhyg;
                let exp = {
                    let body = &self.macs[mi].rules[r].body;
                    self.mac_subst_h(body, &ns, &vs, hyg)
                };
                {
                    let mut q = 0;
                    while q < exp.len() {
                        if word_is(&exp[q], "macro_metavariable_unbound_by_its_pattern")
                            && self.perr == 0
                        {
                            self.perr = self.t[start].lo + 1;
                            self.perrexp=String::from("a macro rule whose pattern binds every metavariable its body uses (attributes / doc comments in a pattern are not modelled)");
                            self.perrgot = String::from("an unbound metavariable");
                        }
                        q = q + 1;
                    }
                } // MR-UNBOUND: a hard refusal (stringify! would have swallowed the marker)
                let mut nt: Vec<Tok> = Vec::new();
                let mut k = 0;
                while k < start {
                    nt.push(tok_clone(&self.t[k]));
                    k = k + 1;
                }
                let mut e = 0;
                while e < exp.len() {
                    nt.push(tok_clone(&exp[e]));
                    e = e + 1;
                }
                let mut z = after;
                while z < self.t.len() {
                    nt.push(tok_clone(&self.t[z]));
                    z = z + 1;
                }
                self.t = nt;
                self.i = start;
                return true;
            }
            r = r + 1;
        }
        self.i = start;
        false
    }
    // `macro_rules! name { (pat) => (body) ; .. }` -- each rule's pattern and body are kept as TOKEN RUNS
    fn read_macro(&mut self) {
        let _ = self.adv();
        if self.is_tok("!") {
            let _ = self.adv();
        }
        let nm = if self.i < self.t.len() {
            tok_word(&self.t[self.i])
        } else {
            String::from("")
        };
        let _ = self.adv();
        if !(self.is_tok("{") || self.is_tok("(") || self.is_tok("[")) {
            return;
        }
        let _ = self.adv();
        let mut rules: Vec<MacRule> = Vec::new();
        loop {
            if self.at_end() {
                break;
            }
            if self.is_tok("}") || self.is_tok(")") || self.is_tok("]") {
                let _ = self.adv();
                break;
            }
            if self.is_tok(";") || self.is_tok(",") {
                let _ = self.adv();
                continue;
            }
            // the PATTERN group
            if !(self.is_tok("(") || self.is_tok("[") || self.is_tok("{")) {
                let _ = self.adv();
                continue;
            }
            let _ = self.adv();
            let mut pat: Vec<Tok> = Vec::new();
            let mut d = 1;
            while self.i < self.t.len() {
                if tok_opens(&self.t[self.i]) {
                    d = d + 1;
                } else if tok_closes(&self.t[self.i]) {
                    d = d - 1;
                    if d == 0 {
                        let _ = self.adv();
                        break;
                    }
                }
                pat.push(tok_clone(&self.t[self.i]));
                let _ = self.adv();
            }
            if self.is_tok("=>") {
                let _ = self.adv();
            }
            if !(self.is_tok("(") || self.is_tok("[") || self.is_tok("{")) {
                continue;
            }
            let _ = self.adv();
            let mut body: Vec<Tok> = Vec::new();
            let mut d2 = 1;
            while self.i < self.t.len() {
                if tok_opens(&self.t[self.i]) {
                    d2 = d2 + 1;
                } else if tok_closes(&self.t[self.i]) {
                    d2 = d2 - 1;
                    if d2 == 0 {
                        let _ = self.adv();
                        break;
                    }
                }
                body.push(tok_clone(&self.t[self.i]));
                let _ = self.adv();
            }
            rules.push(MacRule {
                pat: pat,
                body: body,
            });
        }
        if self.is_tok(";") {
            let _ = self.adv();
        }
        if nm.len() > 0 {
            self.macs.push(Mac {
                name: nm,
                rules: rules,
            });
        }
    }
    fn is_word_at(&self, k: usize, w: &str) -> bool {
        if k >= self.t.len() {
            return false;
        }
        match &self.t[k].k {
            Tk::Ident(x) => x.as_str() == w,
            _ => false,
        }
    }
    fn is_word(&self, w: &str) -> bool {
        if self.i >= self.t.len() {
            return false;
        }
        match &self.t[self.i].k {
            Tk::Ident(x) => x.as_str() == w,
            _ => false,
        }
    }
    fn noprog(&mut self, at: usize) -> bool {
        if self.i == at {
            if self.perr == 0 {
                self.perr = self.lo() + 1;
                self.perrexp = String::from("an expression");
                self.perrgot = self.tok_text();
            }
            let _ = self.adv();
            true
        } else {
            false
        }
    }
    fn peek_is(&self, d: usize, k: &str) -> bool {
        if self.i + d >= self.t.len() {
            return false;
        }
        tk_is(&self.t[self.i + d].k, k)
    }
    // the assignment TARGET for a compound assignment: rebuild the place from the parsed left side
    fn reparse_place(&mut self, l: &E, a: u32, b: u32) -> E {
        match l {
            E::Var(n, _l, _h) => E::Var(String::from(n.as_str()), a, b),
            // a global LOAD is a place: `COUNT += 1` rebuilds the same load and lowering turns the
            // assignment into a store
            E::Call(n, ar, _l, _h) if ar.len() == 0 && olower::is_gname(n) => {
                let z: Vec<E> = Vec::new();
                E::Call(String::from(n.as_str()), z, a, b)
            }
            E::Field(o, f, _l, _h) => E::Field(
                Box::new(self.reparse_place(o, a, b)),
                String::from(f.as_str()),
                a,
                b,
            ),
            E::TupIdx(o, ix, _l, _h) => E::TupIdx(Box::new(self.reparse_place(o, a, b)), *ix, a, b),
            // an INDEX place is only rebuilt when the index is side-effect-free (a local or a literal);
            // anything else would evaluate it twice, so it is refused rather than mis-compiled
            E::Index(bb, ix, _l, _h) => match ix.as_ref() {
                E::Var(n2, _l2, _h2) => E::Index(
                    Box::new(self.reparse_place(bb, a, b)),
                    Box::new(E::Var(String::from(n2.as_str()), a, b)),
                    a,
                    b,
                ),
                E::Int(v2, _l2, _h2) => E::Index(
                    Box::new(self.reparse_place(bb, a, b)),
                    Box::new(E::Int(*v2, a, b)),
                    a,
                    b,
                ),
                _ => E::Var(String::from("$noplace"), a, b),
            },
            _ => E::Var(String::from("$noplace"), a, b),
        }
    }
    // GEN-IMPL: inside `impl<T> S<T>`, the impl's own instance type `S<T>` (`S$T`) IS the generic struct S
    fn parse_tyname(&mut self) -> String {
        self.lastgargs = String::new();
        let t = self.parse_tyname0();
        self.lastfull = String::from(t.as_str());
        if self.implinst.len() > 0 && t.as_str() == self.implinst.as_str() {
            return String::from(self.implbase.as_str());
        }
        if t.as_str() == "Vec$" {
            return String::from("Vec");
        } /* VEC-INFER: `Vec<_>` is a Vec whose element comes from its initializer */
        // F32-TYPE: there is no f32 value domain (P-FLOAT f32 is its own rung) -- `fn f() -> f32 { 9007199791611905.0 }`
        // and `parse::<f32>()` computed in f64 and printed 9007199791611905 (rustc 9007200000000000), a WRONG answer.
        // Refuse f32 in EVERY type position, as `let x: f32` and the `3f32` suffix already are.
        // R4 REFCELL-TYPE: a RefCell is modelled only from a `RefCell::new(..)` CONSTRUCTION (a one-slot cell); a RefCell
        // in a TYPE position (a param `x: RefCell<String>`, a field) has no cell typing -- `x.borrow()` there read garbage
        // (issue-23338: assert failed; issue-3447 printed a pointer). Refuse until typed cells exist.
        // CELL-TYPE (W1/R4): `RefCell<X>` / `Mutex<X>` / `RwLock<X>` in a TYPE position is the same typed one-slot cell a
        // `RefCell::new(x)` value is -- spelled the way `Cell<X>` is (`Vec$X`, decell), so a param / field / annotation
        // reads it through the cell (`x.borrow()` -> the cell's value). Only an argument-less `RefCell` stays refused.
        let t = if oast::str_starts(t.as_str(), "RefCell$")
            || oast::str_starts(t.as_str(), "Mutex$")
            || oast::str_starts(t.as_str(), "RwLock$")
        {
            let tb = t.as_bytes();
            let mut c = 0;
            while c < tb.len() && tb[c] != 36 {
                c = c + 1;
            }
            let mut r = String::from("Vec$");
            r.push_str(&t[c + 1..]);
            r
        } else {
            t
        };
        if t.as_str() == "RefCell" && self.perr == 0 {
            self.perr = self.t[if self.i > 0 { self.i - 1 } else { 0 }].lo + 1;
            self.perrexp = String::from("a type other than RefCell<..> (a RefCell is supported only as a `RefCell::new` value)");
            self.perrgot = String::from("RefCell");
        }
        if (t.as_str() == "f32" || oast::str_contains2(t.as_str(), "$f32")) && self.perr == 0 {
            self.perr = self.t[if self.i > 0 { self.i - 1 } else { 0 }].lo + 1;
            self.perrexp = String::from("a type other than `f32` (f32 is not implemented; f64 is)");
            self.perrgot = String::from("f32");
        }
        t
    }
    fn parse_tyname0(&mut self) -> String {
        if self.is_tok("crate") && self.peek_is(1, "::") {
            let _ = self.adv();
            let _ = self.adv();
            return self.parse_tyname();
        }
        if self.depth > 96 {
            if self.perr == 0 {
                self.perr = self.lo() + 1;
                self.perrexp = String::from("a type (nesting too deep)");
                self.perrgot = self.tok_text();
            }
            let _ = self.adv();
            return String::from("");
        }
        self.depth = self.depth + 1;
        let r = self.parse_tyname_inner();
        self.depth = self.depth - 1;
        r
    }
    fn parse_tyname_inner(&mut self) -> String {
        if self.is_tok("::") {
            let _ = self.adv();
        }
        // R1 HRTB: `for<'a> Fn(&'a T) -> U` -- a higher-ranked binder only names lifetimes, which carry no codegen
        // meaning here; skip it and read the type it binds (`dyn for<'b> Fn(..)` refused at the `<`)
        if (self.is_tok("for") || self.is_word("for")) && self.peek_is(1, "<") {
            let _ = self.adv();
            self.skip_generics();
            return self.parse_tyname();
        }
        if self.is_tok("&&") {
            let _ = self.adv();
            if self.is_tok("mut") {
                let _ = self.adv();
            }
            return self.parse_tyname();
        } // REF-REF: `&&T` lexes as one token
        if self.is_tok("&") {
            let _ = self.adv();
            if self.is_tok("mut") {
                let _ = self.adv();
            }
            return self.parse_tyname();
        }
        // `()` is the unit type; `(A, B)` a tuple; `[T]` / `[T; N]` a slice or array; `<T as Tr>::It` a
        // qualified path. Each renders to a NAME, the representation every layer below speaks.
        if self.is_tok("(") {
            let _ = self.adv();
            let mut nc = 0;
            let mut es: Vec<String> = Vec::new();
            loop {
                if self.is_tok(")") {
                    let _ = self.adv();
                    break;
                }
                if self.at_end() {
                    break;
                }
                if self.is_tok(",") {
                    let _ = self.adv();
                } else {
                    es.push(self.parse_tyname());
                    nc = nc + 1;
                }
            }
            if nc == 0 {
                return String::from("unit");
            }
            return tup_name(nc as u32, &es);
        }
        if self.is_tok("[") {
            let _ = self.adv();
            let el = self.parse_tyname();
            // `[T; N]` is an ARRAY (length at slot 0, data from 1); `[T]` is a SLICE, and the only values
            // that reach a slice parameter are vectors -- typing it `arr` read the vec header as data
            let mut semi = false;
            loop {
                if self.is_tok("]") {
                    let _ = self.adv();
                    break;
                }
                if self.at_end() {
                    break;
                }
                if self.is_tok(";") {
                    semi = true;
                }
                let _ = self.adv();
            }
            if semi {
                self.lastarrel = String::from(el.as_str());
                return String::from("arr");
            }
            let mut r = String::from("slice");
            if el.len() > 0 {
                r.push('$');
                r.push_str(el.as_str());
            }
            return r;
        }
        if self.is_tok("<") {
            let _ = self.adv();
            let base = self.parse_tyname();
            if self.is_tok("as") {
                let _ = self.adv();
                let _t = self.parse_tyname();
            }
            if self.at_gt() {
                let _ = self.eat_gt();
            }
            if self.is_tok("::") {
                let _ = self.adv();
                let ii = self.adv();
                let an = match &self.t[ii].k {
                    Tk::Ident(x) => String::from(x.as_str()),
                    _ => String::from(""),
                };
                // `<S as T1>::Out` is the impl's own `type Out = ..`
                let r = self.assoc_ty(&base, &an);
                if r.len() > 0 {
                    return r;
                }
                if an.len() > 0 {
                    // PROJ-FIELD (R3): unresolved here (the impl may come later); a struct field resolves it after the parse
                    self.lproj = String::from(base.as_str());
                    self.lproj.push('|');
                    self.lproj.push_str(&an);
                }
            }
            return base;
        }
        // R10 ABI-FN-TY: `extern "C" fn(i32) -> i32` / `unsafe extern "C" fn(..)` / `unsafe fn(..)` -- the ABI string and `unsafe`
        // do not change our value model of a Rust-defined fn pointer, so they are read past (the type failed to parse)
        {
            let save = self.i;
            if self.is_tok("unsafe") {
                let _ = self.adv();
            }
            if self.is_tok("extern") {
                let _ = self.adv();
                if let Tk::StrL(_s) = self.t[self.i].k.clone() {
                    let _ = self.adv();
                }
            }
            if !(self.is_tok("fn") && self.peek_is(1, "(")) {
                self.i = save;
            }
        }
        // FN-PTR-TY: `fn(A, B) -> R` is a function-pointer type -- the callable value type Fn (the same one closures use)
        if self.is_tok("fn") && self.peek_is(1, "(") {
            let _ = self.adv();
            let _ = self.adv();
            let mut d = 1;
            while !self.at_end() && d > 0 {
                if self.is_tok("(") {
                    d = d + 1;
                } else if self.is_tok(")") {
                    d = d - 1;
                }
                let _ = self.adv();
            }
            if self.is_tok("-") && self.peek_is(1, ">") {
                let _ = self.adv();
                let _ = self.adv();
                let _ = self.parse_tyname();
            }
            return String::from("Fn");
        }
        if self.is_tok("impl") {
            let _ = self.adv();
            // R1.4 HRTB-IMPL: `impl for<'a> Fn(&'a u32) -> &'a u32` -- the binder is not the trait name (the dyn branch already skips it)
            if (self.is_tok("for") || self.is_word("for")) && self.peek_is(1, "<") {
                let _ = self.adv();
                self.skip_generics();
            }
            let itn = if self.i < self.t.len() {
                tok_word(&self.t[self.i])
            } else {
                String::from("")
            }; // CLOS-VALUE: `impl Fn(..)` is a callable value
            if self.i < self.t.len() {
                let _ = self.adv();
            } // the trait name
            self.skip_bound_tail();
            loop {
                if self.is_tok("+") {
                    let _ = self.adv();
                    // R1.3b IMPL-PLUS-LT: a dropped lifetime (`impl Fn() -> T + '_ {`) leaves `+` before the closer or the fn
                    // BODY's `{`: consume a bound only when one is there (the `{` was eaten as the bound, and the body lost)
                    if self.i < self.t.len()
                        && !(self.is_tok("{")
                            || self.is_tok(">")
                            || self.is_tok(">>")
                            || self.is_tok(",")
                            || self.is_tok(")")
                            || self.is_tok("=")
                            || self.is_tok(";")
                            || self.is_tok("where"))
                    {
                        let _ = self.adv();
                        self.skip_bound_tail();
                    }
                } else {
                    break;
                }
            }
            if itn.as_str() == "Fn" || itn.as_str() == "FnMut" || itn.as_str() == "FnOnce" {
                return String::from("Fn");
            }
            return String::from("");
        }
        if self.is_tok("dyn") {
            let _ = self.adv();
            if (self.is_tok("for") || self.is_word("for")) && self.peek_is(1, "<") {
                let _ = self.adv();
                self.skip_generics();
            } // R1 HRTB: `dyn for<'b> Fn(..)` -- the binder is not the trait name
            let i = self.adv();
            let mut n = match &self.t[i].k {
                Tk::Ident(x) => String::from(x.as_str()),
                _ => String::from(""),
            };
            // DYN-PATH (R2a P5): `dyn l::Sh` / `dyn crate::l::Sh` -- a trait named through a path is its last segment (traits
            // are registered by name, as `use l::Sh; dyn Sh` already is); only the first segment was read and `::` was stray
            while self.is_tok("::") {
                let _ = self.adv();
                let j = self.adv();
                n = match &self.t[j].k {
                    Tk::Ident(x) => String::from(x.as_str()),
                    _ => String::from(""),
                };
            }
            // `dyn Trait<T>` / `dyn Trait<Assoc = T>` -- the arguments do not change the object's name
            self.skip_generics();
            self.skip_bound_tail();
            // R1.3 DYN-PLUS: `dyn FnMut() + 'static`, `dyn Debug + Send`, a trailing `dyn Debug +` -- the extra bounds do not
            // change the object either. The lexer DROPS lifetimes, so `+ 'static` reaches here as a `+` followed by the
            // type's closer: consume a bound only when one is actually there.
            while self.is_tok("+") {
                let _ = self.adv();
                if !(self.is_tok(">")
                    || self.is_tok(">>")
                    || self.is_tok(",")
                    || self.is_tok(")")
                    || self.is_tok("{")
                    || self.is_tok("=")
                    || self.is_tok(";")
                    || self.at_end())
                {
                    let _ = self.adv();
                    self.skip_generics();
                    self.skip_bound_tail();
                }
            }
            if n.as_str() == "Fn" || n.as_str() == "FnMut" || n.as_str() == "FnOnce" {
                return String::from("Fn");
            } // CLOS-VALUE: `dyn Fn(..)` is the callable value type
            let mut r = String::from("dyn$");
            r.push_str(&n);
            return r;
        }
        // a raw pointer type `*const T` / `*mut T` is the referent's type, like a reference
        if self.is_tok("*") {
            let _ = self.adv();
            if self.is_tok("mut") {
                let _ = self.adv();
            }
            let i2 = self.i;
            if self.is_tok("const") {
                let _ = self.adv();
            }
            let _ = i2;
            return self.parse_tyname();
        }
        let i = self.adv();
        let mut n = match &self.t[i].k {
            Tk::Ident(x) => String::from(x.as_str()),
            _ => String::from(""),
        };
        if n.as_str() == "_" {
            return String::from("");
        }
        if n.as_str() == "Self" && self.is_tok("::") && self.selfty.len() > 0 {
            let _ = self.adv();
            let ii = self.adv();
            let an = match &self.t[ii].k {
                Tk::Ident(x) => String::from(x.as_str()),
                _ => String::from(""),
            };
            // R1.9 GAT-ARGS: `Self::Iter<'a>` / `Self::Output<O>` -- a generic associated type's arguments follow its name
            // (inside an impl, where Self is known); they were left unread and the fn signature broke at the `<`
            if self.is_tok("<") {
                self.skip_generics();
            }
            let sty = String::from(self.selfty.as_str());
            let r = self.assoc_ty(&sty, &an);
            if r.len() > 0 {
                return r;
            }
            return sty;
        }
        if n.as_str() == "Self" && self.selfty.len() > 0 {
            return String::from(self.selfty.as_str());
        }
        // `M::T` inside a path, and a SHORT name inside `mod M` resolving to `M$T` -- without the latter a
        // parameter typed `Pt` inside `mod geom` missed the `geom$Pt` declaration and every field read
        // collapsed to offset 0
        if self.is_tok("::") {
            // every segment, not just one: `std::vec::Vec` is three
            loop {
                if !self.is_tok("::") {
                    break;
                }
                if self.peek_is(1, "<") {
                    break;
                }
                let _ = self.adv();
                if self.i >= self.t.len() {
                    break;
                }
                let w = tok_word(&self.t[self.i]);
                if w.len() == 0 {
                    break;
                }
                let _ = self.adv();
                n.push('$');
                n.push_str(&w);
            }
            // SUPER-TY (R2a P6): `super::T` / `self::T` in a TYPE is relative to the current module -- it collapsed to the bare
            // last segment, which named whichever `T` was registered first (`fn rd(t: &super::T)` in `n::sub` read `m::T`'s
            // fields: `$unknown_field$_$y`). Strip one module level per `super`, then use the qualified name if it is known.
            {
                let segs = split_dollar(&n);
                if segs.len() > 1 && (segs[0].as_str() == "super" || segs[0].as_str() == "self") {
                    let mut lv: Vec<String> = split_dollar(self.mpfx.as_str());
                    if lv.len() > 0 && lv[lv.len() - 1].len() == 0 {
                        let _ = lv.pop();
                    }
                    let mut k = 0;
                    while k < segs.len() - 1
                        && (segs[k].as_str() == "super" || segs[k].as_str() == "self")
                    {
                        if segs[k].as_str() == "super" && lv.len() > 0 {
                            let _ = lv.pop();
                        }
                        k = k + 1;
                    }
                    let mut cand = String::new();
                    let mut q = 0;
                    while q < lv.len() {
                        cand.push_str(lv[q].as_str());
                        cand.push('$');
                        q = q + 1;
                    }
                    while k < segs.len() {
                        cand.push_str(segs[k].as_str());
                        if k + 1 < segs.len() {
                            cand.push('$');
                        }
                        k = k + 1;
                    }
                    if self.sknown(&cand) || self.enames_has(&cand) {
                        n = cand;
                    }
                }
            }
            // the LAST segment is the type; a std path is transparent (`std$vec$Vec` is `Vec`)
            {
                let segs = split_dollar(&n);
                if segs.len() > 1 {
                    let last = String::from(segs[segs.len() - 1].as_str());
                    if !self.sknown(&n) && !self.enames_has(&n) {
                        n = last;
                    }
                }
            }
        }
        // P-MC-GLOBTYPE (REFUTED here, kept so the next rung does not retry it): rewriting a glob-imported
        // type name to the declaring module's name (`Arm` -> `oast$Arm`) fixes the field/element lookups
        // that key on the name, and the composed unit advanced 9.2% -> 11.4% -- but it REGRESSED a
        // module's own enum literal (`E::M(v)` inside the declaring module became "cannot find enum E")
        // and took the error count 26 -> 57. The lookups are what should become glob-aware, not the
        // recorded name. gres_ty below is kept: it is correct and unused until that rung.
        else if self.mpfx.len() > 0 {
            let q = self.qual(&n);
            if self.sknown(&q) || self.enames_has(&q) {
                n = q;
            }
        }
        // a std smart pointer is transparent -- but ONLY when the program has not declared a type of that
        // name (a user `struct Rc { w, h }` must win, or its fields vanish)
        if (n.as_str() == "Box" || n.as_str() == "Rc" || n.as_str() == "Arc")
            && !self.sknown(&n)
            && !self.enames_has(&n)
        {
            if self.is_tok("<") {
                let _ = self.adv();
                let inner = self.parse_tyname();

                if self.at_gt() {
                    let _ = self.eat_gt();
                }
                return inner;
            }
            return String::from("int");
        }
        if n.as_str() == "String" && !self.sknown(&n) && !self.enames_has(&n) {
            return String::from("str");
        }
        {
            let mut hit = false;
            let mut ai = 0;
            while ai < self.aliasn.len() {
                if self.aliasn[ai].as_str() == n.as_str() {
                    n = String::from(self.aliast[ai].as_str());
                    hit = true;
                    ai = self.aliasn.len();
                } else {
                    ai = ai + 1;
                }
            }
            if hit && self.is_tok("<") {
                self.skip_generics();
                return n;
            }
        }
        if self.is_tok("<") {
            // DBG-PAYLOAD: an Option/Result keeps its NAME (every check reads "Option"/"Result"), but the payload types are
            // remembered in lastpay ("T" or "T,E") for the fn-return record
            if self.enames_has(&n) && (n.as_str() == "Option" || n.as_str() == "Result") {
                let _ = self.adv();
                let t1 = self.parse_tyname();
                let mut pay = String::from(t1.as_str());
                if self.is_tok(",") {
                    let _ = self.adv();
                    let t2 = self.parse_tyname();
                    pay.push(',');
                    pay.push_str(&t2);
                }
                if self.at_gt() {
                    let _ = self.eat_gt();
                }
                self.lastpay = pay;
                return n;
            }
            if self.enames_has(&n) {
                // PARAM-GINST (W1): remember a single type argument for the fn-param record; the name stays erased
                let save = self.i;
                let _ = self.adv();
                let a = self.parse_tyname();
                if a.len() > 0 && self.at_gt() {
                    let _ = self.eat_gt();
                    self.lastgarg = a;
                } else {
                    self.i = save;
                    self.skip_generics();
                }
                return n;
            }
            // more than one argument (or an empty/lifetime-only list) -> keep the base name and skip the group
            {
                let mut d = 0;
                let mut j = self.i;
                let mut commas = 0;
                let mut pd = 0; // TUP-ARG: a comma INSIDE a tuple element (`Vec<(&str, i64)>`) is not an argument separator
                while j < self.t.len() {
                    if tkv_l_paren(&self.t[j].k) || tkv_l_brack(&self.t[j].k) {
                        pd = pd + 1;
                    } else if tkv_r_paren(&self.t[j].k) || tkv_r_brack(&self.t[j].k) {
                        pd = pd - 1;
                    } else if tkv_lt(&self.t[j].k) {
                        d = d + 1;
                    } else if tkv_gt(&self.t[j].k) {
                        d = d - 1;
                        if d == 0 {
                            j = self.t.len();
                            continue;
                        }
                    } else if tkv_ge(&self.t[j].k) {
                        d = d - 1;
                        if d == 0 {
                            j = self.t.len();
                            continue;
                        }
                    } else if tkv_shr(&self.t[j].k) {
                        d = d - 2;
                        if d <= 0 {
                            j = self.t.len();
                            continue;
                        }
                    } else if d == 1 && pd == 0 && tkv_comma(&self.t[j].k) {
                        commas = commas + 1;
                    }
                    j = j + 1;
                }
                if commas > 0 {
                    {
                        // R12e: the multi-argument list as written, when every argument is ONE identifier (`Pair<B, A>` -> "B,A")
                        let mut s = String::new();
                        let mut k = self.i + 1;
                        let mut ok = 1;
                        while k < self.t.len() && !tkv_gt(&self.t[k].k) {
                            if tkv_comma(&self.t[k].k) {
                                s.push(',');
                            } else {
                                let w = tok_word(&self.t[k]);
                                if w.len() == 0 {
                                    ok = 0;
                                }
                                s.push_str(w.as_str());
                            }
                            k = k + 1;
                        }
                        self.lastgargs = if ok == 1 { s } else { String::new() };
                    }
                    self.skip_generics();
                    return n;
                }
            }
            let _ = self.adv();
            if self.at_gt() {
                let _ = self.eat_gt();
                return n;
            }
            self.lastpay = String::new();
            let inner0 = self.parse_tyname();
            // OPT-ELEM-PAY (R2a P4): a NESTED Option/Result keeps its payload in the spelling (`Vec<Option<String>>` ->
            // `Vec$Option$str`); parse_tyname returns the bare name (top-level consumers read lastpay), so a Vec element lost
            // it and a matched `Some(s)` printed the String's pointer
            let inner = if (inner0.as_str() == "Option" || inner0.as_str() == "Result")
                && self.lastpay.len() > 0
            {
                let mut w = String::from(inner0.as_str());
                let pb = self.lastpay.as_bytes();
                w.push('$');
                let mut q = 0;
                while q < pb.len() {
                    if pb[q] == 44 {
                        w.push('$');
                    } else {
                        w.push(pb[q] as char);
                    }
                    q = q + 1;
                }
                w
            } else {
                inner0
            };
            if self.at_gt() {
                let _ = self.eat_gt();
            }
            n.push('$');
            n.push_str(&inner);
        }
        n
    }
    fn expr(&mut self) -> E {
        if self.is_word("macro_rules") {
            self.read_macro();
            return self.expr();
        }
        // DESTR-ASSIGN: `(a, b) = rhs;` / `[a, _] = rhs;` at statement start is `{ let $daN = rhs; a = $daN.0; b = $daN.1; }`
        // (rhs evaluated ONCE first, so `(a, b) = (b, a)` swaps). It parsed as a tuple expression and the `= rhs` was dropped --
        // a silent no-op (a WRONG answer). Elements other than a plain name or `_` are refused, never guessed.
        // DESTR-ASSIGN-STRUCT: `P { x: a, .. } = rhs;` / `T(a, b) = rhs;` (a struct / tuple-struct pattern as an assignment target)
        // was a silent no-op too -- REFUSE it (only the tuple/array forms above are lowered)
        if self.hasmain
            && (match self.t[self.i].k.clone() {
                Tk::Ident(_) => true,
                _ => false,
            })
            && self.i + 1 < self.t.len()
            && (tkv_l_brace(&self.t[self.i + 1].k)
                || tkv_l_paren(&self.t[self.i + 1].k)
                || tkv_path_sep(&self.t[self.i + 1].k))
        {
            let mut q = self.i + 1;
            while q + 1 < self.t.len()
                && (tkv_path_sep(&self.t[q].k)
                    || (match self.t[q].k.clone() {
                        Tk::Ident(_) => true,
                        _ => false,
                    }))
            {
                q = q + 1;
            }
            if q < self.t.len() && (tkv_l_brace(&self.t[q].k) || tkv_l_paren(&self.t[q].k)) {
                let mut d = 0;
                let mut c = q;
                while c < self.t.len() {
                    if tok_opens(&self.t[c]) {
                        d = d + 1;
                    } else if tok_closes(&self.t[c]) {
                        d = d - 1;
                        if d == 0 {
                            break;
                        }
                    }
                    c = c + 1;
                }
                if c + 1 < self.t.len()
                    && (match self.t[c + 1].k {
                        Tk::Eq => true,
                        _ => false,
                    })
                    && self.perr == 0
                {
                    self.perr = self.t[self.i].lo + 1;
                    self.perrexp=String::from("a destructuring assignment of a tuple or array (struct / tuple-struct targets not supported yet)");
                    self.perrgot = self.tok_text();
                }
            }
        }
        if self.hasmain && (self.is_tok("(") || self.is_tok("[")) {
            let arr = self.is_tok("[");
            let st = self.i;
            let mut q = st + 1;
            let mut d = 0;
            let mut close = 0;
            while q < self.t.len() {
                if tok_opens(&self.t[q]) {
                    d = d + 1;
                } else if tok_closes(&self.t[q]) {
                    if d == 0 {
                        close = q;
                        break;
                    }
                    d = d - 1;
                }
                q = q + 1;
            }
            if close > st
                && close + 1 < self.t.len()
                && (match self.t[close + 1].k {
                    Tk::Eq => true,
                    _ => false,
                })
            {
                let mut names: Vec<String> = Vec::new();
                let mut ok = true;
                let mut k = st + 1;
                while k < close {
                    let w = tok_word(&self.t[k]);
                    let wb = w.as_bytes();
                    let isname = wb.len() > 0 && ((wb[0] >= 97 && wb[0] <= 122) || wb[0] == 95);
                    if isname && k + 1 <= close && (k + 1 == close || tkv_comma(&self.t[k + 1].k)) {
                        names.push(w);
                        k = k + 2;
                    } else {
                        ok = false;
                        k = close;
                    }
                }
                if !ok || names.len() == 0 {
                    if self.perr == 0 {
                        self.perr = self.t[st].lo + 1;
                        self.perrexp=String::from("a destructuring assignment of plain names (nested patterns, `..`, fields not supported yet)");
                        self.perrgot = self.tok_text();
                    }
                } else {
                    let mut e = close + 2;
                    let mut d2 = 0;
                    while e < self.t.len() {
                        if tok_opens(&self.t[e]) {
                            d2 = d2 + 1;
                        } else if tok_closes(&self.t[e]) {
                            if d2 == 0 {
                                break;
                            }
                            d2 = d2 - 1;
                        } else if d2 == 0 && tkv_semi(&self.t[e].k) {
                            break;
                        }
                        e = e + 1;
                    }
                    let lo = self.t[st].lo;
                    let hi = self.t[st].hi;
                    let mk = |k: Tk| Tok {
                        k: k,
                        lo: lo,
                        hi: hi,
                    };
                    self.nhyg = self.nhyg + 1;
                    let mut tv = String::from("$da");
                    tv.push_str(&oast::u32s2(self.nhyg));
                    let mut nt: Vec<Tok> = Vec::new();
                    nt.push(mk(Tk::LBrace));
                    nt.push(mk(Tk::Let));
                    nt.push(mk(Tk::Ident(String::from(tv.as_str()))));
                    nt.push(mk(Tk::Eq));
                    let mut r = close + 2;
                    while r < e {
                        nt.push(tok_clone(&self.t[r]));
                        r = r + 1;
                    }
                    nt.push(mk(Tk::Semi));
                    let mut j = 0;
                    while j < names.len() {
                        if names[j].as_str() != "_" {
                            nt.push(mk(Tk::Ident(String::from(names[j].as_str()))));
                            nt.push(mk(Tk::Eq));
                            nt.push(mk(Tk::Ident(String::from(tv.as_str()))));
                            if arr {
                                nt.push(mk(Tk::LBrack));
                                nt.push(mk(Tk::Num(j as i64)));
                                nt.push(mk(Tk::RBrack));
                            } else {
                                nt.push(mk(Tk::Dot));
                                nt.push(mk(Tk::Num(j as i64)));
                            }
                            nt.push(mk(Tk::Semi));
                        }
                        j = j + 1;
                    }
                    nt.push(mk(Tk::RBrace));
                    let mut out: Vec<Tok> = Vec::new();
                    let mut z = 0;
                    while z < st {
                        out.push(tok_clone(&self.t[z]));
                        z = z + 1;
                    }
                    let mut y = 0;
                    while y < nt.len() {
                        out.push(tok_clone(&nt[y]));
                        y = y + 1;
                    }
                    z = e;
                    while z < self.t.len() {
                        out.push(tok_clone(&self.t[z]));
                        z = z + 1;
                    }
                    self.t = out;
                }
            }
        }
        if self.is_tok("}") || self.at_end() {
            let l = if self.i < self.t.len() {
                self.t[self.i].lo
            } else {
                0
            };
            return E::Int(0, l, l);
        }
        if self.is_tok("return") {
            let lo = self.lo();
            let _ = self.adv();
            let v = if self.is_tok(";") || self.is_tok("}") {
                E::Int(0, lo, lo)
            } else {
                self.cmp_e()
            };
            let hi = oast::e_hi(&v);
            let r = E::RetE(Box::new(v), lo, hi);
            if self.is_tok(";") {
                let _ = self.adv();
                if self.is_tok("}") || self.at_end() {
                    return r;
                }
                let rest = self.expr();
                let b = oast::e_hi(&rest);
                return E::Seq(Box::new(r), Box::new(rest), lo, b);
            }
            return r;
        }
        // C-BLOCKSTMT: a bare `{ .. }` in STATEMENT position is a statement, and the statements after it
        // continue the enclosing block (rustc). Our block-form test knew `if`/`while`/`match` and not a
        // plain block, so everything after one was read as if the enclosing block had ended -- which is how
        // a comment-free `{ .. }` in olex's lexer derailed the whole composed unit. A statement that BEGAN
        // with `{` is the precise test; an expression that merely CONTAINS a block is untouched.
        let stmt0 = self.i;
        let e = self.cmp_e();
        // UNSAFE-STMT (R2a P5): `unsafe { .. }` in statement position is a block statement like `{ .. }` -- the next
        // statement followed it with no `;` and was refused ("expected `}`, found `println`")
        let was_block = stmt0 < self.t.len()
            && (tkv_l_brace(&self.t[stmt0].k)
                || (stmt0 + 1 < self.t.len()
                    && (tkv_unsafe_t(&self.t[stmt0].k) || word_is(&self.t[stmt0], "unsafe"))
                    && tkv_l_brace(&self.t[stmt0 + 1].k)));
        if self.is_tok(";") {
            let semi = self.lo();
            let _ = self.adv();
            if self.is_tok("}") || self.at_end() {
                // SEMI-TAIL-UNIT (R2a P4): in Rust a block whose last statement ends in `;` has type `()` -- it answered the
                // statement's value, so a tail `if c { v.remove(k); } else { k = k + 1; }` compared str with int and was
                // refused. () is int here (C-PUSHUNIT); a diverging tail (return / break / continue / panic) keeps its value.
                if self.hasmain && !tail_diverges(&e) {
                    let a = oast::e_lo(&e);
                    return E::Seq(Box::new(e), Box::new(E::Int(0, semi, semi)), a, semi);
                }
                return e;
            }
            let r = self.expr();
            let a = oast::e_lo(&e);
            let b = oast::e_hi(&r);
            return E::Seq(Box::new(e), Box::new(r), a, b);
        }
        if (block_form(&e) || was_block) && !self.is_tok("}") && !self.at_end() {
            let r = self.expr();
            let a = oast::e_lo(&e);
            let b = oast::e_hi(&r);
            return E::Seq(Box::new(e), Box::new(r), a, b);
        }
        e
    }
    fn cmp_e(&mut self) -> E {
        let l = self.or_e();
        {
            let mut op = String::from("");
            // `+=` is ONE token: otherwise `add()` consumes the `+` before this layer ever sees it
            if self.is_tok("+=") {
                op = String::from("+");
            } else if self.is_tok("-=") {
                op = String::from("-");
            } else if self.is_tok("*=") {
                op = String::from("*");
            } else if self.is_tok("/=") {
                op = String::from("/");
            } else {
                match self.t[self.i].k.clone() {
                    Tk::OpEq(s) => {
                        op = s;
                    }
                    _ => {}
                }
            } // COMPOUND-OPS: `%= ^= |= &= <<= >>=`
            if op.len() > 0 {
                let _ = self.adv();
                let v = self.or_e();
                let a = oast::e_lo(&l);
                let b = oast::e_hi(&v);
                // `l op= v` == `l = l op v`; the place is re-parsed from the same tokens, which is sound because
                // our places (a local, a field, an index) have no side effects
                // HM-ENTRY: `*X.entry(K).or_insert(D) op= V` (the counting idiom) -- the map hands out no place, so it is rewritten to
                // `{ if !X.contains_key(K) { X.insert(K, D); } X.insert(K, X[K] op V) }`. `*` is identity here, so l is the call chain.
                {
                    let mut hit = false;
                    let mut hkey = String::from("");
                    let mut xe = E::Int(0, a, b);
                    let mut ke = E::Int(0, a, b);
                    let mut de = E::Int(0, a, b);
                    match &l {
                        E::MethodCall(inner, m1, a1, _l1, _h1) => {
                            if m1.as_str() == "or_insert" && a1.len() == 1 {
                                match inner.as_ref() {
                                    E::MethodCall(x0, m0, a0, _l0, _h0) => {
                                        if m0.as_str() == "entry" && a0.len() == 1 {
                                            xe = clone_simple(x0, a, b);
                                            // HM-ENTRY-KEY: a computed key (`w.to_string()`) is evaluated ONCE into a temp
                                            // -- clone_simple cannot copy a call (it became `$noclone`)
                                            if is_simple_operand(&a0[0]) {
                                                ke = clone_simple(&a0[0], a, b);
                                            } else {
                                                self.nhyg = self.nhyg + 1;
                                                hkey = String::from("$hk");
                                                hkey.push_str(&oast::u32s2(self.nhyg));
                                                ke = E::Var(String::from(hkey.as_str()), a, b);
                                            }
                                            de = clone_simple(&a1[0], a, b);
                                            hit = true;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                    if hit {
                        let mut ca: Vec<E> = Vec::new();
                        ca.push(clone_simple(&ke, a, b));
                        let has = E::MethodCall(
                            Box::new(clone_simple(&xe, a, b)),
                            String::from("contains_key"),
                            ca,
                            a,
                            b,
                        );
                        let mut ia: Vec<E> = Vec::new();
                        ia.push(clone_simple(&ke, a, b));
                        ia.push(de);
                        let ins0 = E::MethodCall(
                            Box::new(clone_simple(&xe, a, b)),
                            String::from("insert"),
                            ia,
                            a,
                            b,
                        );
                        let ensure = E::If(
                            Box::new(E::Not(Box::new(has), a, b)),
                            Box::new(ins0),
                            Box::new(E::Int(0, a, a)),
                            a,
                            b,
                        );
                        let cur = E::Index(
                            Box::new(clone_simple(&xe, a, b)),
                            Box::new(clone_simple(&ke, a, b)),
                            a,
                            b,
                        );
                        let nv = E::Bin(op, Box::new(cur), Box::new(v), a, b);
                        let mut sa: Vec<E> = Vec::new();
                        sa.push(ke);
                        sa.push(nv);
                        let set = E::MethodCall(Box::new(xe), String::from("insert"), sa, a, b);
                        let res = E::Seq(Box::new(ensure), Box::new(set), a, b);
                        if hkey.len() > 0 {
                            // move the key expression out of l (E has no Clone) and bind it once
                            let kexpr = match l {
                                E::MethodCall(inner, _m1, _a1, _l1, _h1) => match *inner {
                                    E::MethodCall(_x0, _m0, mut a0v, _l0, _h0) => a0v.remove(0),
                                    _ => E::Int(0, a, b),
                                },
                                _ => E::Int(0, a, b),
                            };
                            return E::Let(hkey, Box::new(kexpr), Box::new(res), a, b);
                        }
                        return res;
                    }
                }
                let lhs2 = self.reparse_place(&l, a, b);
                let rhs = E::Bin(op, Box::new(l), Box::new(v), a, b);
                return E::AssignTo(Box::new(lhs2), Box::new(rhs), a, b);
            }
        }
        if self.is_tok("=") {
            let _ = self.adv();
            let v = self.or_e();
            let a = oast::e_lo(&l);
            let b = oast::e_hi(&v);
            return E::AssignTo(Box::new(l), Box::new(v), a, b);
        }
        l
    }
    fn or_e(&mut self) -> E {
        let mut l = self.and_e();
        while self.is_tok("||") {
            let _ = self.adv();
            let r = self.and_e();
            let a = oast::e_lo(&l);
            let b = oast::e_hi(&r);
            l = E::Bin(String::from("||"), Box::new(l), Box::new(r), a, b);
        }
        l
    }
    fn and_e(&mut self) -> E {
        let mut l = self.rel_e();
        while self.is_tok("&&") {
            let _ = self.adv();
            let r = self.rel_e();
            let a = oast::e_lo(&l);
            let b = oast::e_hi(&r);
            l = E::Bin(String::from("&&"), Box::new(l), Box::new(r), a, b);
        }
        l
    }
    fn rel_e(&mut self) -> E {
        let l = self.bor_e();
        let ops = ["<", "<=", ">", ">=", "==", "!="];
        let mut oi = 0;
        while oi < ops.len() {
            if self.is_tok(ops[oi]) {
                let _ = self.adv();
                let r = self.bor_e();
                let a = oast::e_lo(&l);
                let b = oast::e_hi(&r);
                return E::Bin(String::from(ops[oi]), Box::new(l), Box::new(r), a, b);
            }
            oi = oi + 1;
        }
        l
    }
}
pub fn split_commas(s: &str) -> Vec<String> {
    let b = s.as_bytes();
    let mut o: Vec<String> = Vec::new();
    let mut st = 0;
    let mut i = 0;
    while i <= b.len() {
        if i == b.len() || b[i] == 44 {
            if i > st {
                o.push(String::from(&s[st..i]));
            }
            st = i + 1;
        }
        i = i + 1;
    }
    o
}
// a shallow clone for the elements of a repeat literal (`[0; 32]`): literals and variables are enough, and
// anything else is reproduced as 0 rather than silently duplicating side effects
pub fn clone_simple(e: &E, lo: u32, hi: u32) -> E {
    match e {
        E::Int(v, _l, _h) => E::Int(*v, lo, hi),
        E::Bin(o, a, b, _l, _h) => E::Bin(
            String::from(o.as_str()),
            Box::new(clone_simple(a, lo, hi)),
            Box::new(clone_simple(b, lo, hi)),
            lo,
            hi,
        ), // OVF-SHIFT: pure arithmetic of simple parts (`-64i8`)
        E::Not(x, _l, _h) => E::Not(Box::new(clone_simple(x, lo, hi)), lo, hi),
        E::Bool(b, _l, _h) => E::Bool(*b, lo, hi),
        E::StrLit(sx, _l, _h) => E::StrLit(String::from(sx.as_str()), lo, hi),
        E::Var(n, _l, _h) => E::Var(String::from(n.as_str()), lo, hi),
        // a CAST of something simple is simple -- without this, `['x'; 3]` (a char literal is a
        // cast to `char`) silently became zeros, which is the worst kind of answer
        E::Cast(x, t, _l, _h) => E::Cast(
            Box::new(clone_simple(x, lo, hi)),
            String::from(t.as_str()),
            lo,
            hi,
        ),
        // C-CLONEREAD: a pure READ is duplicable, and the open-ended slice `x[a..]` needs its base
        // duplicated to ask for its length. Index, Field, TupIdx and Ref were not in the list, so
        // `g[i][3..]` asked seq_len(0) -- the arc's invented-default shape again, an answer where a
        // refusal belonged. The recursion keeps it honest: a part that is not duplicable makes the
        // whole not duplicable.
        E::Index(b, ix, _l, _h) => E::Index(
            Box::new(clone_simple(b, lo, hi)),
            Box::new(clone_simple(ix, lo, hi)),
            lo,
            hi,
        ),
        E::Field(o, f, _l, _h) => E::Field(
            Box::new(clone_simple(o, lo, hi)),
            String::from(f.as_str()),
            lo,
            hi,
        ),
        E::TupIdx(b, ix, _l, _h) => E::TupIdx(Box::new(clone_simple(b, lo, hi)), *ix, lo, hi),
        E::Ref(x, m, _l, _h) => E::Ref(Box::new(clone_simple(x, lo, hi)), *m, lo, hi),
        // NOT duplicable: answer a call to an undefined name, which the driver's IR verifier reports,
        // instead of the silent 0 that read as a length of zero and emptied the slice.
        _ => E::Call(String::from("$noclone"), Vec::new(), lo, hi),
    }
}
fn block_form(e: &E) -> bool {
    match e {
        E::Let(_n, _v, b, _l, _h) => block_form(b),
        E::LetT(_n, _t, _v, b, _l, _h) => block_form(b),
        E::Match(_s, _a, _l, _h) => true,
        E::While(_c, _b, _l, _h) => true,
        E::If(_c, _t, _f, _l, _h) => true,
        _ => false,
    }
}
// C-CAPTY: a lifted closure's CAPTURE had no recorded type, so a field read on a captured struct could
// not find its field (the last three of the composed unit's 6918). A capture's type is not DECLARED
// anywhere, which is why rung 59 left it empty -- but it IS knowable: the capture is a local of the
// ENCLOSING function, and at the point the enclosing fn item finishes parsing, this parser holds that
// function's parameter names and types. So fill each capture's pty from them, by name, right there.
// Done here rather than at the lifting site or in lowering: the lifting site cannot see the enclosing
// item's locals, and lowering walks functions in declaration order, so a closure body can be lowered
// before the code that would record the type (rung 76 rejected both routes for those reasons).
pub fn fill_capture_tys(cl: &mut Vec<Fun>, from: usize, ps: &Vec<String>, pt: &Vec<String>) {
    let mut i = from;
    while i < cl.len() {
        let nc = cl[i].ncap as usize;
        let mut k = 0;
        while k < nc {
            if k < cl[i].ptys.len() && cl[i].ptys[k].len() == 0 {
                let cname = String::from(cl[i].params[k].as_str());
                let mut q = 0;
                while q < ps.len() {
                    if ps[q].as_str() == cname.as_str() && q < pt.len() && pt[q].len() > 0 {
                        cl[i].ptys[k] = String::from(pt[q].as_str());
                        q = ps.len();
                    } else {
                        q = q + 1;
                    }
                }
            }
            k = k + 1;
        }
        i = i + 1;
    }
}
pub fn parse(t: Vec<Tok>) -> E {
    let mut p = P {
        lvlab: Vec::new(),
        trnames: Vec::new(),
        edflt: Vec::new(),
        farity: Vec::new(),
        refpn: Vec::new(),
        refpe: Vec::new(),
        gen_fn: false,
        cgen: Vec::new(),
        cgty: Vec::new(),
        tfconst: Vec::new(),
        cghid: Vec::new(),
        closlets: Vec::new(),
        cbind: Vec::new(),
        fretn: Vec::new(),
        lnames: Vec::new(),
        lastpay: String::from(""),
        lastgarg: String::from(""),
        chantx: Vec::new(),
        ptrinfo: Vec::new(),
        spawncl: Vec::new(),
        chanrx: Vec::new(),
        lastarrel: String::from(""),
        annel: String::from(""),
        fpays: Vec::new(),
        strvals: Vec::new(),
        mutcaps: Vec::new(),
        immlets: Vec::new(),
        implinst: String::from(""),
        lastfull: String::new(),
        lastgargs: String::new(),
        fnrets: Vec::new(),
        fnargsv: Vec::new(),
        implbase: String::from(""),
        hasmain: false,
        boxid: -1,
        boxvars: Vec::new(),
        nextpub: false,
        privfns: Vec::new(),
        pcfns: Vec::new(),
        nextpubcrate: false,
        privflds: Vec::new(),
        usechk: Vec::new(),
        usechklo: Vec::new(),
        cparams: Vec::new(),
        lbls: Vec::new(),
        plabel: String::from(""),
        lentry: false,
        tfish: String::from(""),
        tfish2: String::from(""),
        tfishtx: String::from(""),
        tfcalls: Vec::new(),
        annty: String::from(""),
        plname: Vec::new(),
        plidx: Vec::new(),
        plofs: Vec::new(),
        mstr: Vec::new(),
        mstri: Vec::new(),
        mstrt: Vec::new(),
        globs: Vec::new(),
        globm: Vec::new(),
        globp: Vec::new(),
        fnames: Vec::new(),
        lvhit: Vec::new(),
        loopv: Vec::new(),
        mscal: Vec::new(),
        smut: Vec::new(),
        vfn: Vec::new(),
        atn: Vec::new(),
        lproj: String::new(),
        att: Vec::new(),
        selfty: String::from(""),
        usen: Vec::new(),
        uset: Vec::new(),
        namesonly: false,
        perrexp: String::from(""),
        perrgot: String::from(""),
        nhyg: 0,
        macs: Vec::new(),
        depth: 0,
        vnames: Vec::new(),
        vowner: Vec::new(),
        sfnames: Vec::new(),
        sawcopy: false,
        sawreprc: false,
        saword: false,
        aliasn: Vec::new(),
        aliast: Vec::new(),
        perr: 0,
        badfmt: 0,
        consts: Vec::new(),
        ucyc: Vec::new(),
        cpatuse: Vec::new(),
        ordneed: Vec::new(),
        ordhave: Vec::new(),
        ctys: Vec::new(),
        t: t,
        i: 0,
        snames: Vec::new(),
        enames: Vec::new(),
        clos: Vec::new(),
        nfor: 0,
        mpfx: String::from(""),
        mods: Vec::new(),
        retmut: false,
        refvars: Vec::new(),
        refret: Vec::new(),
        lastretmut: false,
    };
    p.expr()
}
// MONOMORPHIZE bound generics. Lowering is where concrete types are known (locals carry recorded struct
// types), so run it once as a DISCOVERY pass -- `olower::mono_name` records every `f|Conc` it routes -- then
// materialize `f$Conc` (ptys/ret substituted, body shared via `dflt`) and let the real lowering find them.
// Lowering is side-effect-free apart from id allocation, so running it twice is sound.
// An unannotated return type is INFERRED from the body: `fn mk(a) { P { a, 2 } }` returns P. Runs before
// monomorphization and typeck, so callers see the real type (without it, a struct-returning fn had to be
// annotated or its callers saw `int` and every field read failed).
// Generic STRUCTS are monomorphized from the type names that actually appear: for a demanded `Wrap$int`
// where `Wrap` is generic, materialize an Sdef with the field types substituted. Iterated to a fixpoint so a
// nested instance (`Wrap<Vec<int>>`) also lands.
pub fn find_fun_ix(fs: &Vec<Fun>, n: &str) -> i32 {
    let mut i = 0;
    while i < fs.len() {
        if fs[i].name.as_str() == n {
            return i as i32;
        }
        i = i + 1;
    }
    0 - 1
}
// P-ITEM: a nested item (`fn` / `struct` / `enum` / `impl` / `trait` / `const` / `static` / `type`
// declared INSIDE a function body) is position-independent in Rust -- it captures nothing from the
// enclosing scope -- so it is HOISTED out of the body to the top level before parsing, where the one
// item reader already handles it. The alternative is a second item reader reachable from statement
// position, which would be two readers for one grammar.
fn item_kw(t: &Tok) -> i32 {
    if tkv_fn(&t.k) {
        return 1;
    }
    if tkv_impl(&t.k) || tkv_trait(&t.k) || tkv_mod_t(&t.k) {
        return 1;
    }
    if tkv_struct(&t.k) || tkv_enum(&t.k) {
        return 2;
    }
    if tkv_const_t(&t.k) || tkv_static_t(&t.k) || tkv_type_t(&t.k) || tkv_use_t(&t.k) {
        return 3;
    }
    0
}
pub fn hoist_items(t: Vec<Tok>) -> Vec<Tok> {
    let mut keep: Vec<Tok> = Vec::new();
    let mut out: Vec<Tok> = Vec::new();
    // the brace stack records what each open brace OPENS: 1 = a function body (items inside it are
    // hoistable), anything else = an item container (an `impl`/`trait`/`mod`/`struct`/`enum` body, whose
    // items already belong to the one item reader) -- a plain block inherits its enclosing kind
    let mut st: Vec<i32> = Vec::new();
    let mut pend: i32 = 0;
    let mut i = 0;
    let mut prev_ok = true;
    while i < t.len() {
        if tkv_l_brace(&t[i].k) {
            let top = if st.len() > 0 { st[st.len() - 1] } else { 0 };
            if pend > 0 {
                st.push(pend);
                pend = 0;
            } else {
                st.push(top);
            }
        } else if tkv_r_brace(&t[i].k) {
            if st.len() > 0 {
                let _ = st.pop();
            }
        }
        let depth: i32 = if st.len() > 0 { st[st.len() - 1] } else { 0 };
        let kw = item_kw(&t[i]);
        // `const fn` / `unsafe fn` keep their modifier with them; a `const` BLOCK expression does not
        // start a statement with a following `fn`
        let mut k2 = kw;
        if kw == 3 && i + 1 < t.len() && tkv_fn(&t[i + 1].k) {
            k2 = 1;
        }
        if k2 > 0 {
            pend = if k2 == 1 && tkv_fn(&t[i].k) {
                1
            } else if k2 == 1 {
                2
            } else {
                3
            };
        }
        if depth == 1 && k2 > 0 && prev_ok {
            // find the item's end
            let mut j = i;
            let mut end = i;
            if k2 == 1 {
                // scan to the body brace, then to its match; a `fn` DECLARATION in a trait ends at `;`
                let mut d2: i32 = 0;
                let mut started = false;
                while j < t.len() {
                    if tkv_l_brace(&t[j].k) {
                        d2 = d2 + 1;
                        started = true;
                    } else if tkv_r_brace(&t[j].k) {
                        d2 = d2 - 1;
                        if started && d2 == 0 {
                            end = j;
                            j = t.len();
                            continue;
                        }
                    } else if tkv_semi(&t[j].k) && !started && d2 == 0 {
                        end = j;
                        j = t.len();
                        continue;
                    }
                    j = j + 1;
                }
                if end == i {
                    end = t.len() - 1;
                }
            } else if k2 == 2 {
                let mut d2: i32 = 0;
                let mut started = false;
                while j < t.len() {
                    if tkv_l_brace(&t[j].k) || tkv_l_paren(&t[j].k) {
                        d2 = d2 + 1;
                        started = true;
                    } else if tkv_r_brace(&t[j].k) || tkv_r_paren(&t[j].k) {
                        d2 = d2 - 1;
                        if started && d2 == 0 {
                            // a tuple struct is `struct S(..);` -- take the `;` too
                            if j + 1 < t.len() && tkv_semi(&t[j + 1].k) {
                                end = j + 1;
                            } else {
                                end = j;
                            }
                            j = t.len();
                            continue;
                        }
                    } else if tkv_semi(&t[j].k) && !started && d2 == 0 {
                        end = j;
                        j = t.len();
                        continue;
                    }
                    j = j + 1;
                }
                if end == i {
                    end = t.len() - 1;
                }
            } else {
                let mut d2: i32 = 0;
                while j < t.len() {
                    if tkv_l_brace(&t[j].k) || tkv_l_paren(&t[j].k) || tkv_l_brack(&t[j].k) {
                        d2 = d2 + 1;
                    } else if tkv_r_brace(&t[j].k) || tkv_r_paren(&t[j].k) || tkv_r_brack(&t[j].k) {
                        d2 = d2 - 1;
                    } else if tkv_semi(&t[j].k) && d2 == 0 {
                        end = j;
                        j = t.len();
                        continue;
                    }
                    j = j + 1;
                }
                if end == i {
                    end = t.len() - 1;
                }
            }
            let mut q = i;
            while q <= end {
                out.push(Tok {
                    k: t[q].k.clone(),
                    lo: t[q].lo,
                    hi: t[q].hi,
                });
                q = q + 1;
            }
            i = end + 1;
            prev_ok = true;
            continue;
        }
        prev_ok = tkv_l_brace(&t[i].k) || tkv_r_brace(&t[i].k) || tkv_semi(&t[i].k);
        keep.push(Tok {
            k: t[i].k.clone(),
            lo: t[i].lo,
            hi: t[i].hi,
        });
        i = i + 1;
    }
    // the hoisted items go in FIRST: the reader registers const and enum names as it reads them, and a
    // body that uses one must be read after its declaration
    if out.len() == 0 {
        return keep;
    }
    {
        let mut q = 0;
        while q < keep.len() {
            out.push(Tok {
                k: keep[q].k.clone(),
                lo: keep[q].lo,
                hi: keep[q].hi,
            });
            q = q + 1;
        }
    }
    out
}
// K-DECL-SCOPE (P-03-FNSCOPE): a nested item's name is scoped to its block and SHADOWS an outer
// same-name item (pin-art sweep sub-arcs/declaration-topology/sweep-blockscope/, byte-identical to
// rustc). P-ITEM hoists nested items to top level under their BARE name, which collides -> $dup$. So
// before hoisting, rename each fn/struct/enum declared inside a fn body to a block-unique name and
// rewrite the references WITHIN its own block. Innermost blocks are processed first, so an inner
// shadow is renamed before the outer scope's pass runs, leaving the outer references to bind outward.
pub fn scope_block_items(t0: Vec<Tok>) -> Vec<Tok> {
    let n = t0.len();
    let mut t: Vec<Tok> = Vec::new();
    {
        let mut q = 0;
        while q < n {
            t.push(Tok {
                k: t0[q].k.clone(),
                lo: t0[q].lo,
                hi: t0[q].hi,
            });
            q = q + 1;
        }
    }
    // matching close index for every open brace
    let mut close: Vec<i32> = Vec::new();
    {
        let mut q = 0;
        while q < n {
            close.push(0 - 1);
            q = q + 1;
        }
    }
    {
        let mut stk: Vec<usize> = Vec::new();
        let mut q = 0;
        while q < n {
            if tkv_l_brace(&t[q].k) {
                stk.push(q);
            } else if tkv_r_brace(&t[q].k) {
                if stk.len() > 0 {
                    let o = stk[stk.len() - 1];
                    let _ = stk.pop();
                    close[o] = q as i32;
                }
            }
            q = q + 1;
        }
    }
    // collect nested fn/struct/enum items at fn-body depth==1: (height, openidx, closeidx, declidx)
    let mut hv: Vec<i32> = Vec::new();
    let mut oi: Vec<usize> = Vec::new();
    let mut ci: Vec<usize> = Vec::new();
    let mut di: Vec<usize> = Vec::new();
    {
        let mut st: Vec<i32> = Vec::new();
        let mut bo: Vec<usize> = Vec::new();
        let mut pend: i32 = 0;
        let mut i = 0;
        let mut prev_ok = true;
        while i < n {
            if tkv_l_brace(&t[i].k) {
                let top = if st.len() > 0 { st[st.len() - 1] } else { 0 };
                if pend > 0 {
                    st.push(pend);
                } else {
                    st.push(top);
                }
                bo.push(i);
                pend = 0;
            } else if tkv_r_brace(&t[i].k) {
                if st.len() > 0 {
                    let _ = st.pop();
                }
                if bo.len() > 0 {
                    let _ = bo.pop();
                }
            }
            let depth: i32 = if st.len() > 0 { st[st.len() - 1] } else { 0 };
            let kw = item_kw(&t[i]);
            let mut k2 = kw;
            if kw == 3 && i + 1 < n && tkv_fn(&t[i + 1].k) {
                k2 = 1;
            }
            if k2 > 0 {
                pend = if k2 == 1 && tkv_fn(&t[i].k) {
                    1
                } else if k2 == 1 {
                    2
                } else {
                    3
                };
            }
            if depth == 1
                && prev_ok
                && i + 1 < n
                && (tkv_fn(&t[i].k) || tkv_struct(&t[i].k) || tkv_enum(&t[i].k))
            {
                if is_ident(&t[i + 1]) && bo.len() > 0 {
                    let op = bo[bo.len() - 1];
                    let cl = close[op];
                    if cl > 0 {
                        hv.push(st.len() as i32);
                        oi.push(op);
                        ci.push(cl as usize);
                        di.push(i + 1);
                    }
                }
            }
            prev_ok = tkv_l_brace(&t[i].k) || tkv_r_brace(&t[i].k) || tkv_semi(&t[i].k);
            i = i + 1;
        }
    }
    // innermost-first: order indices by height DESC (selection sort)
    let m = hv.len();
    let mut order: Vec<usize> = Vec::new();
    {
        let mut q = 0;
        while q < m {
            order.push(q);
            q = q + 1;
        }
    }
    {
        let mut a = 0;
        while a < m {
            let mut b = a + 1;
            while b < m {
                if hv[order[b]] > hv[order[a]] {
                    let tmp = order[a];
                    order[a] = order[b];
                    order[b] = tmp;
                }
                b = b + 1;
            }
            a = a + 1;
        }
    }
    let mut ctr = 0;
    {
        let mut oidx = 0;
        while oidx < m {
            let it = order[oidx];
            let op = oi[it];
            let cl = ci[it];
            let dd = di[it];
            let nm = tok_word(&t[dd]);
            if nm.len() > 0 {
                let mut uniq = String::from(nm.as_str());
                uniq.push_str("$s");
                uniq.push_str(&oast::u32s2(ctr as u32));
                ctr = ctr + 1;
                let mut q = op + 1;
                while q < cl {
                    if word_is(&t[q], &*nm.as_str()) {
                        let mut isref = false;
                        if q == dd {
                            isref = true;
                        } else if q + 1 < n {
                            if tkv_l_paren(&t[q + 1].k)
                                || tkv_l_brace(&t[q + 1].k)
                                || tkv_path_sep(&t[q + 1].k)
                            {
                                isref = true;
                            }
                        }
                        // FN-ITEM-VAL for a NESTED fn: a VALUE use (`let g = dbl;`, `ap(dbl, 4)`) names the same item -- rename it too
                        if !isref
                            && dd > 0
                            && tkv_fn(&t[dd - 1].k)
                            && q > 0
                            && q + 1 < n
                            && (tkv_l_paren(&t[q - 1].k)
                                || tkv_comma(&t[q - 1].k)
                                || tkv_eq(&t[q - 1].k)
                                || tkv_colon(&t[q - 1].k))
                            && (tkv_comma(&t[q + 1].k)
                                || tkv_r_paren(&t[q + 1].k)
                                || tkv_semi(&t[q + 1].k)
                                || tkv_r_brace(&t[q + 1].k))
                        {
                            isref = true;
                        }
                        if isref {
                            t[q].k = Tk::Ident(String::from(uniq.as_str()));
                        }
                    }
                    q = q + 1;
                }
            }
            oidx = oidx + 1;
        }
    }
    t
}

pub fn parse_prog(t0: Vec<Tok>) -> Prog {
    let t_cfg = cfg_strip(t0);
    // ITEM-TYPE-SHAPES reads the tokens BEFORE block-local items are rewritten (a fn-local `const X = [];` was rewritten away)
    let raw_t = {
        let mut v: Vec<Tok> = Vec::new();
        let mut k = 0;
        while k < t_cfg.len() {
            v.push(tok_clone(&t_cfg[k]));
            k = k + 1;
        }
        v
    };
    let t = hoist_items(scope_block_items(marker_strip(t_cfg)));
    let ex_t = {
        // R9-EXH: the match patterns AS WRITTEN (later token passes rewrite tuple / or patterns)
        let mut v: Vec<Tok> = Vec::new();
        let mut k = 0;
        while k < t.len() {
            v.push(tok_clone(&t[k]));
            k = k + 1;
        }
        v
    };
    let mut p = P {
        lvlab: Vec::new(),
        trnames: Vec::new(),
        edflt: Vec::new(),
        farity: Vec::new(),
        refpn: Vec::new(),
        refpe: Vec::new(),
        gen_fn: false,
        cgen: Vec::new(),
        cgty: Vec::new(),
        tfconst: Vec::new(),
        cghid: Vec::new(),
        closlets: Vec::new(),
        cbind: Vec::new(),
        fretn: Vec::new(),
        lnames: Vec::new(),
        lastpay: String::from(""),
        lastgarg: String::from(""),
        chantx: Vec::new(),
        ptrinfo: Vec::new(),
        spawncl: Vec::new(),
        chanrx: Vec::new(),
        lastarrel: String::from(""),
        annel: String::from(""),
        fpays: Vec::new(),
        strvals: Vec::new(),
        mutcaps: Vec::new(),
        immlets: Vec::new(),
        implinst: String::from(""),
        lastfull: String::new(),
        lastgargs: String::new(),
        fnrets: Vec::new(),
        fnargsv: Vec::new(),
        implbase: String::from(""),
        hasmain: false,
        boxid: -1,
        boxvars: Vec::new(),
        nextpub: false,
        privfns: Vec::new(),
        pcfns: Vec::new(),
        nextpubcrate: false,
        privflds: Vec::new(),
        usechk: Vec::new(),
        usechklo: Vec::new(),
        cparams: Vec::new(),
        lbls: Vec::new(),
        plabel: String::from(""),
        lentry: false,
        tfish: String::from(""),
        tfish2: String::from(""),
        tfishtx: String::from(""),
        tfcalls: Vec::new(),
        annty: String::from(""),
        plname: Vec::new(),
        plidx: Vec::new(),
        plofs: Vec::new(),
        mstr: Vec::new(),
        mstri: Vec::new(),
        mstrt: Vec::new(),
        globs: Vec::new(),
        globm: Vec::new(),
        globp: Vec::new(),
        fnames: Vec::new(),
        lvhit: Vec::new(),
        loopv: Vec::new(),
        mscal: Vec::new(),
        smut: Vec::new(),
        vfn: Vec::new(),
        atn: Vec::new(),
        lproj: String::new(),
        att: Vec::new(),
        selfty: String::from(""),
        usen: Vec::new(),
        uset: Vec::new(),
        namesonly: false,
        perrexp: String::from(""),
        perrgot: String::from(""),
        nhyg: 0,
        macs: Vec::new(),
        depth: 0,
        vnames: Vec::new(),
        vowner: Vec::new(),
        sfnames: Vec::new(),
        sawcopy: false,
        sawreprc: false,
        saword: false,
        aliasn: Vec::new(),
        aliast: Vec::new(),
        perr: 0,
        badfmt: 0,
        consts: Vec::new(),
        ucyc: Vec::new(),
        cpatuse: Vec::new(),
        ordneed: Vec::new(),
        ordhave: Vec::new(),
        ctys: Vec::new(),
        t: t,
        i: 0,
        snames: Vec::new(),
        enames: Vec::new(),
        clos: Vec::new(),
        nfor: 0,
        mpfx: String::from(""),
        mods: Vec::new(),
        retmut: false,
        refvars: Vec::new(),
        refret: Vec::new(),
        lastretmut: false,
    };
    {
        // A1 FEATURE-GATE (rustc E0554): the oracle is STABLE rustc, which rejects any `#![feature(..)]` crate attribute
        // ("`#![feature]` may not be used on the stable release channel"); the lexer marks it, the parser reports it here
        let mut q = 0;
        while q < p.t.len() && p.perr == 0 {
            if word_is(&p.t[q], "__feature_gate__") {
                p.perr = p.t[q].lo + 1;
                p.perrexp = String::from("a stable-Rust crate (`#![feature]` may not be used on the stable release channel: E0554)");
                p.perrgot = String::from("#![feature]");
            }
            q = q + 1;
        }
    }
    let mut funs: Vec<Fun> = Vec::new();
    let mut ginit: Vec<E> = Vec::new();
    let mut sdefs: Vec<Sdef> = Vec::new();
    let mut edefs: Vec<Edef> = Vec::new();
    let mut tdefs: Vec<Tdef> = Vec::new();
    let mut impls: Vec<ImplRec> = Vec::new();
    {
        let mut q = 0;
        while q + 1 < p.t.len() {
            if tkv_trait(&p.t[q].k) {
                let w = tok_word(&p.t[q + 1]);
                if w.len() > 0 {
                    p.trnames.push(w);
                }
            }
            q = q + 1;
        }
    } // UFCS-TRAIT: the trait names, so `Tr::m(&x, ..)` reads as `x.m(..)`
      // P-ORDER: Rust item names are order-independent -- a function may use a struct declared
      // below it. Our reader builds its name tables as it reads, so the FIRST pass exists only to
      // populate them (its outputs are thrown away) and the second does the real parse. One reader,
      // two passes: the alternative is a second name-only scanner, i.e. two readers for one grammar.
    let mut pass = 0;
    let tsave: Vec<Tok> = {
        let mut v: Vec<Tok> = Vec::new();
        let mut q = 0;
        while q < p.t.len() {
            v.push(tok_clone(&p.t[q]));
            q = q + 1;
        }
        v
    };
    while pass < 2 {
        p.i = 0;
        p.namesonly = pass == 0;
        {
            let mut rt2: Vec<Tok> = Vec::new();
            let mut q = 0;
            while q < tsave.len() {
                rt2.push(tok_clone(&tsave[q]));
                q = q + 1;
            }
            p.t = rt2;
        }
        loop {
            let progress_at = p.i;
            if p.is_tok("struct") {
                let _ = p.adv();
                let ni = p.adv();
                let nm = match &p.t[ni].k {
                    Tk::Ident(x) => String::from(x.as_str()),
                    _ => String::from("_"),
                };
                let mut sgp: Vec<String> = Vec::new();
                if p.is_tok("<") {
                    let _ = p.adv();
                    loop {
                        if p.at_gt() {
                            let _ = p.eat_gt();
                            break;
                        }
                        if p.at_end() {
                            break;
                        }
                        let gi = p.adv();
                        match p.t[gi].k.clone() {
                            Tk::Ident(x) => sgp.push(String::from(x.as_str())),
                            _ => {}
                        }
                        // SGP-BOUND (R3): `<R: Tr>` -- the bound is not a parameter (sgp read [R, Tr], so every one-parameter
                        // rule skipped a bounded struct)
                        if p.is_tok(":") {
                            let _ = p.adv();
                            let mut d = 0;
                            while !p.at_end() {
                                if d == 0 && (p.is_tok(",") || p.at_gt()) {
                                    break;
                                }
                                if p.at_gt() {
                                    let _ = p.eat_gt(); // splits a `>>`, leaving the struct's own `>`
                                    d = d - 1;
                                    continue;
                                }
                                if p.is_tok("<") {
                                    d = d + 1;
                                }
                                let _ = p.adv();
                            }
                        }
                        if p.is_tok(",") {
                            let _ = p.adv();
                        }
                    }
                }
                p.skip_where(); // `struct G<T> where T: Trait { .. }`
                if p.is_tok("(") {
                    let _ = p.adv();
                    let mut fl: Vec<String> = Vec::new();
                    let mut ft: Vec<String> = Vec::new();
                    let mut ps: Vec<String> = Vec::new();
                    let mut cargs: Vec<E> = Vec::new();
                    let mut fi2 = 0;
                    loop {
                        if p.is_tok(")") {
                            let _ = p.adv();
                            break;
                        }
                        if p.at_end() {
                            break;
                        }
                        if p.is_tok("&") {
                            let mut rf = String::from("@rf:");
                            rf.push_str(p.qual(&nm).as_str());
                            rf.push('.');
                            rf.push_str(&u32s2(fi2));
                            p.fpays.push(rf);
                        } // REF-FIELD (W1): a reference field is borrowed -- its struct's drop glue must not drop through it
                        p.lastpay = String::new();
                        let t = p.parse_tyname();
                        if (t.as_str() == "Option" || t.as_str() == "Result")
                            && p.lastpay.len() > 0
                            && p.namesonly
                        {
                            // DBG-FPAY (W7): a positional field's payload, as the named path records it
                            let mut fe = String::from("@fpay:");
                            fe.push_str(p.qual(&nm).as_str());
                            fe.push('.');
                            fe.push_str(&u32s2(fi2));
                            fe.push('=');
                            fe.push_str(p.lastpay.as_str());
                            p.fpays.push(fe);
                        }
                        fl.push(u32s2(fi2));
                        ft.push(t);
                        let mut pn = String::from("f");
                        pn.push_str(&u32s2(fi2));
                        cargs.push(E::Var(String::from(pn.as_str()), 0, 0));
                        ps.push(pn);
                        fi2 = fi2 + 1;
                        if p.is_tok(",") {
                            let _ = p.adv();
                        }
                    }
                    if p.is_tok(";") {
                        let _ = p.adv();
                    }
                    let qn = p.qual(&nm);
                    {
                        let mut j = String::from("");
                        let mut k = 0;
                        while k < fl.len() {
                            if k > 0 {
                                j.push(',');
                            }
                            j.push_str(fl[k].as_str());
                            k = k + 1;
                        }
                        p.snames.push(String::from(nm.as_str()));
                        p.sfnames.push(String::from(j.as_str()));
                        if qn.len() != nm.len() {
                            p.snames.push(String::from(qn.as_str()));
                            p.sfnames.push(j);
                        }
                    }
                    // R15.4 MANUAL-COPY: a struct is Copy by `#[derive(Copy)]` OR by a written `impl Copy for S<..>` (rustc treats both alike;
                    // `drop(x); drop(x);` of a manually-Copy value is valid)
                    let cp = p.sawcopy || manual_copy(&p.t, nm.as_str());
                    p.sawcopy = false;
                    let mut pt2: Vec<String> = Vec::new();
                    {
                        let mut k = 0;
                        while k < ft.len() {
                            pt2.push(String::from(ft[k].as_str()));
                            k = k + 1;
                        }
                    }
                    let ctor = E::StructLit(String::from(qn.as_str()), cargs, 0, 0);
                    funs.push(Fun {
                        gsub: Vec::new(),
                        pref: Vec::new(),
                        ncap: 0,
                        gbounds: Vec::new(),
                        dflt: 0 - 1,
                        name: String::from(qn.as_str()),
                        params: ps,
                        ptys: pt2,
                        ret: String::from(qn.as_str()),
                        gparams: Vec::new(),
                        owner: String::from(""),
                        body: ctor,
                    });
                    sdefs.push(Sdef {
                        copy: cp,
                        sgp: sgp,
                        name: qn,
                        fields: fl,
                        ftys: ft,
                    });
                    continue;
                }
                if p.is_tok(";") {
                    let _ = p.adv();
                    let fl: Vec<String> = Vec::new();
                    let ft: Vec<String> = Vec::new();
                    p.snames.push(String::from(nm.as_str()));
                    p.sfnames.push(String::from(""));
                    {
                        let q = p.qual(&nm);
                        if q.len() != nm.len() {
                            p.snames.push(q);
                            p.sfnames.push(String::from(""));
                        }
                    }
                    if p.sawreprc {
                        p.sawreprc = false;
                        let mut rc = String::from("@reprc:");
                        rc.push_str(p.qual(&nm).as_str());
                        p.fpays.push(rc);
                    } // R4a LAYOUT
                    sdefs.push(Sdef {
                        copy: {
                            let c = p.sawcopy || manual_copy(&p.t, nm.as_str()); // R15.4 MANUAL-COPY
                            p.sawcopy = false;
                            c
                        },
                        sgp: sgp,
                        name: p.qual(&nm),
                        fields: fl,
                        ftys: ft,
                    });
                    continue;
                }
                p.eat("{");
                let mut fl: Vec<String> = Vec::new();
                let mut ft: Vec<String> = Vec::new();
                loop {
                    if p.is_tok("}") {
                        let _ = p.adv();
                        break;
                    }
                    if p.at_end() {
                        break;
                    }
                    let fprog = p.i;
                    let _ = fprog;
                    // VIS-FIELD (R2a P6): `pub` / `pub(..)` before a field makes it public; a field without it is private to the
                    // struct's module (rustc E0451 on a literal naming it from outside)
                    let mut fpub = false;
                    if match p.t[p.i].k {
                        Tk::PubT => true,
                        _ => false,
                    } {
                        let _ = p.adv();
                        fpub = true;
                        if p.is_tok("(") {
                            while !p.at_end() && !p.is_tok(")") {
                                let _ = p.adv();
                            }
                            let _ = p.adv();
                        }
                    }
                    let fi = p.adv();
                    match p.t[fi].k.clone() {
                        Tk::Ident(x) => {
                            if !fpub && p.namesonly && p.mpfx.len() > 0 {
                                let mut pk = p.qual(&nm);
                                pk.push('|');
                                pk.push_str(x.as_str());
                                p.privflds.push(pk);
                            }
                            fl.push(String::from(x.as_str()))
                        }
                        _ => {}
                    }
                    let mut tn = String::from("int");
                    if p.is_tok(":") {
                        let _ = p.adv();
                        let mut wasref = false;
                        if p.is_tok("&") {
                            wasref = true;
                        }
                        let _ = &wasref;
                        if wasref && fl.len() > 0 {
                            let mut rf = String::from("@rf:");
                            rf.push_str(p.qual(&nm).as_str());
                            rf.push('.');
                            rf.push_str(fl[fl.len() - 1].as_str());
                            p.fpays.push(rf);
                        } // REF-FIELD (W1)
                        p.lproj = String::new();
                        p.lastpay = String::new();
                        let wasmut = p.is_tok("&") && p.peek_is(1, "mut");
                        tn = sl_param(&p.parse_tyname());
                        if (tn.as_str() == "Option" || tn.as_str() == "Result")
                            && p.lastpay.len() > 0
                            && fl.len() > 0
                            && p.namesonly
                        {
                            // DBG-FPAY (W7): an Option/Result FIELD's payload ("@fpay:<struct>.<field>=T" / "=T,E") for Debug
                            // (the type stays the bare `Option`; its payload printed as an address)
                            let mut fe = String::from("@fpay:");
                            fe.push_str(p.qual(&nm).as_str());
                            fe.push('.');
                            fe.push_str(fl[fl.len() - 1].as_str());
                            fe.push('=');
                            fe.push_str(p.lastpay.as_str());
                            p.fpays.push(fe);
                        }
                        if p.lproj.len() > 0
                            && oast::str_starts(p.lproj.as_str(), tn.as_str())
                            && p.lproj.as_bytes()[tn.len()] == 124
                        {
                            let mut m = String::from("$proj$");
                            m.push_str(p.lproj.as_str());
                            tn = m;
                        }
                        p.lproj = String::new();
                    }
                    ft.push(tn);
                    if p.is_tok(",") {
                        let _ = p.adv();
                    }
                }
                {
                    let mut j = String::from("");
                    let mut k = 0;
                    while k < fl.len() {
                        if k > 0 {
                            j.push(',');
                        }
                        j.push_str(fl[k].as_str());
                        k = k + 1;
                    }
                    p.snames.push(String::from(nm.as_str()));
                    p.sfnames.push(String::from(j.as_str()));
                    {
                        let q = p.qual(&nm);
                        if q.len() != nm.len() {
                            p.snames.push(q);
                            p.sfnames.push(j);
                        }
                    }
                }
                if p.sawreprc {
                    p.sawreprc = false;
                    let mut rc = String::from("@reprc:");
                    rc.push_str(p.qual(&nm).as_str());
                    p.fpays.push(rc);
                } // R4a LAYOUT
                if p.saword {
                    p.saword = false;
                    p.ordhave.push(String::from(nm.as_str()));
                    let mut k = 0;
                    while k < ft.len() {
                        let mut e = String::from(nm.as_str());
                        e.push('=');
                        e.push_str(ft[k].as_str());
                        p.ordneed.push(e);
                        k = k + 1;
                    }
                    if !p.namesonly && fl.len() > 0 && sgp.len() == 0 && ord_fields_prim(&ft) {
                        // R13 DERIVE-ORD: `$sortcmp$S(a, b)` -- the derived lexicographic order (field order), as a capture-less closure-
                        // shaped fn so `v.sort()` can hand it to vsort_by. Each field: `if a.f != b.f { a.f.cmp(&b.f) } else { .. }`
                        let sn = p.qual(&nm);
                        let mut body = E::EnumLit(
                            String::from("Ordering"),
                            String::from("Equal"),
                            Vec::new(),
                            0,
                            0,
                        );
                        let mut k = fl.len();
                        while k > 0 {
                            k = k - 1;
                            let fa = E::Field(
                                Box::new(E::Var(String::from("a"), 0, 0)),
                                String::from(fl[k].as_str()),
                                0,
                                0,
                            );
                            let fb = E::Field(
                                Box::new(E::Var(String::from("b"), 0, 0)),
                                String::from(fl[k].as_str()),
                                0,
                                0,
                            );
                            let fa2 = E::Field(
                                Box::new(E::Var(String::from("a"), 0, 0)),
                                String::from(fl[k].as_str()),
                                0,
                                0,
                            );
                            let fb2 = E::Field(
                                Box::new(E::Var(String::from("b"), 0, 0)),
                                String::from(fl[k].as_str()),
                                0,
                                0,
                            );
                            let ne = E::Bin(String::from("!="), Box::new(fa), Box::new(fb), 0, 0);
                            let mut ca: Vec<E> = Vec::new();
                            ca.push(fb2);
                            let c = E::MethodCall(Box::new(fa2), String::from("cmp"), ca, 0, 0);
                            body = E::If(Box::new(ne), Box::new(c), Box::new(body), 0, 0);
                        }
                        let mut cn = String::from("$sortcmp$");
                        cn.push_str(sn.as_str());
                        let mut ps: Vec<String> = Vec::new();
                        ps.push(String::from("a"));
                        ps.push(String::from("b"));
                        let mut pt: Vec<String> = Vec::new();
                        pt.push(String::from(sn.as_str()));
                        pt.push(String::from(sn.as_str()));
                        funs.push(Fun {
                            gsub: Vec::new(),
                            pref: Vec::new(),
                            ncap: 0,
                            gbounds: Vec::new(),
                            dflt: 0 - 1,
                            name: cn,
                            params: ps,
                            ptys: pt,
                            ret: String::from("Ordering"),
                            gparams: Vec::new(),
                            owner: String::new(),
                            body: body,
                        });
                    }
                }
                sdefs.push(Sdef {
                    copy: {
                        let c = p.sawcopy || manual_copy(&p.t, nm.as_str()); // R15.4 MANUAL-COPY
                        p.sawcopy = false;
                        c
                    },
                    sgp: sgp,
                    name: p.qual(&nm),
                    fields: fl,
                    ftys: ft,
                });
            } else if p.is_tok("enum") {
                let _ = p.adv();
                let ni = p.adv();
                let en = match &p.t[ni].k {
                    Tk::Ident(x) => String::from(x.as_str()),
                    _ => String::from("_"),
                };
                p.skip_generics();
                p.skip_where();
                p.eat("{");
                let mut vs: Vec<Vdef> = Vec::new();
                let mut disc: i64 = 0 - 1; // ENUM-DISCR (R7): `A = 3, B, Z = 10` -- explicit, else previous + 1
                let mut discs: Vec<i64> = Vec::new();
                let mut unit_only = true;
                let mut disc_known = true; // ENUM-DISCR: every explicit value a plain literal
                let mut disc_mono = true; // ENUM-DISCR: discriminants increase in declaration order (index order == value order)
                loop {
                    if p.is_tok("}") {
                        let _ = p.adv();
                        break;
                    }
                    if p.at_end() {
                        break;
                    }
                    let vprog = p.i;
                    let _ = vprog;
                    let isdef = word_is(&p.t[p.i], "__default_variant__");
                    if isdef {
                        let _ = p.adv();
                    } // DFLT-ENUM
                    let vi = p.adv();
                    let vn = match &p.t[vi].k {
                        Tk::Ident(x) => String::from(x.as_str()),
                        _ => String::from("_"),
                    };
                    if isdef {
                        let mut dv = String::from(en.as_str());
                        dv.push('=');
                        dv.push_str(vn.as_str());
                        p.edflt.push(dv);
                        if !p.namesonly {
                            let mut fnm = String::from(en.as_str());
                            fnm.push_str("$default");
                            funs.push(Fun {
                                gsub: Vec::new(),
                                pref: Vec::new(),
                                ncap: 0,
                                gbounds: Vec::new(),
                                dflt: 0 - 1,
                                name: fnm,
                                params: Vec::new(),
                                ptys: Vec::new(),
                                ret: String::from(en.as_str()),
                                gparams: Vec::new(),
                                owner: String::from(en.as_str()),
                                body: E::EnumLit(
                                    String::from(en.as_str()),
                                    String::from(vn.as_str()),
                                    Vec::new(),
                                    0,
                                    0,
                                ),
                            });
                        }
                    } // DFLT-ENUM: also `E$default()`, which a derived struct Default calls for an enum field
                    let mut np = 0;
                    let mut pts: Vec<String> = Vec::new();
                    let mut fns: Vec<String> = Vec::new();
                    // `C { x: i32, y: u8 }` -- a STRUCT variant: a positional payload whose components are named
                    if p.is_tok("{") {
                        let _ = p.adv();
                        loop {
                            if p.is_tok("}") {
                                let _ = p.adv();
                                break;
                            }
                            if p.at_end() {
                                break;
                            }
                            if p.is_tok(",") {
                                let _ = p.adv();
                                continue;
                            }
                            let fi2 = p.adv();
                            let fn2 = match &p.t[fi2].k {
                                Tk::Ident(x) => String::from(x.as_str()),
                                _ => String::from("_"),
                            };
                            let mut ft2 = String::from("");
                            if p.is_tok(":") {
                                let _ = p.adv();
                                ft2 = p.parse_tyname();
                            }
                            fns.push(fn2);
                            pts.push(ft2);
                            np = np + 1;
                        }
                    }
                    if p.is_tok("(") {
                        let _ = p.adv();
                        loop {
                            if p.is_tok(")") {
                                let _ = p.adv();
                                break;
                            }
                            if p.at_end() {
                                break;
                            }
                            let t = p.parse_tyname();
                            // `Some(v)` names a binding, `Add(E, E)` names a TYPE: a known type counts as a type
                            if p.sknown(&t)
                                || p.enames_has(&t)
                                || t.as_str() == "int"
                                || t.as_str() == "str"
                                || t.as_str() == "bool"
                                || t.as_str() == "char"
                                || t.as_str() == "f64"
                                || oast::is_int_width(t.as_str()) // PAY-PRIM (W7): a char / f64 / sized-int payload is a TYPE too (it was "",
                                // read as a type parameter, so `V3(c) => println!("{}", c)` printed a char as 113)
                                || olower::vec_elem_of(&t).len() > 0
                            {
                                pts.push(t);
                            } else {
                                pts.push(String::from(""));
                            }
                            np = np + 1;
                            if p.is_tok(",") {
                                let _ = p.adv();
                            }
                        }
                    }
                    p.vnames.push(String::from(vn.as_str()));
                    p.vowner.push(String::from(en.as_str()));
                    {
                        let mut j = String::from("");
                        let mut k = 0;
                        while k < fns.len() {
                            if k > 0 {
                                j.push(',');
                            }
                            j.push_str(fns[k].as_str());
                            k = k + 1;
                        }
                        p.vfn.push(j);
                    }
                    if np > 0 {
                        unit_only = false;
                    }
                    if p.is_tok("=") {
                        let _ = p.adv();
                        let neg = p.is_tok("-");
                        if neg {
                            let _ = p.adv();
                        }
                        // ENUM-DISCR exact-or-nothing: only a plain integer LITERAL followed by `,` or `}` is a known value;
                        // a const, an expression, a cast (`= QUUX`, `= Foo::X as isize - 3`, `= !0 as u8`) makes the enum
                        // UNKNOWN -- no `$discr`, casts stay refused (they were read as garbage values: WRONG answers)
                        // DISCR-SUFFIX (W7): `= -2147483648i32` -- a literal with an integer-type SUFFIX is the same literal (it made
                        // the enum unknown; one cast path then compiled a garbage value: opt-repr-i32-min was WRONG)
                        // the lexer spells a suffixed literal as the cast `( n as i32 )` (LIT-SUFFIX)
                        let sfx = p.i + 5 < p.t.len()
                            && tkv_l_paren(&p.t[p.i].k)
                            && ilit(&p.t[p.i + 1]) > 0
                            && (match p.t[p.i + 2].k {
                                Tk::AsT => true,
                                _ => false,
                            })
                            && oast::is_int_width(tok_word(&p.t[p.i + 3]).as_str())
                            && tkv_r_paren(&p.t[p.i + 4].k)
                            && (tkv_comma(&p.t[p.i + 5].k) || tkv_r_brace(&p.t[p.i + 5].k));
                        if sfx {
                            let _ = p.adv(); // `(`
                        }
                        let lit = p.i < p.t.len()
                            && ilit(&p.t[p.i]) > 0
                            && p.i + 1 < p.t.len()
                            && (sfx || tkv_comma(&p.t[p.i + 1].k) || tkv_r_brace(&p.t[p.i + 1].k));
                        if lit {
                            let dv = ilit_val(&p.t[p.i]);
                            let _ = p.adv();
                            if sfx {
                                let _ = p.adv(); // `as`
                                let _ = p.adv(); // the width
                                let _ = p.adv(); // `)`
                            }
                            let nd = if neg { 0 - dv } else { dv };
                            if discs.len() > 0 && nd <= disc {
                                disc_mono = false;
                            } // ORD guard: explicit values out of declaration order
                            disc = nd;
                        } else {
                            disc_known = false;
                            while p.i < p.t.len() && !p.is_tok(",") && !p.is_tok("}") {
                                let _ = p.adv();
                            }
                        }
                    } else {
                        disc = disc + 1;
                    } // ENUM-DISCR
                    discs.push(disc);
                    vs.push(Vdef {
                        name: vn,
                        npayload: np,
                        ptys: pts,
                        fnames: fns,
                    });
                    if p.is_tok(",") {
                        let _ = p.adv();
                    }
                }
                p.enames.push(String::from(en.as_str()));
                {
                    let q = p.qual(&en);
                    if q.len() != en.len() {
                        p.enames.push(q);
                    }
                }
                edefs.push(Edef {
                    name: p.qual(&en),
                    variants: vs,
                });
                if unit_only && (!disc_mono || !disc_known) && !p.namesonly {
                    // ENUM-DISCR ORD guard: tags are declaration INDICES; when explicit discriminants are out of order (or not
                    // literals), index order is not rustc's value order -- a marker fn makes `<`/`>` refuse instead of answer
                    let mut mk = p.qual(&en);
                    mk.push_str("$ord_by_index_unsound");
                    funs.push(Fun {
                        gsub: Vec::new(),
                        pref: Vec::new(),
                        ncap: 0,
                        gbounds: Vec::new(),
                        dflt: 0 - 1,
                        name: mk,
                        params: Vec::new(),
                        ptys: Vec::new(),
                        ret: String::from("int"),
                        gparams: Vec::new(),
                        owner: String::from(""),
                        body: E::Int(0, 0, 0),
                    });
                }
                if unit_only && disc_known && discs.len() > 0 && !p.namesonly {
                    // ENUM-DISCR (R7): `x as int` on a fieldless enum is its DISCRIMINANT -- synthesized as `Enum$discr(e)`,
                    // a match from variant to value (the runtime tag is the variant INDEX, which differs once `= N` appears)
                    let eq = p.qual(&en);
                    let mut arms: Vec<Arm> = Vec::new();
                    let nv = discs.len();
                    let base = p.vnames.len() - nv;
                    let mut q = 0;
                    while q < nv {
                        arms.push(Arm {
                            slit: String::from(""),
                            has_slit: false,
                            lit: 0,
                            has_lit: false,
                            has_guard: false,
                            guard: E::Bool(true, 0, 0),
                            variant: String::from(p.vnames[base + q].as_str()),
                            binds: Vec::new(),
                            body: E::Int(discs[q], 0, 0),
                            wild: false,
                        });
                        q = q + 1;
                    }
                    let mut fnm = String::from(eq.as_str());
                    fnm.push_str("$discr");
                    let mut prm: Vec<String> = Vec::new();
                    prm.push(String::from("__de"));
                    let mut pty: Vec<String> = Vec::new();
                    pty.push(String::from(eq.as_str()));
                    funs.push(Fun {
                        gsub: Vec::new(),
                        pref: Vec::new(),
                        ncap: 0,
                        gbounds: Vec::new(),
                        dflt: 0 - 1,
                        name: fnm,
                        params: prm,
                        ptys: pty,
                        ret: String::from("int"),
                        gparams: Vec::new(),
                        owner: String::from(""),
                        body: E::Match(Box::new(E::Var(String::from("__de"), 0, 0)), arms, 0, 0),
                    });
                }
            } else if p.is_word("macro_rules") {
                p.read_macro();
            } else if p.is_tok(";") {
                let _ = p.adv();
            } else
            // MR-ITEM: the `;` that ends an item-position macro call (`getter!(a, 1);`)
            if {
                let mi = {
                    let w = if p.i < p.t.len() {
                        tok_word(&p.t[p.i])
                    } else {
                        String::from("")
                    };
                    p.mac_index(&w)
                };
                mi >= 0 && p.peek_is(1, "!")
            } {
                // a macro invoked at ITEM level: expand it in place and parse the items it produced
                let w = tok_word(&p.t[p.i]);
                let mi = p.mac_index(&w) as usize;
                if !p.mac_expand(mi) {
                    let _ = p.adv();
                    let _ = p.adv();
                    if p.perr == 0 {
                        p.perr = p.lo() + 1;
                    }
                } else if p.is_tok("{") {
                    // MR-ITEM: a rule body keeps its outer `{ .. }` (an expression-position macro needs the block); at ITEM level the
                    // braces are the rule's delimiters, not a block -- drop the matching pair so the items inside are top-level items
                    let o = p.i;
                    let mut d = 0;
                    let mut c = o;
                    while c < p.t.len() {
                        if tok_opens(&p.t[c]) {
                            d = d + 1;
                        } else if tok_closes(&p.t[c]) {
                            d = d - 1;
                            if d == 0 {
                                break;
                            }
                        }
                        c = c + 1;
                    }
                    if c < p.t.len() {
                        let mut nt: Vec<Tok> = Vec::new();
                        let mut k = 0;
                        while k < p.t.len() {
                            if k != o && k != c {
                                nt.push(tok_clone(&p.t[k]));
                            }
                            k = k + 1;
                        }
                        p.t = nt;
                    }
                }
                if p.perr == 0 {
                    continue;
                } // MR-ITEM: the expansion was SPLICED at p.i, so no progress is expected -- re-read it as items (the no-progress guard reported "expected an item")
            } else if p.is_tok("type") {
                let _ = p.adv();
                let ni = p.adv();
                let an = match &p.t[ni].k {
                    Tk::Ident(x) => String::from(x.as_str()),
                    _ => String::from("_"),
                };
                p.skip_generics();
                p.eat("=");
                let at = p.parse_tyname();
                loop {
                    if p.at_end() {
                        break;
                    }
                    if p.is_tok(";") {
                        let _ = p.adv();
                        break;
                    }
                    let _ = p.adv();
                }
                p.aliasn.push(an);
                p.aliast.push(at);
            } else
            // EXTERN-ABI-FN: `extern "C" fn f(..) { .. }` is an ordinary fn with a foreign calling convention -- drop the qualifier (a body-less `extern { .. }` block stays refused)
            if p.is_tok("extern")
                && p.i + 2 < p.t.len()
                && (match p.t[p.i + 1].k {
                    Tk::StrL(_) => true,
                    _ => false,
                })
                && (tkv_fn(&p.t[p.i + 2].k) || tkv_unsafe_t(&p.t[p.i + 2].k))
            {
                let _ = p.adv();
                let _ = p.adv();
            } else if p.is_tok("use") && use_has_group(&p.t, p.i) && use_ends_semi(&p.t, p.i) {
                // USE-TREE (R2a P2): `use P::{a, b as c, d::{e}, self};` is one `use` per leaf -- the reader below flattened
                // the group into ONE path (`use a::{P, Q}` bound Q to `a$P$Q`). Splice the leaves in and re-read them.
                let mut e = p.i;
                while e < p.t.len() && !tkv_semi(&p.t[e].k) {
                    e = e + 1;
                }
                let mut leaves: Vec<Tok> = Vec::new();
                let pre: Vec<Tok> = Vec::new();
                let _ = use_tree_flat(&p.t, p.i + 1, e, &pre, p.i, e, &mut leaves);
                let mut nt: Vec<Tok> = Vec::new();
                let mut k = 0;
                while k < p.i {
                    nt.push(tok_clone(&p.t[k]));
                    k = k + 1;
                }
                let mut q = 0;
                while q < leaves.len() {
                    nt.push(tok_clone(&leaves[q]));
                    q = q + 1;
                }
                k = e + 1;
                while k < p.t.len() {
                    nt.push(tok_clone(&p.t[k]));
                    k = k + 1;
                }
                p.t = nt;
                continue;
            } else if p.is_tok("use") || p.is_tok("extern") {
                // `use` / `extern crate` carry no meaning here; an `extern "C" { .. }` BLOCK is skipped whole.
                // A `use` path may itself contain braces (`use std::task::{Context, Poll};`), so only `extern` can
                // open a block.
                let was_extern = p.is_tok("extern");
                let ulo = p.t[p.i].lo;
                let _ = p.adv();
                let mut braced = false;
                let mut glob = false;
                let mut asn = String::from("");
                let mut segs: Vec<String> = Vec::new();
                loop {
                    if p.at_end() {
                        break;
                    }
                    if p.is_tok("{") && was_extern {
                        braced = true;
                        break;
                    }
                    if p.is_tok(";") {
                        let _ = p.adv();
                        break;
                    }
                    // RES-USE-AS: `use m::f as g;` binds g (the segment after `as` is the NAME, not a path segment)
                    if !was_extern && p.is_tok("as") {
                        let _ = p.adv();
                        asn = tok_word(&p.t[p.i]);
                        let _ = p.adv();
                        continue;
                    }
                    if !was_extern {
                        if p.is_tok("*") {
                            glob = true;
                        } else {
                            let w = tok_word(&p.t[p.i]);
                            if w.len() > 0 {
                                segs.push(w);
                            }
                        }
                    }
                    let _ = p.adv();
                }
                if !was_extern && !glob && segs.len() >= 1 && p.namesonly {
                    // USE-CYCLE (W7): every alias, as written (path relative, first segment unresolved)
                    let nm = if asn.len() > 0 {
                        String::from(asn.as_str())
                    } else {
                        String::from(segs[segs.len() - 1].as_str())
                    };
                    let mut e = String::from(nm.as_str());
                    e.push('=');
                    let mut q = 0;
                    while q < segs.len() {
                        if q > 0 {
                            e.push('$');
                        }
                        e.push_str(segs[q].as_str());
                        q = q + 1;
                    }
                    p.ucyc.push(e);
                }
                // `use m::fifteen;` makes `fifteen` name `m$fifteen` -- the alias the module prefixing produced
                if !was_extern && segs.len() >= 2 {
                    let last = if asn.len() > 0 {
                        String::from(asn.as_str())
                    } else {
                        String::from(segs[segs.len() - 1].as_str())
                    };
                    let mut full = String::from(segs[0].as_str());
                    let mut q = 1;
                    while q < segs.len() {
                        full.push('$');
                        full.push_str(segs[q].as_str());
                        q = q + 1;
                    }
                    // USE-REL (R2): inside `mod M`, a use path is relative to M (Rust 2018) -- `use self::area::sq` and
                    // `use area::sq` both name `M$area$sq`; it was resolved from the crate root ("expected an item that
                    // exists, found self$area$sq"). `crate::` / `super::` / std paths keep their own handling.
                    if p.mpfx.len() > 0 {
                        let s0 = String::from(segs[0].as_str());
                        if s0.as_str() == "self" && segs.len() >= 2 {
                            let mut r = String::from(p.mpfx.as_str());
                            r.push_str(&full[5..]);
                            full = r;
                        } else if s0.as_str() != "crate"
                            && s0.as_str() != "super"
                            && s0.as_str() != "std"
                            && s0.as_str() != "core"
                            && s0.as_str() != "alloc"
                        {
                            let mut child = String::from(p.mpfx.as_str());
                            child.push_str(s0.as_str());
                            if p.prefix_known(&child) {
                                let mut r = String::from(p.mpfx.as_str());
                                r.push_str(full.as_str());
                                full = r;
                            }
                        }
                    }
                    if last.as_str() != "self" && last.as_str() != "*" {
                        let mut sseg: Vec<String> = Vec::new();
                        let mut q2 = 0;
                        while q2 < segs.len() {
                            let w = segs[q2].as_str();
                            if !(q2 == 0 && (w == "std" || w == "core" || w == "alloc")) {
                                sseg.push(String::from(w));
                            }
                            q2 = q2 + 1;
                        }
                        let mapped = std_path(&sseg, false);
                        let mut target = if mapped.len() > 0 {
                            mapped.clone()
                        } else {
                            String::from(full.as_str())
                        };
                        {
                            // USE-REL: `use geo::sq;` where `geo$sq` is itself a re-export -- follow it to the item
                            let mut j = 0;
                            while j < p.usen.len() {
                                if p.usen[j].as_str() == target.as_str() {
                                    target = String::from(p.uset[j].as_str());
                                    break;
                                }
                                j = j + 1;
                            }
                        }
                        {
                            let s0 = segs[0].as_str();
                            if mapped.len() == 0 && s0 != "std" && s0 != "core" && s0 != "alloc" {
                                p.usechk.push(String::from(full.as_str()));
                                p.usechklo.push(ulo);
                            }
                        }
                        // HASHMAP: `use std::collections::HashMap;` names a type the PRELUDE defines -- an unmapped std path to a known type
                        // is not aliased (the alias rewrote the prelude's own `HashMap { .. }` literal)
                        let stdkn = mapped.len() == 0
                            && {
                                let s0 = segs[0].as_str();
                                s0 == "std" || s0 == "core" || s0 == "alloc"
                            }
                            && (p.sknown(&last) || p.enames_has(&last));
                        if !stdkn {
                            p.usen.push(p.qual(last.as_str())); // USE-REL: a use inside `mod M` binds `M$last` (`M::last` resolves)
                            p.uset.push(target);
                        }
                    }
                }
                if !was_extern && glob && segs.len() >= 1 {
                    let mut pfx = String::from(segs[0].as_str());
                    let mut q3 = 1;
                    while q3 < segs.len() {
                        pfx.push('$');
                        pfx.push_str(segs[q3].as_str());
                        q3 = q3 + 1;
                    }
                    let mut key = String::from(p.mpfx.as_str());
                    key.push('|');
                    key.push_str(pfx.as_str());
                    p.globs.push(key);
                    p.globm.push(String::from(p.mpfx.as_str()));
                    p.globp.push(String::from(pfx.as_str()));
                }
                if braced {
                    let mut d = 0;
                    loop {
                        if p.at_end() {
                            break;
                        }
                        if p.is_tok("{") {
                            d = d + 1;
                        }
                        if p.is_tok("}") {
                            d = d - 1;
                            let _ = p.adv();
                            if d == 0 {
                                break;
                            }
                            continue;
                        }
                        let _ = p.adv();
                    }
                }
            } else if p.is_tok("const") && (p.peek_is(1, "fn") || p.peek_is(1, "unsafe")) {
                let _ = p.adv();
            } else
            // CONST-FN: `const fn f` is an ordinary fn at run time -- drop the qualifier (it was read as a `const` ITEM)
            if p.is_tok("const") || p.is_tok("static") {
                let isstatic = p.is_tok("static"); // CONST-PAT-TY: a static is never a pattern (E0530)
                let _ = p.adv();
                let mut ismut = false;
                if p.is_tok("mut") {
                    let _ = p.adv();
                    ismut = true;
                }
                let ni = p.adv();
                let cn = match &p.t[ni].k {
                    Tk::Ident(x) => String::from(x.as_str()),
                    _ => String::from("_"),
                };
                let mut ct = String::from("");
                let mut cref = false;
                if p.is_tok(":") {
                    let _ = p.adv();
                    let ts = p.i;
                    cref = tkv_amp(&p.t[ts].k);
                    ct = sl_const(&p.parse_tyname());
                    if ty_raw(&p.t, ts, p.i) {
                        ct = String::from("*raw"); // CONST-PAT-TY: a raw / fn pointer const is never a pattern
                    }
                }
                if ct.starts_with("Atomic") && ct.as_str() != "AtomicBool" {
                    ismut = true; // R4f ATOMIC: a static atomic IS a mutable integer global (single-threaded schedule)
                }
                p.eat("=");
                let cv = if p.namesonly {
                    p.skip_to_semi()
                } else {
                    p.expr_no_seq()
                };
                if p.is_tok(";") {
                    let _ = p.adv();
                }
                if ismut {
                    // a MUTABLE static is a global, not a value: the name is recorded so reads become a load
                    // and writes a store, and the initializer is kept to run first in main
                    p.smut.push(String::from(cn.as_str()));
                    let mut gn = String::from("$gstore$");
                    gn.push_str(&cn);
                    let mut ia: Vec<E> = Vec::new();
                    ia.push(cv);
                    ginit.push(E::Call(gn, ia, 0, 1));
                } else {
                    p.consts.push(String::from(cn.as_str()));
                    {
                        // CONST-PAT-TY (W7): "name=type" (or "name=!static") -- only a primitive-typed CONST is a pattern
                        let mut e = String::from(cn.as_str());
                        e.push('=');
                        if isstatic {
                            e.push_str("!static");
                        } else {
                            if cref && ct.as_str() != "str" {
                                e.push('&'); // REGRESS-6: a reference const (`&'static u32`) keeps its `&` for CONST-PAT-REF
                            }
                            // REGRESS-E: a raw-pointer const DERIVED FROM AN INTEGER (`30 as *const i32`) is a valid pattern (by address)
                            let int_ptr = ct.as_str() == "*raw"
                                && match &cv {
                                    E::Cast(x, _t, _l, _h) => match x.as_ref() {
                                        E::Int(_v, _a, _b) => true,
                                        _ => false,
                                    },
                                    E::Int(_v, _a, _b) => true,
                                    _ => false,
                                };
                            e.push_str(if int_ptr { "usize" } else { ct.as_str() });
                        }
                        p.ctys.push(e);
                    }
                    funs.push(Fun {
                        gsub: Vec::new(),
                        pref: Vec::new(),
                        ncap: 0,
                        gbounds: Vec::new(),
                        dflt: 0 - 1,
                        name: p.qual(&cn),
                        params: Vec::new(),
                        ptys: Vec::new(),
                        ret: ct,
                        gparams: Vec::new(),
                        owner: String::from(""),
                        body: cv,
                    });
                }
            } else if (match p.t[p.i].k {
                Tk::PubT => true,
                _ => false,
            }) {
                let _ = p.adv();
                p.nextpub = true;
            } else
            // VIS-PUB: the next fn is public
            if p.is_tok("#copy") {
                let _ = p.adv();
                p.sawcopy = true;
            } else if word_is(&p.t[p.i], "__pubcrate__") {
                let _ = p.adv(); // PUB-CRATE: `pub(crate)` -- public in its own crate, crate-private across crates
                p.nextpub = true;
                p.nextpubcrate = true;
            } else if word_is(&p.t[p.i], "__nonexh__") {
                let _ = p.adv(); // R2a P6 NONEXH: read again post-parse (nonexh_check) from the token stream
            } else if word_is(&p.t[p.i], "__derive_peq__") {
                let _ = p.adv(); // CONST-PAT-TY widening: read again post-parse from the token stream
            } else if word_is(&p.t[p.i], "__derive_ord__") {
                let _ = p.adv();
                p.saword = true; // R13 DERIVE-ORD
            } else if tok_word(&p.t[p.i]).starts_with("__repr_")
                && !word_is(&p.t[p.i], "__repr_c__")
            {
                let _ = p.adv(); // A2 ENUM-DISCR-WIDTH: a `#[repr(<int>)]` marker, read post-parse from the token stream
            } else if word_is(&p.t[p.i], "__repr_c__") {
                let _ = p.adv();
                p.sawreprc = true;
            } else if p.is_tok("unsafe") {
                let _ = p.adv();
            } else if p.is_tok("mod") {
                // modules are flat PREFIX scopes in the one item loop: `mod M {` pushes `M$`, the matching `}` pops it
                let _ = p.adv();
                let ni = p.adv();
                let mn = match &p.t[ni].k {
                    Tk::Ident(x) => String::from(x.as_str()),
                    _ => String::from("_"),
                };
                p.mods.push(String::from(mn.as_str()));
                let mut np = String::from(p.mpfx.as_str());
                np.push_str(&mn);
                np.push('$');
                if p.namesonly {
                    let mut mp = String::from("@modp:"); // DBG-MODNAME (W7): lowering strips module prefixes from Debug names
                    mp.push_str(np.as_str());
                    p.fpays.push(mp);
                }
                p.mpfx = np;
                p.eat("{");
            } else if p.is_tok("}") && p.mpfx.len() > 0 {
                let _ = p.adv();
                // drop the innermost `Name$` segment
                let b = p.mpfx.as_bytes();
                let mut cut = 0;
                let mut k = 0;
                while k + 1 < b.len() {
                    if b[k] == 36 {
                        cut = k + 1;
                    }
                    k = k + 1;
                }
                p.mpfx = String::from(&p.mpfx[..cut]);
            } else if p.is_tok("trait") {
                let _ = p.adv();
                let ni = p.adv();
                let trn = match &p.t[ni].k {
                    Tk::Ident(x) => String::from(x.as_str()),
                    _ => String::from("_"),
                };
                p.skip_generics();
                if p.is_tok(":") {
                    loop {
                        if p.at_end() {
                            break;
                        }
                        if p.is_tok("{") || p.is_tok("where") {
                            break;
                        }
                        let _ = p.adv();
                    }
                }
                p.skip_where();
                let mut mns: Vec<String> = Vec::new();
                p.eat("{");
                loop {
                    if p.is_tok("}") {
                        let _ = p.adv();
                        break;
                    }
                    if p.at_end() {
                        break;
                    }
                    if p.is_tok("type") {
                        loop {
                            if p.at_end() {
                                break;
                            }
                            if p.is_tok(";") {
                                let _ = p.adv();
                                break;
                            }
                            let _ = p.adv();
                        }
                        continue;
                    }
                    if p.is_tok("const") && p.peek_is(1, "fn") {
                        let _ = p.adv();
                    } // CONST-FN in an impl: a const method is an ordinary method
                    if p.is_tok("const") {
                        // an associated CONST: a zero-argument function, like a free `const`
                        let _ = p.adv();
                        let ci = p.adv();
                        let cn = match &p.t[ci].k {
                            Tk::Ident(x) => String::from(x.as_str()),
                            _ => String::from("_"),
                        };
                        let mut ct = String::from("");
                        if p.is_tok(":") {
                            let _ = p.adv();
                            ct = sl_const(&p.parse_tyname());
                        }
                        mns.push(String::from(cn.as_str()));
                        if p.is_tok("=") {
                            let _ = p.adv();
                            let cv = if p.namesonly {
                                p.skip_to_semi()
                            } else {
                                p.cmp_e()
                            };
                            let mut dn = String::from(trn.as_str());
                            dn.push_str("$");
                            dn.push_str(&cn);
                            funs.push(Fun {
                                gsub: Vec::new(),
                                pref: Vec::new(),
                                ncap: 0,
                                gbounds: Vec::new(),
                                dflt: 0 - 1,
                                name: dn,
                                params: Vec::new(),
                                ptys: Vec::new(),
                                ret: ct,
                                gparams: Vec::new(),
                                owner: String::from(trn.as_str()),
                                body: cv,
                            });
                        }
                        if p.is_tok(";") {
                            let _ = p.adv();
                        }
                        continue;
                    }
                    if !p.is_tok("fn") {
                        let _ = p.adv();
                        continue;
                    }
                    let _ = p.adv();
                    let mi = p.adv();
                    let mn = match &p.t[mi].k {
                        Tk::Ident(x) => String::from(x.as_str()),
                        _ => String::from("_"),
                    };
                    {
                        let mut tgp: Vec<String> = Vec::new();
                        let mut tgb: Vec<String> = Vec::new();
                        p.read_generics(&mut tgp, &mut tgb);
                    }
                    p.eat("(");
                    let mut ps: Vec<String> = Vec::new();
                    let mut pt: Vec<String> = Vec::new();
                    let mut pr: Vec<bool> = Vec::new();
                    loop {
                        if p.is_tok(")") {
                            let _ = p.adv();
                            break;
                        }
                        if p.at_end() {
                            break;
                        }
                        let lead_ref = p.is_tok("&");
                        if p.is_tok("&") {
                            let _ = p.adv();
                            if p.is_tok("mut") {
                                let _ = p.adv();
                            }
                        }
                        if p.is_tok("mut") {
                            let _ = p.adv();
                        }
                        let pi = p.adv();
                        let pname = match &p.t[pi].k {
                            Tk::Ident(x) => String::from(x.as_str()),
                            _ => String::from("_"),
                        };
                        let mut tnm = if pname.as_str() == "self" {
                            String::from("Self")
                        } else {
                            String::from("")
                        };
                        let mut wasref = lead_ref && pname.as_str() == "self"; // REF-SELF (W1): `&self` borrows the receiver
                        if p.is_tok(":") {
                            let _ = p.adv();
                            if p.is_tok("&") {
                                wasref = true;
                            }
                            let wasmut = p.is_tok("&") && p.peek_is(1, "mut");
                            tnm = sl_param(&p.parse_tyname());
                            if wasmut && scalar_ty(&tnm) {
                                let nm0 = if ps.len() > 0 {
                                    String::from(ps[ps.len() - 1].as_str())
                                } else {
                                    String::from("")
                                };
                                if nm0.len() > 0 {
                                    p.mscal.push(nm0);
                                }
                            }
                        }
                        ps.push(pname);
                        pt.push(tnm);
                        pr.push(wasref);
                        if p.is_tok(",") {
                            let _ = p.adv();
                        }
                    }
                    let mut rt = String::from("int");
                    if p.is_tok("-") {
                        let _ = p.adv();
                        if p.is_tok(">") {
                            let _ = p.adv();
                        }
                        if p.is_tok("(") {
                            let _ = p.adv();
                            let mut nc = 0;
                            let mut es: Vec<String> = Vec::new();
                            loop {
                                if p.is_tok(")") {
                                    let _ = p.adv();
                                    break;
                                }
                                if p.at_end() {
                                    break;
                                }
                                if p.is_tok(",") {
                                    let _ = p.adv();
                                } else {
                                    es.push(p.parse_tyname());
                                    nc = nc + 1;
                                }
                            }
                            rt = tup_name(nc as u32, &es);
                        } else {
                            let wasmut2 = p.is_tok("&") && p.peek_is(1, "mut");
                            rt = sl_param(&p.parse_tyname());
                            if wasmut2 {
                                p.retmut = true;
                            }
                            if wasmut2 && p.perr == 0 && p.depth > 2000000000 {
                                p.perr = p.lo() + 1;
                                p.perrexp=String::from("a return type that is not a `&mut` (returning a PLACE is not implemented)");
                                p.perrgot = p.tok_text();
                            }
                        }
                    }
                    p.skip_where();
                    mns.push(String::from(mn.as_str()));
                    if p.is_tok("{") {
                        // a DEFAULT body: kept as `Trait$m`, the template every implementing type's entry points at
                        let _ = p.adv();
                        p.cparams = ps.clone();
                        p.lnames = Vec::new();
                        let c0m = p.clos.len();
                        let body = if p.namesonly { p.skip_body() } else { p.expr() };
                        p.cparams = Vec::new();
                        p.mut_encl(c0m, &body);
                        p.eat("}");
                        if p.retmut {
                            p.mstr = Vec::new();
                            p.mstri = Vec::new();
                            p.mstrt = Vec::new();
                            if !p.namesonly
                                && !tail_place_ref_p(&body)
                                && !is_bare_var_p(&body)
                                && p.perr == 0
                            {
                                p.perr = p.lo() + 1;
                                p.perrexp=String::from("a `&mut`-returning function whose body is not a `&mut` place (returning a non-place through `&mut` is not implemented)");
                                p.perrgot = p.tok_text();
                            }
                            p.retmut = false;
                        } // C2-R2: a &mut-RETURN fn is a REFERENCE returner, not an out-param place-fn, so clear its scalar-place registration in BOTH passes (keeps the call a Call for the olower inline). An aggregate &mut param (m1 first) was never in mstr, so this is a no-op there.
                        {
                            loop {
                                match p.mscal.pop() {
                                    Some(nm0) => {
                                        if assigns_to(&body, &nm0) && p.perr == 0 {
                                            p.perr = p.lo() + 1;
                                            p.perrexp=String::from("a `&mut` to an aggregate (writing through a `&mut` to a SCALAR needs a place our value model does not have)");
                                            p.perrgot = p.tok_text();
                                        }
                                    }
                                    None => {
                                        break;
                                    }
                                }
                            }
                        }
                        let mut dn = String::from(trn.as_str());
                        dn.push_str("$");
                        dn.push_str(&mn);
                        funs.push(Fun {
                            gsub: Vec::new(),
                            pref: pr,
                            ncap: 0,
                            gbounds: Vec::new(),
                            dflt: 0 - 1,
                            name: dn,
                            params: ps,
                            ptys: pt,
                            ret: rt,
                            gparams: Vec::new(),
                            owner: String::from(trn.as_str()),
                            body: body,
                        });
                    } else {
                        if p.is_tok(";") {
                            let _ = p.adv();
                        }
                    }
                }
                tdefs.push(Tdef {
                    name: trn,
                    mnames: mns,
                });
            } else if p.is_tok("impl") {
                let _ = p.adv();
                // GEN-IMPL: the impl's OWN generics (`impl<T> S<T>`) are read, not skipped: they make each method generic
                let mut igps: Vec<String> = Vec::new();
                {
                    let mut igbs: Vec<String> = Vec::new();
                    p.read_generics(&mut igps, &mut igbs);
                }
                let ti = p.i;
                let mut tn = sl_param(&p.parse_tyname());
                let mut trn = String::from("");
                let _ = ti;
                if p.is_tok("for") {
                    let _ = p.adv();
                    trn = String::from(tn.as_str());
                    let tpos = if p.i < p.t.len() { p.t[p.i].lo } else { 0 };
                    tn = sl_param(&p.parse_tyname());
                    impls.push(ImplRec {
                        tr: String::from(trn.as_str()),
                        ty: String::from(tn.as_str()),
                        lo: tpos,
                        hi: tpos + 1,
                    });
                }
                p.skip_where();
                p.eat("{");
                // GEN-IMPL: `impl<T> S<T>` is an impl on the GENERIC struct S (every instance shares its layout here), so the
                // self type is the base S, not the instance named by the impl's own parameter (`S$T`, which mono then built as a struct)
                p.implinst = String::from("");
                p.implbase = String::from("");
                // GEN-IMPL-TRAIT: `impl<T> Drop for S<T>` too -- its methods were named `S$T$drop`, so the drop never ran (WRONG)
                if igps.len() > 0 {
                    // MOD-GEN-IMPL (R2a P5): the base is the LONGEST proper `$`-prefix naming a known struct -- a module type`s
                    // instance `l$G$T` cut at its FIRST `$` gave `l`, so `impl<T> G<T>` in a module kept the instance name and
                    // its methods (and its Drop) were never found (`no method get on l$G`; the Drop never ran: a silent WRONG)
                    let tb = tn.as_bytes();
                    let mut cut = tb.len();
                    {
                        let mut j = 0;
                        while j < tb.len() {
                            if tb[j] == 36 && p.sknown(&tn[..j]) {
                                cut = j;
                            }
                            j = j + 1;
                        }
                    }
                    if cut < tb.len() {
                        let base = String::from(&tn[..cut]);
                        if p.sknown(&base) {
                            p.implinst = String::from(tn.as_str());
                            p.implbase = String::from(base.as_str());
                            if trn.len() > 0 && impls.len() > 0 {
                                let li = impls.len() - 1;
                                impls[li].ty = String::from(base.as_str());
                            }
                            tn = base;
                        }
                    }
                }
                p.selfty = String::from(tn.as_str());
                loop {
                    if p.is_tok("}") {
                        let _ = p.adv();
                        p.implinst = String::from("");
                        p.implbase = String::from("");
                        break;
                    }
                    if p.at_end() {
                        break;
                    }
                    // P-SELF: an associated TYPE is a per-impl entry -- `type Out = u8;` inside `impl T1 for S`
                    // makes `<S as T1>::Out` and `Self::Out` name `u8`
                    if p.is_tok("type") {
                        let _ = p.adv();
                        let ai = p.adv();
                        let an = match &p.t[ai].k {
                            Tk::Ident(x) => String::from(x.as_str()),
                            _ => String::from("_"),
                        };
                        p.skip_generics();
                        if p.is_tok("=") {
                            let _ = p.adv();
                            let av = p.parse_tyname();
                            let mut key = String::from(tn.as_str());
                            key.push('$');
                            key.push_str(&an);
                            p.atn.push(key);
                            p.att.push(av);
                        }
                        loop {
                            if p.at_end() {
                                break;
                            }
                            if p.is_tok(";") {
                                let _ = p.adv();
                                break;
                            }
                            let _ = p.adv();
                        }
                        continue;
                    }
                    if p.is_tok("const") && p.peek_is(1, "fn") {
                        let _ = p.adv();
                    } // CONST-FN in an impl: a const method is an ordinary method
                    if p.is_tok("const") {
                        let _ = p.adv();
                        let ci = p.adv();
                        let cn = match &p.t[ci].k {
                            Tk::Ident(x) => String::from(x.as_str()),
                            _ => String::from("_"),
                        };
                        let mut ct = String::from("");
                        if p.is_tok(":") {
                            let _ = p.adv();
                            ct = sl_const(&p.parse_tyname());
                        }
                        if ct.as_str() == "Self" {
                            ct = String::from(tn.as_str());
                        }
                        if p.is_tok("=") {
                            let _ = p.adv();
                            let cv = if p.namesonly {
                                p.skip_to_semi()
                            } else {
                                p.cmp_e()
                            };
                            let mut dn = String::from(tn.as_str());
                            dn.push_str("$");
                            dn.push_str(&cn);
                            funs.push(Fun {
                                gsub: Vec::new(),
                                pref: Vec::new(),
                                ncap: 0,
                                gbounds: Vec::new(),
                                dflt: 0 - 1,
                                name: dn,
                                params: Vec::new(),
                                ptys: Vec::new(),
                                ret: ct,
                                gparams: Vec::new(),
                                owner: String::from(tn.as_str()),
                                body: cv,
                            });
                        }
                        if p.is_tok(";") {
                            let _ = p.adv();
                        }
                        continue;
                    }
                    if !p.is_tok("fn") {
                        let _ = p.adv();
                        continue;
                    }
                    let _ = p.adv();
                    let mi = p.adv();
                    let mn = match &p.t[mi].k {
                        Tk::Ident(x) => String::from(x.as_str()),
                        _ => String::from("_"),
                    };
                    let mut gps: Vec<String> = Vec::new();
                    {
                        let mut mgbs: Vec<String> = Vec::new();
                        p.read_generics(&mut gps, &mut mgbs);
                    }
                    {
                        let mut g = 0;
                        while g < igps.len() {
                            if !oast::in_list(&gps, igps[g].as_str()) {
                                gps.push(String::from(igps[g].as_str()));
                            }
                            g = g + 1;
                        }
                    } // GEN-IMPL: the impl's params are the method's too
                    p.eat("(");
                    let mut ps: Vec<String> = Vec::new();
                    let mut pt: Vec<String> = Vec::new();
                    let mut pr: Vec<bool> = Vec::new();
                    loop {
                        if p.is_tok(")") {
                            let _ = p.adv();
                            break;
                        }
                        if p.at_end() {
                            break;
                        }
                        let lead_ref = p.is_tok("&");
                        if p.is_tok("&") {
                            let _ = p.adv();
                            if p.is_tok("mut") {
                                let _ = p.adv();
                            }
                        }
                        if p.is_tok("mut") {
                            let _ = p.adv();
                        }
                        let pi = p.adv();
                        let pname = match &p.t[pi].k {
                            Tk::Ident(x) => String::from(x.as_str()),
                            _ => String::from("_"),
                        };
                        let mut tnm = if pname.as_str() == "self" {
                            String::from(tn.as_str())
                        } else {
                            String::from("")
                        };
                        let mut wasref = lead_ref && pname.as_str() == "self"; // REF-SELF (W1): `&self` borrows the receiver
                        if p.is_tok(":") {
                            let _ = p.adv();
                            if p.is_tok("&") {
                                wasref = true;
                            }
                            let wasmut = p.is_tok("&") && p.peek_is(1, "mut");
                            // V4.3d IMPL-ARG-METHOD: the free-fn IMPL-ARG rule for a method -- a universal `impl Trait` argument
                            // (not Fn) is an anonymous generic, so mono instances the method per argument type. Without it the
                            // parameter's type was unknown and `{:?}` of it printed nothing (universal_in_trait_defn_parameters)
                            let mut implarg = false;
                            if p.is_tok("impl") && p.i + 1 < p.t.len() {
                                let bw = tok_word(&p.t[p.i + 1]);
                                implarg = bw.len() > 0
                                    && !(bw.as_str() == "Fn"
                                        || bw.as_str() == "FnMut"
                                        || bw.as_str() == "FnOnce");
                            }
                            tnm = sl_param(&p.parse_tyname());
                            if implarg {
                                let mut gn = String::from("__impl");
                                gn.push_str(&oast::u32s2(gps.len() as u32));
                                gps.push(String::from(gn.as_str()));
                                tnm = gn;
                            }
                            if wasmut && scalar_ty(&tnm) {
                                let nm0 = if ps.len() > 0 {
                                    String::from(ps[ps.len() - 1].as_str())
                                } else {
                                    String::from("")
                                };
                                if nm0.len() > 0 {
                                    p.mscal.push(nm0);
                                }
                            }
                        }
                        ps.push(pname);
                        pt.push(tnm);
                        pr.push(wasref);
                        if p.is_tok(",") {
                            let _ = p.adv();
                        }
                    }
                    let mut rt = String::from("int");
                    if p.is_tok("-") {
                        let _ = p.adv();
                        if p.is_tok(">") {
                            let _ = p.adv();
                        }
                        if p.is_tok("(") {
                            let _ = p.adv();
                            let mut nc = 0;
                            let mut es: Vec<String> = Vec::new();
                            loop {
                                if p.is_tok(")") {
                                    let _ = p.adv();
                                    break;
                                }
                                if p.at_end() {
                                    break;
                                }
                                if p.is_tok(",") {
                                    let _ = p.adv();
                                } else {
                                    es.push(p.parse_tyname());
                                    nc = nc + 1;
                                }
                            }
                            rt = tup_name(nc as u32, &es);
                        } else {
                            let wasmut2 = p.is_tok("&") && p.peek_is(1, "mut");
                            rt = sl_param(&p.parse_tyname());
                            if p.namesonly
                                && p.lastgargs.len() > 0
                                && p.lastfull.as_str() == tn.as_str()
                            {
                                // R12e RET-GINST: `fn swap(&self) -> Pair<B, A>` in `impl<A, B> Pair<A, B>` records "@rg:Pair$swap=B,A"
                                let mut fe = String::from("@rg:");
                                fe.push_str(tn.as_str());
                                fe.push('$');
                                fe.push_str(mn.as_str());
                                fe.push('=');
                                fe.push_str(p.lastgargs.as_str());
                                p.fpays.push(fe);
                            }
                            if wasmut2 {
                                p.retmut = true;
                            }
                            if wasmut2 && p.perr == 0 && p.depth > 2000000000 {
                                p.perr = p.lo() + 1;
                                p.perrexp=String::from("a return type that is not a `&mut` (returning a PLACE is not implemented)");
                                p.perrgot = p.tok_text();
                            }
                        }
                    }
                    p.skip_where();
                    p.skip_where();
                    let cl0 = p.clos.len();
                    p.closlets = Vec::new();
                    p.lastretmut = p.retmut;
                    p.refvars = Vec::new();
                    p.eat("{");
                    p.cparams = ps.clone();
                    p.lnames = Vec::new();
                    let c0m = p.clos.len();
                    let body = if p.namesonly { p.skip_body() } else { p.expr() };
                    p.cparams = Vec::new();
                    p.mut_encl(c0m, &body);
                    p.eat("}");
                    if p.retmut {
                        p.mstr = Vec::new();
                        p.mstri = Vec::new();
                        p.mstrt = Vec::new();
                        if !p.namesonly
                            && !tail_place_ref_p(&body)
                            && !is_bare_var_p(&body)
                            && p.perr == 0
                        {
                            p.perr = p.lo() + 1;
                            p.perrexp=String::from("a `&mut`-returning function whose body is not a `&mut` place (returning a non-place through `&mut` is not implemented)");
                            p.perrgot = p.tok_text();
                        }
                        p.retmut = false;
                    } // C2-R2: a &mut-RETURN fn is a REFERENCE returner, not an out-param place-fn, so clear its scalar-place registration in BOTH passes (keeps the call a Call for the olower inline). An aggregate &mut param (m1 first) was never in mstr, so this is a no-op there.
                    {
                        // CLOS-CAP-CLOS (R2a P4): a capture of a `let`-bound closure is callable -- typed Fn (900) like an `impl Fn` param,
                        // so `let h = || g(1)` calls the captured g (it was "cannot find function g")
                        let mut ps2: Vec<String> = Vec::new();
                        let mut pt2: Vec<String> = Vec::new();
                        let mut z = 0;
                        while z < ps.len() {
                            ps2.push(String::from(ps[z].as_str()));
                            pt2.push(if z < pt.len() {
                                String::from(pt[z].as_str())
                            } else {
                                String::from("")
                            });
                            z = z + 1;
                        }
                        z = 0;
                        while z < p.closlets.len() {
                            ps2.push(String::from(p.closlets[z].as_str()));
                            pt2.push(String::from("Fn"));
                            z = z + 1;
                        }
                        fill_capture_tys(&mut p.clos, cl0, &ps2, &pt2);
                    }
                    // a `&mut` SCALAR parameter that the body WRITES needs a place; one it only reads behaves
                    // identically by value, so only the write is refused
                    {
                        loop {
                            match p.mscal.pop() {
                                Some(nm0) => {
                                    if assigns_to(&body, &nm0) && p.perr == 0 {
                                        p.perr = p.lo() + 1;
                                        p.perrexp=String::from("a `&mut` to an aggregate (writing through a `&mut` to a SCALAR needs a place our value model does not have)");
                                        p.perrgot = p.tok_text();
                                    }
                                }
                                None => {
                                    break;
                                }
                            }
                        }
                    }
                    if rt.as_str() == "Self" {
                        rt = String::from(tn.as_str());
                    }
                    let mut mangled = String::from(tn.as_str());
                    mangled.push_str("$");
                    mangled.push_str(&mn);
                    funs.push(Fun {
                        gsub: Vec::new(),
                        pref: pr,
                        ncap: 0,
                        gbounds: Vec::new(),
                        dflt: 0 - 1,
                        name: mangled,
                        params: ps,
                        ptys: pt,
                        ret: rt,
                        gparams: gps,
                        owner: String::from(tn.as_str()),
                        body: body,
                    });
                }
            } else if p.is_tok("fn") {
                let _ = p.adv();
                let ni = p.adv();
                let nm = match &p.t[ni].k {
                    Tk::Ident(x) => String::from(x.as_str()),
                    _ => String::from("_"),
                };
                {
                    let ispub = p.nextpub;
                    p.nextpub = false;
                    let ispc = p.nextpubcrate;
                    p.nextpubcrate = false;
                    // PUB-CRATE (R2a P6): a pub(crate) fn inside an EXTERN crate unit (`l$__xcrate__`) is visible only under `l$`
                    if p.namesonly && ispc && p.mpfx.len() > 0 {
                        let mb = p.mpfx.as_bytes();
                        let mut c = 0;
                        while c < mb.len() && mb[c] != 36 {
                            c = c + 1;
                        }
                        let root = String::from(&p.mpfx[..c]);
                        let mut mk = String::from(root.as_str());
                        mk.push_str("$__xcrate__");
                        if p.fknown(&mk) {
                            let mut e = p.qual(&nm);
                            e.push('|');
                            e.push_str(root.as_str());
                            e.push('$');
                            p.pcfns.push(e);
                        }
                    }
                    if p.namesonly && p.mpfx.len() == 0 && nm.as_str() == "main" {
                        p.hasmain = true;
                    }
                    if p.namesonly && !ispub && p.mpfx.len() > 0 {
                        let q = p.qual(&nm);
                        p.privfns.push(q);
                    }
                } // VIS-PUB: a module fn without pub is private to its module
                let mut gps: Vec<String> = Vec::new();
                let mut gbs: Vec<String> = Vec::new();
                p.gen_fn = true;
                p.cgen = Vec::new();
                p.cgty = Vec::new();
                p.cbind = Vec::new();
                p.read_generics(&mut gps, &mut gbs);
                p.gen_fn = false;
                p.eat("(");
                let mut ps: Vec<String> = Vec::new();
                let mut pt: Vec<String> = Vec::new();
                let mut pr: Vec<bool> = Vec::new();
                // R15.1 ELISION: rustc's input-lifetime count -- a `&self` / `&mut self` receiver, or every `&` anywhere in the
                // parameter types (a reference inside a tuple counts; one inside an `fn(..)` type is its own scope), and whether
                // a type could hide a lifetime the lexer dropped (`Foo<'_>`)
                let mut el_self = false;
                let mut el_refs = 0;
                let mut el_opaque = false;
                loop {
                    if p.is_tok(")") {
                        let _ = p.adv();
                        break;
                    }
                    if p.at_end() {
                        break;
                    }
                    if p.is_tok("&")
                        && (p.peek_is(1, "self") || (p.peek_is(1, "mut") && p.peek_is(2, "self")))
                    {
                        el_self = true;
                    }
                    if p.is_tok("mut") {
                        let _ = p.adv();
                    } // PARAM-MUT: `mut x: T` -- `mut` was read AS the parameter name, so the names and types fell out of step
                    if p.is_tok("(") && p.peek_is(1, ")") {
                        let _ = p.adv();
                        let mut un = String::from("__unitp");
                        un.push_str(&oast::u32s2(ps.len() as u32));
                        ps.push(un);
                    } // PARAM-UNIT-PAT: `fn foo(():())` binds nothing
                    let pi = p.adv();
                    match p.t[pi].k.clone() {
                        Tk::Ident(x) => ps.push(String::from(x.as_str())),
                        _ => {}
                    }
                    let mut tn = String::from("");
                    let mut wasref = false;
                    if p.is_tok(":") {
                        let _ = p.adv();
                        if p.is_tok("&") {
                            wasref = true;
                        }
                        el_refs = el_refs + elide_refs(&p.t, p.i);
                        if elide_opaque(&p.t, p.i) {
                            el_opaque = true;
                        }
                        let wasmut = p.is_tok("&") && p.peek_is(1, "mut");
                        {
                            // IMPL-NESTED (W7): `iter: &Vec<impl Display>` -- an `impl Bound` NESTED in the param type is an anonymous
                            // generic too. It was left unnamed, so the element type was unknown (strings printed as addresses).
                            // Rewrite it in place to a fresh generic `__implN` bounded by Bound (as `<__implN: Bound>` would be).
                            let mut k = p.i + 1;
                            let mut d = 0;
                            while k < p.t.len() {
                                if tkv_lt(&p.t[k].k)
                                    || tkv_l_paren(&p.t[k].k)
                                    || tkv_l_brack(&p.t[k].k)
                                {
                                    d = d + 1;
                                } else if tkv_gt(&p.t[k].k)
                                    || tkv_r_paren(&p.t[k].k)
                                    || tkv_r_brack(&p.t[k].k)
                                {
                                    if d == 0 {
                                        break;
                                    }
                                    d = d - 1;
                                } else if d == 0 && tkv_comma(&p.t[k].k) {
                                    break;
                                }
                                if d > 0 && tkv_impl(&p.t[k].k) && k + 1 < p.t.len() {
                                    let bd = tok_word(&p.t[k + 1]);
                                    let fnish = bd.as_str() == "Fn"
                                        || bd.as_str() == "FnMut"
                                        || bd.as_str() == "FnOnce";
                                    if bd.len() > 0
                                        && !fnish
                                        && k + 2 < p.t.len()
                                        && (tkv_gt(&p.t[k + 2].k) || tkv_comma(&p.t[k + 2].k))
                                    {
                                        let mut gn = String::from("__impl");
                                        gn.push_str(&oast::u32s2(gps.len() as u32));
                                        gps.push(String::from(gn.as_str()));
                                        while gbs.len() + 1 < gps.len() {
                                            gbs.push(String::from(""));
                                        }
                                        gbs.push(bd);
                                        p.t[k].k = Tk::Ident(gn);
                                        let _ = p.t.remove(k + 1); // the one-word bound; the generic carries it
                                    }
                                }
                                k = k + 1;
                            }
                        }
                        // IMPL-ARG: a universal `impl Trait` argument (not Fn) is an anonymous generic parameter -- mono instantiates it per argument type
                        let implbd = if p.is_tok("impl") && p.i + 1 < p.t.len() {
                            tok_word(&p.t[p.i + 1])
                        } else {
                            String::from("")
                        };
                        let implarg = implbd.len() > 0
                            && !(implbd.as_str() == "Fn"
                                || implbd.as_str() == "FnMut"
                                || implbd.as_str() == "FnOnce");
                        if p.cgen.len() > 0 && p.is_tok("[") && ps.len() > 0 {
                            // CONST-GEN-LEN: `a: [T; N]` binds N to a's length
                            let mut k = p.i + 1;
                            let mut d = 0;
                            while k < p.t.len() {
                                if tkv_l_brack(&p.t[k].k) {
                                    d = d + 1;
                                } else if tkv_r_brack(&p.t[k].k) {
                                    if d == 0 {
                                        break;
                                    }
                                    d = d - 1;
                                } else if d == 0
                                    && tkv_semi(&p.t[k].k)
                                    && k + 2 < p.t.len()
                                    && tkv_r_brack(&p.t[k + 2].k)
                                {
                                    let w = tok_word(&p.t[k + 1]);
                                    if oast::in_list(&p.cgen, &w) {
                                        let mut b = String::from(w.as_str());
                                        b.push('=');
                                        b.push_str(ps[ps.len() - 1].as_str());
                                        p.cbind.push(b);
                                    }
                                }
                                k = k + 1;
                            }
                        }
                        p.lastgarg = String::from("");
                        let isrc = p.tok_text().as_str() == "Rc" || p.tok_text().as_str() == "Arc"; // RC-IDENTITY: an `Rc<T>` param (erased to T)
                        p.lastpay = String::new();
                        tn = sl_param(&p.parse_tyname());
                        if (tn.as_str() == "Option" || tn.as_str() == "Result")
                            && p.lastpay.len() > 0
                            && ps.len() > 0
                            && p.namesonly
                        {
                            // OPT-PARAM-PAY (W7): "@pp:<fn>#<param>=<payload>" -- an Option/Result PARAM's slot type carries its payload,
                            // as a let-bound local's does (`f == Some(T::Bar)` on a param `f: Option<T>` compared handles: false)
                            let mut fe = String::from("@pp:");
                            if p.mpfx.len() == 0 && nm.as_str() == "main" {
                                fe.push_str("user_main");
                            } else {
                                fe.push_str(p.qual(&nm).as_str());
                            }
                            fe.push('#');
                            fe.push_str(&oast::u32s2((ps.len() - 1) as u32));
                            fe.push('=');
                            fe.push_str(p.lastpay.replace(",", "$").as_str());
                            p.fpays.push(fe);
                        }
                        if isrc && ps.len() > 0 {
                            let mut fe = String::from("@rcp:");
                            if p.mpfx.len() == 0 && nm.as_str() == "main" {
                                fe.push_str("user_main");
                            } else {
                                fe.push_str(p.qual(&nm).as_str());
                            }
                            fe.push('#');
                            fe.push_str(&oast::u32s2((ps.len() - 1) as u32));
                            p.fpays.push(fe);
                        } // RC-IDENTITY (R4c): the callee holds an Rc handle in this param (drop decrements; clone shares)
                        if p.lastgarg.len() > 0 && ps.len() > 0 {
                            // PARAM-GINST (W1): "@pg:<fn>#<param index>=<arg>" -- param_drops reads it to drop `_r: Op<P>`
                            let mut fe = String::from("@pg:");
                            if p.mpfx.len() == 0 && nm.as_str() == "main" {
                                fe.push_str("user_main");
                            } else {
                                fe.push_str(p.qual(&nm).as_str());
                            }
                            fe.push('#');
                            fe.push_str(&oast::u32s2((ps.len() - 1) as u32));
                            fe.push('=');
                            fe.push_str(p.lastgarg.as_str());
                            p.fpays.push(fe);
                        }
                        if implarg {
                            let mut gn = String::from("__impl");
                            gn.push_str(&oast::u32s2(gps.len() as u32));
                            gps.push(String::from(gn.as_str()));
                            while gbs.len() + 1 < gps.len() {
                                gbs.push(String::from(""));
                            }
                            gbs.push(String::from(implbd.as_str()));
                            tn = gn;
                        }
                        if wasmut && scalar_ty(&tn) {
                            let nm0 = if ps.len() > 0 {
                                String::from(ps[ps.len() - 1].as_str())
                            } else {
                                String::from("")
                            };
                            // C-PLACERET: a `&mut String` parameter is the one place shape we can give a real
                            // place to, by OUT-PARAM-BY-RETURN: the callee returns it and the call site assigns
                            // it back. Recorded here instead of going straight onto the refusal list.
                            // C-PLACERET generalized (rung 88 did `str` only): the transform never looked at
                            // WHICH scalar it was carrying back, so every scalar `&mut` parameter -- int, bool,
                            // char, f64, the sized widths -- takes the same route. `*x += 1` is an ordinary
                            // assignment in a model where `&T` IS `T`, so the callee half needs nothing new.
                            if nm0.len() > 0 {
                                p.mstri.push((ps.len() - 1) as i32);
                                p.mstrt.push(String::from(tn.as_str()));
                                p.mstr.push(nm0);
                            }
                        }
                    }
                    pt.push(tn);
                    pr.push(wasref);
                    if p.is_tok(",") {
                        let _ = p.adv();
                    }
                }
                {
                    // CONST-GEN-ARG (R3): a const param NOT bound to an array length (`fn f<const N: usize>() -> usize { N }`)
                    // is a hidden trailing VALUE parameter; the call's literal turbofish (`f::<3>()`) supplies it. Any other
                    // call shape is an arity mismatch -- refused, never a wrong value
                    let mut hid = String::new();
                    let mut q = 0;
                    while q < p.cgen.len() {
                        let mut pre = String::from(p.cgen[q].as_str());
                        pre.push('=');
                        let mut bound = false;
                        let mut b = 0;
                        while b < p.cbind.len() {
                            if oast::str_starts(p.cbind[b].as_str(), pre.as_str()) {
                                bound = true;
                            }
                            b = b + 1;
                        }
                        if !bound {
                            ps.push(String::from(p.cgen[q].as_str()));
                            if hid.len() > 0 {
                                hid.push(',');
                            }
                            hid.push_str(&oast::u32s2(q as u32));
                            pt.push(String::from(p.cgty[q].as_str()));
                            pr.push(false);
                        }
                        q = q + 1;
                    }
                    if p.namesonly && p.cgen.len() > 0 {
                        let mut e = String::from(nm.as_str());
                        e.push('=');
                        e.push_str(&oast::u32s2(p.cgen.len() as u32));
                        e.push('|');
                        e.push_str(hid.as_str());
                        p.cghid.push(e);
                    }
                }
                let mut rt = String::from(""); // unannotated -> INFERRED from the body (see infer_rets)
                let _ = &rt;
                if p.is_tok("-") {
                    let _ = p.adv();
                    if p.is_tok(">") {
                        let _ = p.adv();
                    }
                    // LT-ELIDE (E0106): an ELIDED return reference borrows from the one reference parameter; with none (or several)
                    // there is nothing to borrow from. Lifetimes are dropped by the lexer, so an explicit one (`&'a T`, `&'static`) is seen
                    // as a >=3-char GAP between `&` and the next token. Real Rust only.
                    if !p.namesonly
                        && p.hasmain
                        && p.perr == 0
                        && p.is_tok("&")
                        && p.i + 1 < p.t.len()
                        && p.t[p.i + 1].lo < p.t[p.i].hi + 3
                    {
                        let _ = &pr;
                        let el_bad = el_refs >= 2 || (el_refs == 0 && !el_opaque);
                        if !el_self && el_bad {
                            p.perr = p.t[p.i].lo + 1;
                            p.perrexp=String::from("a lifetime: the returned reference has no single reference parameter to borrow from");
                            p.perrgot = String::from("&");
                        }
                    }
                    if p.is_tok("(") {
                        let _ = p.adv();
                        let mut nc = 0;
                        let mut es: Vec<String> = Vec::new();
                        loop {
                            if p.is_tok(")") {
                                let _ = p.adv();
                                break;
                            }
                            if p.at_end() {
                                break;
                            }
                            if p.is_tok(",") {
                                let _ = p.adv();
                            } else {
                                es.push(p.parse_tyname());
                                nc = nc + 1;
                            }
                        }
                        rt = tup_name(nc as u32, &es);
                    } else {
                        let wasmut2 = p.is_tok("&") && p.peek_is(1, "mut");
                        p.lastpay = String::from("");
                        rt = sl_param(&p.parse_tyname());
                        if p.lastpay.len() > 0
                            && (rt.as_str() == "Option" || rt.as_str() == "Result")
                        {
                            let mut fe = if p.mpfx.len() == 0 && nm.as_str() == "main" {
                                String::from("user_main")
                            } else {
                                p.qual(&nm)
                            };
                            fe.push('=');
                            fe.push_str(p.lastpay.as_str());
                            p.fpays.push(fe);
                        } /* DBG-PAYLOAD: fn -> its return payload types */
                        if wasmut2 {
                            p.retmut = true;
                        }
                        if wasmut2 && p.perr == 0 && p.depth > 2000000000 {
                            p.perr = p.lo() + 1;
                            p.perrexp=String::from("a return type that is not a `&mut` (returning a PLACE is not implemented)");
                            p.perrgot = p.tok_text();
                        }
                    }
                }
                {
                    let gpc = oast::vc(&gps);
                    p.read_where(&gpc, &mut gbs, &gpc);
                }
                {
                    // FN-BOUND-ARGS (W7): "@fnargs:<fn>#<param>=A,B" -- a closure passed for that param takes these param types
                    while p.fnargsv.len() > 0 {
                        let e = p.fnargsv.remove(0);
                        let eb = e.as_bytes();
                        let mut c = 0;
                        while c < eb.len() && eb[c] != 61 {
                            c = c + 1;
                        }
                        let gname = String::from(&e[..c]);
                        let tys = if c < e.len() {
                            String::from(&e[c + 1..])
                        } else {
                            String::new()
                        };
                        let mut pk = 0;
                        while pk < ps.len() && pk < pt.len() && p.namesonly {
                            if pt[pk].as_str() == gname.as_str() {
                                let mut fe = String::from("@fnargs:");
                                if p.mpfx.len() == 0 && nm.as_str() == "main" {
                                    fe.push_str("user_main");
                                } else {
                                    fe.push_str(p.qual(&nm).as_str());
                                }
                                fe.push('#');
                                fe.push_str(ps[pk].as_str());
                                fe.push('=');
                                fe.push_str(tys.as_str());
                                p.fpays.push(fe);
                            }
                            pk = pk + 1;
                        }
                    }
                }
                {
                    // FN-BOUND-RET (W7): "@fnret:<fn>#F=R" for lowering (a call through `f: F` answers R)
                    while p.fnrets.len() > 0 {
                        let e = p.fnrets.remove(0);
                        // keyed by the PARAMETER NAME typed with the bound generic (`f: F`): lowering sees the param's type as `Fn`
                        let eb = e.as_bytes();
                        let mut c = 0;
                        while c < eb.len() && eb[c] != 61 {
                            c = c + 1;
                        }
                        let gname = String::from(&e[..c]);
                        let ret = if c < e.len() {
                            String::from(&e[c + 1..])
                        } else {
                            String::new()
                        };
                        let mut pk = 0;
                        while pk < ps.len() && pk < pt.len() && p.namesonly {
                            if pt[pk].as_str() == gname.as_str() {
                                let mut fe = String::from("@fnret:");
                                if p.mpfx.len() == 0 && nm.as_str() == "main" {
                                    fe.push_str("user_main");
                                } else {
                                    fe.push_str(p.qual(&nm).as_str());
                                }
                                fe.push('#');
                                fe.push_str(ps[pk].as_str());
                                fe.push('=');
                                fe.push_str(ret.as_str());
                                p.fpays.push(fe);
                            }
                            pk = pk + 1;
                        }
                        if false {
                            let mut fe = String::from("@fnret:");
                            if p.mpfx.len() == 0 && nm.as_str() == "main" {
                                fe.push_str("user_main");
                            } else {
                                fe.push_str(p.qual(&nm).as_str());
                            }
                            fe.push('#');
                            fe.push_str(e.as_str());
                            p.fpays.push(fe);
                        }
                    }
                }
                let cl0 = p.clos.len();
                p.closlets = Vec::new();
                p.lastretmut = p.retmut;
                p.refvars = Vec::new();
                p.eat("{");
                p.cparams = ps.clone();
                p.lnames = Vec::new();
                let c0m = p.clos.len();
                let mut body = if p.namesonly { p.skip_body() } else { p.expr() };
                p.cparams = Vec::new();
                p.mut_encl(c0m, &body);
                p.eat("}");
                if p.retmut {
                    p.mstr = Vec::new();
                    p.mstri = Vec::new();
                    p.mstrt = Vec::new();
                    if !p.namesonly
                        && !tail_place_ref_p(&body)
                        && !is_bare_var_p(&body)
                        && p.perr == 0
                    {
                        p.perr = p.lo() + 1;
                        p.perrexp=String::from("a `&mut`-returning function whose body is not a `&mut` place (returning a non-place through `&mut` is not implemented)");
                        p.perrgot = p.tok_text();
                    }
                    p.retmut = false;
                } // C2-R2: a &mut-RETURN fn is a REFERENCE returner, not an out-param place-fn, so clear its scalar-place registration in BOTH passes (keeps the call a Call for the olower inline). An aggregate &mut param (m1 first) was never in mstr, so this is a no-op there.
                {
                    // CLOS-CAP-CLOS (R2a P4): a capture of a `let`-bound closure is callable -- typed Fn (900) like an `impl Fn` param,
                    // so `let h = || g(1)` calls the captured g (it was "cannot find function g")
                    let mut ps2: Vec<String> = Vec::new();
                    let mut pt2: Vec<String> = Vec::new();
                    let mut z = 0;
                    while z < ps.len() {
                        ps2.push(String::from(ps[z].as_str()));
                        pt2.push(if z < pt.len() {
                            String::from(pt[z].as_str())
                        } else {
                            String::from("")
                        });
                        z = z + 1;
                    }
                    z = 0;
                    while z < p.closlets.len() {
                        ps2.push(String::from(p.closlets[z].as_str()));
                        pt2.push(String::from("Fn"));
                        z = z + 1;
                    }
                    fill_capture_tys(&mut p.clos, cl0, &ps2, &pt2);
                }
                // C-PLACERET: exactly one `&mut String` parameter and no declared return -- the callee RETURNS
                // that parameter and every call site assigns the answer back to the variable it was handed,
                // which is a real place in a value model that has none. More than one such parameter, or a
                // declared return, keeps the refusal: one returned value cannot carry two places back.
                {
                    let qn9 = if p.mpfx.len() == 0 && nm.as_str() == "main" {
                        String::from("user_main")
                    } else {
                        p.qual(&nm)
                    };
                    if p.mstr.len() > 0 {
                        if !p.place_known(qn9.as_str()) {
                            let mut z9 = 0;
                            // the bare name is registered ONLY when it differs from the qualified one: with no
                            // module prefix they are the same string, and registering both made a ONE-place
                            // function look like a two-place one (its indices are read back by name).
                            let ofs9 = if rt.len() > 0 { 1 } else { 0 };
                            while z9 < p.mstri.len() {
                                let ix = p.mstri[z9];
                                p.plname.push(String::from(qn9.as_str()));
                                p.plidx.push(ix);
                                p.plofs.push(ofs9);
                                if qn9.as_str() != nm.as_str() {
                                    p.plname.push(String::from(nm.as_str()));
                                    p.plidx.push(ix);
                                    p.plofs.push(ofs9);
                                }
                                z9 = z9 + 1;
                            }
                        }
                        if !p.namesonly {
                            if has_ret(&body) {
                                if p.perr == 0 {
                                    p.perr = p.lo() + 1;
                                    p.perrexp=String::from("a `&mut` scalar parameter in a function with no early return (its place is handed back as the return value)");
                                    p.perrgot = p.tok_text();
                                }
                            } else {
                                let lo9 = oast::e_lo(&body);
                                let hi9 = oast::e_hi(&body);
                                // C-PLACETUP: SEVERAL places ride back in a TUPLE. One place is still returned
                                // bare, so the single-place shape (and the IR for every program that uses it)
                                // is untouched.
                                if rt.len() > 0 {
                                    // C-PLACEANDRET: the function already answers something, so the places ride
                                    // WITH it: slot 0 is the declared value, slots 1.. are the places. The body's
                                    // own value is bound first, because building the tuple would otherwise lose it.
                                    let mut els: Vec<E> = Vec::new();
                                    let mut ts: Vec<String> = Vec::new();
                                    let mut rv9 = String::from("$rv");
                                    rv9.push_str(&oast::u32s2(p.nhyg));
                                    p.nhyg = p.nhyg + 1;
                                    els.push(E::Var(String::from(rv9.as_str()), lo9, hi9));
                                    ts.push(String::from(rt.as_str()));
                                    let mut z7 = 0;
                                    while z7 < p.mstr.len() {
                                        els.push(E::Var(
                                            String::from(p.mstr[z7].as_str()),
                                            lo9,
                                            hi9,
                                        ));
                                        ts.push(String::from(p.mstrt[z7].as_str()));
                                        z7 = z7 + 1;
                                    }
                                    let n7 = els.len() as u32;
                                    body = E::Let(
                                        rv9,
                                        Box::new(body),
                                        Box::new(E::TupLit(els, lo9, hi9)),
                                        lo9,
                                        hi9,
                                    );
                                    rt = tup_name(n7, &ts);
                                } else if p.mstr.len() == 1 {
                                    let tail = E::Var(String::from(p.mstr[0].as_str()), lo9, hi9);
                                    body = E::Seq(Box::new(body), Box::new(tail), lo9, hi9);
                                    rt = String::from(p.mstrt[0].as_str());
                                } else {
                                    let mut els: Vec<E> = Vec::new();
                                    let mut ts: Vec<String> = Vec::new();
                                    let mut z8 = 0;
                                    while z8 < p.mstr.len() {
                                        els.push(E::Var(
                                            String::from(p.mstr[z8].as_str()),
                                            lo9,
                                            hi9,
                                        ));
                                        ts.push(String::from(p.mstrt[z8].as_str()));
                                        z8 = z8 + 1;
                                    }
                                    let n8 = els.len() as u32;
                                    body = E::Seq(
                                        Box::new(body),
                                        Box::new(E::TupLit(els, lo9, hi9)),
                                        lo9,
                                        hi9,
                                    );
                                    rt = tup_name(n8, &ts);
                                }
                            }
                        }
                    } else {
                        let mut q9 = 0;
                        while q9 < p.mstr.len() {
                            let nmx = String::from(p.mstr[q9].as_str());
                            p.mscal.push(nmx);
                            q9 = q9 + 1;
                        }
                    }
                    p.mstr = Vec::new();
                    p.mstri = Vec::new();
                    p.mstrt = Vec::new();
                }
                {
                    loop {
                        match p.mscal.pop() {
                            Some(nm0) => {
                                if assigns_to(&body, &nm0) && p.perr == 0 {
                                    p.perr = p.lo() + 1;
                                    p.perrexp=String::from("a `&mut` to an aggregate (writing through a `&mut` to a SCALAR needs a place our value model does not have)");
                                    p.perrgot = p.tok_text();
                                }
                            }
                            None => {
                                break;
                            }
                        }
                    }
                }
                {
                    let nm2 = if p.mpfx.len() == 0 && nm.as_str() == "main" {
                        String::from("user_main")
                    } else {
                        p.qual(&nm)
                    };
                    p.fnames.push(String::from(nm2.as_str()));
                    {
                        let mut fa = String::from(nm2.as_str());
                        fa.push('=');
                        fa.push_str(&oast::u32s2(pt.len() as u32));
                        p.farity.push(fa);
                        let mut fr = String::from(nm2.as_str());
                        fr.push('=');
                        fr.push_str(rt.as_str());
                        p.fretn.push(fr);
                    } // FN-ITEM-VAL: arity, for wrapping a fn used as a value
                    if p.lastretmut {
                        p.refret.push(String::from(nm2.as_str()));
                        if nm2.as_str() != nm.as_str() {
                            p.refret.push(String::from(nm.as_str()));
                        }
                    } // C2-R3: a &mut-returning fn is a REFERENCE returner; a local bound from a call to it holds an address
                    {
                        let mut used: Vec<String> = Vec::new();
                        let mut k = 0;
                        while k < pt.len() {
                            if fn_bounded(&gps, &gbs, pt[k].as_str()) {
                                used.push(String::from(pt[k].as_str()));
                                pt[k] = String::from("Fn");
                            }
                            k = k + 1;
                        }
                        if used.len() > 0 {
                            let mut ngps: Vec<String> = Vec::new();
                            let mut ngbs: Vec<String> = Vec::new();
                            let mut g = 0;
                            while g < gps.len() {
                                let mut drop_it = false;
                                let mut u = 0;
                                while u < used.len() {
                                    if used[u].as_str() == gps[g].as_str() {
                                        drop_it = true;
                                        u = used.len();
                                    } else {
                                        u = u + 1;
                                    }
                                }
                                if !drop_it {
                                    ngps.push(String::from(gps[g].as_str()));
                                    ngbs.push(if g < gbs.len() {
                                        String::from(gbs[g].as_str())
                                    } else {
                                        String::from("")
                                    });
                                }
                                g = g + 1;
                            }
                            gps = ngps;
                            gbs = ngbs;
                        }
                    }
                    funs.push(Fun {
                        gsub: Vec::new(),
                        pref: pr,
                        ncap: 0,
                        gbounds: gbs,
                        dflt: 0 - 1,
                        name: nm2,
                        params: ps,
                        ptys: pt,
                        ret: rt,
                        gparams: gps,
                        owner: String::from(""),
                        body: body,
                    });
                }
            } else {
                break;
            }
            // NO-PROGRESS GUARD: any iteration that consumed nothing would loop forever (`None::<String>` hung the
            // compiler). Report it and step over one token instead.
            if p.i == progress_at {
                if p.perr == 0 {
                    p.perr = p.lo() + 1;
                    p.perrexp = String::from("an item");
                    p.perrgot = p.tok_text();
                }
                let _ = p.adv();
                if p.at_end() {
                    break;
                }
            }
        }
        // a file that is only ITEMS has no trailing expression: its program is a call to `main`, the way Rust
        // works. A trailing expression still wins, so every program written in our own dialect is unaffected.

        if pass == 0 {
            funs = Vec::new();
            ginit = Vec::new();
            sdefs = Vec::new();
            edefs = Vec::new();
            tdefs = Vec::new();
            impls = Vec::new();
            p.perr = 0;
            p.perrexp = String::from("");
            p.perrgot = String::from("");
            p.badfmt = 0;
            loop {
                match p.clos.pop() {
                    Some(_f) => {}
                    None => {
                        break;
                    }
                }
            }
        }
        pass = pass + 1;
    }
    {
        // USE-CYCLE (W7): `use e as b; use b::C as e;` -- each alias's path starts at the other alias, so neither resolves
        // (rustc: "cannot determine resolution for the import"). From each alias, follow its path's FIRST segment through the
        // other aliases (`ucyc`, as written); reaching the starting alias again is a cycle -> refused (it resolved silently).
        let mut q = 0;
        while q < p.ucyc.len() && p.perr == 0 {
            let e0 = String::from(p.ucyc[q].as_str());
            let b0 = e0.as_bytes();
            let mut c0 = 0;
            while c0 < b0.len() && b0[c0] != 61 {
                c0 = c0 + 1;
            }
            let start = String::from(&e0[..c0]);
            let mut cur = if c0 < e0.len() {
                String::from(&e0[c0 + 1..])
            } else {
                String::new()
            };
            let mut steps = 0;
            while steps < 16 && cur.len() > 0 {
                let b = cur.as_bytes();
                let mut c = 0;
                while c < b.len() && b[c] != 36 {
                    c = c + 1;
                }
                let first = String::from(&cur[..c]);
                if steps > 0 && first.as_str() == start.as_str() {
                    p.perr = 1;
                    p.perrexp = String::from("an import that resolves (these `use` aliases refer to each other: a cycle)");
                    p.perrgot = String::from(start.as_str());
                    break;
                }
                // the alias named `first` (not the starting entry itself)
                let mut nx = String::new();
                let mut j = 0;
                while j < p.ucyc.len() {
                    let e = p.ucyc[j].as_str();
                    // SELF-NAME-IMPORT (R2a P6): `use crate::a;` records `a=a` -- a name imported as itself (one per module that
                    // imports it) is not an alias step; following it made two such imports read as a cycle
                    let selfname =
                        e.len() == 2 * first.len() + 1 && &e[first.len() + 1..] == first.as_str();
                    if j != q
                        && !selfname
                        && e.len() > first.len()
                        && &e[..first.len()] == first.as_str()
                        && e.as_bytes()[first.len()] == 61
                    {
                        nx = String::from(&e[first.len() + 1..]);
                    }
                    j = j + 1;
                }
                if nx.len() == 0 {
                    break;
                }
                cur = nx;
                steps = steps + 1;
            }
            q = q + 1;
        }
    }
    ord_field_check(&mut p);
    cpat_check(&mut p);
    ex_check(&mut p, &ex_t);
    main_term_check(&mut p, &ex_t);
    missing_ret_check(&mut p, &ex_t);
    lit_width_check(&mut p, &ex_t);
    arr_len_check(&mut p, &ex_t);
    let_lit_pat_check(&mut p, &ex_t);
    cast_char_check(&mut p, &ex_t);
    uninf_init_check(&mut p, &ex_t);
    unit_cast_check(&mut p, &ex_t);
    vec_iter_meth_check(&mut p, &ex_t);
    cpat_ref_check(&mut p, &ex_t);
    unused_tparam_check(&mut p, &ex_t);
    unsized_str_check(&mut p, &ex_t);
    item_placeholder_check(&mut p, &ex_t);
    sibling_path_check(&mut p, &ex_t);
    orphan_impl_check(&mut p, &impls, &ex_t);
    nonexh_check(&mut p, &ex_t);
    param_lit_pat_check(&mut p, &ex_t);
    ref_mut_write_check(&mut p, &ex_t);
    impl_pos_check(&mut p, &ex_t);
    e282_more_check(&mut p, &ex_t);
    maybe_bound_check(&mut p, &ex_t);
    dyn_compat_check(&mut p, &ex_t);
    e277_shapes_check(&mut p, &ex_t);
    array_len_expr_check(&mut p, &ex_t);
    closure_fn_ptr_check(&mut p, &ex_t);
    let_ref_shapes_check(&mut p, &ex_t);
    shapes3_check(&mut p, &ex_t);
    assoc_dyn_variant_check(&mut p, &ex_t);
    bound_derive_check(&mut p, &ex_t);
    enum_discr_width_check(&mut p, &ex_t);
    bound_not_trait_check(&mut p, &ex_t);
    item_type_shapes_check(&mut p, &raw_t);
    assoc_name_check(&mut p, &ex_t);
    bound_shapes2_check(&mut p, &ex_t);
    unsized_arg_check(&mut p, &ex_t);
    super_auto_check(&mut p, &ex_t);
    static_sync_check(&mut p, &ex_t);
    inherent_foreign_check(&mut p, &ex_t);
    const_index_oob_check(&mut p, &ex_t);
    stray_item_lit_check(&mut p, &ex_t);
    value_observe_check(&mut p, &ex_t);
    wrong_guard_check(&mut p, &ex_t);
    user_debug_check(&mut p, &ex_t);
    let m = if p.at_end() {
        let ar: Vec<E> = Vec::new();
        let mut entry = String::from("user_main");
        let mut ux = find_fun_ix(&funs, "user_main");
        if ux < 0 {
            // USE-MAIN (W7): `use foo::bar as main;` -- the entry point is the aliased fn (it ran nothing and exited 0)
            let mut q = 0;
            while q < p.usen.len() {
                if p.usen[q].as_str() == "main" {
                    let t = String::from(p.uset[q].as_str());
                    let tx = find_fun_ix(&funs, t.as_str());
                    if tx >= 0 {
                        entry = t;
                        ux = tx;
                    }
                }
                q = q + 1;
            }
        }
        if ux >= 0 {
            let call = E::Call(entry, ar, 0, 0);
            let rt = String::from(funs[ux as usize].ret.as_str());
            // a `()`-returning main DISCARDS its value: the process exits 0. A `Result`-returning main
            // exits with the VARIANT TAG -- our prelude's Result is Ok=0, Err=1, which is exactly rustc's
            // 0-on-Ok / 1-on-Err. Any other aggregate return is not an exit code at all, so it exits 0.
            if rt.len() == 0 || rt.as_str() == "()" || rt.as_str() == "unit" {
                E::Seq(Box::new(call), Box::new(E::Int(0, 0, 0)), 0, 0)
            } else if rt.as_str() == "Result" {
                let mut ra: Vec<E> = Vec::new();
                ra.push(call);
                E::Call(String::from("rexit"), ra, 0, 0)
            } else if find_sname(&sdefs, &rt) || rt.as_str() == "Option" {
                E::Seq(Box::new(call), Box::new(E::Int(0, 0, 0)), 0, 0)
            } else {
                call
            }
        } else {
            E::Int(0, 0, 0)
        }
    } else {
        p.expr()
    };
    // the global initializers run before anything else in the program
    let m = if ginit.len() > 0 {
        let mut acc = m;
        loop {
            match ginit.pop() {
                Some(g) => {
                    acc = E::Seq(Box::new(g), Box::new(acc), 0, 1);
                }
                None => {
                    break;
                }
            }
        }
        acc
    } else {
        m
    };
    // tokens left over after the program means the input was not understood
    let tr_pos = if p.i + 1 < p.t.len() { p.t[p.i].lo } else { 0 };
    // MATERIALIZE trait defaults: for every `impl Trait for Type` and every trait method the impl did not
    // provide, give Type its own entry pointing at the trait's default body (dflt), with `self` typed as Type.
    {
        let mut ii = 0;
        while ii < impls.len() {
            let ti = tridx(&tdefs, impls[ii].tr.as_str());
            if ti >= 0 {
                let td = &tdefs[ti as usize];
                let mut mi = 0;
                while mi < td.mnames.len() {
                    let mut own = String::from(impls[ii].ty.as_str());
                    own.push_str("$");
                    own.push_str(td.mnames[mi].as_str());
                    if find_fun_ix(&funs, &own) < 0 {
                        let mut dn = String::from(impls[ii].tr.as_str());
                        dn.push_str("$");
                        dn.push_str(td.mnames[mi].as_str());
                        let dx = find_fun_ix(&funs, &dn);
                        if dx >= 0 {
                            let d = &funs[dx as usize];
                            let mut ps: Vec<String> = Vec::new();
                            let mut pt: Vec<String> = Vec::new();
                            let mut pr: Vec<bool> = Vec::new();
                            let mut k = 0;
                            while k < d.params.len() {
                                ps.push(String::from(d.params[k].as_str()));
                                if k == 0 && d.params[k].as_str() == "self" {
                                    pt.push(String::from(impls[ii].ty.as_str()));
                                } else {
                                    pt.push(String::from(d.ptys[k].as_str()));
                                }
                                k = k + 1;
                            }
                            let rt0 = String::from(d.ret.as_str());
                            let rt = if rt0.as_str() == "Self" {
                                String::from(impls[ii].ty.as_str())
                            } else {
                                rt0
                            };
                            funs.push(Fun {
                                gsub: Vec::new(),
                                pref: pr,
                                ncap: 0,
                                gbounds: Vec::new(),
                                dflt: dx,
                                name: own,
                                params: ps,
                                ptys: pt,
                                ret: rt,
                                gparams: Vec::new(),
                                owner: String::from(impls[ii].ty.as_str()),
                                body: E::Int(0, 0, 0),
                            });
                        }
                    }
                    mi = mi + 1;
                }
            }
            ii = ii + 1;
        }
    }
    // MOVE the lifted closures into the program (popping, since `E` has no Clone -- order is irrelevant,
    // a closure's identity rides in its NAME `$closN`)
    loop {
        match p.clos.pop() {
            Some(f) => {
                funs.push(f);
            }
            None => {
                break;
            }
        }
    }
    // RES-USE-EXISTS (rustc E0432 unresolved import): a `use` of a crate path must name an item that exists -- a
    // function, type, enum, or a module (some item lives under `path$`). Capitalised leaves (consts, statics,
    // variants) are not tracked by name here, so they are not judged.
    {
        let mut k = 0;
        while k < p.usechk.len() && p.perr == 0 {
            let t = String::from(p.usechk[k].as_str());
            let tb = t.as_bytes();
            let mut ls = 0;
            let mut q = 0;
            while q < tb.len() {
                if tb[q] == 36 {
                    ls = q + 1;
                }
                q = q + 1;
            }
            let upper = ls < tb.len() && tb[ls] >= 65 && tb[ls] <= 90;
            let mut ok = upper
                || p.fknown(&t)
                || p.sknown(&t)
                || p.enames_has(&t)
                || oast::in_list(&p.usen, &t); // USE-REL: a re-export (`pub use` inside a module) is an item too
            if !ok {
                // SELF-DIFF: an early-return helper -- the inline `while .. && !ok { if .. { ok = true; } }` (an outer flag set
                // in a nested if, plus a byte-slice compare) was MISCOMPILED by SEM0: the SEM0-built ourc refused
                // `use baz::zed;` (a module) that ourc' -- ourc compiled by ourc -- accepted, as rustc does
                ok = fnames_has_prefix(&p.fnames, t.as_str());
            }
            if !ok {
                p.perr = p.usechklo[k] + 1;
                p.perrexp = String::from("an item that exists");
                p.perrgot = t;
            }
            k = k + 1;
        }
    }
    {
        // SPAWN-PANIC (heavy pass 2): a spawned closure that can REACH an explicit panic (through any user fn / method it calls)
        // is refused -- rustc's thread dies alone and join() answers Err; here the whole process exits 101. The closure-body
        // check missed `thread::spawn(f)` with `fn f() { panic!() }` (unwind-unique, issue-30018-panic, slice-panic-2 ...).
        let mut seen: Vec<String> = Vec::new();
        let mut work: Vec<String> = Vec::new();
        let mut q = 0;
        while q < p.spawncl.len() {
            work.push(String::from(p.spawncl[q].as_str()));
            q = q + 1;
        }
        let mut hit = false;
        while work.len() > 0 && !hit {
            let n = work.pop().unwrap();
            if oast::in_list(&seen, n.as_str()) {
                continue;
            }
            seen.push(String::from(n.as_str()));
            if n.as_str() == "panic_exit" {
                hit = true;
                continue;
            }
            let mut calls: Vec<String> = Vec::new();
            let mut fi = 0;
            while fi < funs.len() {
                let fname = funs[fi].name.as_str();
                let m = n.starts_with(".") && fname.ends_with(n.replacen(".", "$", 1).as_str());
                if fname == n.as_str() || m {
                    oast::collect_calls(&funs[fi].body, &mut calls);
                }
                fi = fi + 1;
            }
            let mut ci = 0;
            while ci < p.clos.len() {
                if p.clos[ci].name.as_str() == n.as_str() {
                    oast::collect_calls(&p.clos[ci].body, &mut calls);
                }
                ci = ci + 1;
            }
            while calls.len() > 0 {
                work.push(calls.pop().unwrap());
            }
        }
        if hit && p.perr == 0 {
            p.perr = 1;
            p.perrexp = String::from("a spawned thread that cannot panic (a thread panic would end the whole process here, not just the thread)");
            p.perrgot = String::from("thread::spawn");
        }
    }
    {
        // CONST-DROP (heavy pass 2): a `const` of a type with `impl Drop` is a fresh TEMPORARY at every use, dropped after it
        // (`BAR.0` runs A::drop). Const reads are folded here, so those drops never ran (dropck/issue-34053 counted 0) --
        // REFUSED until const uses materialise their temporaries.
        let mut ci = 0;
        while ci < p.consts.len() && p.perr == 0 {
            let mut fi = 0;
            while fi < funs.len() {
                if funs[fi].name.as_str() == p.consts[ci].as_str()
                    && funs[fi].params.len() == 0
                    && funs[fi].ret.len() > 0
                {
                    let mut dn = String::from(funs[fi].ret.as_str());
                    dn.push_str("$drop");
                    let mut fj = 0;
                    while fj < funs.len() {
                        if funs[fj].name.as_str() == dn.as_str() && p.perr == 0 {
                            p.perr = 1;
                            p.perrexp = String::from("a const whose type has no Drop impl (a Drop const is a temporary at every use -- not modelled)");
                            p.perrgot = String::from(p.consts[ci].as_str());
                        }
                        fj = fj + 1;
                    }
                }
                fi = fi + 1;
            }
            ci = ci + 1;
        }
    }
    // G1 G-P2: turbofish user calls -> `@tf:lo|T1|T2` facts for lowering's mono routing
    {
        let mut q = 0;
        while q < p.tfcalls.len() {
            let mut fk = String::from("@tf:");
            fk.push_str(p.tfcalls[q].as_str());
            p.fpays.push(fk);
            q = q + 1;
        }
    }
    // VIS-FIELD-READ (R2a P6): hand the private fields to lowering as `@priv:m$S|f` facts -- here, once `hasmain` is final (a field
    // can be read before `fn main` is; the .src dialect has no `pub`, so it records none)
    if p.hasmain {
        let mut q = 0;
        while q < p.privflds.len() {
            let mut fk = String::from("@priv:");
            fk.push_str(p.privflds[q].as_str());
            p.fpays.push(fk);
            q = q + 1;
        }
    }
    Prog {
        fpays: {
            let mut v: Vec<String> = Vec::new();
            let mut i = 0;
            while i < p.fpays.len() {
                v.push(String::from(p.fpays[i].as_str()));
                i = i + 1;
            }
            v
        },
        mutcaps: {
            let mut v: Vec<String> = Vec::new();
            let mut i = 0;
            while i < p.mutcaps.len() {
                v.push(String::from(p.mutcaps[i].as_str()));
                i = i + 1;
            }
            v
        },
        strvals: {
            let mut v: Vec<u32> = Vec::new();
            let mut i = 0;
            while i < p.strvals.len() {
                v.push(p.strvals[i]);
                i = i + 1;
            }
            v
        },
        realrs: p.hasmain,
        immlets: {
            let mut v: Vec<u32> = Vec::new();
            let mut i = 0;
            while i < p.immlets.len() {
                v.push(p.immlets[i]);
                i = i + 1;
            }
            v
        },
        placefns: {
            let mut v: Vec<String> = Vec::new();
            let mut i = 0;
            while i < p.plname.len() {
                v.push(String::from(p.plname[i].as_str()));
                i = i + 1;
            }
            v
        },
        placeidx: {
            let mut v: Vec<i32> = Vec::new();
            let mut i = 0;
            while i < p.plidx.len() {
                v.push(p.plidx[i]);
                i = i + 1;
            }
            v
        },
        perrexp: String::from(p.perrexp.as_str()),
        perrgot: String::from(p.perrgot.as_str()),
        perr: p.perr,
        badfmt: p.badfmt,
        trailing: tr_pos,
        sdefs: proj_fields(sdefs, &p.atn, &p.att),
        edefs: edefs,
        funs: funs,
        tdefs: tdefs,
        impls: impls,
        main: m,
    }
}
// FMT-NAMED (core::fmt): `{name}` captures the variable `name`, `{N}` is the N-th argument (reusable), `{}` takes the
// next in order. Normalize the string to plain `{}`/`{:spec}` holes with the argument list in hole order, so the
// walkers below stay positional. Only when a named/positional hole occurs; plain strings pass through untouched.
pub fn fmt_named(fmt: &str, args: Vec<E>, lo: u32, hi: u32) -> (String, Vec<E>) {
    let b = fmt.as_bytes();
    let mut any = false;
    let mut k = 0;
    while k < b.len() {
        if b[k] == 123 {
            if k + 1 < b.len() && b[k + 1] == 123 {
                k = k + 2;
                continue;
            }
            if k + 1 < b.len() && b[k + 1] != 125 && b[k + 1] != 58 {
                any = true;
            }
        }
        k = k + 1;
    }
    if !any {
        return (String::from(fmt), args);
    }
    let mut out = String::from("");
    let mut na: Vec<E> = Vec::new();
    let mut seq = 0;
    k = 0;
    // the arguments MOVE into their first hole (E has no Clone); a positional reuse copies the first use when it is simple
    let mut slots: Vec<Option<E>> = Vec::new();
    for a in args {
        slots.push(Some(a));
    }
    let mut at: Vec<usize> = Vec::new();
    let mut q0 = 0;
    while q0 < slots.len() {
        at.push(usize::MAX);
        q0 = q0 + 1;
    }
    while k < b.len() {
        if b[k] == 123 && k + 1 < b.len() && b[k + 1] == 123 {
            out.push_str("{{");
            k = k + 2;
            continue;
        }
        if b[k] == 125 && k + 1 < b.len() && b[k + 1] == 125 {
            out.push_str("}}");
            k = k + 2;
            continue;
        }
        if b[k] != 123 {
            out.push_str(&fmt[k..(k + 1)]);
            k = k + 1;
            continue;
        }
        let mut j = k + 1;
        while j < b.len() && b[j] != 125 && b[j] != 58 {
            j = j + 1;
        }
        let head = String::from(&fmt[(k + 1)..j]);
        let mut e2 = j;
        while e2 < b.len() && b[e2] != 125 {
            e2 = e2 + 1;
        }
        out.push('{');
        out.push_str(&fmt[j..e2]);
        out.push('}');
        let hb = head.as_bytes();
        let mut n = usize::MAX;
        if hb.len() == 0 {
            n = seq;
            seq = seq + 1;
        } else if hb[0] >= 48 && hb[0] <= 57 {
            n = 0;
            let mut q = 0;
            while q < hb.len() {
                n = n * 10 + ((hb[q] - 48) as usize);
                q = q + 1;
            }
        } else {
            na.push(E::Var(head, lo, hi));
        }
        if n < slots.len() {
            if at[n] == usize::MAX {
                match slots[n].take() {
                    Some(a) => {
                        at[n] = na.len();
                        na.push(a);
                    }
                    None => {}
                }
            } else {
                let c = clone_simple(&na[at[n]], lo, hi);
                na.push(c);
            }
        }
        k = e2 + 1;
    }
    (out, na)
}
// PAT-RANGE-CONST: `T::MIN` / `T::MAX` of an integer width as a pattern endpoint (our values are i64, so u64::MAX is -1)
pub fn int_bound(t: &str, c: &str) -> i64 {
    let mx = c == "MAX";
    if t == "i8" {
        return if mx { 127 } else { 0 - 128 };
    }
    if t == "i16" {
        return if mx { 32767 } else { 0 - 32768 };
    }
    if t == "i32" {
        return if mx { 2147483647 } else { 0 - 2147483648 };
    }
    if t == "u8" {
        return if mx { 255 } else { 0 };
    }
    if t == "u16" {
        return if mx { 65535 } else { 0 };
    }
    if t == "u32" {
        return if mx { 4294967295 } else { 0 };
    }
    if t == "u64" || t == "usize" {
        return if mx { 0 - 1 } else { 0 };
    }
    if mx {
        9223372036854775807
    } else {
        0 - 9223372036854775807 - 1
    }
}
pub fn is_simple_operand(e: &E) -> bool {
    match e {
        E::Var(_n, _l, _h) => true,
        E::Int(_v, _l, _h) => true,
        E::Cast(x, _t, _l, _h) => is_simple_operand(x),
        E::Bin(_o, a, b, _l, _h) => is_simple_operand(a) && is_simple_operand(b),
        E::Not(x, _l, _h) => is_simple_operand(x),
        _ => false,
    }
} // OVF-METHOD: safe to evaluate twice
  // MAC-HYG-PAT: an identifier that can be a BINDING in a pattern: lowercase or `_`-led, and not the bare wildcard `_`
pub fn is_bind_word(t: &Tok) -> bool {
    match &t.k {
        Tk::Ident(x) => {
            let b = x.as_bytes();
            b.len() > 0 && ((b[0] >= 97 && b[0] <= 122) || (b[0] == 95 && b.len() > 1))
        }
        _ => false,
    }
}
// TUP-REST-ARITY: the field index of pattern slot q when a `..` sits at `restat` among `n` pattern slots of a tuple struct
// with `nf` fields -- slots after the rest count from the END (`S(.., b, c)` on 3 fields binds .1 and .2)
pub fn rest_slot(q: usize, restat: i32, n: usize, nf: usize) -> usize {
    if restat >= 0 && (q as i32) >= restat && nf >= n {
        return nf - (n - q);
    }
    q
}
// TUP-REST-END: the TupIdx index of plain-tuple pattern slot q -- after a `..` at `restat` among `n` slots, it is marked
// from-the-END (the arity is resolved by typeck / ety_of / lowering, which know it)
pub fn tup_slot(q: usize, restat: i32, n: usize) -> u32 {
    if restat >= 0 && (q as i32) >= restat {
        return oast::tix_from_end((n - q) as u32);
    }
    q as u32
}
// CFG-ITEM (W2): drop each `__cfg_off__` marker and the item it governs -- an item / `let` keyword start, through a
// depth-0 `;` or the `}` that closes its first block. Anything else (a field, an expression) keeps today's tokens.
// R4e MARKER: zero-cost std markers and hints, removed at the token level (before any stage resolves names):
// `[path::]black_box(x)` -> `(x)` (an identity); `[path::]PhantomData[::<T>]` in VALUE position (not followed by `<`)
// -> `()` (a zero-sized unit value). Type positions (`PhantomData<T>`) and `use` statements are left alone.
fn mk_tok(k: Tk, lo: u32, hi: u32) -> Tok {
    Tok {
        k: k,
        lo: lo,
        hi: hi,
    }
}
fn in_use_stmt(out: &Vec<Tok>) -> bool {
    let mut j = out.len();
    while j > 0 {
        j = j - 1;
        if tkv_semi(&out[j].k) || tkv_r_brace(&out[j].k) {
            j = j + 1;
            break;
        }
        if j == 0 {
            break;
        }
    }
    while j < out.len() && word_is(&out[j], "pub") {
        j = j + 1;
    }
    j < out.len() && tkv_use_t(&out[j].k)
}
fn drop_path_prefix(out: &mut Vec<Tok>) {
    // pop trailing `seg ::` pairs (`std :: hint ::`)
    while out.len() >= 2 && tk_is(&out[out.len() - 1].k, "::") {
        let _ = out.pop();
        let _ = out.pop();
    }
}
pub fn marker_strip(t: Vec<Tok>) -> Vec<Tok> {
    let mut out: Vec<Tok> = Vec::new();
    let mut t = t;
    let mut rev: Vec<Tok> = Vec::new();
    while t.len() > 0 {
        rev.push(t.pop().unwrap());
    }
    while rev.len() > 0 {
        let tk = rev.pop().unwrap();
        let w = tok_word(&tk);
        let nx_paren = rev.len() > 0 && tk_is(&rev[rev.len() - 1].k, "(");
        let nx_lt = rev.len() > 0 && tk_is(&rev[rev.len() - 1].k, "<");
        if w.as_str() == "black_box" && nx_paren && !in_use_stmt(&out) {
            drop_path_prefix(&mut out);
            continue; // `(x)` remains: the identity
        }
        if w.as_str() == "PhantomData" && !nx_lt && !in_use_stmt(&out) {
            drop_path_prefix(&mut out);
            // a turbofish `PhantomData::<T>` is dropped with it
            if rev.len() >= 2
                && tk_is(&rev[rev.len() - 1].k, "::")
                && tk_is(&rev[rev.len() - 2].k, "<")
            {
                let _ = rev.pop();
                let mut d = 0;
                while rev.len() > 0 {
                    let x = rev.pop().unwrap();
                    if tkv_lt(&x.k) {
                        d = d + 1;
                    } else if tkv_gt(&x.k) {
                        d = d - 1;
                    } else if tkv_shr(&x.k) {
                        d = d - 2;
                    }
                    if d <= 0 {
                        break;
                    }
                }
            }
            out.push(mk_tok(Tk::LParen, tk.lo, tk.hi));
            out.push(mk_tok(Tk::RParen, tk.lo, tk.hi));
            continue;
        }
        out.push(tk);
    }
    out
}
fn has_ttsep(v: &Vec<Tok>) -> bool {
    let mut k = 0;
    while k < v.len() {
        if word_is(&v[k], "__ttsep__") {
            return true;
        }
        k = k + 1;
    }
    false
}
pub fn cfg_strip(t: Vec<Tok>) -> Vec<Tok> {
    let mut out: Vec<Tok> = Vec::new();
    let mut i = 0;
    while i < t.len() {
        if word_is(&t[i], "__cfg_off__") {
            let mut j = i + 1;
            let mut s = j;
            while s < t.len() && word_is(&t[s], "pub") {
                s = s + 1;
            }
            let itemish = s < t.len()
                && (tkv_fn(&t[s].k)
                    || tkv_struct(&t[s].k)
                    || tkv_enum(&t[s].k)
                    || tkv_impl(&t[s].k)
                    || tkv_mod_t(&t[s].k)
                    || tkv_const_t(&t[s].k)
                    || tkv_static_t(&t[s].k)
                    || tkv_use_t(&t[s].k)
                    || tkv_type_t(&t[s].k)
                    || tkv_trait(&t[s].k)
                    || tkv_let(&t[s].k)
                    || word_is(&t[s], "macro_rules"));
            if !itemish {
                // CFG-STMT (heavy pass 2): a false cfg on a STATEMENT (`#[cfg(target_pointer_width = "32")] assert_eq!(..);`)
                // must remove the statement -- dropping only the marker RAN the 32-bit assert (const-negation went WRONG).
                // In statement position (after `;` `{` `}`), strip through a depth-0 `;` or a closed block not continued by
                // `else` / `.`; anywhere else (an arm, a field, an expression) KEEP the marker, so the program is refused.
                let stmtpos = out.len() == 0 || {
                    let p = &out[out.len() - 1];
                    tkv_semi(&p.k) || tkv_l_brace(&p.k) || tkv_r_brace(&p.k)
                };
                if !stmtpos {
                    out.push(tok_clone(&t[i]));
                    i = i + 1;
                    continue;
                }
                let mut d = 0;
                let mut opened = false;
                while j < t.len() {
                    if tok_opens(&t[j]) {
                        d = d + 1;
                        if tkv_l_brace(&t[j].k) && d == 1 {
                            opened = true;
                        }
                    } else if tok_closes(&t[j]) {
                        d = d - 1;
                        if d == 0 && opened && tkv_r_brace(&t[j].k) {
                            let cont =
                                j + 1 < t.len() && (tkv_else(&t[j + 1].k) || tkv_dot(&t[j + 1].k));
                            if !cont {
                                j = j + 1;
                                if j < t.len() && tkv_semi(&t[j].k) {
                                    j = j + 1;
                                }
                                break;
                            }
                            opened = false;
                        }
                    } else if d == 0 && tkv_semi(&t[j].k) {
                        j = j + 1;
                        break;
                    }
                    j = j + 1;
                }
                i = j;
                continue;
            }
            let mut d = 0;
            let mut opened = false;
            while j < t.len() {
                if tok_opens(&t[j]) {
                    d = d + 1;
                    if tkv_l_brace(&t[j].k) {
                        opened = true;
                    }
                } else if tok_closes(&t[j]) {
                    d = d - 1;
                    if d == 0 && opened && tkv_r_brace(&t[j].k) {
                        j = j + 1;
                        break;
                    }
                } else if d == 0 && tkv_semi(&t[j].k) {
                    j = j + 1;
                    break;
                }
                j = j + 1;
            }
            i = j;
            continue;
        }
        out.push(tok_clone(&t[i]));
        i = i + 1;
    }
    out
}
// PROJ-FIELD (R3): a struct field typed `<R as Tr>::Buf` was parsed before the impls, so it read as the bare parameter `R` (and a
// `==` on it as an aggregate compare). After the parse, it is the type every impl's `type Buf = ..` agrees on; disagreement (or none)
// keeps the old bare-parameter reading.
fn proj_fields(
    mut sdefs: Vec<oast::Sdef>,
    atn: &Vec<String>,
    att: &Vec<String>,
) -> Vec<oast::Sdef> {
    let mut i = 0;
    while i < sdefs.len() {
        let mut k = 0;
        while k < sdefs[i].ftys.len() {
            if oast::str_starts(sdefs[i].ftys[k].as_str(), "$proj$") {
                let body = String::from(&sdefs[i].ftys[k][6..]);
                let bb = body.as_bytes();
                let mut c = 0;
                while c < bb.len() && bb[c] != 124 {
                    c = c + 1;
                }
                let base = String::from(&body[..c]);
                let mut sfx = String::from("$");
                sfx.push_str(&body[c + 1..]);
                let mut tab = String::new(); // PROJ-PICK: every impl's answer, `X=u32;Y=str`, for lowering to pick per instance
                let mut got = String::new();
                let mut ok = true;
                let mut j = 0;
                while j < atn.len() {
                    if atn[j].ends_with(sfx.as_str()) {
                        if tab.len() > 0 {
                            tab.push(';');
                        }
                        tab.push_str(&atn[j][..atn[j].len() - sfx.len()]);
                        tab.push('=');
                        tab.push_str(att[j].as_str());
                        if got.len() == 0 {
                            got = String::from(att[j].as_str());
                        } else if got.as_str() != att[j].as_str() {
                            ok = false;
                        }
                    }
                    j = j + 1;
                }
                sdefs[i].ftys[k] = if ok && got.len() > 0 {
                    got
                } else if got.len() > 0 {
                    let mut m = String::from("$proj$");
                    m.push_str(base.as_str());
                    m.push('|');
                    m.push_str(tab.as_str());
                    m
                } else {
                    base
                };
            }
            k = k + 1;
        }
        i = i + 1;
    }
    sdefs
}
fn cg_num(s: &str) -> i64 {
    let b = s.as_bytes();
    let mut n: i64 = 0;
    let mut i = 0;
    while i < b.len() {
        n = n * 10 + (b[i] - 48) as i64;
        i = i + 1;
    }
    n
}
fn cg_has(v: &Vec<usize>, x: usize) -> bool {
    let mut i = 0;
    while i < v.len() {
        if v[i] == x {
            return true;
        }
        i = i + 1;
    }
    false
}
// USE-TREE guard (REGRESS-D): a grouped `use` with NO terminating `;` (`use foo::{bar}::baz` then EOF) made use_tree_flat
// read past the token vector (ourc SEGFAULTED). Only a `;`-terminated group is flattened; otherwise the plain reader reports it.
fn use_ends_semi(t: &Vec<Tok>, i: usize) -> bool {
    let mut e = i;
    while e < t.len() {
        if tkv_semi(&t[e].k) {
            return true;
        }
        e = e + 1;
    }
    false
}
// DERIVE-ORD scope (REGRESS-D): the synthesized comparator typechecks only for primitive fields (ints, bool, char) of a
// NON-generic struct -- a generic or &str-field derive(Ord) struct failed typeck (`incompatible types: int vs struct`) and a
// program that never sorted it was REFUSED. Others keep no comparator (their `.sort()` stays refused).
fn ord_fields_prim(ft: &Vec<String>) -> bool {
    let mut k = 0;
    while k < ft.len() {
        let t = ft[k].as_str();
        if !(t == "int" || t == "bool" || t == "char" || oast::is_int_width(t)) {
            return false;
        }
        k = k + 1;
    }
    true
}
// REGRESS-5 (heavy pass 5): `#[derive(Ord)]` needs every field to be Ord. A float, a Range*, or a user struct that does not
// itself derive Ord is rustc E0277; once DERIVE-ORD-SCOPE stopped synthesizing comparators for those, they were ACCEPTED.
fn ord_field_check(p: &mut P) {
    let mut k = 0;
    while k < p.ordneed.len() && p.perr == 0 {
        let e = String::from(p.ordneed[k].as_str());
        let b = e.as_bytes();
        let mut c = 0;
        while c < b.len() && b[c] != 61 {
            c = c + 1;
        }
        let t = String::from(&e[c + 1..]);
        let mut bad = t == "f32" || t == "f64" || oast::str_starts(t.as_str(), "Range");
        if !bad && vs_has(&p.snames, &t) {
            bad = !vs_has(&p.ordhave, &t);
        }
        if bad {
            p.perr = 1;
            p.perrexp =
                String::from("a field type that implements `Ord` (required by #[derive(Ord)])");
            p.perrgot = t;
        }
        k = k + 1;
    }
}
fn vs_has(v: &Vec<String>, s: &String) -> bool {
    let mut k = 0;
    while k < v.len() {
        if v[k].as_str() == s.as_str() {
            return true;
        }
        k = k + 1;
    }
    false
}
// CONST-PAT-TY: a const whose type is a raw pointer (`*const T`) or fn pointer is never an equality pattern
fn ty_raw(t: &Vec<Tok>, a: usize, b: usize) -> bool {
    let mut k = a;
    while k < b && k < t.len() {
        // REGRESS-E: a raw pointer is `*const` / `*mut` -- a bare `*` is the multiplication in an array length (`[u8; 2 * 2]`)
        if tkv_star(&t[k].k) && k + 1 < b && (tkv_const_t(&t[k + 1].k) || tkv_mut(&t[k + 1].k)) {
            return true;
        }
        if word_is(&t[k], "fn") {
            return true;
        }
        k = k + 1;
    }
    false
}
// CONST-PAT-TY widening (REGRESS-D): a non-primitive const used as a pattern (cpatuse) is allowed when every user type
// named in its declared type is structural (`#[derive(PartialEq)]`); a user type without it is rustc's "constant of
// non-structural type in a pattern" -> refused. Run after every item is read (types may be declared after the use).
fn cpat_check(p: &mut P) {
    let n = p.t.len();
    let mut users: Vec<String> = Vec::new();
    let mut peq: Vec<String> = Vec::new();
    let mut ufields: Vec<String> = Vec::new();
    let mut j = 0;
    while j + 1 < n {
        if tkv_struct(&p.t[j].k) || tkv_enum(&p.t[j].k) {
            let nm = tok_word(&p.t[j + 1]);
            if nm.len() > 0 {
                let mut d = false;
                let mut b = j;
                let mut steps = 0;
                while b > 0 && steps < 8 {
                    b = b - 1;
                    steps = steps + 1;
                    if tkv_semi(&p.t[b].k) || tkv_r_brace(&p.t[b].k) {
                        break;
                    }
                    if word_is(&p.t[b], "__derive_peq__") {
                        d = true;
                    }
                }
                if d {
                    peq.push(String::from(nm.as_str()));
                }
                // REGRESS-6: the type's FIELD words (its `( .. )` / `{ .. }` body), so structural-ness is checked through
                // embedded types too (`struct Wrap(NoDerive)` derives PartialEq but is not structural)
                {
                    let mut q = j + 2;
                    while q < n
                        && !tkv_l_paren(&p.t[q].k)
                        && !tkv_l_brace(&p.t[q].k)
                        && !tkv_semi(&p.t[q].k)
                    {
                        q = q + 1;
                    }
                    let mut fw = String::new();
                    if q < n && !tkv_semi(&p.t[q].k) {
                        let mut dd = 0;
                        while q < n {
                            if tk_open(&p.t[q]) {
                                dd = dd + 1;
                            } else if tk_close(&p.t[q]) {
                                dd = dd - 1;
                                if dd == 0 {
                                    break;
                                }
                            }
                            let w = tok_word(&p.t[q]);
                            if w.len() > 0 {
                                fw.push_str(w.as_str());
                                fw.push(',');
                            }
                            q = q + 1;
                        }
                    }
                    ufields.push(fw);
                }
                users.push(nm);
            }
        }
        j = j + 1;
    }
    let mut q = 0;
    while q < p.cpatuse.len() && p.perr == 0 {
        let cn = String::from(p.cpatuse[q].as_str());
        let mut k = 0;
        while k + 2 < n {
            if (tkv_const_t(&p.t[k].k) || word_is(&p.t[k], "const"))
                && word_is(&p.t[k + 1], &*cn.as_str())
                && tkv_colon(&p.t[k + 2].k)
            {
                let mut e = k + 3;
                // R15.6 CPAT-VALUE: rustc judges the constant's VALUE, not its type -- a unit variant (`Eek::TheConst`) or a literal
                // is structural even when another variant of the enum holds a manual-PartialEq type
                let unit_init = cpat_unit_init(&p.t, k + 3);
                while !unit_init && e < n && !tkv_eq(&p.t[e].k) && !tkv_semi(&p.t[e].k) {
                    let w = tok_word(&p.t[e]);
                    if vs_has(&users, &w) && !cp_structural(&w, &users, &peq, &ufields, 0) {
                        p.perr = 1;
                        p.perrexp = String::from(
                            "a constant of a structural type (#[derive(PartialEq)]) in a pattern",
                        );
                        p.perrgot = String::from(cn.as_str());
                    }
                    e = e + 1;
                }
                break;
            }
            k = k + 1;
        }
        q = q + 1;
    }
}
// R9-EXH (REGRESS-D): match EXHAUSTIVENESS by the usefulness algorithm over the match's token patterns. OR-STRUCT / OR-NEST
// split structured alternatives into separate arms and nested payloads were never checked, so non-exhaustive matches were
// ACCEPTED (pattern/usefulness, or-patterns/exhaustiveness-non-exhaustive). Biased to ACCEPT: any pattern form it does
// not model (range, slice, struct, const, macro) counts as a wildcard, so it refuses only a match it can prove open.
struct Pt {
    k: u32, // 0 wild, 1 ctor, 2 literal, 3 or
    c: String,
    a: Vec<Pt>,
}
fn pt(k: u32, c: &str, a: Vec<Pt>) -> Pt {
    Pt {
        k: k,
        c: String::from(c),
        a: a,
    }
}
fn pt_cl(p: &Pt) -> Pt {
    let mut a: Vec<Pt> = Vec::new();
    let mut i = 0;
    while i < p.a.len() {
        a.push(pt_cl(&p.a[i]));
        i = i + 1;
    }
    pt(p.k, p.c.as_str(), a)
}
fn pts_cl(v: &Vec<Pt>, from: usize) -> Vec<Pt> {
    let mut o: Vec<Pt> = Vec::new();
    let mut i = from;
    while i < v.len() {
        o.push(pt_cl(&v[i]));
        i = i + 1;
    }
    o
}
fn tk_open(t: &Tok) -> bool {
    tkv_l_paren(&t.k) || tkv_l_brack(&t.k) || tkv_l_brace(&t.k)
}
fn tk_close(t: &Tok) -> bool {
    tkv_r_paren(&t.k) || tkv_r_brack(&t.k) || tkv_r_brace(&t.k)
}
// top-level split of [a, b) at `sep`
fn ex_split(t: &Vec<Tok>, a: usize, b: usize, sep: &str) -> Vec<usize> {
    let mut cuts: Vec<usize> = Vec::new();
    let mut d = 0;
    let mut q = a;
    while q < b {
        if tk_open(&t[q]) {
            d = d + 1;
        } else if tk_close(&t[q]) {
            d = d - 1;
        } else if d == 0 && tk_is(&t[q].k, sep) {
            cuts.push(q);
        }
        q = q + 1;
    }
    cuts
}
fn ex_has_range(t: &Vec<Tok>, a: usize, b: usize) -> bool {
    let mut q = a;
    while q < b {
        if tkv_dot_dot(&t[q].k) || tk_is(&t[q].k, "..=") || tkv_dot(&t[q].k) {
            return true;
        }
        q = q + 1;
    }
    false
}
// the variant table entry "Enum|Variant|arity" for a (prefix, name), or ""
fn ex_variant(ev: &Vec<String>, pre: &str, nm: &str) -> String {
    let mut k = 0;
    let mut hit = String::new();
    while k < ev.len() {
        let e = ev[k].as_str();
        let parts: Vec<String> = vec![ex_bar_field(e, 0), ex_bar_field(e, 1), ex_bar_field(e, 2)];
        let nparts = ex_bar_count(e) + 1;
        if nparts == 3
            && parts[1].as_str() == nm
            && (pre.len() == 0 || pre == parts[0].as_str() || pre == "Self")
        {
            if hit.len() == 0 {
                hit = String::from(e);
            }
        }
        k = k + 1;
    }
    hit
}
fn ex_pat(t: &Vec<Tok>, a0: usize, b: usize, ev: &Vec<String>) -> Pt {
    let alts = ex_split(t, a0, b, "|");
    if alts.len() > 0 {
        let mut v: Vec<Pt> = Vec::new();
        let mut s = a0;
        let mut i = 0;
        while i <= alts.len() {
            let mut e = b;
            if i < alts.len() {
                e = alts[i];
            }
            if e > s {
                v.push(ex_pat(t, s, e, ev));
            }
            s = e + 1;
            i = i + 1;
        }
        return pt(3, "", v);
    }
    let mut a = a0;
    while a < b
        && (tkv_amp(&t[a].k)
            || tkv_and_and(&t[a].k)
            || tkv_mut(&t[a].k)
            || word_is(&t[a], "ref")
            || word_is(&t[a], "box"))
    {
        a = a + 1;
    }
    if a + 1 < b && is_ident(&t[a]) && tk_is(&t[a + 1].k, "@") {
        return ex_pat(t, a + 2, b, ev);
    }
    if a >= b {
        return pt(0, "", Vec::new());
    }
    if tkv_l_paren(&t[a].k) && tkv_r_paren(&t[b - 1].k) {
        let cuts = ex_split(t, a + 1, b - 1, ",");
        if cuts.len() == 0 {
            return ex_pat(t, a + 1, b - 1, ev); // parenthesized
        }
        if ex_has_range(t, a + 1, b - 1) {
            return pt(0, "", Vec::new());
        }
        let mut v: Vec<Pt> = Vec::new();
        let mut s = a + 1;
        let mut i = 0;
        while i <= cuts.len() {
            let mut e = b - 1;
            if i < cuts.len() {
                e = cuts[i];
            }
            if e > s {
                v.push(ex_pat(t, s, e, ev));
            }
            s = e + 1;
            i = i + 1;
        }
        let mut c = String::from("()");
        c.push_str(v.len().to_string().as_str());
        return pt(1, c.as_str(), v);
    }
    if tk_is(&t[a].k, "true")
        || word_is(&t[a], "true")
        || match t[a].k {
            Tk::TrueT => true,
            _ => false,
        }
    {
        return pt(1, "bool|true", Vec::new());
    }
    if tk_is(&t[a].k, "false")
        || word_is(&t[a], "false")
        || match t[a].k {
            Tk::FalseT => true,
            _ => false,
        }
    {
        return pt(1, "bool|false", Vec::new());
    }
    if ex_has_range(t, a, b) && !tkv_l_paren(&t[a].k) {
        // a range, or a `..` rest inside a payload: not modelled
        let w = tok_word(&t[a]);
        if w.len() == 0 {
            return pt(0, "", Vec::new());
        }
    }
    if b == a + 1 || (b == a + 2 && tkv_minus(&t[a].k)) {
        let tok = &t[b - 1];
        let key = match tok.k.clone() {
            Tk::Num(v) => {
                let mut s = String::from(if b == a + 2 { "n-" } else { "n" });
                s.push_str(v.to_string().as_str());
                s
            }
            Tk::ChL(v) => {
                let mut s = String::from("c");
                s.push_str(v.to_string().as_str());
                s
            }
            Tk::StrL(v) => {
                let mut s = String::from("s");
                s.push_str(v.as_str());
                s
            }
            _ => String::new(),
        };
        if key.len() > 0 {
            return pt(2, key.as_str(), Vec::new());
        }
    }
    // a path `A::B::V` then optional `( .. )` / `{ .. }`
    let mut pre = String::new();
    let mut nm = String::new();
    let mut q = a;
    while q < b {
        let w = tok_word(&t[q]);
        if w.len() > 0 {
            pre = nm;
            nm = w;
            q = q + 1;
        } else if tkv_path_sep(&t[q].k) || tkv_colon(&t[q].k) {
            q = q + 1;
        } else {
            break;
        }
    }
    if nm.len() == 0 {
        return pt(0, "", Vec::new());
    }
    let hasargs = q < b && tkv_l_paren(&t[q].k);
    let hasbrace = q < b && tkv_l_brace(&t[q].k);
    if q < b && !hasargs && !hasbrace {
        return pt(0, "", Vec::new());
    }
    let ent = if nm.as_str() == "Some" && pre.len() == 0 {
        String::from("Option|Some|1")
    } else if nm.as_str() == "None" && pre.len() == 0 {
        String::from("Option|None|0")
    } else if nm.as_str() == "Ok" && pre.len() == 0 {
        String::from("Result|Ok|1")
    } else if nm.as_str() == "Err" && pre.len() == 0 {
        String::from("Result|Err|1")
    } else {
        ex_variant(ev, pre.as_str(), nm.as_str())
    };
    if ent.len() == 0 {
        return pt(0, "", Vec::new()); // a binding, a const, a struct: not modelled (covers all)
    }
    let parts: Vec<String> = vec![
        ex_bar_field(ent.as_str(), 0),
        ex_bar_field(ent.as_str(), 1),
        ex_bar_field(ent.as_str(), 2),
    ];
    let ar: usize = ex_usz(parts[2].as_str());
    let mut c = String::from(parts[0].as_str());
    c.push('|');
    c.push_str(parts[1].as_str());
    let mut args: Vec<Pt> = Vec::new();
    if hasargs && !hasbrace && tkv_r_paren(&t[b - 1].k) && !ex_has_range(t, q + 1, b - 1) {
        let cuts = ex_split(t, q + 1, b - 1, ",");
        let mut s = q + 1;
        let mut i = 0;
        while i <= cuts.len() {
            let mut e = b - 1;
            if i < cuts.len() {
                e = cuts[i];
            }
            if e > s {
                args.push(ex_pat(t, s, e, ev));
            }
            s = e + 1;
            i = i + 1;
        }
    }
    if args.len() != ar {
        args = Vec::new();
        let mut i = 0;
        while i < ar {
            args.push(pt(0, "", Vec::new()));
            i = i + 1;
        }
    }
    pt(1, c.as_str(), args)
}
// the complete constructor set of `c`'s type ("name|arity"), or empty when infinite / unknown
fn ex_sig(c: &str, ev: &Vec<String>) -> Vec<String> {
    let mut o: Vec<String> = Vec::new();
    if c.starts_with("()") {
        let mut s = String::from(c);
        s.push('|');
        s.push_str(&c[2..]);
        o.push(s);
        return o;
    }
    let en_s = ex_bar_field(c, 0); // (self-host: ourc has no str::find)
    let en: &str = en_s.as_str();
    if en == "bool" {
        o.push(String::from("bool|true|0"));
        o.push(String::from("bool|false|0"));
    } else if en == "Option" {
        o.push(String::from("Option|Some|1"));
        o.push(String::from("Option|None|0"));
    } else if en == "Result" {
        o.push(String::from("Result|Ok|1"));
        o.push(String::from("Result|Err|1"));
    } else {
        let mut k = 0;
        while k < ev.len() {
            if ev[k].starts_with(en)
                && ev[k].as_bytes().len() > en.len()
                && ev[k].as_bytes()[en.len()] == 124
            {
                o.push(String::from(ev[k].as_str()));
            }
            k = k + 1;
        }
    }
    o
}
fn ex_spec(rows: &Vec<Vec<Pt>>, c: &str, ar: usize) -> Vec<Vec<Pt>> {
    let mut o: Vec<Vec<Pt>> = Vec::new();
    let mut r = 0;
    while r < rows.len() {
        let h = &rows[r][0];
        if h.k == 0 {
            let mut v: Vec<Pt> = Vec::new();
            let mut i = 0;
            while i < ar {
                v.push(pt(0, "", Vec::new()));
                i = i + 1;
            }
            let rest = pts_cl(&rows[r], 1);
            for x in rest {
                v.push(x);
            }
            o.push(v);
        } else if h.c.as_str() == c && h.a.len() == ar {
            let mut v = pts_cl(&h.a, 0);
            let rest = pts_cl(&rows[r], 1);
            for x in rest {
                v.push(x);
            }
            o.push(v);
        }
        r = r + 1;
    }
    o
}
fn ex_useful(rows0: &Vec<Vec<Pt>>, q: &Vec<Pt>, ev: &Vec<String>, depth: u32) -> bool {
    if depth > 40 {
        return false;
    }
    if q.len() == 0 {
        return rows0.len() == 0;
    }
    // expand or-patterns in the first column
    let mut rows: Vec<Vec<Pt>> = Vec::new();
    let mut r = 0;
    while r < rows0.len() {
        if rows0[r][0].k == 3 {
            let mut i = 0;
            while i < rows0[r][0].a.len() {
                let mut v: Vec<Pt> = Vec::new();
                v.push(pt_cl(&rows0[r][0].a[i]));
                for x in pts_cl(&rows0[r], 1) {
                    v.push(x);
                }
                rows.push(v);
                i = i + 1;
            }
        } else {
            rows.push(pts_cl(&rows0[r], 0));
        }
        r = r + 1;
    }
    let h = &q[0];
    if h.k == 3 {
        let mut i = 0;
        while i < h.a.len() {
            let mut q2: Vec<Pt> = Vec::new();
            q2.push(pt_cl(&h.a[i]));
            for x in pts_cl(q, 1) {
                q2.push(x);
            }
            if ex_useful(&rows, &q2, ev, depth + 1) {
                return true;
            }
            i = i + 1;
        }
        return false;
    }
    if h.k == 1 || h.k == 2 {
        let mut q2 = pts_cl(&h.a, 0);
        for x in pts_cl(q, 1) {
            q2.push(x);
        }
        return ex_useful(&ex_spec(&rows, h.c.as_str(), h.a.len()), &q2, ev, depth + 1);
    }
    // wildcard head: complete signature -> split on every constructor; otherwise the default matrix
    let mut heads: Vec<String> = Vec::new();
    let mut first = String::new();
    let mut r = 0;
    while r < rows.len() {
        if rows[r][0].k == 1 || rows[r][0].k == 2 {
            if first.len() == 0 {
                first = String::from(rows[r][0].c.as_str());
            }
            heads.push(String::from(rows[r][0].c.as_str()));
        }
        r = r + 1;
    }
    let sig = if first.len() > 0 {
        ex_sig(first.as_str(), ev)
    } else {
        Vec::new()
    };
    let mut complete = sig.len() > 0;
    let mut i = 0;
    while i < sig.len() && complete {
        let p = sig[i].as_str();
        let cut = ex_bar_last(p);
        if !vs_has(&heads, &String::from(&p[..cut])) {
            complete = false;
        }
        i = i + 1;
    }
    if complete {
        let mut i = 0;
        while i < sig.len() {
            let p = sig[i].as_str();
            let cut = ex_bar_last(p);
            let ar: usize = ex_usz(&p[cut + 1..]);
            let mut q2: Vec<Pt> = Vec::new();
            let mut j = 0;
            while j < ar {
                q2.push(pt(0, "", Vec::new()));
                j = j + 1;
            }
            for x in pts_cl(q, 1) {
                q2.push(x);
            }
            if ex_useful(&ex_spec(&rows, &p[..cut], ar), &q2, ev, depth + 1) {
                return true;
            }
            i = i + 1;
        }
        return false;
    }
    let mut dm: Vec<Vec<Pt>> = Vec::new();
    let mut r = 0;
    while r < rows.len() {
        if rows[r][0].k == 0 {
            dm.push(pts_cl(&rows[r], 1));
        }
        r = r + 1;
    }
    ex_useful(&dm, &pts_cl(q, 1), ev, depth + 1)
}
// the enum table: "Enum|Variant|arity" for every `enum E { .. }` (struct variants: arity 0, any field values)
fn ex_enums(t: &Vec<Tok>) -> Vec<String> {
    let mut ev: Vec<String> = Vec::new();
    let mut i = 0;
    while i + 2 < t.len() {
        if tkv_enum(&t[i].k) && is_ident(&t[i + 1]) {
            let en = tok_word(&t[i + 1]);
            let mut j = i + 2;
            while j < t.len() && !tkv_l_brace(&t[j].k) && !tkv_semi(&t[j].k) {
                j = j + 1;
            }
            if j < t.len() && tkv_l_brace(&t[j].k) {
                let mut d = 0;
                let mut q = j + 1;
                let mut expect = true;
                while q < t.len() {
                    if tkv_r_brace(&t[q].k) && d == 0 {
                        break;
                    }
                    if d == 0 && expect && is_ident(&t[q]) {
                        let vn = tok_word(&t[q]);
                        let mut ar = 0;
                        if q + 1 < t.len() && tkv_l_paren(&t[q + 1].k) {
                            let mut d2 = 0;
                            let mut r = q + 1;
                            let mut empty = true;
                            ar = 1;
                            while r < t.len() {
                                if tk_open(&t[r]) {
                                    d2 = d2 + 1;
                                } else if tk_close(&t[r]) {
                                    d2 = d2 - 1;
                                    if d2 == 0 {
                                        break;
                                    }
                                } else if d2 == 1 && tkv_comma(&t[r].k) {
                                    if r + 1 < t.len() && !tkv_r_paren(&t[r + 1].k) {
                                        ar = ar + 1;
                                    }
                                } else {
                                    empty = false;
                                }
                                r = r + 1;
                            }
                            if empty {
                                ar = 0;
                            }
                        }
                        let mut e = String::from(en.as_str());
                        e.push('|');
                        e.push_str(vn.as_str());
                        e.push('|');
                        e.push_str(ar.to_string().as_str());
                        ev.push(e);
                        expect = false;
                    }
                    if tk_open(&t[q]) {
                        d = d + 1;
                    } else if tk_close(&t[q]) {
                        d = d - 1;
                    } else if d == 0 && tkv_comma(&t[q].k) {
                        expect = true;
                    } else if d == 0 && tk_is(&t[q].k, "#") {
                        expect = false; // an attribute: the variant name follows its `]`
                    } else if d == 0 && tkv_r_brack(&t[q].k) {
                        expect = true;
                    }
                    q = q + 1;
                }
            }
        }
        i = i + 1;
    }
    ev
}
fn ex_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 {
        return;
    }
    if t.len() > 300000 {
        return; // BUDGET: the usefulness algorithm clones rows per split -- on a giant input (ourc's own 1.8 MB source) it exhausted memory
    }

    let ev = ex_enums(t);
    let n = t.len();
    let mut i = 0;
    let mut bad_at: u32 = 0;
    while i < n && bad_at == 0 {
        if tkv_match(&t[i].k) {
            // scrutinee: up to the first depth-0 `{`
            let mut j = i + 1;
            let mut d = 0;
            let mut skip = false;
            while j < n {
                if d == 0 && tkv_l_brace(&t[j].k) {
                    break;
                }
                if tk_open(&t[j]) {
                    d = d + 1;
                } else if tk_close(&t[j]) {
                    d = d - 1;
                }
                match t[j].k {
                    Tk::Dollar => skip = true,
                    _ => {}
                }
                j = j + 1;
            }
            let mut rows: Vec<Vec<Pt>> = Vec::new();
            let mut q = j + 1;
            let mut ok = j < n;
            while ok && q < n && !tkv_r_brace(&t[q].k) {
                // pattern [q, pe), optional guard, `=>`
                let ps = q;
                let mut d = 0;
                let mut guard = false;
                let mut pe = q;
                while q < n {
                    if d == 0 && tkv_fat_arrow(&t[q].k) {
                        break;
                    }
                    if d == 0 && tkv_if(&t[q].k) && !guard {
                        guard = true;
                        pe = q;
                    }
                    if tk_open(&t[q]) {
                        d = d + 1;
                    } else if tk_close(&t[q]) {
                        d = d - 1;
                    }
                    match t[q].k {
                        Tk::Dollar => skip = true,
                        _ => {}
                    }
                    q = q + 1;
                }
                if q >= n {
                    ok = false;
                    break;
                }
                if !guard {
                    pe = q;
                }
                if pe > ps {
                    let mut a = ps;
                    if tkv_bar(&t[a].k) {
                        a = a + 1; // a leading `|`
                    }
                    if !guard {
                        let mut v: Vec<Pt> = Vec::new();
                        v.push(ex_pat(t, a, pe, &ev));
                        rows.push(v);
                    }
                }
                q = q + 1; // past `=>`
                           // body: a block (then optional `,`), or an expression to a depth-0 `,` / the closing `}`
                if q < n && tkv_l_brace(&t[q].k) {
                    let mut d = 0;
                    while q < n {
                        if tk_open(&t[q]) {
                            d = d + 1;
                        } else if tk_close(&t[q]) {
                            d = d - 1;
                            if d == 0 {
                                q = q + 1;
                                break;
                            }
                        }
                        q = q + 1;
                    }
                    if q < n && tkv_comma(&t[q].k) {
                        q = q + 1;
                    } else if q < n
                        && !tkv_r_brace(&t[q].k)
                        && (tkv_dot(&t[q].k) || tkv_quest(&t[q].k))
                    {
                        ok = false; // `{ .. }.method()` -- not a plain block body
                    }
                } else {
                    let mut d = 0;
                    while q < n {
                        if d == 0 && (tkv_comma(&t[q].k) || tkv_r_brace(&t[q].k)) {
                            break;
                        }
                        if tk_open(&t[q]) {
                            d = d + 1;
                        } else if tk_close(&t[q]) {
                            d = d - 1;
                        }
                        q = q + 1;
                    }
                    if q < n && tkv_comma(&t[q].k) {
                        q = q + 1;
                    }
                }
            }
            if ok && !skip && rows.len() > 0 {
                let mut qv: Vec<Pt> = Vec::new();
                qv.push(pt(0, "", Vec::new()));
                if ex_useful(&rows, &qv, &ev, 0) {
                    bad_at = t[i].lo + 1;
                }
            }
        }
        i = i + 1;
    }
    if bad_at > 0 {
        p.perr = bad_at;
        p.perrexp = String::from("an exhaustive match (some patterns are not covered)");
        p.perrgot = String::from("non-exhaustive patterns");
    }
}
// A2 MAIN-TERM: `fn main() -> T` needs `T: Termination` (`()`, `!`, `Result<(), E>`, `ExitCode`, `impl Termination`).
// A primitive or a reference return type (`char`, `i32`, `bool`, `String`, `&T`) is rustc E0277; it was accepted. Other
// names (an alias, a user type) are not judged here.
fn main_term_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 {
        return;
    }
    let n = t.len();
    let mut i = 0;
    while i + 6 < n {
        if tkv_fn(&t[i].k)
            && word_is(&t[i + 1], "main")
            && tkv_l_paren(&t[i + 2].k)
            && tkv_r_paren(&t[i + 3].k)
            && tkv_minus(&t[i + 4].k)
            && tkv_gt(&t[i + 5].k)
        {
            let r = &t[i + 6];
            let w = tok_word(r);
            let bad = tkv_amp(&r.k)
                || w.as_str() == "char"
                || w.as_str() == "bool"
                || w.as_str() == "str"
                || w.as_str() == "String"
                || w.as_str() == "f32"
                || w.as_str() == "f64"
                || oast::is_int_width(w.as_str());
            if bad {
                p.perr = r.lo + 1;
                p.perrexp = String::from(
                    "a `main` return type that implements `Termination` ((), Result, ExitCode)",
                );
                p.perrgot = if w.len() > 0 {
                    w
                } else {
                    String::from("a reference")
                };
                return;
            }
        }
        i = i + 1;
    }
}
// A2 MISSING-RET: `fn f() -> T { }` / `fn f() -> T { stmt; }` -- a body that ends in a statement (or is empty) yields `()`,
// which is not T: rustc E0308. Judged only when the body holds no `return` / `?` / diverging call / loop / macro
// repetition (any of those might supply or divert the value), so it refuses only a body that provably falls off its end.
fn missing_ret_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 {
        return;
    }
    let n = t.len();
    let mut i = 0;
    while i + 3 < n {
        if tkv_fn(&t[i].k) && is_ident(&t[i + 1]) {
            // the `->` of this signature, then the body's `{` (paren / bracket depth 0)
            let mut j = i + 2;
            let mut d = 0;
            let mut ad = 0; // `<..>` depth: an arrow inside generics (`F: Fn() -> u8`) is a bound's
            let mut inwhere = false;
            let mut arrow = 0;
            while j < n {
                if d == 0 && (tkv_l_brace(&t[j].k) || tkv_semi(&t[j].k)) {
                    break;
                }
                if tkv_where_t(&t[j].k) || word_is(&t[j], "where") {
                    inwhere = true;
                }
                if tkv_lt(&t[j].k) {
                    ad = ad + 1;
                } else if tkv_gt(&t[j].k) && !(j > 0 && tkv_minus(&t[j - 1].k)) {
                    ad = ad - 1;
                }
                if tkv_l_paren(&t[j].k) || tkv_l_brack(&t[j].k) {
                    d = d + 1;
                } else if tkv_r_paren(&t[j].k) || tkv_r_brack(&t[j].k) {
                    d = d - 1;
                } else if d == 0
                    && ad == 0
                    && !inwhere
                    && arrow == 0
                    && tkv_minus(&t[j].k)
                    && j + 1 < n
                    && tkv_gt(&t[j + 1].k)
                {
                    arrow = j + 2;
                }
                j = j + 1;
            }
            if arrow > 0 && arrow < n && j < n && tkv_l_brace(&t[j].k) {
                let r = &t[arrow];
                // only a CONCRETE std / primitive return type is judged (an alias, assoc type or `impl Trait` may be `()`)
                let rw = tok_word(r);
                let concrete = oast::is_int_width(rw.as_str())
                    || rw.as_str() == "bool"
                    || rw.as_str() == "char"
                    || rw.as_str() == "f32"
                    || rw.as_str() == "f64"
                    || rw.as_str() == "String"
                    || rw.as_str() == "Vec"
                    || rw.as_str() == "Option"
                    || rw.as_str() == "Result"
                    || rw.as_str() == "Box";
                if concrete {
                    // the body [j, c]
                    let mut c = j;
                    let mut d = 0;
                    let mut divert = false;
                    while c < n {
                        if tkv_l_brace(&t[c].k) || tkv_l_paren(&t[c].k) || tkv_l_brack(&t[c].k) {
                            d = d + 1;
                        } else if tkv_r_brace(&t[c].k)
                            || tkv_r_paren(&t[c].k)
                            || tkv_r_brack(&t[c].k)
                        {
                            d = d - 1;
                            if d == 0 {
                                break;
                            }
                        }
                        let w = tok_word(&t[c]);
                        if tkv_ret(&t[c].k)
                            || tkv_loop_t(&t[c].k)
                            || tkv_quest(&t[c].k)
                            || tkv_bang(&t[c].k)
                            || tkv_fn(&t[c].k)
                            || tkv_match(&t[c].k)
                            || w.as_str() == "return"
                            || w.as_str() == "loop"
                            || w.as_str() == "exit"
                            || w.as_str() == "abort"
                        {
                            divert = true;
                        }
                        match t[c].k {
                            Tk::Dollar => divert = true,
                            _ => {}
                        }
                        c = c + 1;
                    }
                    let empty = c == j + 1;
                    let semi_end = c > j + 1 && c < n && tkv_semi(&t[c - 1].k);
                    if c < n && !divert && (empty || semi_end) {
                        p.perr = t[c].lo + 1;
                        p.perrexp = String::from("a body whose value is the declared return type (it ends in a statement: `()`)");
                        p.perrgot = String::from("mismatched types");
                        return;
                    }
                }
            }
        }
        i = i + 1;
    }
}
// A2 LIT-WIDTH: a SUFFIXED literal (lexed as the synthetic cast `(n as W)`, its `(` zero-width) has exactly type W, so
// `let x: u32 = 20i32;` and `if c { 10i32 } else { 10u32 }` are rustc E0308; the synthetic cast converted silently.
fn ex_suflit(t: &Vec<Tok>, i: usize) -> String {
    if i + 4 < t.len()
        && tkv_l_paren(&t[i].k)
        && t[i].lo == t[i].hi
        && tkv_as_t(&t[i + 2].k)
        && tkv_r_paren(&t[i + 4].k)
    {
        let ok = match t[i + 1].k {
            Tk::Num(_) => true,
            _ => false,
        };
        let w = tok_word(&t[i + 3]);
        if ok && oast::is_int_width(w.as_str()) {
            return w;
        }
    }
    String::new()
}
fn lit_width_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 {
        return;
    }
    let n = t.len();
    let mut i = 0;
    while i + 8 < n {
        // `let [mut] x: W1 = <lit W2> ;`
        if tkv_let(&t[i].k) {
            let mut j = i + 1;
            if tkv_mut(&t[j].k) {
                j = j + 1;
            }
            if j + 8 < n && is_ident(&t[j]) && tkv_colon(&t[j + 1].k) && tkv_eq(&t[j + 3].k) {
                let w1 = tok_word(&t[j + 2]);
                let w2 = ex_suflit(t, j + 4);
                if oast::is_int_width(w1.as_str())
                    && w2.len() > 0
                    && w1 != w2
                    && tkv_semi(&t[j + 9].k)
                {
                    p.perr = t[j + 4].lo + 1;
                    p.perrexp = String::from("a literal of the annotated type (mismatched types)");
                    p.perrgot = w2;
                    return;
                }
            }
        }
        // `{ <lit W1> } else { <lit W2> }`
        if tkv_l_brace(&t[i].k) && i + 14 < n {
            let w1 = ex_suflit(t, i + 1);
            if w1.len() > 0
                && tkv_r_brace(&t[i + 6].k)
                && tkv_else(&t[i + 7].k)
                && tkv_l_brace(&t[i + 8].k)
                && tkv_r_brace(&t[i + 14].k)
            {
                let w2 = ex_suflit(t, i + 9);
                if w2.len() > 0 && w1 != w2 {
                    p.perr = t[i + 9].lo + 1;
                    p.perrexp = String::from("if and else branches of one type (mismatched types)");
                    p.perrgot = w2;
                    return;
                }
            }
        }
        i = i + 1;
    }
}
// A2 ARR-LEN (E0308): `: [T; N] = [e0, .., ek]` -- an array LITERAL (not a `[x; n]` repeat) whose element count is not the
// annotated literal length N. It was accepted (the annotation's length was not checked against the literal).
fn arr_len_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 {
        return;
    }
    let n = t.len();
    let mut i = 0;
    while i + 8 < n {
        if tkv_colon(&t[i].k) && tkv_l_brack(&t[i + 1].k) {
            // the type `[T; N]`: N is the literal right before the type's closing `]`
            let mut j = i + 2;
            let mut d = 1;
            let mut semi = 0;
            while j < n {
                if tk_open(&t[j]) {
                    d = d + 1;
                } else if tk_close(&t[j]) {
                    d = d - 1;
                    if d == 0 {
                        break;
                    }
                } else if d == 1 && tkv_semi(&t[j].k) {
                    semi = j;
                }
                j = j + 1;
            }
            // (self-host: an if/else whose branch is a `match` is untyped in ourc -- assigned, not a value)
            let mut want: i64 = -1;
            if semi > 0 && j == semi + 2 {
                if let Tk::Num(v) = t[semi + 1].k {
                    want = v;
                }
            }
            if want >= 0 && j + 2 < n && tkv_eq(&t[j + 1].k) && tkv_l_brack(&t[j + 2].k) {
                // the literal `[ .. ]`: count depth-1 commas; a `;` at depth 1 is a repeat (not judged)
                let mut q = j + 3;
                let mut d = 1;
                let mut commas = 0;
                let mut repeat = false;
                let mut last_comma = false;
                let mut empty = true;
                while q < n {
                    if tk_open(&t[q]) {
                        d = d + 1;
                    } else if tk_close(&t[q]) {
                        d = d - 1;
                        if d == 0 {
                            break;
                        }
                    }
                    if d == 1 && tkv_comma(&t[q].k) {
                        commas = commas + 1;
                        last_comma = true;
                    } else if d == 1 && tkv_semi(&t[q].k) {
                        repeat = true;
                    } else if !tk_close(&t[q]) || d >= 1 {
                        last_comma = false;
                        empty = false;
                    }
                    match t[q].k {
                        Tk::Dollar => repeat = true,
                        _ => {}
                    }
                    q = q + 1;
                }
                let count: i64 = if empty {
                    0
                } else if last_comma {
                    commas
                } else {
                    commas + 1
                };
                if q < n && !repeat && count != want {
                    p.perr = t[j + 2].lo + 1;
                    p.perrexp =
                        String::from("an array literal of the annotated length (mismatched types)");
                    p.perrgot = String::from("a different element count");
                    return;
                }
            }
        }
        i = i + 1;
    }
}
// A2 LET-LIT-PAT: `let true = 22;` / `let 5 = x;` / `let None: isize = 42;` -- a `let` whose WHOLE pattern is a literal or
// the unit variant `None` is refutable / mismatched (rustc E0308 / E0005); it was accepted as a binding or dropped.
// (`let ... else` is the refutable form rustc allows: not judged.)
fn let_lit_pat_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 {
        return;
    }
    let n = t.len();
    let mut i = 0;
    while i + 2 < n {
        if tkv_let(&t[i].k) {
            let a = &t[i + 1];
            let lit = tk_is(&a.k, "true")
                || tk_is(&a.k, "false")
                || tok_word(a).as_str() == "true"
                || tok_word(a).as_str() == "false"
                || tok_word(a).as_str() == "None"
                || match a.k {
                    Tk::Num(_) => true,
                    Tk::StrL(_) => true,
                    Tk::TrueT => true,
                    Tk::FalseT => true,
                    Tk::ChL(_) => true,
                    _ => false,
                };
            let next = &t[i + 2];
            // `if let` / `while let` / a let-chain `&& let` may be refutable; `r#true` is a raw identifier (lexed as
            // true, its span longer); a bare `let None = a;` is valid when the payload is uninhabited -- `None` is
            // judged only with a (non-Option) type annotation
            let cond = i > 0
                && (tkv_if(&t[i - 1].k)
                    || tkv_while(&t[i - 1].k)
                    || tkv_and_and(&t[i - 1].k)
                    || tkv_amp(&t[i - 1].k)
                    || word_is(&t[i - 1], "if")
                    || word_is(&t[i - 1], "while"));
            let raw = a.hi > a.lo + 5;
            let none_ok = tok_word(a).as_str() == "None"
                && !(tkv_colon(&next.k) && i + 3 < n && !word_is(&t[i + 3], "Option"));
            if lit && !cond && !raw && !none_ok && (tkv_eq(&next.k) || tkv_colon(&next.k)) {
                // a let-else (`let None = x else { .. };`) is allowed: scan to the statement's `;` at depth 0
                let mut q = i + 2;
                let mut d = 0;
                let mut has_else = false;
                while q < n {
                    if tk_open(&t[q]) {
                        d = d + 1;
                    } else if tk_close(&t[q]) {
                        if d == 0 {
                            break;
                        }
                        d = d - 1;
                    } else if d == 0 && tkv_semi(&t[q].k) {
                        break;
                    } else if d == 0 && tkv_else(&t[q].k) {
                        has_else = true;
                    }
                    q = q + 1;
                }
                if !has_else {
                    p.perr = a.lo + 1;
                    p.perrexp =
                        String::from("an irrefutable `let` pattern (a binding), not a literal");
                    p.perrgot = String::from("a refutable literal pattern");
                    return;
                }
            }
        }
        i = i + 1;
    }
}
// A2 CAST-CHAR (E0604): only `u8` casts to `char`. A suffixed literal of another width (`1u32 as char`, the synthetic
// `(1 as u32) as char`) or a cast chain `x as u32 as char` was accepted.
fn cast_char_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 {
        return;
    }
    let n = t.len();
    let mut i = 0;
    while i + 2 < n {
        if tkv_as_t(&t[i + 1].k) && word_is(&t[i + 2], "char") {
            // the width cast into `char`: a suffixed literal `(n as W)` ending at i, or a chain `.. as W as char`
            let mut w = String::new();
            if tkv_r_paren(&t[i].k) && i >= 4 {
                let s = ex_suflit(t, i - 4);
                if s.len() > 0 {
                    w = s;
                }
            }
            if w.len() == 0 && i >= 1 && tkv_as_t(&t[i - 1].k) {
                let ww = tok_word(&t[i]);
                if oast::is_int_width(ww.as_str()) {
                    w = ww;
                }
            }
            if w.len() > 0 && w.as_str() != "u8" {
                p.perr = t[i + 1].lo + 1;
                p.perrexp = String::from("a `u8` cast to `char` (only `u8` casts to `char`)");
                p.perrgot = w;
                return;
            }
        }
        i = i + 1;
    }
}
// A2 UNINF-INIT (E0282): `let x = Vec::new();` / `= None;` / `= HashMap::new();` (no annotation, no turbofish) where
// `x` is never mentioned again in its block -- nothing fixes the element type, so rustc needs an annotation.
fn uninf_init_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 {
        return;
    }
    let n = t.len();
    let mut i = 0;
    while i + 4 < n {
        if tkv_let(&t[i].k) {
            let mut j = i + 1;
            if tkv_mut(&t[j].k) {
                j = j + 1;
            }
            let nm = tok_word(&t[j]);
            if nm.len() > 0 && j + 2 < n && tkv_eq(&t[j + 1].k) {
                // the initializer up to `;`: exactly `None`, `Vec :: new ( )`, `HashMap :: new ( )`, `VecDeque/HashSet ..`
                let a = j + 2;
                let w = tok_word(&t[a]);
                let mut end = 0;
                if w.as_str() == "None" && a + 1 < n && tkv_semi(&t[a + 1].k) {
                    end = a + 1;
                } else if (w.as_str() == "Vec"
                    || w.as_str() == "HashMap"
                    || w.as_str() == "HashSet"
                    || w.as_str() == "VecDeque"
                    || w.as_str() == "BTreeMap")
                    && a + 5 < n
                    && tkv_path_sep(&t[a + 1].k)
                    && word_is(&t[a + 2], "new")
                    && tkv_l_paren(&t[a + 3].k)
                    && tkv_r_paren(&t[a + 4].k)
                    && tkv_semi(&t[a + 5].k)
                {
                    end = a + 5;
                }
                if end > 0 {
                    // is `nm` mentioned later in the enclosing block?
                    let mut q = end + 1;
                    let mut d = 0;
                    let mut used = false;
                    while q < n {
                        if tk_open(&t[q]) {
                            d = d + 1;
                        } else if tk_close(&t[q]) {
                            if d == 0 {
                                break;
                            }
                            d = d - 1;
                        }
                        if word_is(&t[q], &*nm.as_str()) {
                            used = true;
                            break;
                        }
                        match t[q].k {
                            Tk::Dollar => {
                                used = true;
                                break;
                            }
                            _ => {}
                        }
                        q = q + 1;
                    }
                    if !used {
                        p.perr = t[j].lo + 1;
                        p.perrexp = String::from(
                            "an annotated type (nothing fixes this binding's element type)",
                        );
                        p.perrgot = String::from("type annotations needed");
                        return;
                    }
                }
            }
        }
        i = i + 1;
    }
}
// A2 UNIT-CAST (E0605): `assert!(..) as u32` / `println!(..) as i32` -- the macros that expand to `()` cast to a scalar is a
// non-primitive cast; it was accepted. (`as` right after the macro's closing delimiter.)
fn unit_cast_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 {
        return;
    }
    let n = t.len();
    let mut i = 0;
    while i + 3 < n {
        let w = tok_word(&t[i]);
        let unitmac = w.as_str() == "assert"
            || w.as_str() == "assert_eq"
            || w.as_str() == "assert_ne"
            || w.as_str() == "debug_assert"
            || w.as_str() == "println"
            || w.as_str() == "print"
            || w.as_str() == "eprintln"
            || w.as_str() == "eprint";
        if unitmac && tkv_bang(&t[i + 1].k) && tk_open(&t[i + 2]) {
            let mut q = i + 2;
            let mut d = 0;
            while q < n {
                if tk_open(&t[q]) {
                    d = d + 1;
                } else if tk_close(&t[q]) {
                    d = d - 1;
                    if d == 0 {
                        break;
                    }
                }
                q = q + 1;
            }
            if q + 1 < n && tkv_as_t(&t[q + 1].k) {
                p.perr = t[q + 1].lo + 1;
                p.perrexp = String::from("a primitive value to cast (this macro's value is `()`)");
                p.perrgot = String::from("non-primitive cast");
                return;
            }
        }
        i = i + 1;
    }
}
// A2 VEC-ITER-METH (E0599): an ITERATOR adapter / consumer called straight on a Vec (`vec![..].map(..)`, `v.filter(..)`
// where v's latest binding is a Vec) -- Vec and slices have no such method (`.iter()` / `.into_iter()` is missing); it was
// accepted because iterators are modelled as Vecs. A name's Vec-ness follows its most recent `let` / param in token order.
fn iter_only_method(m: &str) -> bool {
    m == "map"
        || m == "filter"
        || m == "fold"
        || m == "enumerate"
        || m == "zip"
        || m == "rev"
        || m == "sum"
        || m == "product"
        || m == "filter_map"
        || m == "flat_map"
        || m == "skip_while"
        || m == "take_while"
        || m == "for_each"
        || m == "collect"
}
fn vec_iter_meth_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 {
        return;
    }
    let n = t.len();
    let mut vecnames: Vec<String> = Vec::new();
    let mut i = 0;
    while i + 3 < n {
        // a binding: `let [mut] NAME [: TY] = INIT` or a param `NAME : TY`
        if tkv_let(&t[i].k) {
            let mut j = i + 1;
            if tkv_mut(&t[j].k) {
                j = j + 1;
            }
            let nm = tok_word(&t[j]);
            if nm.len() > 0 && j + 2 < n {
                let mut isvec = false;
                if tkv_colon(&t[j + 1].k) && word_is(&t[j + 2], "Vec") {
                    isvec = true;
                } else if tkv_eq(&t[j + 1].k) {
                    let w = tok_word(&t[j + 2]);
                    if w.as_str() == "vec" && j + 3 < n && tkv_bang(&t[j + 3].k) {
                        // `vec![..]` followed directly by `;` (not `vec![..].iter()..`)
                        let mut q = j + 4;
                        let mut d = 0;
                        while q < n {
                            if tk_open(&t[q]) {
                                d = d + 1;
                            } else if tk_close(&t[q]) {
                                d = d - 1;
                                if d == 0 {
                                    break;
                                }
                            }
                            q = q + 1;
                        }
                        isvec = q + 1 < n && tkv_semi(&t[q + 1].k);
                    } else if w.as_str() == "Vec"
                        && j + 6 < n
                        && tkv_path_sep(&t[j + 3].k)
                        && (word_is(&t[j + 4], "new") || word_is(&t[j + 4], "with_capacity"))
                    {
                        isvec = true;
                    }
                }
                // most recent binding wins
                let mut k = 0;
                while k < vecnames.len() {
                    if vecnames[k].as_str() == nm.as_str() {
                        vecnames.remove(k);
                    } else {
                        k = k + 1;
                    }
                }
                if isvec {
                    vecnames.push(nm);
                }
            }
        }
        // `vec![..] . m (` / `NAME . m (`
        if tkv_dot(&t[i].k)
            && i + 2 < n
            && tkv_l_paren(&t[i + 2].k)
            && iter_only_method(tok_word(&t[i + 1]).as_str())
        {
            let recv_vec = if i >= 1 && tkv_r_brack(&t[i - 1].k) {
                // a `vec![..]` literal ending right here?
                let mut q = i - 1;
                let mut d = 0;
                let mut found = false;
                loop {
                    if tk_close(&t[q]) {
                        d = d + 1;
                    } else if tk_open(&t[q]) {
                        d = d - 1;
                        if d == 0 {
                            found = q >= 2 && tkv_bang(&t[q - 1].k) && word_is(&t[q - 2], "vec");
                            break;
                        }
                    }
                    if q == 0 {
                        break;
                    }
                    q = q - 1;
                }
                found
            } else if i >= 1 && !(i >= 2 && tkv_dot(&t[i - 2].k)) {
                let w = tok_word(&t[i - 1]);
                w.len() > 0 && vs_has(&vecnames, &w)
            } else {
                false
            };
            if recv_vec {
                p.perr = t[i + 1].lo + 1;
                p.perrexp = String::from(
                    "an iterator (`.iter()` / `.into_iter()`): a Vec has no such method",
                );
                p.perrgot = tok_word(&t[i + 1]);
                return;
            }
        }
        i = i + 1;
    }
}
// REGRESS-6: a user type is structural when it derives PartialEq AND every user type among its field words is structural
fn cp_structural(
    nm: &String,
    users: &Vec<String>,
    peq: &Vec<String>,
    uf: &Vec<String>,
    depth: u32,
) -> bool {
    if !vs_has(peq, nm) {
        return false;
    }
    if depth > 8 {
        return true;
    }
    let mut k = 0;
    while k < users.len() {
        if users[k].as_str() == nm.as_str() && k < uf.len() {
            let ws: Vec<&str> = uf[k].split(',').collect();
            let mut i = 0;
            while i < ws.len() {
                let w = String::from(ws[i]);
                if w.len() > 0 && w.as_str() != nm.as_str() && vs_has(users, &w) {
                    if !cp_structural(&w, users, peq, uf, depth + 1) {
                        return false;
                    }
                }
                i = i + 1;
            }
            return true;
        }
        k = k + 1;
    }
    true
}
// REGRESS-6 CONST-PAT-REF (E0308): `match &f { FOO => .. }` -- a const pattern of a NON-reference type against a reference
// scrutinee (`&expr`) is a type mismatch (default binding modes do not apply to constants). A `str` const is `&str`.
fn cpat_ref_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 {
        return;
    }
    let n = t.len();
    let mut i = 0;
    while i + 3 < n {
        if tkv_match(&t[i].k) && tkv_amp(&t[i + 1].k) {
            // the arms' block: first depth-0 `{` after the scrutinee
            let mut j = i + 2;
            let mut d = 0;
            while j < n {
                if d == 0 && tkv_l_brace(&t[j].k) {
                    break;
                }
                if tk_open(&t[j]) {
                    d = d + 1;
                } else if tk_close(&t[j]) {
                    d = d - 1;
                }
                j = j + 1;
            }
            // each arm start: right after `{` or a depth-0 `,` / closing `}` of a block body
            let mut q = j + 1;
            let mut d = 0;
            let mut at_start = true;
            while q < n {
                if tk_close(&t[q]) && d == 0 {
                    break;
                }
                if at_start && d == 0 && q + 1 < n && tkv_fat_arrow(&t[q + 1].k) {
                    let w = tok_word(&t[q]);
                    if w.len() > 0 && p.cknown(&w) {
                        let mut pre = String::from(w.as_str());
                        pre.push('=');
                        let mut k = 0;
                        while k < p.ctys.len() {
                            if oast::str_starts(p.ctys[k].as_str(), pre.as_str()) {
                                let ty = String::from(&p.ctys[k][pre.len()..]);
                                if ty.len() > 0
                                    && ty.as_str() != "str"
                                    && !ty.starts_with('&')
                                    && !ty.starts_with('!')
                                {
                                    p.perr = t[q].lo + 1;
                                    p.perrexp = String::from("a constant of the scrutinee's reference type (mismatched types)");
                                    p.perrgot = w;
                                    return;
                                }
                            }
                            k = k + 1;
                        }
                    }
                }
                if tk_open(&t[q]) {
                    d = d + 1;
                } else if tk_close(&t[q]) {
                    d = d - 1;
                }
                at_start = d == 0 && (tkv_comma(&t[q].k) || tkv_r_brace(&t[q].k));
                q = q + 1;
            }
        }
        i = i + 1;
    }
}
// A2 UNUSED-TPARAM (E0392): a struct / enum TYPE parameter that its body never mentions (`struct Foo<A> {}`,
// `struct Foo<T> where T: Copy;`) -- rustc requires every type parameter to be used (PhantomData). Const parameters may
// go unused; lifetimes never reach the parser.
fn unused_tparam_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 {
        return;
    }
    let n = t.len();
    let mut i = 0;
    while i + 3 < n {
        if (tkv_struct(&t[i].k) || tkv_enum(&t[i].k)) && is_ident(&t[i + 1]) && tkv_lt(&t[i + 2].k)
        {
            // the parameter list `< .. >`: type params are the words right after `<` or a depth-1 `,` (not `const`)
            let mut params: Vec<String> = Vec::new();
            let mut gused: Vec<String> = Vec::new();
            let mut j = i + 3;
            let mut d = 1;
            let mut expect = true;
            let mut skip = false;
            while j < n && d > 0 {
                if tkv_lt(&t[j].k) {
                    d = d + 1;
                } else if tkv_gt(&t[j].k) {
                    d = d - 1;
                } else if tkv_shr(&t[j].k) {
                    d = d - 2;
                } else if d == 1 && tkv_comma(&t[j].k) {
                    expect = true;
                    j = j + 1;
                    continue;
                } else if d == 1 && expect {
                    let w = tok_word(&t[j]);
                    if tkv_const_t(&t[j].k) || w.as_str() == "const" {
                        expect = false;
                    } else if w.len() > 0 {
                        params.push(w);
                        expect = false;
                    }
                } else {
                    // a use inside ANOTHER parameter's bound (`H: Handle<Inner = T>`) constrains T: it counts
                    let w = tok_word(&t[j]);
                    if w.len() > 0 && !(j + 1 < n && tkv_colon(&t[j + 1].k)) {
                        gused.push(w);
                    }
                }
                match t[j].k {
                    Tk::Dollar => skip = true,
                    _ => {}
                }
                j = j + 1;
            }
            // skip a `where` clause to the body: `{ .. }`, `( .. )`, or `;` at ANGLE depth 0 (a bound like
            // `C<'c, As = &'c ()>` holds a `(`); a where-clause word is a use unless it is a predicate subject (`T:`)
            let mut ad = 0;
            let mut inwhere = false; // in a where-clause a `(` is a bound's (`F: FnMut(&mut T)`), not a tuple body
            while j < n {
                if tkv_where_t(&t[j].k) || word_is(&t[j], "where") {
                    inwhere = true;
                }
                if ad == 0
                    && (tkv_l_brace(&t[j].k)
                        || (!inwhere && tkv_l_paren(&t[j].k))
                        || tkv_semi(&t[j].k))
                {
                    break;
                }
                if tkv_lt(&t[j].k) {
                    ad = ad + 1;
                } else if tkv_gt(&t[j].k) && !(j > 0 && tkv_minus(&t[j - 1].k)) {
                    ad = ad - 1;
                } else if tkv_shr(&t[j].k) {
                    ad = ad - 2;
                }
                let w = tok_word(&t[j]);
                if w.len() > 0 && !(j + 1 < n && tkv_colon(&t[j + 1].k)) {
                    gused.push(w);
                }
                j = j + 1;
            }
            let mut used: Vec<String> = Vec::new();
            if j < n && !tkv_semi(&t[j].k) {
                let mut q = j;
                let mut d = 0;
                while q < n {
                    if tk_open(&t[q]) {
                        d = d + 1;
                    } else if tk_close(&t[q]) {
                        d = d - 1;
                        if d == 0 {
                            break;
                        }
                    }
                    let w = tok_word(&t[q]);
                    if w.len() > 0 {
                        used.push(w);
                    }
                    match t[q].k {
                        Tk::Dollar => skip = true,
                        _ => {}
                    }
                    q = q + 1;
                }
            }
            if !skip {
                let mut k = 0;
                while k < params.len() {
                    if !vs_has(&used, &params[k]) && !vs_has(&gused, &params[k]) {
                        p.perr = t[i + 1].lo + 1;
                        p.perrexp = String::from(
                            "every type parameter used in the type's fields (or PhantomData)",
                        );
                        p.perrgot = String::from(params[k].as_str());
                        return;
                    }
                    k = k + 1;
                }
            }
        }
        i = i + 1;
    }
}
// A2 UNSIZED-STR (E0277): `str` BY VALUE -- a fn parameter (`fn f(ref s: str)`), a `let x: str`, or a `const` / `static` of
// type `str` -- has no size known at compile time. (A struct's last field may be `str`: struct bodies are not judged.)
// (It scans only the user source, after the prelude's `__usrc__` marker: the prelude takes `s: str` by value.)
fn us_open_paren(t: &Vec<Tok>, i: usize) -> usize {
    // the index of the unmatched `(` / `[` / `{` enclosing position i, or t.len()
    let mut q = i;
    let mut d = 0;
    while q > 0 {
        q = q - 1;
        if tk_close(&t[q]) {
            d = d + 1;
        } else if tk_open(&t[q]) {
            if d == 0 {
                return q;
            }
            d = d - 1;
        }
    }
    t.len()
}
fn us_is_fn_params(t: &Vec<Tok>, q: usize) -> bool {
    // is the `(` at q a fn's parameter list: `fn NAME (` or `fn NAME < .. > (`? (no && chains of calls: SEM0)
    if q >= t.len() {
        return false;
    }
    if !tkv_l_paren(&t[q].k) {
        return false;
    }
    let mut r = q;
    if r >= 1 {
        if tkv_gt(&t[r - 1].k) {
            let mut ad = 0;
            while r > 0 {
                r = r - 1;
                if tkv_gt(&t[r].k) {
                    ad = ad + 1;
                } else if tkv_lt(&t[r].k) {
                    ad = ad - 1;
                    if ad == 0 {
                        break;
                    }
                }
            }
        }
    }
    if r < 2 {
        return false;
    }
    if !is_ident(&t[r - 1]) {
        return false;
    }
    tkv_fn(&t[r - 2].k)
}
fn us_hit(t: &Vec<Tok>, i: usize) -> bool {
    let n = t.len();
    if i < 2 {
        return false;
    }
    if i + 2 >= n {
        return false;
    }
    if !tkv_colon(&t[i].k) {
        return false;
    }
    // the unsized type (`str`, `dyn Trait`, a bare slice `[T]`) and the token right after it
    let te = us_unsized_end(t, i + 1);
    if te == 0 {
        return false;
    }
    if te >= n {
        return false;
    }
    let mut end = false;
    if tkv_comma(&t[te].k) {
        end = true;
    }
    if tkv_r_paren(&t[te].k) {
        end = true;
    }
    if tkv_eq(&t[te].k) {
        end = true;
    }
    if tkv_semi(&t[te].k) {
        end = true;
    }
    if !end {
        return false;
    }
    if tkv_const_t(&t[i - 2].k) {
        return true;
    }
    if tkv_static_t(&t[i - 2].k) {
        return true;
    }
    if tkv_let(&t[i - 2].k) {
        return true;
    }
    let bw = tok_word(&t[i - 2]);
    if bw.as_str() == "const" {
        return true;
    }
    if bw.as_str() == "static" {
        return true;
    }
    us_is_fn_params(t, us_open_paren(t, i))
}
fn unsized_str_check(p: &mut P, t: &Vec<Tok>) {
    // Rust programs only: ourc's own `.src` dialect has no `fn main` and its `str` IS a by-value string type
    if p.perr != 0 || !p.hasmain {
        return;
    }
    // only the USER source: the prelude (ending at its `__usrc__` marker fn) takes `s: str` by value
    let mut i = 2;
    let mut k = 0;
    while k < t.len() {
        if word_is(&t[k], "__usrc__") {
            i = k + 1;
        }
        k = k + 1;
    }
    if us_dialect(t, i) || !p.hasmain {
        return; // heavy pass 7: an ourc-dialect `.src` program (`x: int`, or a mainless script) takes `s: str` by value
    }
    while i + 2 < t.len() {
        if us_hit(t, i) {
            p.perr = t[i + 1].lo + 1;
            p.perrexp =
                String::from("a sized type (`&str`, `&dyn T`, `&[T]`): this type has no size known at compile time");
            p.perrgot = String::from("str");
            return;
        }
        i = i + 1;
    }
}
// A2 ITEM-PLACEHOLDER (E0121): `_` is not allowed in an ITEM signature -- a const / static type (`const A: &_ = 0;`), a fn
// return type (`fn f() -> _`), or a fn parameter type. (`let x: _` and closure params are inference sites: not judged.)
// (Written without && chains of calls / outer flags in nested loops: SEM0.)
fn ph_is_underscore(t: &Tok) -> bool {
    tok_word(t).as_str() == "_"
}
fn ph_in_type(t: &Vec<Tok>, a: usize, stop_eq: bool) -> usize {
    // the index of a `_` in the type starting at a (to `=` / `;` / `{` / `,` / `)` at depth 0), or t.len()
    let mut q = a;
    let mut d = 0;
    while q < t.len() {
        // a type never holds `{` or `;`: stop there at ANY depth (a `>>` that closes two generics must not run the scan
        // into the fn body, where `|_|` is a closure parameter)
        if tkv_semi(&t[q].k) {
            return t.len();
        }
        if tkv_l_brace(&t[q].k) {
            return t.len();
        }
        if tkv_shr(&t[q].k) {
            d = d - 2;
        }
        if d == 0 {
            if tkv_semi(&t[q].k) {
                return t.len();
            }
            if tkv_l_brace(&t[q].k) {
                return t.len();
            }
            if stop_eq {
                if tkv_eq(&t[q].k) {
                    return t.len();
                }
            } else {
                if tkv_comma(&t[q].k) {
                    return t.len();
                }
                if tkv_r_paren(&t[q].k) {
                    return t.len();
                }
                if tkv_where_t(&t[q].k) {
                    return t.len();
                }
            }
        }
        if tkv_lt(&t[q].k) || tkv_l_paren(&t[q].k) || tkv_l_brack(&t[q].k) {
            d = d + 1;
        } else if tkv_gt(&t[q].k) || tkv_r_paren(&t[q].k) || tkv_r_brack(&t[q].k) {
            d = d - 1;
        }
        if ph_is_underscore(&t[q]) {
            return q;
        }
        q = q + 1;
    }
    t.len()
}
fn ph_hit(t: &Vec<Tok>, i: usize) -> usize {
    let n = t.len();
    // `const NAME :` / `static NAME :` -> the type up to `=`
    if i + 3 < n {
        if tkv_const_t(&t[i].k) || tkv_static_t(&t[i].k) {
            let mut generic_param = false;
            if i > 0 {
                if tkv_lt(&t[i - 1].k) {
                    generic_param = true; // `<const N: usize>` is a const GENERIC parameter, not an item
                }
                if tkv_comma(&t[i - 1].k) {
                    generic_param = true;
                }
            }
            if !generic_param && tkv_colon(&t[i + 2].k) {
                return ph_in_type(t, i + 3, true);
            }
        }
    }
    // `fn NAME ( .. ) -> TYPE` -> the return type; each `name : TYPE` parameter
    if i + 2 < n {
        if tkv_fn(&t[i].k) {
            if is_ident(&t[i + 1]) {
                let mut q = i + 2;
                while q < n {
                    if tkv_l_paren(&t[q].k) {
                        break;
                    }
                    q = q + 1;
                }
                let mut d = 0;
                let mut r = q;
                while r < n {
                    if tkv_l_paren(&t[r].k) {
                        d = d + 1;
                    } else if tkv_r_paren(&t[r].k) {
                        d = d - 1;
                        if d == 0 {
                            break;
                        }
                    } else if d == 1 {
                        if tkv_colon(&t[r].k) {
                            let u = ph_in_type(t, r + 1, false);
                            if u < n {
                                return u;
                            }
                        }
                    }
                    r = r + 1;
                }
                if r + 3 < n {
                    if tkv_minus(&t[r + 1].k) {
                        if tkv_gt(&t[r + 2].k) {
                            return ph_in_type(t, r + 3, true);
                        }
                    }
                }
            }
        }
    }
    n
}
fn item_placeholder_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 {
        return;
    }
    let mut i = 0;
    let mut k = 0;
    while k < t.len() {
        if word_is(&t[k], "__usrc__") {
            i = k + 1;
        }
        k = k + 1;
    }
    while i < t.len() {
        let u = ph_hit(t, i);
        if u < t.len() {
            p.perr = t[u].lo + 1;
            p.perrexp =
                String::from("a concrete type: `_` is not allowed in an item signature (E0121)");
            p.perrgot = String::from("_");
            return;
        }
        i = i + 1;
    }
}
// A2 PARAM-LIT-PAT (E0005): a fn PARAMETER whose pattern is a literal (`fn f(0: Box) {}`) is refutable -- parameters
// must be irrefutable. User source only (after the prelude's `__usrc__` marker).
fn plp_is_lit(t: &Tok) -> bool {
    match t.k {
        Tk::Num(_) => true,
        Tk::StrL(_) => true,
        Tk::ChL(_) => true,
        Tk::TrueT => true,
        Tk::FalseT => true,
        _ => false,
    }
}
fn plp_hit(t: &Vec<Tok>, i: usize) -> bool {
    // t[i] is a literal right after a fn parameter-list `(` or `,`, followed by `:`
    if i < 1 {
        return false;
    }
    if i + 1 >= t.len() {
        return false;
    }
    if !plp_is_lit(&t[i]) {
        return false;
    }
    if !tkv_colon(&t[i + 1].k) {
        return false;
    }
    let mut start = false;
    if tkv_l_paren(&t[i - 1].k) {
        start = true;
    }
    if tkv_comma(&t[i - 1].k) {
        start = true;
    }
    if !start {
        return false;
    }
    us_is_fn_params(t, us_open_paren(t, i))
}
fn param_lit_pat_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 {
        return;
    }
    let mut i = 0;
    let mut k = 0;
    while k < t.len() {
        if word_is(&t[k], "__usrc__") {
            i = k + 1;
        }
        k = k + 1;
    }
    while i < t.len() {
        if plp_hit(t, i) {
            p.perr = t[i].lo + 1;
            p.perrexp = String::from("an irrefutable parameter pattern (a binding), not a literal");
            p.perrgot = String::from("refutable pattern in function argument");
            return;
        }
        i = i + 1;
    }
}
// the ourc dialect (`.src` programs) types with `int`, which real Rust has no such type as: `: int` / `-> int` from `from` on
fn us_dialect(t: &Vec<Tok>, from: usize) -> bool {
    let mut q = from;
    while q + 1 < t.len() {
        if word_is(&t[q + 1], "int") {
            if tkv_colon(&t[q].k) {
                return true;
            }
            if tkv_gt(&t[q].k) {
                return true;
            }
        }
        q = q + 1;
    }
    false
}
// REGRESS-E REF-MUT-WRITE: a WRITE through a `ref mut` pattern binding (`Some(ref mut p) => { *p += 1; }`) does not reach
// the matched place -- the binding is a copy of the payload, so `v` kept `Some(22)` (WRONG once OPT-EQ compared payloads;
// tag-only equality had hidden it). Refused until the binding is modelled as a place. (A `ref mut` only READ is fine.)
fn rmw_writes(t: &Vec<Tok>, from: usize, nm: &String) -> bool {
    // `* NAME` followed by `=` or an op-assign (`+=` ..) within the next stretch of tokens
    let mut q = from;
    let end = if from + 400 < t.len() {
        from + 400
    } else {
        t.len()
    };
    while q + 2 < end {
        if tkv_star(&t[q].k) {
            if word_is(&t[q + 1], &*nm.as_str()) {
                if tkv_eq(&t[q + 2].k) {
                    return true;
                }
                let isop = match t[q + 2].k {
                    Tk::OpEq(_) => true,
                    Tk::PlusEq => true,
                    _ => false,
                };
                if isop {
                    return true;
                }
                if tkv_minus_eq(&t[q + 2].k) {
                    return true;
                }
            }
        }
        q = q + 1;
    }
    false
}
fn ref_mut_write_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 {
        return;
    }
    let mut i = 0;
    let mut k = 0;
    while k < t.len() {
        if word_is(&t[k], "__usrc__") {
            i = k + 1;
        }
        k = k + 1;
    }
    while i + 2 < t.len() {
        if word_is(&t[i], "ref") {
            if tkv_mut(&t[i + 1].k) {
                let nm = tok_word(&t[i + 2]);
                if nm.len() > 0 {
                    if rmw_writes(t, i + 3, &nm) {
                        p.perr = t[i].lo + 1;
                        p.perrexp = String::from("a write through a `ref mut` pattern binding (not modelled: it would not reach the matched place)");
                        p.perrgot = nm;
                        return;
                    }
                }
            }
        }
        i = i + 1;
    }
}
// A2 IMPL-POS (E0562): `impl Trait` is only a fn RETURN type or a fn PARAMETER type. In a `let` annotation
// (`let x: impl Sized`, `let x: (impl Copy,)`) or after the `->` of a `Fn(..)` trait (`Fn() -> impl Sized`) it is refused;
// and `fn main() -> impl ..` is not a Termination type (E0277). User source only.
fn ip_let_has_impl(t: &Vec<Tok>, i: usize) -> usize {
    // t[i] is `let`: the annotation `: TYPE` up to `=` / `;` at depth 0 -- the index of an `impl` in it, or t.len()
    let mut q = i + 1;
    while q < t.len() {
        if tkv_colon(&t[q].k) {
            break;
        }
        if tkv_eq(&t[q].k) {
            return t.len();
        }
        if tkv_semi(&t[q].k) {
            return t.len();
        }
        q = q + 1;
    }
    let mut d = 0;
    while q < t.len() {
        if tkv_l_brace(&t[q].k) {
            return t.len(); // a type never holds `{`: the annotation ended without a depth-0 `=` / `;`
        }
        if tkv_shr(&t[q].k) {
            d = d - 2; // `Option<State<S, T>>`
        }
        if d == 0 {
            if tkv_eq(&t[q].k) {
                return t.len();
            }
            if tkv_semi(&t[q].k) {
                return t.len();
            }
        }
        if tkv_l_paren(&t[q].k) || tkv_l_brack(&t[q].k) || tkv_lt(&t[q].k) {
            d = d + 1;
        } else if tkv_r_paren(&t[q].k) || tkv_r_brack(&t[q].k) || tkv_gt(&t[q].k) {
            d = d - 1;
        }
        if tkv_impl(&t[q].k) {
            return q;
        }
        q = q + 1;
    }
    t.len()
}
fn ip_hit(t: &Vec<Tok>, i: usize) -> usize {
    let n = t.len();
    if tkv_let(&t[i].k) {
        return ip_let_has_impl(t, i);
    }
    // `Fn ( .. ) -> impl` / `FnMut` / `FnOnce`
    let w = tok_word(&t[i]);
    let mut fnt = false;
    if w.as_str() == "Fn" {
        fnt = true;
    }
    if w.as_str() == "FnMut" {
        fnt = true;
    }
    if w.as_str() == "FnOnce" {
        fnt = true;
    }
    if fnt {
        if i + 1 < n {
            if tkv_l_paren(&t[i + 1].k) {
                let mut d = 0;
                let mut q = i + 1;
                while q < n {
                    if tkv_l_paren(&t[q].k) {
                        d = d + 1;
                    } else if tkv_r_paren(&t[q].k) {
                        d = d - 1;
                        if d == 0 {
                            break;
                        }
                    }
                    q = q + 1;
                }
                if q + 3 < n {
                    if tkv_minus(&t[q + 1].k) {
                        if tkv_gt(&t[q + 2].k) {
                            if tkv_impl(&t[q + 3].k) {
                                return q + 3;
                            }
                        }
                    }
                }
            }
        }
    }
    // `fn main ( ) -> impl`
    if tkv_fn(&t[i].k) {
        if i + 6 < n {
            if word_is(&t[i + 1], "main") {
                if tkv_l_paren(&t[i + 2].k) {
                    if tkv_r_paren(&t[i + 3].k) {
                        if tkv_minus(&t[i + 4].k) {
                            if tkv_gt(&t[i + 5].k) {
                                // R1 MAIN-IMPL-TERM: `fn main() -> impl Termination` (also `std::process::Termination`) is
                                // valid -- only an impl of some OTHER trait is not a Termination type
                                if tkv_impl(&t[i + 6].k) && !impl_names_termination(t, i + 7) {
                                    return i + 6;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    n
}
fn impl_pos_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 {
        return;
    }
    let mut i = 0;
    let mut k = 0;
    while k < t.len() {
        if word_is(&t[k], "__usrc__") {
            i = k + 1;
        }
        k = k + 1;
    }
    while i < t.len() {
        let u = ip_hit(t, i);
        if u < t.len() {
            p.perr = t[u].lo + 1;
            p.perrexp = String::from("`impl Trait` only as a fn return or parameter type (E0562)");
            p.perrgot = String::from("impl");
            return;
        }
        i = i + 1;
    }
}
// A2 UNSIZED (E0277): the end of an UNSIZED type starting at a -- `str`, `dyn Trait..` (to the depth-0 `,` `)` `=` `;`),
// or a bare slice `[T]` (brackets with no depth-1 `;`) -- or 0 when the type there is sized
fn us_unsized_end(t: &Vec<Tok>, a: usize) -> usize {
    let n = t.len();
    if a >= n {
        return 0;
    }
    if word_is(&t[a], "str") || word_is(&t[a], "Path") {
        return a + 1;
    }
    if tkv_dyn_t(&t[a].k) || word_is(&t[a], "dyn") {
        let mut q = a + 1;
        let mut d = 0;
        while q < n {
            if d == 0 {
                if tkv_comma(&t[q].k)
                    || tkv_r_paren(&t[q].k)
                    || tkv_eq(&t[q].k)
                    || tkv_semi(&t[q].k)
                {
                    return q;
                }
                if tkv_l_brace(&t[q].k) {
                    return 0;
                }
            }
            if tkv_l_paren(&t[q].k) || tkv_lt(&t[q].k) || tkv_l_brack(&t[q].k) {
                d = d + 1;
            } else if tkv_r_paren(&t[q].k) || tkv_gt(&t[q].k) || tkv_r_brack(&t[q].k) {
                d = d - 1;
            }
            q = q + 1;
        }
        return 0;
    }
    if tkv_l_brack(&t[a].k) {
        let mut q = a;
        let mut d = 0;
        while q < n {
            if tkv_l_brack(&t[q].k) || tkv_l_paren(&t[q].k) || tkv_lt(&t[q].k) {
                d = d + 1;
            } else if tkv_r_brack(&t[q].k) || tkv_r_paren(&t[q].k) || tkv_gt(&t[q].k) {
                d = d - 1;
                if d == 0 {
                    return q + 1;
                }
            } else if d == 1 && tkv_semi(&t[q].k) {
                return 0; // `[T; N]` is a sized array
            }
            q = q + 1;
        }
    }
    0
}
// A2 E0282-MORE: three more "type annotations needed" shapes -- `for x in Vec::new() {` (no element type), a bare statement
// `<expr>.parse();` (the parsed type is never fixed), and an empty array literal `[]` as a format argument. User source only.
fn e282_hit(t: &Vec<Tok>, i: usize) -> bool {
    let n = t.len();
    // `in Vec :: new ( ) {`
    if tkv_in_t(&t[i].k) {
        if i + 6 < n {
            if word_is(&t[i + 1], "Vec") {
                if tkv_path_sep(&t[i + 2].k) {
                    if word_is(&t[i + 3], "new") {
                        if tkv_l_paren(&t[i + 4].k) {
                            if tkv_r_paren(&t[i + 5].k) {
                                // only an EMPTY body: a body that uses the element (`sum += &i`) fixes its type
                                if tkv_l_brace(&t[i + 6].k) && i + 7 < n && tkv_r_brace(&t[i + 7].k)
                                {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // `. parse ( ) ;` as a whole statement: the statement starts right after `;` / `{` / `}`
    if tkv_dot(&t[i].k) {
        if i + 4 < n {
            if word_is(&t[i + 1], "parse") {
                if tkv_l_paren(&t[i + 2].k) {
                    if tkv_r_paren(&t[i + 3].k) {
                        if tkv_semi(&t[i + 4].k) {
                            // walk back to the statement start: no `let` / `=` / `return` in between
                            let mut q = i;
                            while q > 0 {
                                q = q - 1;
                                if tkv_semi(&t[q].k) || tkv_l_brace(&t[q].k) || tkv_r_brace(&t[q].k)
                                {
                                    return true;
                                }
                                if tkv_let(&t[q].k) || tkv_eq(&t[q].k) || tkv_ret(&t[q].k) {
                                    return false;
                                }
                                if tkv_l_paren(&t[q].k) || tkv_comma(&t[q].k) {
                                    return false; // an argument: the callee may fix the type
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // `, [ ] )` -- an empty array as the last macro / call argument
    if tkv_comma(&t[i].k) {
        if i + 3 < n {
            if tkv_l_brack(&t[i + 1].k) {
                if tkv_r_brack(&t[i + 2].k) {
                    if tkv_r_paren(&t[i + 3].k) {
                        if i >= 2 {
                            // only a format macro's argument list: `println ! ( "..." , [ ] )`
                            let mut q = i;
                            while q > 0 {
                                q = q - 1;
                                if tkv_l_paren(&t[q].k) {
                                    if q >= 2 {
                                        if tkv_bang(&t[q - 1].k) {
                                            let w = tok_word(&t[q - 2]);
                                            if w.as_str() == "println"
                                                || w.as_str() == "print"
                                                || w.as_str() == "format"
                                            {
                                                return true;
                                            }
                                        }
                                    }
                                    return false;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    false
}
fn e282_more_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 {
        return;
    }
    let mut i = 0;
    let mut k = 0;
    while k < t.len() {
        if word_is(&t[k], "__usrc__") {
            i = k + 1;
        }
        k = k + 1;
    }
    while i < t.len() {
        if e282_hit(t, i) {
            p.perr = t[i].lo + 1;
            p.perrexp =
                String::from("a type annotation (nothing fixes this expression's type: E0282)");
            p.perrgot = String::from("type annotations needed");
            return;
        }
        i = i + 1;
    }
}
// A2 MAYBE-BOUND: `?Trait` relaxes ONLY the default `Sized` bound, and takes no generic arguments -- `T: ?Hash` and
// `T: ?Sized<i32>` are refused (rustc: "relaxing a default bound only does something for `?Sized`" / E0107). The shape is
// a `?` right after a bound's `:` or `+`. User source only.
fn mb_hit(t: &Vec<Tok>, i: usize) -> bool {
    let n = t.len();
    if i + 2 >= n {
        return false;
    }
    if !tkv_quest(&t[i + 1].k) {
        return false;
    }
    let mut lead = false;
    if tkv_colon(&t[i].k) {
        lead = true;
    }
    if tkv_plus(&t[i].k) {
        lead = true;
    }
    if !lead {
        return false;
    }
    let w = tok_word(&t[i + 2]);
    if w.len() == 0 {
        return false;
    }
    if w.as_str() != "Sized" {
        return true;
    }
    if i + 3 < n {
        if tkv_lt(&t[i + 3].k) {
            return true;
        }
    }
    false
}
fn maybe_bound_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 {
        return;
    }
    let mut i = 0;
    let mut k = 0;
    while k < t.len() {
        if word_is(&t[k], "__usrc__") {
            i = k + 1;
        }
        k = k + 1;
    }
    while i < t.len() {
        if mb_hit(t, i) {
            p.perr = t[i + 1].lo + 1;
            p.perrexp = String::from(
                "`?Sized` (relaxing a default bound only works for `Sized`, with no arguments)",
            );
            p.perrgot = tok_word(&t[i + 2]);
            return;
        }
        i = i + 1;
    }
}
// A2 DYN-COMPAT (E0038): `dyn Trait` of a trait that is NOT dyn-compatible: the trait requires `Self: Sized` (a where-clause or
// a `Sized` supertrait), has an associated `const`, or has a method -- without its own `where Self: Sized` -- that is generic
// (`fn f<T>`), returns `Self`, or takes no `self` receiver. Judged on traits DECLARED in the user source only.
fn dc_sized_req(t: &Vec<Tok>, a: usize, b: usize) -> bool {
    // a `Self : Sized` (or a bare `: Sized` supertrait) in [a, b)
    let mut q = a;
    while q + 2 < b {
        if word_is(&t[q], "Self") {
            if tkv_colon(&t[q + 1].k) {
                if word_is(&t[q + 2], "Sized") {
                    return true;
                }
            }
        }
        q = q + 1;
    }
    false
}
fn dc_method_bad(t: &Vec<Tok>, f: usize, e: usize) -> bool {
    // the method signature [f = `fn`, e = its `;` or `{`): generic / returns Self / no self receiver, unless `where Self: Sized`
    if dc_sized_req(t, f, e) {
        return false;
    }
    // any `Self: ..` bound on the method may entail Sized (`where Self: Tr1<..>` with `trait Tr1: Sized`): exempt
    if dc_self_bound(t, f, e) {
        return false;
    }
    if f + 3 < e {
        if tkv_lt(&t[f + 2].k) {
            // lifetimes never reach the parser: `fn f<'b>` is an EMPTY `<>`, not a generic method
            if !tkv_gt(&t[f + 3].k) {
                return true; // `fn name<T>`
            }
        }
    }
    let mut q = f;
    let mut paren = 0;
    while q < e {
        if tkv_l_paren(&t[q].k) {
            paren = q;
            break;
        }
        q = q + 1;
    }
    if paren == 0 {
        return false;
    }
    // the first parameter: `self`, `&self`, `&mut self`, `mut self`, `self: ..`
    let mut r = paren + 1;
    while r < e {
        if tkv_amp(&t[r].k) || tkv_mut(&t[r].k) {
            r = r + 1;
        } else {
            break;
        }
    }
    let mut has_self = false;
    if r < e {
        if word_is(&t[r], "self") {
            has_self = true;
        }
        if tk_is(&t[r].k, "self") {
            has_self = true;
        }
    }
    if !has_self {
        return true;
    }
    // `-> Self`
    let mut s = paren;
    while s + 2 < e {
        if tkv_minus(&t[s].k) {
            if tkv_gt(&t[s + 1].k) {
                if word_is(&t[s + 2], "Self") && !(s + 3 < e && tkv_path_sep(&t[s + 3].k)) {
                    return true;
                }
            }
        }
        s = s + 1;
    }
    false
}
fn dc_trait_bad(t: &Vec<Tok>, i: usize) -> bool {
    // t[i] = `trait`, t[i+1] = name: the header to `{`, then the body items
    let n = t.len();
    let mut j = i + 2;
    while j < n {
        if tkv_l_brace(&t[j].k) {
            break;
        }
        j = j + 1;
    }
    if j >= n {
        return false;
    }
    if dc_sized_req(t, i + 2, j) {
        return true;
    }
    // a bare `: Sized` supertrait: `trait T : Sized`
    let mut h = i + 2;
    while h + 1 < j {
        if tkv_where_t(&t[h].k) || word_is(&t[h], "where") {
            break; // supertraits end at `where` (a where-predicate `(dyn T + 'a): Sized` is not one)
        }
        if tkv_colon(&t[h].k) || tkv_plus(&t[h].k) {
            if word_is(&t[h + 1], "Sized") {
                return true;
            }
        }
        h = h + 1;
    }
    // body items at depth 1
    let mut q = j + 1;
    let mut d = 1;
    while q < n {
        if tkv_l_brace(&t[q].k) {
            d = d + 1;
        } else if tkv_r_brace(&t[q].k) {
            d = d - 1;
            if d == 0 {
                return false;
            }
        } else if d == 1 {
            // a GENERIC associated type (`type Bar<T>;`) also makes the trait dyn-incompatible -- unless the item itself is
            // `where Self: Sized` (R15.2: rustc excludes such an item from the vtable, as it does a method)
            if tkv_type_t(&t[q].k) || word_is(&t[q], "type") {
                if q + 2 < n && tkv_lt(&t[q + 2].k) && !dc_item_exempt(t, q) {
                    return true;
                }
            }
            if (tkv_const_t(&t[q].k) || word_is(&t[q], "const")) && !dc_item_exempt(t, q) {
                return true;
            }
            if tkv_fn(&t[q].k) {
                let mut e = q + 1;
                while e < n {
                    if tkv_semi(&t[e].k) || tkv_l_brace(&t[e].k) {
                        break;
                    }
                    e = e + 1;
                }
                if dc_method_bad(t, q, e) {
                    return true;
                }
            }
        }
        q = q + 1;
    }
    false
}
fn dyn_compat_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 {
        return;
    }
    let mut u = 0;
    let mut k = 0;
    while k < t.len() {
        if word_is(&t[k], "__usrc__") {
            u = k + 1;
        }
        k = k + 1;
    }
    let mut bad: Vec<String> = Vec::new();
    let mut i = u;
    while i + 1 < t.len() {
        if tkv_trait(&t[i].k) {
            let nm = tok_word(&t[i + 1]);
            if nm.len() > 0 {
                if dc_trait_bad(t, i) {
                    bad.push(nm);
                }
            }
        }
        i = i + 1;
    }
    if bad.len() == 0 {
        return;
    }
    let mut i = u;
    let mut in_alias = false;
    while i + 1 < t.len() {
        // a `type` ALIAS is not well-formedness-checked by rustc: a `dyn` inside `type A = ..;` is not judged
        if tkv_type_t(&t[i].k) || word_is(&t[i], "type") {
            in_alias = true; // `type A<..> = ..;` (generic aliases too) -- an associated `type T;` ends at its own `;`
        }
        if tkv_semi(&t[i].k) {
            in_alias = false;
        }
        if !in_alias && (tkv_dyn_t(&t[i].k) || word_is(&t[i], "dyn")) {
            let w = tok_word(&t[i + 1]);
            if vs_has(&bad, &w) {
                p.perr = t[i].lo + 1;
                p.perrexp = String::from("a dyn-compatible trait (no generic / Self-returning / receiver-less methods, consts, or Sized)");
                p.perrgot = w;
                return;
            }
        }
        i = i + 1;
    }
}
// DYN-COMPAT: a `Self :` bound anywhere in [a, b)
fn dc_self_bound(t: &Vec<Tok>, a: usize, b: usize) -> bool {
    let mut q = a;
    while q + 1 < b {
        if word_is(&t[q], "Self") {
            if tkv_colon(&t[q + 1].k) {
                return true;
            }
        }
        q = q + 1;
    }
    false
}
// A2 E0277-SHAPES: an index by a suffixed NON-usize literal (`x[0i32]`), a `for` over a string LITERAL (`for c in "asdf"`:
// &str is not an iterator), and `impl .. + ?Sized` in argument position (an APIT is always Sized). User source only.
fn e277_hit(t: &Vec<Tok>, i: usize) -> bool {
    let n = t.len();
    // `[ <suffixed literal W> ]` right after an indexable (a word, `)` or `]`), W not usize
    if tkv_l_brack(&t[i].k) {
        if i >= 1 {
            let prev_ok =
                is_ident(&t[i - 1]) || tkv_r_paren(&t[i - 1].k) || tkv_r_brack(&t[i - 1].k);
            if prev_ok {
                let w = ex_suflit(t, i + 1);
                if w.len() > 0 {
                    if w.as_str() != "usize" {
                        if i + 6 < n {
                            if tkv_r_brack(&t[i + 6].k) {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }
    // `in "literal" {`
    if tkv_in_t(&t[i].k) {
        if i + 2 < n {
            let lit = match t[i + 1].k {
                Tk::StrL(_) => true,
                _ => false,
            };
            if lit {
                if tkv_l_brace(&t[i + 2].k) {
                    return true;
                }
            }
        }
    }
    // `impl ?Sized` (the bound alone) inside a fn parameter list
    if tkv_impl(&t[i].k) {
        if i + 2 < n {
            if tkv_quest(&t[i + 1].k) {
                if word_is(&t[i + 2], "Sized") {
                    return us_is_fn_params(t, us_open_paren(t, i));
                }
            }
        }
    }
    // `impl <bounds> + ?Sized` inside a fn parameter list
    if tkv_plus(&t[i].k) {
        if i + 2 < n {
            if tkv_quest(&t[i + 1].k) {
                if word_is(&t[i + 2], "Sized") {
                    // walk back to the start of this bound list: an `impl` before any `:` / `<` / `,` / `(`
                    // (a balanced `<..>` -- `Iterator<Item = i32>` -- is skipped)
                    let mut q = i;
                    let mut ad = 0;
                    while q > 0 {
                        q = q - 1;
                        if tkv_gt(&t[q].k) {
                            ad = ad + 1;
                        } else if tkv_lt(&t[q].k) {
                            if ad == 0 {
                                return false;
                            }
                            ad = ad - 1;
                        } else if ad == 0 {
                            if tkv_impl(&t[q].k) {
                                return us_is_fn_params(t, us_open_paren(t, q));
                            }
                            if tkv_colon(&t[q].k) || tkv_comma(&t[q].k) || tkv_l_paren(&t[q].k) {
                                return false;
                            }
                        }
                    }
                }
            }
        }
    }
    // `= * "literal" ;` -- a dereferenced string literal is `str`, unsized: it cannot be bound by value
    if tkv_eq(&t[i].k) {
        if i + 3 < n {
            if tkv_star(&t[i + 1].k) {
                let lit = match t[i + 2].k {
                    Tk::StrL(_) => true,
                    _ => false,
                };
                if lit {
                    if tkv_semi(&t[i + 3].k) {
                        return true;
                    }
                }
            }
        }
    }
    false
}
fn e277_shapes_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 {
        return;
    }
    let mut i = 0;
    let mut k = 0;
    while k < t.len() {
        if word_is(&t[k], "__usrc__") {
            i = k + 1;
        }
        k = k + 1;
    }
    while i < t.len() {
        if e277_hit(t, i) {
            p.perr = t[i].lo + 1;
            p.perrexp = String::from("a type satisfying the required trait (E0277)");
            p.perrgot = String::from("the trait bound is not satisfied");
            return;
        }
        i = i + 1;
    }
}
// A2 ARRAY-LEN-EXPR (E0308): the length after `;` in an array repeat `[x; N]` or an array type `[T; N]` must be a usize
// constant. A closure (`|..|` / `||`), a range (`..10`), an array (`[]`) or a loop (`for` / `while` / `loop`) there is a
// type mismatch; it was accepted. User source only.
fn ale_bad_start(t: &Tok) -> bool {
    if tkv_bar(&t.k) {
        return true;
    }
    if tkv_or_or(&t.k) {
        return true;
    }
    if tkv_dot_dot(&t.k) {
        return true;
    }
    if tkv_l_brack(&t.k) {
        return true;
    }
    if tkv_for_t(&t.k) {
        return true;
    }
    if tkv_while(&t.k) {
        return true;
    }
    if tkv_loop_t(&t.k) {
        return true;
    }
    false
}
fn array_len_expr_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 {
        return;
    }
    let mut u = 0;
    let mut k = 0;
    while k < t.len() {
        if word_is(&t[k], "__usrc__") {
            u = k + 1;
        }
        k = k + 1;
    }
    let mut i = u;
    while i < t.len() {
        if tkv_l_brack(&t[i].k) {
            // the `;` at depth 1 of this bracket
            let mut q = i + 1;
            let mut d = 0;
            while q < t.len() {
                if tkv_l_brack(&t[q].k) || tkv_l_paren(&t[q].k) || tkv_l_brace(&t[q].k) {
                    d = d + 1;
                } else if tkv_r_brack(&t[q].k) || tkv_r_paren(&t[q].k) || tkv_r_brace(&t[q].k) {
                    if d == 0 {
                        break;
                    }
                    d = d - 1;
                } else if d == 0 && tkv_semi(&t[q].k) {
                    if q + 1 < t.len() {
                        if ale_bad_start(&t[q + 1]) {
                            p.perr = t[q + 1].lo + 1;
                            p.perrexp = String::from(
                                "a usize constant as the array length (mismatched types)",
                            );
                            p.perrgot = String::from("a closure / range / array / loop");
                            return;
                        }
                    }
                    break;
                }
                q = q + 1;
            }
        }
        i = i + 1;
    }
}
// A2 CLOSURE-FN-PTR (E0308): a closure that CAPTURES a local cannot coerce to a `fn` pointer -- `let f: fn(u8) -> u8 =
// |v| { a += v; a };` with `a` a `let` of the enclosing fn. (A non-capturing closure coerces fine.) User source only.
fn cfp_locals_before(t: &Vec<Tok>, i: usize) -> Vec<String> {
    // the `let` names between the enclosing `fn` and position i
    let mut v: Vec<String> = Vec::new();
    let mut q = i;
    while q > 0 {
        q = q - 1;
        if tkv_fn(&t[q].k) {
            break;
        }
    }
    while q < i {
        if tkv_let(&t[q].k) {
            let mut j = q + 1;
            if j < i {
                if tkv_mut(&t[j].k) {
                    j = j + 1;
                }
            }
            if j < i {
                let w = tok_word(&t[j]);
                if w.len() > 0 {
                    v.push(w);
                }
            }
        }
        q = q + 1;
    }
    v
}
fn cfp_hit(t: &Vec<Tok>, i: usize) -> bool {
    // t[i] = `let`: `let [mut] NAME : fn ( .. ) [-> T] = <closure>`
    let n = t.len();
    let mut j = i + 1;
    if j < n {
        if tkv_mut(&t[j].k) {
            j = j + 1;
        }
    }
    if j + 3 >= n {
        return false;
    }
    if !tkv_colon(&t[j + 1].k) {
        return false;
    }
    if !tkv_fn(&t[j + 2].k) {
        return false;
    }
    // the `=` at depth 0
    let mut q = j + 3;
    let mut d = 0;
    while q < n {
        if tkv_l_paren(&t[q].k) || tkv_lt(&t[q].k) || tkv_l_brack(&t[q].k) {
            d = d + 1;
        } else if tkv_r_paren(&t[q].k) || tkv_gt(&t[q].k) || tkv_r_brack(&t[q].k) {
            d = d - 1;
        } else if d <= 0 && tkv_eq(&t[q].k) {
            break;
        } else if d <= 0 && tkv_semi(&t[q].k) {
            return false;
        }
        q = q + 1;
    }
    if q + 1 >= n {
        return false;
    }
    // the closure's own parameters `| a, b: T |` (or `||`)
    let mut params: Vec<String> = Vec::new();
    let mut b = q + 1;
    if tkv_or_or(&t[b].k) {
        b = b + 1;
    } else if tkv_bar(&t[b].k) {
        b = b + 1;
        let mut expect = true;
        while b < n {
            if tkv_bar(&t[b].k) {
                b = b + 1;
                break;
            }
            if tkv_comma(&t[b].k) {
                expect = true;
            } else if expect {
                let w = tok_word(&t[b]);
                if w.len() > 0 {
                    params.push(w);
                    expect = false;
                }
            }
            b = b + 1;
        }
    } else {
        return false;
    }
    // the body up to the statement's `;` at depth 0: a mention of an enclosing `let` local (not a closure param)
    let locals = cfp_locals_before(t, i);
    let mut d = 0;
    while b < n {
        if tkv_l_paren(&t[b].k) || tkv_l_brace(&t[b].k) || tkv_l_brack(&t[b].k) {
            d = d + 1;
        } else if tkv_r_paren(&t[b].k) || tkv_r_brace(&t[b].k) || tkv_r_brack(&t[b].k) {
            d = d - 1;
        } else if d <= 0 && tkv_semi(&t[b].k) {
            return false;
        }
        let w = tok_word(&t[b]);
        if w.len() > 0 {
            if vs_has(&locals, &w) {
                if !vs_has(&params, &w) {
                    return true;
                }
            }
        }
        b = b + 1;
    }
    false
}
fn closure_fn_ptr_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 {
        return;
    }
    let mut i = 0;
    let mut k = 0;
    while k < t.len() {
        if word_is(&t[k], "__usrc__") {
            i = k + 1;
        }
        k = k + 1;
    }
    while i < t.len() {
        if tkv_let(&t[i].k) {
            if cfp_hit(t, i) {
                p.perr = t[i].lo + 1;
                p.perrexp = String::from(
                    "a non-capturing closure for a `fn` pointer (this closure captures a local)",
                );
                p.perrgot = String::from("mismatched types");
                return;
            }
        }
        i = i + 1;
    }
}
// A2 LET-REF-SHAPES (E0308): a `let` whose annotation and initializer disagree in shape -- `: &..` with a bare array
// initializer `[..]`, `: &mut ..` with a shared `&..` initializer, `: String` with a `Box::new(..)` initializer -- and an
// `assert_eq!` / `assert_ne!` of two suffixed literals of different widths (`assert_eq!(10u64, 10usize)`). User source only.
fn lrs_let_hit(t: &Vec<Tok>, i: usize) -> bool {
    let n = t.len();
    let mut j = i + 1;
    if j < n {
        if tkv_mut(&t[j].k) {
            j = j + 1;
        }
    }
    if j + 2 >= n {
        return false;
    }
    if !tkv_colon(&t[j + 1].k) {
        return false;
    }
    let a = j + 2; // the annotation's first token
                   // its `=` at depth 0
    let mut q = a;
    let mut d = 0;
    while q < n {
        if tkv_l_paren(&t[q].k) || tkv_lt(&t[q].k) || tkv_l_brack(&t[q].k) {
            d = d + 1;
        } else if tkv_r_paren(&t[q].k) || tkv_gt(&t[q].k) || tkv_r_brack(&t[q].k) {
            d = d - 1;
        } else if tkv_shr(&t[q].k) {
            d = d - 2;
        } else if d <= 0 && tkv_eq(&t[q].k) {
            break;
        } else if d <= 0 && tkv_semi(&t[q].k) {
            return false;
        } else if tkv_l_brace(&t[q].k) {
            return false;
        }
        q = q + 1;
    }
    if q + 1 >= n {
        return false;
    }
    let v = q + 1; // the initializer's first token
    let ann_ref = tkv_amp(&t[a].k);
    let ann_mut = ann_ref && a + 1 < n && tkv_mut(&t[a + 1].k);
    if ann_ref && !ann_mut {
        // a REAL `[` spans one char: a byte string `b"abc"` lexes to a synthetic `[..]`
        if tkv_l_brack(&t[v].k) && t[v].hi == t[v].lo + 1 {
            return true; // `: &[T] = [..]`
        }
    }
    if ann_mut {
        if tkv_amp(&t[v].k) {
            if v + 1 < n {
                if !tkv_mut(&t[v + 1].k) {
                    return true; // `: &mut T = &x`
                }
            }
        }
    }
    if word_is(&t[a], "String") {
        if a + 1 < n {
            if tkv_eq(&t[a + 1].k) {
                if v + 2 < n {
                    if word_is(&t[v], "Box") {
                        if tkv_path_sep(&t[v + 1].k) {
                            if word_is(&t[v + 2], "new") {
                                return true; // `: String = Box::new(..)`
                            }
                        }
                    }
                }
            }
        }
    }
    false
}
fn lrs_assert_hit(t: &Vec<Tok>, i: usize) -> bool {
    // `assert_eq ! ( <lit W1> , <lit W2> )`
    let w = tok_word(&t[i]);
    if w.as_str() != "assert_eq" {
        if w.as_str() != "assert_ne" {
            return false;
        }
    }
    let n = t.len();
    if i + 3 >= n {
        return false;
    }
    if !tkv_bang(&t[i + 1].k) {
        return false;
    }
    if !tkv_l_paren(&t[i + 2].k) {
        return false;
    }
    let w1 = ex_suflit(t, i + 3);
    if w1.len() == 0 {
        return false;
    }
    if i + 8 >= n {
        return false;
    }
    if !tkv_comma(&t[i + 8].k) {
        return false;
    }
    let w2 = ex_suflit(t, i + 9);
    if w2.len() == 0 {
        return false;
    }
    w1 != w2
}
fn let_ref_shapes_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 {
        return;
    }
    let mut i = 0;
    let mut k = 0;
    while k < t.len() {
        if word_is(&t[k], "__usrc__") {
            i = k + 1;
        }
        k = k + 1;
    }
    while i < t.len() {
        let mut hit = false;
        if tkv_let(&t[i].k) {
            hit = lrs_let_hit(t, i);
        }
        if !hit {
            hit = lrs_assert_hit(t, i);
        }
        if hit {
            p.perr = t[i].lo + 1;
            p.perrexp = String::from("an initializer of the annotated shape (mismatched types)");
            p.perrgot = String::from("mismatched types");
            return;
        }
        i = i + 1;
    }
}
// A2 SHAPES-3: (1) a primitive annotation with an array initializer (`let x: i32 = [1, 2];`, E0308); (2) a bare `return;` in
// a fn declared to return a concrete type (E0069); (3) a `loop` / `while` body whose final expression is a non-unit literal
// (`loop { true }`, E0308); (4) `impl S for T` / a bound `T: S` where S is a STRUCT or ENUM, not a trait (E0404).
fn s3_lit(t: &Tok) -> bool {
    match t.k {
        Tk::Num(_) => true,
        Tk::StrL(_) => true,
        Tk::ChL(_) => true,
        Tk::TrueT => true,
        Tk::FalseT => true,
        _ => false,
    }
}
fn s3_prim(w: &str) -> bool {
    oast::is_int_width(w) || w == "bool" || w == "char" || w == "f32" || w == "f64"
}
fn s3_concrete_ret(t: &Vec<Tok>, q: usize) -> bool {
    // the enclosing fn of position q returns a concrete primitive / String / Vec / Option type
    let mut f = q;
    while f > 0 {
        f = f - 1;
        if tkv_fn(&t[f].k) {
            break;
        }
    }
    // the parameter list's closing `)`: the return type is the `->` right after it (a `-> bool` inside a parameter type
    // like `impl Fn() -> bool` is not the fn's)
    let mut r = f;
    while r < q {
        if tkv_l_paren(&t[r].k) {
            break;
        }
        r = r + 1;
    }
    let mut d = 0;
    while r < q {
        if tkv_l_paren(&t[r].k) {
            d = d + 1;
        } else if tkv_r_paren(&t[r].k) {
            d = d - 1;
            if d == 0 {
                break;
            }
        }
        r = r + 1;
    }
    if r + 3 >= q {
        return false;
    }
    if !tkv_minus(&t[r + 1].k) {
        return false;
    }
    if !tkv_gt(&t[r + 2].k) {
        return false;
    }
    let w = tok_word(&t[r + 3]);
    s3_prim(w.as_str()) || w.as_str() == "String"
}
fn s3_hit(t: &Vec<Tok>, i: usize, structs: &Vec<String>) -> bool {
    let n = t.len();
    // (1) `let [mut] x : PRIM = [`
    if tkv_let(&t[i].k) {
        let mut j = i + 1;
        if j < n {
            if tkv_mut(&t[j].k) {
                j = j + 1;
            }
        }
        if j + 4 < n {
            if tkv_colon(&t[j + 1].k) {
                if s3_prim(tok_word(&t[j + 2]).as_str()) {
                    if tkv_eq(&t[j + 3].k) {
                        if tkv_l_brack(&t[j + 4].k) {
                            if t[j + 4].hi == t[j + 4].lo + 1 {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }
    // (2) `return ;` in a fn returning a concrete type (not inside a closure: the nearest `fn` / `|` decides)
    if tkv_ret(&t[i].k) {
        if i + 1 < n {
            if tkv_semi(&t[i + 1].k) {
                if s3_concrete_ret(t, i) {
                    return true;
                }
            }
        }
    }
    // (3) `loop {` / `while .. {` ... `<literal> }` as the body's last expression
    if tkv_loop_t(&t[i].k) {
        if i + 3 < n {
            if tkv_l_brace(&t[i + 1].k) {
                if s3_lit(&t[i + 2]) {
                    if tkv_r_brace(&t[i + 3].k) {
                        return true;
                    }
                }
            }
        }
    }
    // (a `while { cond } {` block condition is skipped: its first `{` is not the body)
    if tkv_while(&t[i].k) && !(i + 1 < n && tkv_l_brace(&t[i + 1].k)) {
        // the body `{` at depth 0 after the condition, then `<literal> }`
        let mut q = i + 1;
        let mut d = 0;
        while q < n {
            if tkv_l_paren(&t[q].k) || tkv_l_brack(&t[q].k) {
                d = d + 1;
            } else if tkv_r_paren(&t[q].k) || tkv_r_brack(&t[q].k) {
                d = d - 1;
            } else if d == 0 && tkv_l_brace(&t[q].k) {
                break;
            }
            q = q + 1;
        }
        if q + 2 < n {
            if s3_lit(&t[q + 1]) {
                if tkv_r_brace(&t[q + 2].k) {
                    return true;
                }
            }
        }
    }
    // (4) `impl S for` / `: S` bound where S is a struct or enum
    if tkv_impl(&t[i].k) {
        if i + 2 < n {
            let w = tok_word(&t[i + 1]);
            if vs_has(structs, &w) {
                if tkv_for_t(&t[i + 2].k) {
                    return true;
                }
            }
        }
    }
    false
}
fn shapes3_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 {
        return;
    }
    let mut u = 0;
    let mut k = 0;
    while k < t.len() {
        if word_is(&t[k], "__usrc__") {
            u = k + 1;
        }
        k = k + 1;
    }
    // user struct / enum names (not traits)
    let mut structs: Vec<String> = Vec::new();
    let mut i = u;
    while i + 1 < t.len() {
        if tkv_struct(&t[i].k) || tkv_enum(&t[i].k) {
            let w = tok_word(&t[i + 1]);
            if w.len() > 0 {
                structs.push(w);
            }
        }
        i = i + 1;
    }
    // a name that is ALSO declared as a trait (another module) is not judged
    let mut i = u;
    while i + 1 < t.len() {
        if tkv_trait(&t[i].k) {
            let w = tok_word(&t[i + 1]);
            let mut k = 0;
            while k < structs.len() {
                if structs[k].as_str() == w.as_str() {
                    structs.remove(k);
                } else {
                    k = k + 1;
                }
            }
        }
        i = i + 1;
    }
    let mut i = u;
    while i < t.len() {
        if s3_hit(t, i, &structs) {
            p.perr = t[i].lo + 1;
            p.perrexp = String::from("a well-typed form (mismatched types / expected a trait)");
            p.perrgot = String::from("type error");
            return;
        }
        i = i + 1;
    }
}
// A2 ASSOC-DYN-VARIANT: (1) `dyn Trait` with no `<..>` binding where the user trait declares an associated `type X;`
// (E0191: the value of the associated type must be specified); (2) an enum VARIANT used as a type, `x: Enum::Variant`
// (E0573). User source only.
fn adv_trait_has_assoc(t: &Vec<Tok>, nm: &String, from: usize) -> bool {
    // a user `trait NM .. { .. type X ; .. }` (a plain associated type, no default)
    let n = t.len();
    let mut i = from;
    while i + 1 < n {
        if tkv_trait(&t[i].k) {
            if word_is(&t[i + 1], &*nm.as_str()) {
                let mut j = i + 2;
                while j < n {
                    if tkv_l_brace(&t[j].k) {
                        break;
                    }
                    j = j + 1;
                }
                let mut q = j + 1;
                let mut d = 1;
                while q + 2 < n {
                    if tkv_l_brace(&t[q].k) {
                        d = d + 1;
                    } else if tkv_r_brace(&t[q].k) {
                        d = d - 1;
                        if d == 0 {
                            return false;
                        }
                    } else if d == 1 {
                        if tkv_type_t(&t[q].k) || word_is(&t[q], "type") {
                            if tkv_semi(&t[q + 2].k) {
                                return true;
                            }
                        }
                    }
                    q = q + 1;
                }
                return false;
            }
        }
        i = i + 1;
    }
    false
}
fn adv_enum_has_variant(t: &Vec<Tok>, en: &String, v: &String, from: usize) -> bool {
    let n = t.len();
    let mut i = from;
    while i + 2 < n {
        if tkv_enum(&t[i].k) {
            if word_is(&t[i + 1], &*en.as_str()) {
                let mut j = i + 2;
                while j < n {
                    if tkv_l_brace(&t[j].k) {
                        break;
                    }
                    j = j + 1;
                }
                let mut q = j + 1;
                let mut d = 1;
                while q < n {
                    if tkv_l_brace(&t[q].k) || tkv_l_paren(&t[q].k) {
                        d = d + 1;
                    } else if tkv_r_brace(&t[q].k) || tkv_r_paren(&t[q].k) {
                        d = d - 1;
                        if d == 0 {
                            return false;
                        }
                    } else if d == 1 {
                        if word_is(&t[q], &*v.as_str()) {
                            return true;
                        }
                    }
                    q = q + 1;
                }
                return false;
            }
        }
        i = i + 1;
    }
    false
}
fn assoc_dyn_variant_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 {
        return;
    }
    let mut u = 0;
    let mut k = 0;
    while k < t.len() {
        if word_is(&t[k], "__usrc__") {
            u = k + 1;
        }
        k = k + 1;
    }
    let n = t.len();
    let mut i = u;
    while i + 3 < n {
        // (1) `dyn NAME` not followed by `<`
        if tkv_dyn_t(&t[i].k) || word_is(&t[i], "dyn") {
            let w = tok_word(&t[i + 1]);
            if w.len() > 0 {
                if !tkv_lt(&t[i + 2].k) {
                    if adv_trait_has_assoc(t, &w, u) {
                        p.perr = t[i].lo + 1;
                        p.perrexp = String::from("`dyn Trait<Assoc = ..>` (the associated type must be specified: E0191)");
                        p.perrgot = w;
                        return;
                    }
                }
            }
        }
        // (2) `: Enum :: Variant` followed by `,` / `)` (a parameter type)
        if tkv_colon(&t[i].k) {
            if i + 4 < n {
                if tkv_path_sep(&t[i + 2].k) {
                    if tkv_comma(&t[i + 4].k) || tkv_r_paren(&t[i + 4].k) {
                        let en = tok_word(&t[i + 1]);
                        let v = tok_word(&t[i + 3]);
                        if en.len() > 0 && v.len() > 0 {
                            // the cheap parameter-list test FIRST: the enum scan allocates per token (ourc's arena never frees), and
                            // this shape is in every struct literal `k: Tk::Num(..),` -- run first, it OOM-killed ourc on its own source
                            if us_is_fn_params(t, us_open_paren(t, i))
                                && adv_enum_has_variant(t, &en, &v, u)
                            {
                                p.perr = t[i + 1].lo + 1;
                                p.perrexp =
                                    String::from("a type (an enum variant is not a type: E0573)");
                                p.perrgot = v;
                                return;
                            }
                        }
                    }
                }
            }
        }
        i = i + 1;
    }
}
// A2 BOUND-DERIVE (E0277): a call `f(S)` / `f(S { .. })` / `f(S(..))` to a user fn `f<T: Tr>(x: T)` where S is a user
// struct / enum that neither derives Tr (judged for the traits the lexer marks: PartialEq, Copy, Ord) nor has an
// `impl Tr for S`. User source only.
fn bd_marker(tr: &str) -> String {
    if tr == "PartialEq" {
        return String::from("__derive_peq__");
    }
    if tr == "Ord" {
        return String::from("__derive_ord__");
    }
    if tr == "Copy" {
        return String::from("#copy");
    }
    String::new()
}
fn bd_has_trait(t: &Vec<Tok>, s: &String, tr: &str, from: usize) -> bool {
    // a derive marker right before `struct S` / `enum S`, or an `impl Tr for S`
    let n = t.len();
    let mk = bd_marker(tr);
    let mut i = from;
    while i + 3 < n {
        if tkv_impl(&t[i].k) {
            if word_is(&t[i + 1], &*tr) {
                if tkv_for_t(&t[i + 2].k) {
                    if word_is(&t[i + 3], &*s.as_str()) {
                        return true;
                    }
                }
            }
        }
        if tkv_struct(&t[i].k) || tkv_enum(&t[i].k) {
            if word_is(&t[i + 1], &*s.as_str()) {
                let mut b = i;
                let mut steps = 0;
                while b > 0 && steps < 8 {
                    b = b - 1;
                    steps = steps + 1;
                    if tkv_semi(&t[b].k) || tkv_r_brace(&t[b].k) {
                        break;
                    }
                    if mk.as_str() == "#copy" {
                        if tkv_derive_copy(&t[b].k) {
                            return true;
                        }
                    } else if word_is(&t[b], &*mk.as_str()) {
                        return true;
                    }
                }
            }
        }
        i = i + 1;
    }
    false
}
fn bd_is_type(t: &Vec<Tok>, s: &String, from: usize) -> bool {
    let n = t.len();
    let mut i = from;
    while i + 1 < n {
        if tkv_struct(&t[i].k) || tkv_enum(&t[i].k) {
            if word_is(&t[i + 1], &*s.as_str()) {
                return true;
            }
        }
        i = i + 1;
    }
    false
}
fn bound_derive_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 {
        return;
    }
    let mut u = 0;
    let mut k = 0;
    while k < t.len() {
        if word_is(&t[k], "__usrc__") {
            u = k + 1;
        }
        k = k + 1;
    }
    let n = t.len();
    // generic fns `fn F < T : Tr > ( a : T )` -- one type param with one marked bound, a single parameter of type T
    let mut i = u;
    while i + 10 < n {
        if tkv_fn(&t[i].k) {
            let fname = tok_word(&t[i + 1]);
            if fname.len() > 0
                && tkv_lt(&t[i + 2].k)
                && tkv_colon(&t[i + 4].k)
                && tkv_gt(&t[i + 6].k)
            {
                let tp = tok_word(&t[i + 3]);
                let tr = tok_word(&t[i + 5]);
                if tp.len() > 0 && bd_marker(tr.as_str()).len() > 0 && tkv_l_paren(&t[i + 7].k) {
                    // the parameter `NAME : T )` (single parameter)
                    let ok = i + 11 < n
                        && tkv_colon(&t[i + 9].k)
                        && word_is(&t[i + 10], &*tp.as_str())
                        && tkv_r_paren(&t[i + 11].k);
                    if ok {
                        // each call `F ( S` in the user source
                        let mut c = u;
                        while c + 2 < n {
                            if word_is(&t[c], &*fname.as_str()) && tkv_l_paren(&t[c + 1].k) {
                                let prev_fn = c > 0 && tkv_fn(&t[c - 1].k);
                                if !prev_fn {
                                    let s = tok_word(&t[c + 2]);
                                    if s.len() > 0
                                        && bd_is_type(t, &s, u)
                                        && !bd_has_trait(t, &s, tr.as_str(), u)
                                    {
                                        p.perr = t[c].lo + 1;
                                        p.perrexp = String::from(
                                            "an argument whose type implements the bound (E0277)",
                                        );
                                        p.perrgot = s;
                                        return;
                                    }
                                }
                            }
                            c = c + 1;
                        }
                    }
                }
            }
        }
        i = i + 1;
    }
}
// SELF-HOST helpers for the `A|B|C` keys of the exhaustiveness check: ourc's own std layer has no str::find / rfind /
// split / parse, so ourc could not compile itself once R9-EXH used them (`no method rfind on str`).
fn ex_bar_count(s: &str) -> usize {
    let b = s.as_bytes();
    let mut c = 0;
    let mut i = 0;
    while i < b.len() {
        if b[i] == 124 {
            c = c + 1;
        }
        i = i + 1;
    }
    c
}
fn ex_bar_last(s: &str) -> usize {
    // the byte index of the last `|` (0 if none)
    let b = s.as_bytes();
    let mut r = 0;
    let mut i = 0;
    while i < b.len() {
        if b[i] == 124 {
            r = i;
        }
        i = i + 1;
    }
    r
}
fn ex_bar_field(s: &str, k: usize) -> String {
    // the k-th `|`-separated field (empty if absent)
    let b = s.as_bytes();
    let mut f = 0;
    let mut o = String::new();
    let mut i = 0;
    while i < b.len() {
        if b[i] == 124 {
            if f == k {
                return o;
            }
            f = f + 1;
            o = String::new();
        } else if f == k {
            o.push(b[i] as char);
        }
        i = i + 1;
    }
    if f == k {
        o
    } else {
        String::new()
    }
}
fn ex_usz(s: &str) -> usize {
    let b = s.as_bytes();
    let mut v: usize = 0;
    let mut i = 0;
    while i < b.len() {
        if b[i] >= 48 && b[i] <= 57 {
            v = v * 10 + (b[i] - 48) as usize;
        } else {
            return v;
        }
        i = i + 1;
    }
    v
}
// SEMI-TAIL-UNIT: a statement that never completes keeps its value as a block tail (`{ return x; }` is not `()`)
fn tail_diverges(e: &E) -> bool {
    match e {
        E::RetE(_a, _l, _h) => true,
        E::BreakE(_l, _h) => true,
        E::BreakN(_n, _l, _h) => true,
        E::ContE(_l, _h) => true,
        E::ContN(_n, _l, _h) => true,
        E::Call(n, _a, _l, _h) => {
            n.contains("panic")
                || n.contains("exit")
                || n.contains("unreachable")
                || n.contains("todo")
        }
        E::Seq(_a, b, _l, _h) => tail_diverges(b),
        _ => false,
    }
}
// SELF-DIFF: does some fn name start with `<path>$` (i.e. is `path` a module)? Byte loop with early returns: SEM0-safe.
fn fnames_has_prefix(fnames: &Vec<String>, path: &str) -> bool {
    let pb = path.as_bytes();
    let mut i = 0;
    while i < fnames.len() {
        let fb = fnames[i].as_bytes();
        if fb.len() > pb.len() {
            if fb[pb.len()] == 36 {
                let mut j = 0;
                let mut same = true;
                while j < pb.len() {
                    if fb[j] != pb[j] {
                        same = false;
                        break;
                    }
                    j = j + 1;
                }
                if same {
                    return true;
                }
            }
        }
        i = i + 1;
    }
    false
}
// A2 ENUM-DISCR-WIDTH (E0308): a SUFFIXED enum discriminant must have the enum's discriminant type -- its `#[repr(<int>)]`
// width (the lexer's `__repr_<w>__` marker just before `enum`), or isize without one: `enum Foo { A = 1i64, B = 2u8 }`
// is refused. User source only. (Reverted once for lack of repr-awareness: `#[repr(u8)] enum B { P = 2u8 }` is valid.)
fn edw_repr(t: &Vec<Tok>, i: usize) -> String {
    let mut b = i;
    let mut steps = 0;
    while b > 0 && steps < 10 {
        b = b - 1;
        steps = steps + 1;
        if tkv_semi(&t[b].k) || tkv_r_brace(&t[b].k) {
            break;
        }
        let w = tok_word(&t[b]);
        if w.starts_with("__repr_") && w.as_str() != "__repr_c__" {
            let wb = w.as_bytes();
            let mut o = String::new();
            let mut k = 7;
            while k + 2 < wb.len() {
                o.push(wb[k] as char);
                k = k + 1;
            }
            return o;
        }
    }
    String::from("isize")
}
fn edw_hit(t: &Vec<Tok>, i: usize) -> bool {
    let n = t.len();
    let want = edw_repr(t, i);
    if want.as_str() == "any" {
        return false;
    }
    let mut j = i + 2;
    while j < n {
        if tkv_l_brace(&t[j].k) {
            break;
        }
        if tkv_semi(&t[j].k) {
            return false;
        }
        j = j + 1;
    }
    let mut q = j + 1;
    let mut d = 1;
    while q < n {
        if tkv_l_brace(&t[q].k) || tkv_l_paren(&t[q].k) {
            d = d + 1;
        } else if tkv_r_brace(&t[q].k) || tkv_r_paren(&t[q].k) {
            d = d - 1;
            if d == 0 {
                return false;
            }
        } else if d == 1 {
            if tkv_eq(&t[q].k) {
                let w = ex_suflit(t, q + 1);
                if w.len() > 0 {
                    if w.as_str() != want.as_str() {
                        return true;
                    }
                }
            }
        }
        q = q + 1;
    }
    false
}
fn enum_discr_width_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 {
        return;
    }
    let mut i = 0;
    let mut k = 0;
    while k < t.len() {
        if word_is(&t[k], "__usrc__") {
            i = k + 1;
        }
        k = k + 1;
    }
    while i + 2 < t.len() {
        if tkv_enum(&t[i].k) {
            if edw_hit(t, i) {
                p.perr = t[i].lo + 1;
                p.perrexp = String::from(
                    "discriminants of the enum's discriminant type (mismatched types)",
                );
                p.perrgot = tok_word(&t[i + 1]);
                return;
            }
        }
        i = i + 1;
    }
}
// A2 BOUND-NOT-TRAIT: a generic bound `<T: X>` where X is not a trait in scope -- a primitive type (`N: u32`, E0404), a user
// struct / enum (E0404), or a non-prelude std trait used bare without a `use` (`T: Debug` with no `use std::fmt::Debug`,
// E0405). Only the bound right after `T:` inside a generic list is judged. User source only.
fn bnt_prim(w: &str) -> bool {
    oast::is_int_width(w)
        || w == "bool"
        || w == "char"
        || w == "f32"
        || w == "f64"
        || w == "str"
        || w == "String"
}
fn bnt_nonprelude(w: &str) -> bool {
    w == "Debug"
        || w == "Display"
        || w == "Hash"
        || w == "Read"
        || w == "Write"
        || w == "Deref"
        || w == "DerefMut"
        || w == "BufRead"
        || w == "FromStr"
        || w == "Add"
        || w == "Sub"
        || w == "Mul"
        || w == "Div"
        || w == "Neg"
}
fn bnt_imported(t: &Vec<Tok>, u: usize, w: &String) -> bool {
    // `use .. w ..;` anywhere, or a trait / type of that name declared in the user source
    let n = t.len();
    let mut i = u;
    while i < n {
        if tkv_use_t(&t[i].k) || word_is(&t[i], "use") {
            let mut q = i + 1;
            while q < n {
                if tkv_semi(&t[q].k) {
                    break;
                }
                if word_is(&t[q], &*w.as_str()) {
                    return true;
                }
                if word_is(&t[q], "*") || tkv_star(&t[q].k) {
                    return true; // a glob may bring it
                }
                q = q + 1;
            }
        }
        if tkv_trait(&t[i].k) && i + 1 < n {
            if word_is(&t[i + 1], &*w.as_str()) {
                return true;
            }
        }
        i = i + 1;
    }
    false
}
fn bound_not_trait_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 {
        return;
    }
    let mut u = 0;
    let mut k = 0;
    while k < t.len() {
        if word_is(&t[k], "__usrc__") {
            u = k + 1;
        }
        k = k + 1;
    }
    let n = t.len();
    let mut structs: Vec<String> = Vec::new();
    let mut i = u;
    while i + 1 < n {
        if tkv_struct(&t[i].k) || tkv_enum(&t[i].k) {
            let w = tok_word(&t[i + 1]);
            if w.len() > 0 {
                structs.push(w);
            }
        }
        i = i + 1;
    }
    // a name that is also a trait is not judged
    let mut i = u;
    while i + 1 < n {
        if tkv_trait(&t[i].k) {
            let w = tok_word(&t[i + 1]);
            let mut k = 0;
            while k < structs.len() {
                if structs[k].as_str() == w.as_str() {
                    structs.remove(k);
                } else {
                    k = k + 1;
                }
            }
        }
        i = i + 1;
    }
    let mut i = u;
    while i + 4 < n {
        // `< T : X` / `, T : X` followed by `>` `,` `+` -- a generic parameter's first bound (not `const N: u8`)
        let lead = tkv_lt(&t[i].k) || tkv_comma(&t[i].k);
        if lead && is_ident(&t[i + 1]) && tkv_colon(&t[i + 2].k) {
            let is_const = i >= 1 && (tkv_const_t(&t[i].k) || word_is(&t[i], "const"));
            let x = tok_word(&t[i + 3]);
            let after = &t[i + 4];
            let ends = tkv_gt(&after.k) || tkv_comma(&after.k) || tkv_plus(&after.k);
            if !is_const
                && x.len() > 0
                && ends
                && ((tkv_lt(&t[i].k) && bnt_is_def_lt(t, i)) || bnt_in_generics(t, i))
            {
                let mut bad = bnt_prim(x.as_str()) || vs_has(&structs, &x);
                if !bad && bnt_nonprelude(x.as_str()) {
                    bad = !bnt_imported(t, u, &x);
                }
                if bad {
                    // inside a generic list? the nearest unmatched `<` before i
                    p.perr = t[i + 3].lo + 1;
                    p.perrexp = String::from("a trait in scope as the bound (E0404 / E0405)");
                    p.perrgot = x;
                    return;
                }
            }
        }
        i = i + 1;
    }
}
// is position i (a `,`) inside a `<..>` generic list? walk back to the nearest unmatched `<` without crossing `(` `{` `;`
fn bnt_in_generics(t: &Vec<Tok>, i: usize) -> bool {
    let mut q = i;
    let mut ad = 0;
    while q > 0 {
        q = q - 1;
        if tkv_gt(&t[q].k) {
            ad = ad + 1;
        } else if tkv_lt(&t[q].k) {
            if ad == 0 {
                // a GENERIC list only when it opens a definition (`fn f<`, `struct S<`, `enum E<`, `trait T<`, `type A<`,
                // `impl<`) -- a bare `<` is a less-than (`if i < n { |a, b: u32| .. }` refused ourc's own source)
                return bnt_is_def_lt(t, q);
            }
            ad = ad - 1;
        } else if tkv_l_paren(&t[q].k)
            || tkv_l_brace(&t[q].k)
            || tkv_semi(&t[q].k)
            || tkv_r_paren(&t[q].k)
            || tkv_r_brace(&t[q].k)
        {
            return false;
        }
    }
    false
}
// A2 ITEM-TYPE-SHAPES: (1) a `const` / `static` item with no type annotation (`const X = [];` -- rustc: missing type for
// `const` item); (2) a const generic parameter used as a TYPE (`struct S<const C: u8>(C);`, E0573). User source only.
fn its_hit(t: &Vec<Tok>, i: usize) -> bool {
    let n = t.len();
    // (1) `const NAME =` / `static [mut] NAME =` (an item, not `const fn` / `<const N` / a const generic arg)
    if tkv_const_t(&t[i].k) || tkv_static_t(&t[i].k) {
        let mut prev_ok = true;
        if i > 0 {
            if tkv_lt(&t[i - 1].k)
                || tkv_comma(&t[i - 1].k)
                || tkv_star(&t[i - 1].k)
                || tkv_amp(&t[i - 1].k)
            {
                prev_ok = false;
            }
        }
        if prev_ok {
            let mut j = i + 1;
            if j < n {
                if tkv_mut(&t[j].k) {
                    j = j + 1;
                }
            }
            if j + 1 < n {
                if is_ident(&t[j]) {
                    if tkv_eq(&t[j + 1].k) {
                        return true;
                    }
                }
            }
        }
    }
    // (2) `struct NAME < const C : T > ( C` / `( .. , C` -- the const param as a tuple field type
    if tkv_struct(&t[i].k) {
        if i + 8 < n {
            if tkv_lt(&t[i + 2].k) {
                if tkv_const_t(&t[i + 3].k) || word_is(&t[i + 3], "const") {
                    let c = tok_word(&t[i + 4]);
                    if c.len() > 0
                        && tkv_colon(&t[i + 5].k)
                        && tkv_gt(&t[i + 7].k)
                        && tkv_l_paren(&t[i + 8].k)
                    {
                        let mut q = i + 9;
                        while q < n {
                            if tkv_r_paren(&t[q].k) {
                                break;
                            }
                            if word_is(&t[q], &*c.as_str()) {
                                let a = &t[q - 1];
                                if tkv_l_paren(&a.k) || tkv_comma(&a.k) {
                                    return true;
                                }
                            }
                            q = q + 1;
                        }
                    }
                }
            }
        }
    }
    false
}
fn item_type_shapes_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 {
        return;
    }
    let mut i = 0;
    let mut k = 0;
    while k < t.len() {
        if word_is(&t[k], "__usrc__") {
            i = k + 1;
        }
        k = k + 1;
    }
    while i < t.len() {
        if its_hit(t, i) {
            p.perr = t[i].lo + 1;
            p.perrexp = String::from("an item type (a const / static needs one; a const parameter is a value, not a type)");
            p.perrgot = String::from("missing or non-type");
            return;
        }
        i = i + 1;
    }
}
// is the `<` at q the opening of a definition's generic list?
fn bnt_is_def_lt(t: &Vec<Tok>, q: usize) -> bool {
    if q >= 1 {
        if tkv_impl(&t[q - 1].k) {
            return true;
        }
    }
    if q >= 2 {
        if is_ident(&t[q - 1]) {
            let k = &t[q - 2].k;
            if tkv_fn(k) || tkv_struct(k) || tkv_enum(k) || tkv_trait(k) || tkv_type_t(k) {
                return true;
            }
        }
    }
    false
}
// A2 ASSOC-NAME (E0220): an associated-type binding `Trait<NAME = T>` whose NAME the trait does not have -- for the std
// traits with a fixed associated-type set (`Deref<Trget = i32>`: Deref has only Target), and for user traits (the
// `type X` names their body declares). Unknown traits are not judged. User source only.
fn an_std_ok(tr: &str, nm: &str) -> i32 {
    // 1 = has it, 0 = does not, -1 = not a modelled trait
    if tr == "Deref" || tr == "DerefMut" {
        return if nm == "Target" { 1 } else { 0 };
    }
    if tr == "Iterator" || tr == "DoubleEndedIterator" || tr == "ExactSizeIterator" {
        return if nm == "Item" { 1 } else { 0 };
    }
    if tr == "IntoIterator" {
        return if nm == "Item" || nm == "IntoIter" {
            1
        } else {
            0
        };
    }
    if tr == "Add"
        || tr == "Sub"
        || tr == "Mul"
        || tr == "Div"
        || tr == "Rem"
        || tr == "Neg"
        || tr == "Not"
    {
        return if nm == "Output" { 1 } else { 0 };
    }
    if tr == "Fn" || tr == "FnMut" || tr == "FnOnce" {
        return if nm == "Output" { 1 } else { 0 };
    }
    if tr == "FromStr" {
        return if nm == "Err" { 1 } else { 0 };
    }
    -1
}
fn an_user_ok(t: &Vec<Tok>, u: usize, tr: &String, nm: &String) -> i32 {
    let n = t.len();
    let mut i = u;
    while i + 1 < n {
        if tkv_trait(&t[i].k) {
            if word_is(&t[i + 1], &*tr.as_str()) {
                let mut j = i + 2;
                while j < n {
                    if tkv_l_brace(&t[j].k) {
                        break;
                    }
                    j = j + 1;
                }
                // supertraits may contribute names: a trait with any `:` supertrait is not judged
                let mut h = i + 2;
                while h < j {
                    if tkv_colon(&t[h].k) {
                        return -1;
                    }
                    h = h + 1;
                }
                let mut q = j + 1;
                let mut d = 1;
                while q + 1 < n {
                    if tkv_l_brace(&t[q].k) {
                        d = d + 1;
                    } else if tkv_r_brace(&t[q].k) {
                        d = d - 1;
                        if d == 0 {
                            return 0;
                        }
                    } else if d == 1 {
                        if tkv_type_t(&t[q].k) || word_is(&t[q], "type") {
                            if word_is(&t[q + 1], &*nm.as_str()) {
                                return 1;
                            }
                        }
                    }
                    q = q + 1;
                }
                return 0;
            }
        }
        i = i + 1;
    }
    -1
}
fn assoc_name_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 {
        return;
    }
    let mut u = 0;
    let mut k = 0;
    while k < t.len() {
        if word_is(&t[k], "__usrc__") {
            u = k + 1;
        }
        k = k + 1;
    }
    let n = t.len();
    let mut i = u;
    while i + 3 < n {
        // `TRAIT < NAME =` (not `==`)
        if tkv_lt(&t[i + 1].k) {
            if tkv_eq(&t[i + 3].k) {
                let tr = tok_word(&t[i]);
                let nm = tok_word(&t[i + 2]);
                // a DEFINITION's default type parameter (`trait Mul<Rhs = Self>`, `struct S<T = u8>`) is not a binding
                let mut is_def = false;
                if i >= 1 {
                    let k = &t[i - 1].k;
                    if tkv_trait(k)
                        || tkv_struct(k)
                        || tkv_enum(k)
                        || tkv_type_t(k)
                        || tkv_fn(k)
                        || tkv_impl(k)
                    {
                        is_def = true;
                    }
                }
                if tr.len() > 0 && nm.len() > 0 && !is_def {
                    // a user trait of the same name shadows the std one: its declaration decides
                    let mut r = an_user_ok(t, u, &tr, &nm);
                    if r == -1 && !an_user_declares(t, u, &tr) {
                        r = an_std_ok(tr.as_str(), nm.as_str());
                    }
                    if r == 0 {
                        p.perr = t[i + 2].lo + 1;
                        p.perrexp = String::from("an associated type of the trait (E0220)");
                        p.perrgot = nm;
                        return;
                    }
                }
            }
        }
        i = i + 1;
    }
}
// ASSOC-NAME: does the user source declare a trait of this name (then the std table does not apply)?
fn an_user_declares(t: &Vec<Tok>, u: usize, tr: &String) -> bool {
    let mut i = u;
    while i + 1 < t.len() {
        if tkv_trait(&t[i].k) {
            if word_is(&t[i + 1], &*tr.as_str()) {
                return true;
            }
        }
        i = i + 1;
    }
    false
}
// A2 BOUND-SHAPES-2: (1) a bound that names a TYPE PARAMETER of the same generic list (`fn f<T: Read, U, Read>()`: `Read`
// is a type parameter, not the trait -- E0404); (2) a user generic struct `S<I: Tr>` instantiated with a PRIMITIVE that
// cannot satisfy Tr (`struct T(S<u8>);` with Tr = Iterator, or a user trait with no `impl Tr for u8` -- E0277). User source only.
fn bs2_list_params(t: &Vec<Tok>, lt: usize) -> Vec<String> {
    // the parameter names of the generic list opening at `<` lt (depth-1 words after `<` / `,`)
    let mut v: Vec<String> = Vec::new();
    let n = t.len();
    let mut q = lt + 1;
    let mut d = 1;
    let mut expect = true;
    while q < n && d > 0 {
        if tkv_lt(&t[q].k) {
            d = d + 1;
        } else if tkv_shr(&t[q].k) {
            d = d - 2;
        } else if tkv_gt(&t[q].k) {
            d = d - 1;
        } else if d == 1 && tkv_comma(&t[q].k) {
            expect = true;
        } else if d == 1 && expect {
            let w = tok_word(&t[q]);
            if w.len() > 0 && w.as_str() != "const" {
                v.push(w);
            }
            expect = false;
        }
        q = q + 1;
    }
    v
}
fn bs2_prim_impls(t: &Vec<Tok>, u: usize, tr: &String, prim: &String) -> bool {
    // does the user source `impl Tr for prim`?
    let n = t.len();
    let mut i = u;
    while i + 3 < n {
        if tkv_impl(&t[i].k) {
            if word_is(&t[i + 1], &*tr.as_str()) {
                if tkv_for_t(&t[i + 2].k) {
                    if word_is(&t[i + 3], &*prim.as_str()) {
                        return true;
                    }
                }
            }
        }
        i = i + 1;
    }
    false
}
fn bound_shapes2_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 {
        return;
    }
    let mut u = 0;
    let mut k = 0;
    while k < t.len() {
        if word_is(&t[k], "__usrc__") {
            u = k + 1;
        }
        k = k + 1;
    }
    let n = t.len();
    // (1) in a definition's generic list, `P : X` where X is another parameter of the same list
    let mut i = u;
    while i + 1 < n {
        if tkv_lt(&t[i].k) && bnt_is_def_lt(t, i) {
            let ps = bs2_list_params(t, i);
            let mut q = i + 1;
            let mut d = 1;
            while q + 1 < n && d > 0 {
                if tkv_lt(&t[q].k) {
                    d = d + 1;
                } else if tkv_shr(&t[q].k) {
                    d = d - 2; // `HasLife<'a>>` -- lifetimes dropped, the closers are ONE `>>`
                } else if tkv_gt(&t[q].k) {
                    d = d - 1;
                } else if d == 1 && tkv_colon(&t[q].k) {
                    let x = tok_word(&t[q + 1]);
                    if x.len() > 0 && vs_has(&ps, &x) {
                        p.perr = t[q + 1].lo + 1;
                        p.perrexp = String::from(
                            "a trait as the bound (this names a type parameter: E0404)",
                        );
                        p.perrgot = x;
                        return;
                    }
                }
                q = q + 1;
            }
        }
        i = i + 1;
    }
    // (2) user `struct S < I : Tr >` (one bounded param), then a use `S < prim >` with prim unable to implement Tr
    let mut i = u;
    while i + 6 < n {
        if tkv_struct(&t[i].k)
            && tkv_lt(&t[i + 2].k)
            && tkv_colon(&t[i + 4].k)
            && tkv_gt(&t[i + 6].k)
        {
            let s = tok_word(&t[i + 1]);
            let tr = tok_word(&t[i + 5]);
            let trait_ok = tr.as_str() == "Iterator" || an_user_declares(t, u, &tr);
            if s.len() > 0 && trait_ok {
                let mut c = u;
                while c + 3 < n {
                    if word_is(&t[c], &*s.as_str()) && tkv_lt(&t[c + 1].k) && tkv_gt(&t[c + 3].k) {
                        let prim = tok_word(&t[c + 2]);
                        if bnt_prim(prim.as_str()) && !bs2_prim_impls(t, u, &tr, &prim) {
                            p.perr = t[c].lo + 1;
                            p.perrexp = String::from(
                                "a type argument satisfying the struct's bound (E0277)",
                            );
                            p.perrgot = prim;
                            return;
                        }
                    }
                    c = c + 1;
                }
            }
        }
        i = i + 1;
    }
}
// A2 UNSIZED-ARG (E0277): `Option` / `Vec` / `Result` require a SIZED type argument -- `Option::<[u8]>::None`, `Vec<str>`,
// `Option<dyn Trait>` are refused (Box / Rc / Arc / references take unsized ones). User source only.
fn ua_hit(t: &Vec<Tok>, i: usize) -> bool {
    let n = t.len();
    let w = tok_word(&t[i]);
    if w.as_str() != "Option" {
        if w.as_str() != "Vec" {
            if w.as_str() != "Result" {
                return false;
            }
        }
    }
    let mut a = i + 1;
    if a < n {
        if tkv_path_sep(&t[a].k) {
            a = a + 1; // a turbofish `Option::<..>`
        }
    }
    if a + 1 >= n {
        return false;
    }
    if !tkv_lt(&t[a].k) {
        return false;
    }
    let e = us_unsized_end(t, a + 1);
    if e == 0 {
        return false;
    }
    if e >= n {
        return false;
    }
    // the unsized type must be the WHOLE argument: followed by `>` / `,` / `>>`
    tkv_gt(&t[e].k) || tkv_comma(&t[e].k) || tkv_shr(&t[e].k)
}
fn unsized_arg_check(p: &mut P, t: &Vec<Tok>) {
    // Rust programs only (heavy pass 9): ourc's own `.src` dialect has no `fn main`, its `str` is a sized string type and a
    // top-level expression is its program body -- judging it refused 4 cli programs
    if p.perr != 0 || !(p.hasmain || rs_file(t)) {
        return;
    }
    let mut i = 0;
    let mut k = 0;
    while k < t.len() {
        if word_is(&t[k], "__usrc__") {
            i = k + 1;
        }
        k = k + 1;
    }
    let mut in_alias = false; // a `type A = ..;` alias is not well-formedness-checked by rustc
    while i < t.len() {
        if tkv_type_t(&t[i].k) || word_is(&t[i], "type") {
            in_alias = true;
        }
        if tkv_semi(&t[i].k) {
            in_alias = false;
        }
        if !in_alias && ua_hit(t, i) {
            p.perr = t[i].lo + 1;
            p.perrexp = String::from(
                "a sized type argument (`Option` / `Vec` / `Result` need `Sized`: E0277)",
            );
            p.perrgot = tok_word(&t[i]);
            return;
        }
        i = i + 1;
    }
}
// A2 SUPER-AUTO (E0277): an `impl Tr for X` where the user trait Tr has a `Send` / `Sync` supertrait that X cannot have --
// (a) X is `Rc<..>` (neither), `Cell<..>` / `RefCell<..>` (not Sync), or a raw pointer (neither); (b) a blanket
// `impl<T: B..> Tr for T` whose bounds on T do not include the supertrait. User source only.
fn sa_supers(t: &Vec<Tok>, u: usize, tr: &String) -> String {
    // "S" / "Y" / "SY" for Send / Sync in the header of user `trait tr : .. {`
    let n = t.len();
    let mut i = u;
    while i + 1 < n {
        if tkv_trait(&t[i].k) {
            if word_is(&t[i + 1], &*tr.as_str()) {
                let mut o = String::new();
                let mut j = i + 2;
                while j < n {
                    if tkv_l_brace(&t[j].k) {
                        break;
                    }
                    if tkv_where_t(&t[j].k) || word_is(&t[j], "where") {
                        break;
                    }
                    let w = tok_word(&t[j]);
                    if w.as_str() == "Send" {
                        o.push('S');
                    }
                    if w.as_str() == "Sync" {
                        o.push('Y');
                    }
                    j = j + 1;
                }
                return o;
            }
        }
        i = i + 1;
    }
    String::new()
}
fn super_auto_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 {
        return;
    }
    let mut u = 0;
    let mut k = 0;
    while k < t.len() {
        if word_is(&t[k], "__usrc__") {
            u = k + 1;
        }
        k = k + 1;
    }
    let n = t.len();
    let mut i = u;
    while i + 3 < n {
        if tkv_impl(&t[i].k) {
            // (b) `impl < T : bounds > Tr for T`
            let mut j = i + 1;
            let mut tp = String::new();
            let mut tbounds: Vec<String> = Vec::new();
            if tkv_lt(&t[j].k) {
                if j + 2 < n {
                    if tkv_colon(&t[j + 2].k) {
                        tp = tok_word(&t[j + 1]);
                    }
                }
                let mut d = 0;
                while j < n {
                    if tkv_lt(&t[j].k) {
                        d = d + 1;
                    } else if tkv_gt(&t[j].k) {
                        d = d - 1;
                        if d == 0 {
                            j = j + 1;
                            break;
                        }
                    } else {
                        let w = tok_word(&t[j]);
                        if w.len() > 0 {
                            tbounds.push(w);
                        }
                    }
                    j = j + 1;
                }
            }
            // `Tr for X`
            if j + 2 < n {
                let tr = tok_word(&t[j]);
                if tr.len() > 0 && tkv_for_t(&t[j + 1].k) {
                    let sup = sa_supers(t, u, &tr);
                    if sup.len() > 0 {
                        let mut x = j + 2;
                        // skip a path prefix `std :: rc ::`
                        while x + 1 < n && is_ident(&t[x]) && tkv_path_sep(&t[x + 1].k) {
                            x = x + 2;
                        }
                        let xw = tok_word(&t[x]);
                        let raw = tkv_star(&t[j + 2].k);
                        let needs_send = sup.contains('S');
                        let needs_sync = sup.contains('Y');
                        let mut bad = false;
                        if xw.as_str() == "Rc" || raw {
                            bad = true;
                        }
                        if (xw.as_str() == "Cell" || xw.as_str() == "RefCell") && needs_sync {
                            bad = true;
                        }
                        // (b) a blanket impl for the bare type parameter
                        if tp.len() > 0 && xw.as_str() == tp.as_str() {
                            if needs_send && !vs_has(&tbounds, &String::from("Send")) {
                                bad = true;
                            }
                            if needs_sync && !vs_has(&tbounds, &String::from("Sync")) {
                                bad = true;
                            }
                        }
                        if bad {
                            p.perr = t[i].lo + 1;
                            p.perrexp = String::from(
                                "a type that has the trait's Send / Sync supertrait (E0277)",
                            );
                            p.perrgot = tr;
                            return;
                        }
                    }
                }
            }
        }
        i = i + 1;
    }
}
// A2 STATIC-SYNC (E0277): a non-`mut` `static` must be Sync -- a type naming `RefCell` / `Cell` / `Rc` (outside a `Mutex` /
// `RwLock` / `OnceLock` wrapper) is refused. `static mut` is exempt. User source only.
fn ss_hit(t: &Vec<Tok>, i: usize) -> bool {
    let n = t.len();
    if !tkv_static_t(&t[i].k) {
        return false;
    }
    if i + 3 >= n {
        return false;
    }
    if tkv_mut(&t[i + 1].k) {
        return false;
    }
    if !tkv_colon(&t[i + 2].k) {
        return false;
    }
    let mut q = i + 3;
    while q < n {
        if tkv_eq(&t[q].k) || tkv_semi(&t[q].k) {
            return false;
        }
        let w = tok_word(&t[q]);
        if w.as_str() == "Mutex"
            || w.as_str() == "RwLock"
            || w.as_str() == "OnceLock"
            || w.as_str() == "LazyLock"
        {
            return false;
        }
        if w.as_str() == "Atomic" {
            return false;
        }
        if w.as_str() == "RefCell" || w.as_str() == "Cell" || w.as_str() == "Rc" {
            return true;
        }
        q = q + 1;
    }
    false
}
fn static_sync_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 {
        return;
    }
    let mut i = 0;
    let mut k = 0;
    while k < t.len() {
        if word_is(&t[k], "__usrc__") {
            i = k + 1;
        }
        k = k + 1;
    }
    while i < t.len() {
        if ss_hit(t, i) {
            p.perr = t[i].lo + 1;
            p.perrexp =
                String::from("a Sync type for a static (RefCell / Cell / Rc are not Sync: E0277)");
            p.perrgot = String::from("static");
            return;
        }
        i = i + 1;
    }
}
// A4 INHERENT-FOREIGN (E0116): an inherent `impl X { .. }` (no `for` -- not a trait impl) on a type X that the crate does not
// define -- a primitive or a std type (`impl Vec<u8> {}`). Local struct / enum / union / type names and `impl dyn LocalTrait`
// are fine; macro bodies (`$`) are not judged. User source only.
fn inh_hit(t: &Vec<Tok>, i: usize, local: &Vec<String>) -> usize {
    // t[i] = `impl`: returns the index of the offending type name, or t.len()
    let n = t.len();
    let mut j = i + 1;
    if j < n {
        if tkv_lt(&t[j].k) {
            let mut d = 0;
            while j < n {
                if tkv_lt(&t[j].k) {
                    d = d + 1;
                } else if tkv_gt(&t[j].k) && !tkv_minus(&t[j - 1].k) {
                    d = d - 1;
                    if d == 0 {
                        j = j + 1;
                        break;
                    }
                } else if tkv_shr(&t[j].k) {
                    d = d - 2;
                    if d <= 0 {
                        j = j + 1;
                        break;
                    }
                }
                j = j + 1;
            }
        }
    }
    // a trait impl has `for` before its `{` (at angle depth 0)
    let mut q = j;
    let mut d = 0;
    while q < n {
        if d == 0 {
            if tkv_l_brace(&t[q].k) {
                break;
            }
            if tkv_for_t(&t[q].k) {
                return n;
            }
            if tkv_semi(&t[q].k) {
                return n;
            }
        }
        if tkv_lt(&t[q].k) {
            d = d + 1;
        } else if tkv_gt(&t[q].k) && !tkv_minus(&t[q - 1].k) {
            d = d - 1;
        } else if tkv_shr(&t[q].k) {
            d = d - 2;
        }
        match t[q].k {
            Tk::Dollar => return n,
            _ => {}
        }
        q = q + 1;
    }
    if j >= n {
        return n;
    }
    if tkv_dyn_t(&t[j].k) || word_is(&t[j], "dyn") {
        return n; // `impl dyn LocalTrait`
    }
    // skip a `crate::` / `self::` / `super::` path prefix
    let mut x = j;
    while x + 1 < n && is_ident(&t[x]) && tkv_path_sep(&t[x + 1].k) {
        x = x + 2;
    }
    let w = tok_word(&t[x]);
    if w.len() == 0 {
        return n;
    }
    if vs_has(local, &w) {
        return n;
    }
    x
}
fn inherent_foreign_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 {
        return;
    }
    let mut u = 0;
    let mut k = 0;
    while k < t.len() {
        if word_is(&t[k], "__usrc__") {
            u = k + 1;
        }
        k = k + 1;
    }
    let n = t.len();
    // every type the crate defines (struct / enum / union / type alias / trait -- an alias may name a foreign type, but is
    // not judged here)
    let mut local: Vec<String> = Vec::new();
    let mut i = u;
    while i + 1 < n {
        let w = tok_word(&t[i]);
        let def = tkv_struct(&t[i].k)
            || tkv_enum(&t[i].k)
            || tkv_trait(&t[i].k)
            || tkv_type_t(&t[i].k)
            || w.as_str() == "union";
        if def {
            let nm = tok_word(&t[i + 1]);
            if nm.len() > 0 {
                local.push(nm);
            }
        }
        i = i + 1;
    }
    let mut i = u;
    while i + 1 < n {
        // an impl ITEM starts a statement (after `}` `;` `{` `]` or `unsafe`) -- `-> impl Fn()` / `x: impl Trait` are types
        let mut item = i == u;
        if i > u {
            let pk = &t[i - 1].k;
            if tkv_r_brace(pk)
                || tkv_semi(pk)
                || tkv_l_brace(pk)
                || tkv_r_brack(pk)
                || tkv_unsafe_t(pk)
            {
                item = true;
            }
        }
        if item && tkv_impl(&t[i].k) {
            let x = inh_hit(t, i, &local);
            if x < n {
                p.perr = t[x].lo + 1;
                p.perrexp =
                    String::from("a type defined in this crate for an inherent impl (E0116)");
                p.perrgot = tok_word(&t[x]);
                return;
            }
        }
        i = i + 1;
    }
}
// A6 CONST-INDEX-OOB (E0080): a `const` / `static` initializer that indexes an array LITERAL past its end -- `[][0]`,
// `[1, 2][5]` -- fails const evaluation. User source only.
fn cio_hit(t: &Vec<Tok>, i: usize) -> usize {
    // t[i] = `const` / `static` (an item): scan its initializer `= .. ;` for `[ elems ] [ N ]`
    let n = t.len();
    let mut q = i + 1;
    while q < n {
        if tkv_eq(&t[q].k) {
            break;
        }
        if tkv_semi(&t[q].k) || tkv_l_brace(&t[q].k) || tkv_l_paren(&t[q].k) {
            return n;
        }
        q = q + 1;
    }
    let mut a = q + 1;
    while a < n {
        if tkv_semi(&t[a].k) {
            return n;
        }
        if tkv_l_brack(&t[a].k) {
            // the literal's element count (depth-1 commas), then its matching `]`
            let mut d = 0;
            let mut commas = 0;
            let mut empty = true;
            let mut last_comma = false;
            let mut repeat = false;
            let mut b = a;
            while b < n {
                if tkv_l_brack(&t[b].k) || tkv_l_paren(&t[b].k) || tkv_l_brace(&t[b].k) {
                    d = d + 1;
                } else if tkv_r_brack(&t[b].k) || tkv_r_paren(&t[b].k) || tkv_r_brace(&t[b].k) {
                    d = d - 1;
                    if d == 0 {
                        break;
                    }
                } else if d == 1 {
                    if tkv_comma(&t[b].k) {
                        commas = commas + 1;
                        last_comma = true;
                    } else if tkv_semi(&t[b].k) {
                        repeat = true;
                    } else {
                        empty = false;
                        last_comma = false;
                    }
                }
                b = b + 1;
            }
            if !repeat && b + 3 < n {
                if tkv_l_brack(&t[b + 1].k) && tkv_r_brack(&t[b + 3].k) {
                    if let Tk::Num(ix) = t[b + 2].k {
                        let mut len: i64 = commas + 1;
                        if empty {
                            len = 0;
                        } else if last_comma {
                            len = commas;
                        }
                        if ix >= len {
                            return b + 2;
                        }
                    }
                }
            }
            a = b;
        }
        a = a + 1;
    }
    n
}
fn const_index_oob_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 {
        return;
    }
    let mut i = 0;
    let mut k = 0;
    while k < t.len() {
        if word_is(&t[k], "__usrc__") {
            i = k + 1;
        }
        k = k + 1;
    }
    while i < t.len() {
        if tkv_const_t(&t[i].k) || tkv_static_t(&t[i].k) {
            let x = cio_hit(t, i);
            if x < t.len() {
                p.perr = t[x].lo + 1;
                p.perrexp =
                    String::from("an in-bounds index (evaluating this constant fails: E0080)");
                p.perrgot = String::from("index out of bounds");
                return;
            }
        }
        i = i + 1;
    }
}

// A7 STRAY-ITEM-LIT: a literal where a module-root item is expected (`5` at file scope) is rustc's
// "expected item, found `5`". A `.src` script (src_script: top-level exprs) is skipped.
fn stray_item_lit_check(p: &mut P, t: &Vec<Tok>) {
    // Rust programs only (heavy pass 9): ourc's own `.src` dialect has no `fn main`, its `str` is a sized string type and a
    // top-level expression is its program body -- judging it refused 4 cli programs
    if p.perr != 0 || !(p.hasmain || rs_file(t)) {
        return;
    }
    let mut i = 0;
    let mut k = 0;
    while k < t.len() {
        if word_is(&t[k], "__usrc__") {
            i = k + 1;
        }
        k = k + 1;
    }
    let mut d: i32 = 0;
    let mut at_item = false; // i opens `__usrc__`'s `()` `{}`: its `}` sets it
    while i < t.len() {
        let mut lit = false;
        match &t[i].k {
            Tk::Num(_) => lit = true,
            Tk::StrL(_) => lit = true,
            Tk::FloatL(_) => lit = true,
            Tk::ChL(_) => lit = true,
            _ => {}
        }
        if d == 0 && at_item && lit {
            p.perr = t[i].lo + 1;
            p.perrexp = String::from("an item");
            p.perrgot = String::from("a literal (expected item)");
            return;
        }
        at_item = false;
        match &t[i].k {
            Tk::LBrace => d = d + 1,
            Tk::LParen => d = d + 1,
            Tk::LBrack => d = d + 1,
            Tk::RParen => d = d - 1,
            Tk::RBrack => d = d - 1,
            Tk::RBrace => {
                d = d - 1;
                if d == 0 {
                    at_item = true;
                }
            }
            Tk::Semi => {
                if d == 0 {
                    at_item = true;
                }
            }
            _ => {}
        }
        i = i + 1;
    }
}
// a token that may open a module-root item; a mainless file whose root holds anything else is a `.src` script
fn sil_item_start(x: &Tok) -> bool {
    match &x.k {
        Tk::Fn => return true,
        Tk::Struct => return true,
        Tk::Enum => return true,
        Tk::Impl => return true,
        Tk::Trait => return true,
        Tk::UseT => return true,
        Tk::ConstT => return true,
        Tk::StaticT => return true,
        Tk::TypeT => return true,
        Tk::PubT => return true,
        Tk::ExternT => return true,
        Tk::UnsafeT => return true,
        Tk::ModT => return true,
        Tk::DeriveCopy => return true,
        Tk::Semi => return true,
        _ => {}
    }
    let w = tok_word(x);
    w.as_str() == "#" || w.as_str() == "macro_rules" || w.as_str() == "union"
}
// R2a P6 SIB-PATH (E0433): inside a module, a path whose FIRST segment names a sibling top-level module (`a::f()` in
// `mod b`) does not resolve in Rust 2018+: `a` is not in scope there unless imported (`use crate::a;`, `use super::a;`,
// or a glob of the crate root). ourc resolves every `a::x` absolutely, so it was accepted. Pass 1 maps the module tree;
// pass 2 walks each module with its imports and flags the first unresolvable sibling path.
fn sp_mod_at(t: &Vec<Tok>, i: usize) -> String {
    // `mod NAME {` at i -> NAME, else ""
    if i + 2 < t.len() && tkv_mod_t(&t[i].k) && tkv_l_brace(&t[i + 2].k) {
        return tok_word(&t[i + 1]);
    }
    String::new()
}
fn sibling_path_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 || !p.hasmain {
        return;
    }
    let mut start = 0;
    let mut k = 0;
    while k < t.len() {
        if word_is(&t[k], "__usrc__") {
            start = k + 1;
        }
        k = k + 1;
    }
    // pass 1: every module path ("" = root) and each module's children, as `parent|child`
    let mut tops: Vec<String> = Vec::new();
    let mut kids: Vec<String> = Vec::new();
    let mut stack: Vec<String> = Vec::new();
    let mut depths: Vec<usize> = Vec::new();
    let mut d = 0;
    let mut i = start;
    while i < t.len() {
        let nm = sp_mod_at(t, i);
        if nm.len() > 0 {
            let par = if stack.len() > 0 {
                String::from(stack[stack.len() - 1].as_str())
            } else {
                String::new()
            };
            if par.len() == 0 {
                tops.push(String::from(nm.as_str()));
            }
            let mut e = String::from(par.as_str());
            e.push('|');
            e.push_str(nm.as_str());
            kids.push(e);
            let mut full = String::from(par.as_str());
            if full.len() > 0 {
                full.push('$');
            }
            full.push_str(nm.as_str());
            stack.push(full);
            d = d + 1;
            depths.push(d);
            i = i + 3;
            continue;
        }
        if tkv_l_brace(&t[i].k) || tkv_l_paren(&t[i].k) || tkv_l_brack(&t[i].k) {
            d = d + 1;
        } else if tkv_r_brace(&t[i].k) || tkv_r_paren(&t[i].k) || tkv_r_brack(&t[i].k) {
            if depths.len() > 0 && depths[depths.len() - 1] == d {
                let _ = stack.pop();
                let _ = depths.pop();
            }
            if d > 0 {
                d = d - 1;
            }
        }
        i = i + 1;
    }
    if tops.len() < 2 {
        return;
    }
    // pass 2: walk again, tracking the current module, with each module's imports (`m|name`; `m|*` = a root glob)
    let mut imps: Vec<String> = Vec::new();
    let mut stack: Vec<String> = Vec::new();
    let mut depths: Vec<usize> = Vec::new();
    let mut d = 0;
    let mut i = start;
    while i < t.len() {
        let cur = if stack.len() > 0 {
            String::from(stack[stack.len() - 1].as_str())
        } else {
            String::new()
        };
        if tkv_mod_t(&t[i].k) {
            let nm = sp_mod_at(t, i);
            if nm.len() > 0 {
                let mut full = String::from(cur.as_str());
                if full.len() > 0 {
                    full.push('$');
                }
                full.push_str(nm.as_str());
                stack.push(full);
                d = d + 1;
                depths.push(d);
                i = i + 3;
                continue;
            }
        }
        if tkv_l_brace(&t[i].k) || tkv_l_paren(&t[i].k) || tkv_l_brack(&t[i].k) {
            d = d + 1;
            i = i + 1;
            continue;
        }
        if tkv_r_brace(&t[i].k) || tkv_r_paren(&t[i].k) || tkv_r_brack(&t[i].k) {
            if depths.len() > 0 && depths[depths.len() - 1] == d {
                let _ = stack.pop();
                let _ = depths.pop();
            }
            if d > 0 {
                d = d - 1;
            }
            i = i + 1;
            continue;
        }
        if tkv_use_t(&t[i].k) {
            let mut j = i + 1;
            let mut segs: Vec<String> = Vec::new();
            while j < t.len() && !tkv_semi(&t[j].k) {
                let w = tok_word(&t[j]);
                if w.len() > 0 {
                    segs.push(w);
                } else if tkv_star(&t[j].k) {
                    segs.push(String::from("*"));
                } else if tkv_crate_t(&t[j].k) {
                    segs.push(String::from("crate"));
                } else if tk_is(&t[j].k, "super") {
                    segs.push(String::from("super"));
                }
                j = j + 1;
            }
            // `use crate::a;` / `use crate::{a, b};` / `use super::a;` put the named segments in scope here;
            // `use crate::*;` (or `use super::*;` from a top-level module) puts every top-level module in scope
            let rooted = segs.len() > 0
                && (segs[0].as_str() == "crate"
                    || (segs[0].as_str() == "super" && !cur.contains("$")));
            let mut q = 0;
            while q < segs.len() {
                if segs[q].as_str() != "*" || (rooted && segs.len() == 2) {
                    let mut e = String::from(cur.as_str());
                    e.push('|');
                    e.push_str(segs[q].as_str());
                    imps.push(e);
                }
                q = q + 1;
            }
            i = j;
            continue;
        }
        if cur.len() > 0
            && i + 1 < t.len()
            && tkv_path_sep(&t[i + 1].k)
            && !(i > 0 && tkv_path_sep(&t[i - 1].k))
        {
            let w = tok_word(&t[i]);
            if w.len() > 0 && vs_has(&tops, &w) {
                let mut kid = String::from(cur.as_str());
                kid.push('|');
                kid.push_str(w.as_str());
                let mut glob = String::from(cur.as_str());
                glob.push_str("|*");
                if !vs_has(&kids, &kid)
                    && !vs_has(&imps, &kid)
                    && !vs_has(&imps, &glob)
                    && !sp_is_own(&cur, &w)
                {
                    p.perr = t[i].lo + 1;
                    p.perrexp = String::from(
                        "a path that resolves here (`crate::` / `super::`, or `use crate::NAME;`): a sibling module is not in scope inside another module (E0433)",
                    );
                    p.perrgot = w;
                    return;
                }
            }
        }
        i = i + 1;
    }
}
// a module naming ITSELF (`b::x` inside `mod b`) is still a sibling path in rustc -- but its own first segment is
// never a sibling; only a strict ancestor's name equal to w would be (kept conservative: no refusal)
fn sp_is_own(cur: &str, w: &str) -> bool {
    let b = cur.as_bytes();
    let wb = w.as_bytes();
    if b.len() < wb.len() {
        return false;
    }
    let mut k = 0;
    while k < wb.len() {
        if b[k] != wb[k] {
            return false;
        }
        k = k + 1;
    }
    b.len() == wb.len() || b[wb.len()] == 36
}
// is the user source a `.src` script (ourc's own dialect: top-level statements and a tail expression, no `fn main`)?
// A mainless file whose module root holds a non-item -- a `let`, a call, or a literal that continues into an expression
// (`40 + 2`) -- is one. A lone literal (`5`, rustc's "expected item") is not. Checks of Rust-only rules skip scripts.
fn src_script(p: &P, t: &Vec<Tok>) -> bool {
    if p.hasmain {
        return false;
    }
    let mut i = 0;
    let mut k = 0;
    while k < t.len() {
        if word_is(&t[k], "__usrc__") {
            i = k + 1;
        }
        k = k + 1;
    }
    let mut d: i32 = 0;
    let mut at_item = false; // i opens `__usrc__`'s `()` `{}`: its `}` sets it
    while i < t.len() {
        let mut lit = false;
        match &t[i].k {
            Tk::Num(_) => lit = true,
            Tk::StrL(_) => lit = true,
            Tk::FloatL(_) => lit = true,
            Tk::ChL(_) => lit = true,
            _ => {}
        }
        if d == 0 && at_item {
            match &t[i].k {
                Tk::Eof => return false,
                _ => {}
            }
            if lit {
                if i + 1 < t.len() {
                    let mut ends = false;
                    match &t[i + 1].k {
                        Tk::Eof => ends = true,
                        Tk::Semi => ends = true,
                        _ => {}
                    }
                    if !ends {
                        return true;
                    }
                }
            } else if !sil_item_start(&t[i]) {
                return true;
            }
        }
        at_item = false;
        match &t[i].k {
            Tk::LBrace => d = d + 1,
            Tk::LParen => d = d + 1,
            Tk::LBrack => d = d + 1,
            Tk::RParen => d = d - 1,
            Tk::RBrack => d = d - 1,
            Tk::RBrace => {
                d = d - 1;
                if d == 0 {
                    at_item = true;
                }
            }
            Tk::Semi => {
                if d == 0 {
                    at_item = true;
                }
            }
            _ => {}
        }
        i = i + 1;
    }
    false
}
// V1 VALUE-OBSERVE (refusal > WRONG, until V1a / V1b land): two value-model facts ourc cannot represent yet, observed.
// (a) `s.push('\0')`: a String is NUL-terminated here, so a pushed NUL ends it (`len` 3 where rustc says 4). (b) a
// pointer's ADDRESS read as an integer (`p as *const T as usize`): Box::new / clone do not give distinct allocations
// identity-for-identity with rustc, so comparing addresses answered `false` where rustc says `true`. User source only.
fn value_observe_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 || src_script(p, t) {
        return;
    }
    let mut i = 0;
    let mut k = 0;
    while k < t.len() {
        if word_is(&t[k], "__usrc__") {
            i = k + 1;
        }
        k = k + 1;
    }
    // (b) only where a Box is CLONED (`Box::new` and `.clone()` both present): one object's address compared with
    // itself is already right
    let mut has_box = false;
    let mut has_clone = false;
    let mut q0 = i;
    while q0 + 2 < t.len() {
        if word_is(&t[q0], "Box") && tkv_path_sep(&t[q0 + 1].k) && word_is(&t[q0 + 2], "new") {
            has_box = true;
        }
        if tkv_dot(&t[q0].k) && word_is(&t[q0 + 1], "clone") {
            has_clone = true;
        }
        q0 = q0 + 1;
    }
    while i + 3 < t.len() {
        if tkv_dot(&t[i].k) && word_is(&t[i + 1], "push") && tkv_l_paren(&t[i + 2].k) {
            let mut nul = false;
            match &t[i + 3].k {
                Tk::ChL(c) => nul = *c == 0,
                _ => {}
            }
            // V1a LEN-STRING landed: a String holds `\0` (its length is a header), so a pushed NUL is no longer refused
            let _ = nul;
        }
        if has_box
            && has_clone
            && tkv_as_t(&t[i].k)
            && tkv_star(&t[i + 1].k)
            && tkv_const_t(&t[i + 2].k)
        {
            // `as *const T as usize`: find the next `as` before a `;` / `)` and check the target
            let mut q = i + 3;
            let mut hit = false;
            while q + 1 < t.len()
                && !tkv_semi(&t[q].k)
                && !tkv_r_paren(&t[q].k)
                && !tkv_comma(&t[q].k)
            {
                if tkv_as_t(&t[q].k) && word_is(&t[q + 1], "usize") {
                    hit = true;
                }
                q = q + 1;
            }
            // V1b: `&b` / `&*b` of a Box cell has a real address (the cell handle) -- not refused
            if hit && p.boxid == 1 && i >= 2 {
                let nm = tok_word(&t[i - 1]);
                if p.is_boxvar(nm.as_str())
                    && (tkv_amp(&t[i - 2].k)
                        || (i >= 3 && tkv_star(&t[i - 2].k) && tkv_amp(&t[i - 3].k)))
                {
                    hit = false;
                }
            }
            if hit {
                p.perr = t[i].lo + 1;
                p.perrexp = String::from(
                    "no pointer address as an integer (allocation identity is not modelled: V1b)",
                );
                p.perrgot = String::from("as *const _ as usize");
                return;
            }
        }
        i = i + 1;
    }
}
fn box_is_new(v: &E) -> bool {
    match v {
        E::Call(f, _a, _l, _h) => f.as_str() == "$boxnew",
        _ => false,
    }
}
// the receiver name of `x.clone()`, or ""
fn box_clone_of(v: &E) -> String {
    let mut out = String::new();
    if let E::MethodCall(r, m, a, _l, _h) = v {
        if m.as_str() == "clone" && a.len() == 0 {
            if let E::Var(rn, _rl, _rh) = r.as_ref() {
                out = String::from(rn.as_str());
            }
        }
    }
    out
}
// V1b: `let b = a.clone()` of a Box cell -> `$boxnew(a)` (a new cell holding a's value); any other initializer unchanged
fn box_rewrap(cn: String, v: E) -> E {
    if cn.len() == 0 {
        return v;
    }
    let lo = oast::e_lo(&v);
    let hi = oast::e_hi(&v);
    let mut src: Vec<E> = Vec::new();
    src.push(E::Var(cn, lo, hi));
    E::Call(String::from("$boxnew"), src, lo, hi)
}
// the name moved by a bare `let c = a`, or ""
fn box_move_of(v: &E) -> String {
    let mut out = String::new();
    if let E::Var(rn, _rl, _rh) = v {
        out = String::from(rn.as_str());
    }
    out
}
// a Rust SOURCE FILE (heavy pass 9): the driver marks a `.rs` input with `fn __rsfile__() {}`, so a Rust-only check judges a
// main-less Rust file (`5` alone is "expected item") while ourc's own `.src` dialect -- no main, a top-level expression
// body, `str` a sized string -- is never judged by it
fn rs_file(t: &Vec<Tok>) -> bool {
    let mut k = t.len();
    while k > 0 {
        k = k - 1;
        if let Tk::Ident(x) = &t[k].k {
            if x.as_str() == "__rsfile__" {
                return true;
            }
        }
    }
    false
}
// R2a P6 ORPHAN (E0117): `impl ForeignTrait for ForeignType` -- neither the trait nor the type is defined in this crate
// (`impl Display for Vec<i32>`); rustc refuses it in any crate, ourc accepted it. Foreign = a std / primitive type base that is
// not a user type, and a trait the program does not declare. Prelude impls (before the `__usrc__` marker) are ourc's own.
// A bare generic parameter (`impl<T> Tr for T`, rustc E0210) is not in the std list, so it is never judged here.
fn orphan_impl_check(p: &mut P, impls: &Vec<ImplRec>, t: &Vec<Tok>) {
    if p.perr != 0 || !(p.hasmain || rs_file(t)) {
        return;
    }
    let mut ulo = 0;
    let mut k = 0;
    while k < t.len() {
        if let Tk::Ident(x) = &t[k].k {
            if x.as_str() == "__usrc__" {
                ulo = t[k].lo;
            }
        }
        k = k + 1;
    }
    let mut i = 0;
    while i < impls.len() {
        let r = &impls[i];
        if r.lo > ulo && r.tr.len() > 0 {
            // the trait: the path's last segment; the type: its base (std types are never module-qualified)
            let tb = r.tr.as_bytes();
            let mut c = 0;
            let mut q = 0;
            while q < tb.len() {
                if tb[q] == 36 {
                    c = q + 1;
                }
                q = q + 1;
            }
            let tr = String::from(&r.tr[c..]);
            let yb = r.ty.as_bytes();
            let mut e = 0;
            while e < yb.len() && yb[e] != 36 {
                e = e + 1;
            }
            let base = String::from(&r.ty[..e]);
            let std_ty = base.as_str() == "Vec"
                || base.as_str() == "String"
                || base.as_str() == "str"
                || base.as_str() == "Option"
                || base.as_str() == "Result"
                || base.as_str() == "Box"
                || base.as_str() == "HashMap"
                || base.as_str() == "HashSet"
                || base.as_str() == "BTreeMap"
                || base.as_str() == "VecDeque"
                || base.as_str() == "int"
                || base.as_str() == "bool"
                || base.as_str() == "char"
                || base.as_str() == "f64"
                || base.as_str() == "f32"
                || oast::is_int_width(base.as_str());
            // a trait is LOCAL when ANY `$` segment of its name is a declared trait: `$` spells both path segments
            // (`fmt$Display`) and generic arguments (`Foo<i32>` = `Foo$i32`, whose last segment is the argument)
            let mut local_tr = false;
            {
                let mut st = 0;
                let mut q = 0;
                while q <= tb.len() {
                    if q == tb.len() || tb[q] == 36 {
                        if q > st && oast::in_list(&p.trnames, &r.tr[st..q]) {
                            local_tr = true;
                        }
                        // R15.5 ORPHAN-ARG: a LOCAL type among the trait's generic arguments also satisfies the orphan rule
                        // (`impl Mul<Foo> for f64` with Foo local is valid; rustc's rule, simplified to "any local argument")
                        if q > st && st > 0 {
                            let seg = String::from(&r.tr[st..q]);
                            if p.sknown(&seg) || p.enames_has(&seg) {
                                local_tr = true;
                            }
                        }
                        st = q + 1;
                    }
                    q = q + 1;
                }
            }
            if std_ty && !p.sknown(&base) && !p.enames_has(&base) && !local_tr && tr.len() > 0 {
                p.perr = r.lo + 1;
                p.perrexp = String::from(
                    "an impl of a LOCAL trait or for a LOCAL type (only traits defined in the current crate can be implemented for types defined outside of it: E0117)",
                );
                p.perrgot = tr;
                return;
            }
        }
        i = i + 1;
    }
}
// R15.1 ELISION: the end of the parameter type starting at i (the `,` or `)` at depth 0)
fn elide_end(t: &Vec<Tok>, i: usize) -> usize {
    let mut k = i;
    let mut d: i32 = 0;
    while k < t.len() {
        if tkv_l_paren(&t[k].k) || tkv_l_brack(&t[k].k) || tkv_lt(&t[k].k) {
            d = d + 1;
        } else if tkv_r_brack(&t[k].k) || tkv_gt(&t[k].k) {
            d = d - 1;
        } else if tkv_shr(&t[k].k) {
            d = d - 2;
        } else if tkv_r_paren(&t[k].k) {
            if d == 0 {
                return k;
            }
            d = d - 1;
        } else if tkv_comma(&t[k].k) && d == 0 {
            return k;
        }
        k = k + 1;
    }
    k
}
// the input lifetimes of one parameter type: each `&` (a `&&` is two); an `fn(..)` / `Fn(..)` argument list is skipped
// (its references are bound there, not in the outer signature)
fn elide_refs(t: &Vec<Tok>, i: usize) -> i32 {
    let e = elide_end(t, i);
    let mut n = 0;
    let mut k = i;
    while k < e {
        let w = tok_word(&t[k]);
        let fnty = w.as_str() == "fn"
            || w.as_str() == "Fn"
            || w.as_str() == "FnMut"
            || w.as_str() == "FnOnce";
        if fnty && k + 1 < e && tkv_l_paren(&t[k + 1].k) {
            let mut d = 0;
            k = k + 1;
            while k < e {
                if tkv_l_paren(&t[k].k) {
                    d = d + 1;
                }
                if tkv_r_paren(&t[k].k) {
                    d = d - 1;
                    if d == 0 {
                        break;
                    }
                }
                k = k + 1;
            }
        } else if tkv_amp(&t[k].k) {
            n = n + 1;
        } else if tkv_and_and(&t[k].k) {
            n = n + 2;
        }
        k = k + 1;
    }
    n
}
// could this parameter type carry a lifetime the lexer dropped? a user type (`Foo<'_>`), `Self`, `dyn` / `impl` Trait
fn elide_opaque(t: &Vec<Tok>, i: usize) -> bool {
    let e = elide_end(t, i);
    let mut k = i;
    while k < e {
        let w = tok_word(&t[k]);
        let b = w.as_bytes();
        if w.as_str() == "dyn" || w.as_str() == "impl" {
            return true;
        }
        if b.len() > 0 && b[0] >= 65 && b[0] <= 90 {
            let known = w.as_str() == "Vec"
                || w.as_str() == "Option"
                || w.as_str() == "Result"
                || w.as_str() == "String"
                || w.as_str() == "Box"
                || w.as_str() == "Rc"
                || w.as_str() == "Arc"
                || w.as_str() == "HashMap"
                || w.as_str() == "HashSet";
            if !known {
                return true;
            }
        }
        k = k + 1;
    }
    false
}
// R15.2 DYN-COMPAT: an associated item from t[q] to its `;` that carries its own `where Self: Sized` (or any `Self:` bound,
// as dc_method_bad allows) is excluded from the vtable, so it does not make the trait dyn-incompatible
fn dc_item_exempt(t: &Vec<Tok>, q: usize) -> bool {
    let mut e = q;
    while e < t.len() && !tkv_semi(&t[e].k) && !tkv_l_brace(&t[e].k) && !tkv_r_brace(&t[e].k) {
        e = e + 1;
    }
    dc_sized_req(t, q, e) || dc_self_bound(t, q, e)
}
// R15.4 SLIT-SRC-ORDER: do the written field names ns appear out of declaration order?
fn slit_out_of_order(ns: &Vec<String>, order: &Vec<String>) -> bool {
    let mut last: i32 = 0 - 1;
    let mut i = 0;
    while i < ns.len() {
        let mut pos: i32 = 0 - 1;
        let mut k = 0;
        while k < order.len() {
            if order[k].as_str() == ns[i].as_str() {
                pos = k as i32;
            }
            k = k + 1;
        }
        // a written name with no declared slot (an erased marker field such as PhantomData) is not ordered
        if pos >= 0 {
            if pos < last {
                return true;
            }
            last = pos;
        }
        i = i + 1;
    }
    false
}
fn slit_noarg_call(v: &E) -> bool {
    match v {
        E::Call(_f, a, _l, _h) => a.len() == 0,
        _ => false,
    }
}
// R15.4 MANUAL-COPY: does the source hold `impl .. Copy for <name>` (also `marker::Copy for`)?
fn manual_copy(t: &Vec<Tok>, name: &str) -> bool {
    let mut k = 0;
    while k + 2 < t.len() {
        if word_is(&t[k], "Copy") && tkv_for_t(&t[k + 1].k) && word_is(&t[k + 2], &*name) {
            return true;
        }
        k = k + 1;
    }
    false
}
// R15.4 PAT-MUT: does the destructuring pattern starting at t[i] (`(` / `[` / `Name {` / `Name(`) hold a `mut` before its `=`?
fn pat_has_mut(t: &Vec<Tok>, i: usize) -> bool {
    if i >= t.len() {
        return false;
    }
    let opens = tkv_l_paren(&t[i].k)
        || tkv_l_brack(&t[i].k)
        || (i + 1 < t.len() && (tkv_l_brace(&t[i + 1].k) || tkv_l_paren(&t[i + 1].k)));
    if !opens {
        return false;
    }
    let mut k = i;
    let mut d: i32 = 0;
    while k < t.len() {
        if tkv_l_paren(&t[k].k) || tkv_l_brack(&t[k].k) || tkv_l_brace(&t[k].k) {
            d = d + 1;
        } else if tkv_r_paren(&t[k].k) || tkv_r_brack(&t[k].k) || tkv_r_brace(&t[k].k) {
            d = d - 1;
        } else if d == 0 && (tkv_eq(&t[k].k) || tkv_semi(&t[k].k) || tkv_colon(&t[k].k)) {
            return false;
        } else if tkv_mut(&t[k].k) {
            return true;
        }
        k = k + 1;
    }
    false
}
// R2a P6 NONEXH (E0004): a `#[non_exhaustive]` enum of ANOTHER crate cannot be matched exhaustively by listing its variants
// -- a wildcard (or a binding) arm is required. Extern crates are the `mod NAME { fn __xcrate__() {} .. }` units the driver
// splices; their marked enums are judged only at matches OUTSIDE them (the crate`s own matches need no wildcard).
fn nonexh_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 || !(p.hasmain || rs_file(t)) {
        return;
    }
    // extern-crate ranges [lo, hi) and their non_exhaustive enums
    let mut rlo: Vec<usize> = Vec::new();
    let mut rhi: Vec<usize> = Vec::new();
    let mut ne: Vec<String> = Vec::new();
    let n = t.len();
    let mut i = 0;
    while i + 5 < n {
        if tkv_mod_t(&t[i].k)
            && tkv_l_brace(&t[i + 2].k)
            && tkv_fn(&t[i + 3].k)
            && word_is(&t[i + 4], "__xcrate__")
        {
            let mut d = 0;
            let mut j = i + 2;
            while j < n {
                if tk_open(&t[j]) {
                    d = d + 1;
                } else if tk_close(&t[j]) {
                    d = d - 1;
                    if d == 0 {
                        break;
                    }
                }
                j = j + 1;
            }
            rlo.push(i);
            rhi.push(j);
            let mut k = i;
            while k + 2 < j {
                if word_is(&t[k], "__nonexh__") {
                    let mut e = k + 1;
                    while e < j && !tkv_enum(&t[e].k) && e < k + 4 {
                        e = e + 1;
                    }
                    if e + 1 < j && tkv_enum(&t[e].k) {
                        ne.push(tok_word(&t[e + 1]));
                    }
                }
                k = k + 1;
            }
            i = j;
            continue;
        }
        i = i + 1;
    }
    if ne.len() == 0 {
        return;
    }
    let mut i = 0;
    while i < n {
        if tkv_match(&t[i].k) {
            let mut inside = false;
            let mut r = 0;
            while r < rlo.len() {
                if i > rlo[r] && i < rhi[r] {
                    inside = true;
                }
                r = r + 1;
            }
            // the arms: from the first depth-0 `{` after the scrutinee
            let mut j = i + 1;
            let mut d = 0;
            while j < n && !(d == 0 && tkv_l_brace(&t[j].k)) {
                if tk_open(&t[j]) {
                    d = d + 1;
                } else if tk_close(&t[j]) {
                    d = d - 1;
                }
                j = j + 1;
            }
            let mut q = j + 1;
            let mut names_ne = false;
            let mut catch_all = false;
            while !inside && q < n && !tkv_r_brace(&t[q].k) {
                // the pattern [ps, pe) up to a depth-0 `=>`
                let ps = q;
                let mut dd = 0;
                while q < n && !(dd == 0 && tkv_fat_arrow(&t[q].k)) {
                    if tk_open(&t[q]) {
                        dd = dd + 1;
                    } else if tk_close(&t[q]) {
                        dd = dd - 1;
                    }
                    if q + 1 < n && tkv_path_sep(&t[q + 1].k) && vs_has(&ne, &tok_word(&t[q])) {
                        names_ne = true;
                    }
                    q = q + 1;
                }
                let pe = q;
                if pe == ps + 1 && (tk_is(&t[ps].k, "_") || is_ident(&t[ps])) {
                    catch_all = true;
                }
                // the body: a block (then an optional `,`) or up to a depth-0 `,`
                q = q + 1;
                if q < n && tkv_l_brace(&t[q].k) {
                    let mut db = 0;
                    while q < n {
                        if tk_open(&t[q]) {
                            db = db + 1;
                        } else if tk_close(&t[q]) {
                            db = db - 1;
                            if db == 0 {
                                break;
                            }
                        }
                        q = q + 1;
                    }
                    q = q + 1;
                    if q < n && tkv_comma(&t[q].k) {
                        q = q + 1;
                    }
                } else {
                    let mut db = 0;
                    while q < n && !(db == 0 && (tkv_comma(&t[q].k) || tkv_r_brace(&t[q].k))) {
                        if tk_open(&t[q]) {
                            db = db + 1;
                        } else if tk_close(&t[q]) {
                            db = db - 1;
                        }
                        q = q + 1;
                    }
                    if q < n && tkv_comma(&t[q].k) {
                        q = q + 1;
                    }
                }
            }
            if names_ne && !catch_all {
                p.perr = t[i].lo + 1;
                p.perrexp = String::from(
                    "a wildcard arm (`_ =>`): a `#[non_exhaustive]` enum of another crate may gain variants (E0004)",
                );
                p.perrgot = String::from("match");
                return;
            }
        }
        i = i + 1;
    }
}
// R15.6 CPAT-VALUE: is the const initializer (after the `=` found from i) a unit variant path `A::B` or a bare literal?
fn cpat_unit_init(t: &Vec<Tok>, i: usize) -> bool {
    let mut k = i;
    while k < t.len() && !tkv_eq(&t[k].k) && !tkv_semi(&t[k].k) {
        k = k + 1;
    }
    if k >= t.len() || !tkv_eq(&t[k].k) {
        return false;
    }
    let s = k + 1;
    if s + 3 < t.len() && tkv_path_sep(&t[s + 1].k) && tkv_semi(&t[s + 3].k) {
        return true;
    }
    if s + 1 < t.len() && tkv_semi(&t[s + 1].k) {
        match &t[s].k {
            Tk::Num(_) => return true,
            Tk::StrL(_) => return true,
            Tk::ChL(_) => return true,
            _ => {}
        }
    }
    false
}
// R1 MAIN-IMPL-TERM: does the `impl` bound path starting at t[k] (`Termination` or `std::process::Termination`) end in Termination?
fn impl_names_termination(t: &Vec<Tok>, k0: usize) -> bool {
    let mut k = k0;
    let mut last = String::new();
    while k < t.len() {
        let w = tok_word(&t[k]);
        if w.len() > 0 && !tkv_path_sep(&t[k].k) {
            last = w;
            k = k + 1;
            if k < t.len() && tkv_path_sep(&t[k].k) {
                k = k + 1;
                continue;
            }
        }
        break;
    }
    last.as_str() == "Termination"
}
// V4 WRONG-GUARD (refusal > WRONG, v0.1 zero-wrong): two shapes run g' answered WRONG.
// (a) a raw-pointer WRITE inside a `const` initializer (`const X: &u32 = { let b: *mut u32 = &mut a; unsafe { *b = 5; } .. }`):
//     ourc's const evaluation does not see the write through the pointer.
// (b) supertrait method SHADOWING with a conditional impl: a method name provided both by a blanket `impl<T> Tr for T`
//     trait and by another user trait -- rustc picks by where-clause satisfiability, which ourc does not solve.
fn wrong_guard_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 || !(p.hasmain || rs_file(t)) {
        return;
    }
    let mut u = 0;
    let mut k = 0;
    while k < t.len() {
        if word_is(&t[k], "__usrc__") {
            u = k + 1;
        }
        k = k + 1;
    }
    // (a): `const NAME : .. = { .. *mut .. }` containing a deref-assign `* x =`
    let mut i = u;
    while i + 1 < t.len() {
        if tkv_const_t(&t[i].k) && i + 2 < t.len() && tkv_colon(&t[i + 2].k) {
            let mut e = i + 3;
            while e < t.len() && !tkv_eq(&t[e].k) && !tkv_semi(&t[e].k) {
                e = e + 1;
            }
            if e < t.len() && tkv_eq(&t[e].k) && e + 1 < t.len() && tkv_l_brace(&t[e + 1].k) {
                let mut d = 0;
                let mut q = e + 1;
                let mut has_raw = false;
                let mut has_wr = false;
                while q < t.len() {
                    if tkv_l_brace(&t[q].k) {
                        d = d + 1;
                    } else if tkv_r_brace(&t[q].k) {
                        d = d - 1;
                        if d == 0 {
                            break;
                        }
                    }
                    if tkv_star(&t[q].k) && q + 1 < t.len() && tkv_mut(&t[q + 1].k) {
                        has_raw = true;
                    }
                    if tkv_star(&t[q].k)
                        && q + 2 < t.len()
                        && is_ident(&t[q + 1])
                        && tkv_eq(&t[q + 2].k)
                    {
                        has_wr = true;
                    }
                    q = q + 1;
                }
                if has_raw && has_wr {
                    p.perr = t[i].lo + 1;
                    p.perrexp = String::from("a const initializer without a write through a raw pointer (not modelled by ourc's const evaluation)");
                    p.perrgot = String::from("*mut");
                    return;
                }
            }
        }
        i = i + 1;
    }
    // (b): a trait implemented by a blanket `impl<T> NAME for T` whose method name another user trait also provides
    let mut blanket: Vec<String> = Vec::new();
    let mut j = u;
    while j + 6 < t.len() {
        if tkv_impl(&t[j].k) && tkv_lt(&t[j + 1].k) && tkv_gt(&t[j + 3].k) && tkv_for_t(&t[j + 5].k)
        {
            let tv = tok_word(&t[j + 2]);
            if word_is(&t[j + 6], &*tv.as_str()) {
                blanket.push(tok_word(&t[j + 4]));
            }
        }
        j = j + 1;
    }
    if blanket.len() == 0 {
        return;
    }
    // trait -> its method names
    let mut owners: Vec<String> = Vec::new();
    let mut meths: Vec<String> = Vec::new();
    let mut heads: Vec<String> = Vec::new(); // the trait header (supertraits), per method row
    let mut j = u;
    while j + 2 < t.len() {
        if tkv_trait(&t[j].k) {
            let tn = tok_word(&t[j + 1]);
            let mut q = j + 2;
            let mut hd = String::from(",");
            while q < t.len() && !tkv_l_brace(&t[q].k) {
                hd.push_str(tok_word(&t[q]).as_str());
                hd.push(',');
                q = q + 1;
            }
            let mut d = 0;
            while q < t.len() {
                if tkv_l_brace(&t[q].k) {
                    d = d + 1;
                } else if tkv_r_brace(&t[q].k) {
                    d = d - 1;
                    if d == 0 {
                        break;
                    }
                }
                if d == 1 && tkv_fn(&t[q].k) && q + 1 < t.len() {
                    owners.push(String::from(tn.as_str()));
                    meths.push(tok_word(&t[q + 1]));
                    heads.push(String::from(hd.as_str()));
                }
                q = q + 1;
            }
        }
        j = j + 1;
    }
    let mut a = 0;
    while a < owners.len() {
        if vs_has(&blanket, &owners[a]) {
            let mut b = 0;
            while b < owners.len() {
                // only SHADOWING: the other trait is a SUBTRAIT of the blanket one (`trait Downstream: Upstream`)
                let mut sup = String::from(",");
                sup.push_str(owners[a].as_str());
                sup.push(',');
                if b != a
                    && meths[b].as_str() == meths[a].as_str()
                    && owners[b].as_str() != owners[a].as_str()
                    && heads[b].contains(sup.as_str())
                {
                    p.perr = 1;
                    p.perrexp = String::from("no method name shared between a blanket-implemented trait and another trait (supertrait shadowing is not modelled)");
                    p.perrgot = String::from(meths[a].as_str());
                    return;
                }
                b = b + 1;
            }
        }
        a = a + 1;
    }
}
// V4.3 USER-DEBUG (refusal > WRONG): a HAND-WRITTEN `impl Debug for T` (`impl fmt::Debug for Foo { fn fmt(..) .. }`) is not
// dispatched -- `{:?}` printed the derived shape (`Foo`) and dropped the user's output (fmt/issue-23781, basic-trait-impl).
// Refused when the program both writes such an impl and formats with `{:?}`.
fn user_debug_check(p: &mut P, t: &Vec<Tok>) {
    if p.perr != 0 || !(p.hasmain || rs_file(t)) {
        return;
    }
    let mut u = 0;
    let mut k = 0;
    while k < t.len() {
        if word_is(&t[k], "__usrc__") {
            u = k + 1;
        }
        k = k + 1;
    }
    let mut has_impl = 0;
    let mut has_dbg = false;
    let mut i = u;
    while i + 2 < t.len() {
        if tkv_impl(&t[i].k) {
            // `impl [<..>] [fmt::]Debug for`
            let mut q = i + 1;
            if tkv_lt(&t[q].k) {
                let mut d = 0;
                while q < t.len() {
                    if tkv_lt(&t[q].k) {
                        d = d + 1;
                    } else if tkv_gt(&t[q].k) {
                        d = d - 1;
                        if d == 0 {
                            break;
                        }
                    }
                    q = q + 1;
                }
                q = q + 1;
            }
            while q + 1 < t.len() && tkv_path_sep(&t[q + 1].k) {
                q = q + 2;
            }
            if q + 1 < t.len() && word_is(&t[q], "Debug") && tkv_for_t(&t[q + 1].k) && has_impl == 0
            {
                has_impl = i;
            }
        }
        if let Tk::StrL(s) = &t[i].k {
            if s.contains("{:?") || s.contains(":?}") || s.contains(":#?}") {
                has_dbg = true;
            }
        }
        i = i + 1;
    }
    if has_impl > 0 && has_dbg {
        p.perr = t[has_impl].lo + 1;
        p.perrexp = String::from(
            "a derived Debug (a hand-written `impl Debug` is not dispatched by `{:?}` here)",
        );
        p.perrgot = String::from("impl Debug");
    }
}
