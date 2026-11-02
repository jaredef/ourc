pub enum E {
    Cast(Box<E>, String, u32, u32),
    Ref(Box<E>, bool, u32, u32),
    BreakE(u32, u32),
    BreakN(u32, u32, u32),
    ContN(u32, u32, u32),
    ContE(u32, u32),
    Try(Box<E>, u32, u32),
    ClosRef(String, Vec<String>, u32, u32),
    TupLit(Vec<E>, u32, u32),
    TupIdx(Box<E>, u32, u32, u32),
    RetE(Box<E>, u32, u32),
    StrLit(String, u32, u32),
    MethodCall(Box<E>, String, Vec<E>, u32, u32),
    AssignTo(Box<E>, Box<E>, u32, u32),
    ArrLit(Vec<E>, u32, u32),
    Index(Box<E>, Box<E>, u32, u32),
    EnumLit(String, String, Vec<E>, u32, u32),
    Match(Box<E>, Vec<Arm>, u32, u32),
    Not(Box<E>, u32, u32),
    StructLit(String, Vec<E>, u32, u32),
    Field(Box<E>, String, u32, u32),
    While(Box<E>, Box<E>, u32, u32),
    Seq(Box<E>, Box<E>, u32, u32),
    Assign(String, Box<E>, u32, u32),
    Call(String, Vec<E>, u32, u32),
    Int(i64, u32, u32),
    Bool(bool, u32, u32),
    Var(String, u32, u32),
    Bin(String, Box<E>, Box<E>, u32, u32),
    If(Box<E>, Box<E>, Box<E>, u32, u32),
    Let(String, Box<E>, Box<E>, u32, u32),
    LetT(String, String, Box<E>, Box<E>, u32, u32),
}
pub struct Sdef {
    pub copy: bool,
    pub sgp: Vec<String>,
    pub name: String,
    pub fields: Vec<String>,
    pub ftys: Vec<String>,
}
pub struct Vdef {
    pub name: String,
    pub npayload: u32,
    pub ptys: Vec<String>,
    pub fnames: Vec<String>,
}
pub struct Edef {
    pub name: String,
    pub variants: Vec<Vdef>,
}
pub struct Arm {
    pub slit: String,
    pub has_slit: bool,
    pub lit: i64,
    pub has_lit: bool,
    pub has_guard: bool,
    pub guard: E,
    pub variant: String,
    pub binds: Vec<String>,
    pub body: E,
    pub wild: bool,
}
pub struct Fun {
    pub pref: Vec<bool>,
    pub ncap: i32,
    pub gbounds: Vec<String>,
    pub dflt: i32,
    pub name: String,
    pub params: Vec<String>,
    pub ptys: Vec<String>,
    pub ret: String,
    pub gparams: Vec<String>,
    pub owner: String,
    pub body: E,
    pub gsub: Vec<String>, // G1 G-P1: a monomorphized instance's parameter -> type map ("T=u16"); empty otherwise
}
pub struct Tdef {
    pub name: String,
    pub mnames: Vec<String>,
}
pub struct ImplRec {
    pub tr: String,
    pub ty: String,
    pub lo: u32,
    pub hi: u32,
}
pub struct Prog {
    pub fpays: Vec<String>,
    pub mutcaps: Vec<String>,
    pub strvals: Vec<u32>,
    pub realrs: bool,
    pub immlets: Vec<u32>,
    pub placefns: Vec<String>,
    pub placeidx: Vec<i32>,
    pub perrexp: String,
    pub perrgot: String,
    pub perr: u32,
    pub badfmt: u32,
    pub trailing: u32,
    pub sdefs: Vec<Sdef>,
    pub edefs: Vec<Edef>,
    pub funs: Vec<Fun>,
    pub tdefs: Vec<Tdef>,
    pub impls: Vec<ImplRec>,
    pub main: E,
}
pub fn tridx(ts: &Vec<Tdef>, n: &str) -> i32 {
    let mut i = 0;
    while i < ts.len() {
        if ts[i].name.as_str() == n {
            return i as i32;
        }
        i = i + 1;
    }
    0 - 1
}
// A default trait method is MATERIALIZED per implementing type as its own Fun, but its BODY is not copied:
// `dflt` indexes the Fun whose body to use (-1 = own body). Keeps the AST copy-free while still giving each
// implementing type its own typed entry, so `self` inside a default body is the CONCRETE type and any
// self-method call in it resolves per-type (the shared-body alternative reintroduces dup dispatch).
// A generic parameter carrying a trait bound is MONOMORPHIZED: `fn f<T: Area>(x: T)` called with a `Sq`
// materializes `f$Sq` with T substituted, body shared via `dflt` (same device as a trait default). Unbounded
// generics keep the single untyped body, so nothing that already worked is re-routed.
pub fn has_bound(f: &Fun) -> bool {
    let mut i = 0;
    while i < f.gbounds.len() {
        if f.gbounds[i].len() > 0 {
            return true;
        }
        i = i + 1;
    }
    false
}
pub fn ends_with_seg(t: &str, g: &str) -> bool {
    let a = t.as_bytes();
    let b = g.as_bytes();
    if b.len() + 1 > a.len() {
        return false;
    }
    let base = a.len() - b.len();
    if a[base - 1] != 36 {
        return false;
    }
    let mut i = 0;
    while i < b.len() {
        if a[base + i] != b[i] {
            return false;
        }
        i = i + 1;
    }
    true
}
// `Vec$T` + `Vec$int` -> `int`: the concrete type the generic parameter stands for
pub fn unify_seg(pty: &str, aty: &str, g: &str) -> String {
    if pty == g {
        return String::from(aty);
    }
    // UNIFY-INNER: `tup2$T$T` + `tup2$int$int` -> `int` -- g in a NON-final position. Same segment count, every other
    // segment equal (or another single-letter generic), every g position binding the same concrete; else unresolved (a refusal)
    {
        let ps: Vec<&str> = pty.split('$').collect();
        let as2: Vec<&str> = aty.split('$').collect();
        if ps.len() > 1 && ps.len() == as2.len() {
            let mut got = String::from("");
            let mut ok = true;
            let mut i = 0;
            while i < ps.len() {
                if ps[i] == g {
                    if got.len() == 0 {
                        got = String::from(as2[i]);
                    } else if got.as_str() != as2[i] {
                        ok = false;
                    }
                } else if ps[i] != as2[i]
                    && !(ps[i].len() == 1 && ps[i].as_bytes()[0] >= 65 && ps[i].as_bytes()[0] <= 90)
                {
                    ok = false;
                }
                i = i + 1;
            }
            if ok && got.len() > 0 {
                let gb = got.as_bytes();
                let mut tl = false;
                let mut z = 0;
                while z < gb.len() {
                    if gb[z] == 126 {
                        tl = true;
                    }
                    z = z + 1;
                }
                if !tl {
                    return got;
                }
            }
        }
    } // a nested composite (`Vec~int`) has no instance symbol: refuse
    if !ends_with_seg(pty, g) {
        return String::from("");
    }
    let keep = pty.len() - g.len();
    if aty.len() <= keep {
        return String::from("");
    }
    let pb = pty.as_bytes();
    let ab = aty.as_bytes();
    let mut i = 0;
    while i < keep {
        if pb[i] != ab[i] {
            return String::from("");
        }
        i = i + 1;
    }
    String::from(&aty[keep..])
}
pub fn subst_seg(t: &str, g: &str, conc: &str) -> String {
    if t == g {
        return String::from(conc);
    }
    // UNIFY-INNER: substitute EVERY `$` segment equal to g (`tup2$T$T` -> `tup2$str$str`), not just the last
    if has_seg(t, g) {
        let ps: Vec<&str> = t.split('$').collect();
        let mut r = String::from("");
        let mut i = 0;
        while i < ps.len() {
            if i > 0 {
                r.push('$');
            }
            if ps[i] == g {
                r.push_str(conc);
            } else {
                r.push_str(ps[i]);
            }
            i = i + 1;
        }
        return r;
    }
    String::from(t)
}
pub fn has_seg(t: &str, g: &str) -> bool {
    let ps: Vec<&str> = t.split('$').collect();
    let mut i = 0;
    while i < ps.len() {
        if ps[i] == g {
            return true;
        }
        i = i + 1;
    }
    false
} // UNIFY-INNER: g is any `$` segment of t
pub fn has_composite_gp(f: &Fun) -> bool {
    let mut i = 0;
    while i < f.gparams.len() {
        let mut k = 0;
        while k < f.ptys.len() {
            if ends_with_seg(f.ptys[k].as_str(), f.gparams[i].as_str()) {
                return true;
            }
            k = k + 1;
        }
        i = i + 1;
    }
    false
}
pub fn subst_ty(t: &str, gps: &Vec<String>, gbs: &Vec<String>, conc: &str) -> String {
    let mut i = 0;
    while i < gps.len() {
        if gps[i].as_str() == t && gbs[i].len() > 0 {
            return String::from(conc);
        }
        i = i + 1;
    }
    String::from(t)
}
// MEM-LIT: bytes, not ten string literals (each literal evaluation allocates under SEM0)
pub fn is_int_width(t: &str) -> bool {
    let b = t.as_bytes();
    let n = b.len();
    if n < 2 || n > 5 || (b[0] != 117 && b[0] != 105) {
        return false;
    }
    if n == 2 {
        return b[1] == 56;
    }
    if n == 3 {
        return (b[1] == 49 && b[2] == 54)
            || (b[1] == 51 && b[2] == 50)
            || (b[1] == 54 && b[2] == 52);
    }
    n == 5 && b[1] == 115 && b[2] == 105 && b[3] == 122 && b[4] == 101
}
pub fn is_known_ty2(ss: &Vec<Sdef>, ez: &Vec<Edef>, t: &str) -> bool {
    if t == "int"
        || t == "str"
        || t == "bool"
        || t == "f64"
        || t == "char"
        || t == "arr"
        || t == "Vec"
    {
        return true;
    }
    if t.as_bytes().len() > 4 {
        let b = t.as_bytes();
        if b[0] == 86 && b[1] == 101 && b[2] == 99 && b[3] == 36 {
            return true;
        }
    }
    if is_int_width(t) {
        return true;
    }
    if is_struct_name(ss, t) {
        return true;
    }
    if is_enum_name(ez, t) {
        return true;
    }
    let ta = tup_arity(t);
    if ta >= 0 {
        return true;
    }
    false
}
pub fn width_bits(t: &str) -> u32 {
    if t == "u8" || t == "i8" {
        8
    } else if t == "u16" || t == "i16" {
        16
    } else if t == "u32" || t == "i32" {
        32
    } else {
        64
    }
}
pub fn width_signed(t: &str) -> bool {
    let b = t.as_bytes();
    if b.len() == 0 {
        return true;
    }
    b[0] == 105
}
pub fn is_copy_struct(ss: &Vec<Sdef>, n: &str) -> bool {
    let mut i = 0;
    while i < ss.len() {
        if ss[i].name.as_str() == n {
            return ss[i].copy;
        }
        i = i + 1;
    }
    false
}
pub fn str_contains2(s: &str, p: &str) -> bool {
    let a = s.as_bytes();
    let b = p.as_bytes();
    if b.len() == 0 {
        return true;
    }
    if b.len() > a.len() {
        return false;
    }
    let mut i = 0;
    while i + b.len() <= a.len() {
        let mut k = 0;
        while k < b.len() && a[i + k] == b[k] {
            k = k + 1;
        }
        if k == b.len() {
            return true;
        }
        i = i + 1;
    }
    false
}
pub fn str_starts(s: &str, p: &str) -> bool {
    let a = s.as_bytes();
    let b = p.as_bytes();
    if b.len() > a.len() {
        return false;
    }
    let mut i = 0;
    while i < b.len() {
        if a[i] != b[i] {
            return false;
        }
        i = i + 1;
    }
    true
}
pub fn in_u32(v: &Vec<u32>, n: u32) -> bool {
    let mut i = 0;
    while i < v.len() {
        if v[i] == n {
            return true;
        }
        i = i + 1;
    }
    false
}
pub fn in_list(v: &Vec<String>, n: &str) -> bool {
    let mut i = 0;
    while i < v.len() {
        if v[i].as_str() == n {
            return true;
        }
        i = i + 1;
    }
    false
}
// SPAWN-PANIC (heavy pass 2): every callee NAME reachable in e -- calls, closure values, method names ("." + name, matched
// against `Type$name` user methods by the caller). A full walk over every E form (no default arm), so nothing is missed.
pub fn collect_calls(e: &E, out: &mut Vec<String>) {
    match e {
        E::Call(n, a, _l, _h) => {
            out.push(String::from(n.as_str()));
            calls_list(a, out);
        }
        E::MethodCall(r, m, a, _l, _h) => {
            let mut s = String::from(".");
            s.push_str(m.as_str());
            out.push(s);
            collect_calls(r, out);
            calls_list(a, out);
        }
        E::ClosRef(n, _c, _l, _h) => out.push(String::from(n.as_str())),
        E::Cast(x, _t, _l, _h) => collect_calls(x, out),
        E::Ref(x, _m, _l, _h) => collect_calls(x, out),
        E::Try(x, _l, _h) => collect_calls(x, out),
        E::TupIdx(x, _i, _l, _h) => collect_calls(x, out),
        E::RetE(x, _l, _h) => collect_calls(x, out),
        E::Not(x, _l, _h) => collect_calls(x, out),
        E::Field(x, _f, _l, _h) => collect_calls(x, out),
        E::Assign(_n, v, _l, _h) => collect_calls(v, out),
        E::AssignTo(p, v, _l, _h) => {
            collect_calls(p, out);
            collect_calls(v, out);
        }
        E::Index(a, b, _l, _h) => {
            collect_calls(a, out);
            collect_calls(b, out);
        }
        E::Seq(a, b, _l, _h) => {
            collect_calls(a, out);
            collect_calls(b, out);
        }
        E::Bin(_o, a, b, _l, _h) => {
            collect_calls(a, out);
            collect_calls(b, out);
        }
        E::While(a, b, _l, _h) => {
            collect_calls(a, out);
            collect_calls(b, out);
        }
        E::Let(_n, v, b, _l, _h) => {
            collect_calls(v, out);
            collect_calls(b, out);
        }
        E::LetT(_n, _t, v, b, _l, _h) => {
            collect_calls(v, out);
            collect_calls(b, out);
        }
        E::If(c, t, f, _l, _h) => {
            collect_calls(c, out);
            collect_calls(t, out);
            collect_calls(f, out);
        }
        E::TupLit(a, _l, _h) => calls_list(a, out),
        E::ArrLit(a, _l, _h) => calls_list(a, out),
        E::EnumLit(_t, _v, a, _l, _h) => calls_list(a, out),
        E::StructLit(_s, a, _l, _h) => calls_list(a, out),
        E::Match(sc, arms, _l, _h) => {
            collect_calls(sc, out);
            let mut k = 0;
            while k < arms.len() {
                if arms[k].has_guard {
                    collect_calls(&arms[k].guard, out);
                }
                collect_calls(&arms[k].body, out);
                k = k + 1;
            }
        }
        E::BreakE(_l, _h) => {}
        E::BreakN(_n, _l, _h) => {}
        E::ContE(_l, _h) => {}
        E::ContN(_n, _l, _h) => {}
        E::StrLit(_s, _l, _h) => {}
        E::Int(_v, _l, _h) => {}
        E::Bool(_v, _l, _h) => {}
        E::Var(_n, _l, _h) => {}
    }
}
fn calls_list(a: &Vec<E>, out: &mut Vec<String>) {
    let mut k = 0;
    while k < a.len() {
        collect_calls(&a[k], out);
        k = k + 1;
    }
}
pub fn collect_free(e: &E, bound: &Vec<String>, out: &mut Vec<String>) {
    match e {
        E::Var(n, _l, _h) => {
            if !in_list(bound, n) && !in_list(out, n) {
                out.push(String::from(n.as_str()));
            }
        }
        E::Let(n, v, b, _l, _h) => {
            collect_free(v, bound, out);
            let mut b2: Vec<String> = Vec::new();
            let mut i = 0;
            while i < bound.len() {
                b2.push(String::from(bound[i].as_str()));
                i = i + 1;
            }
            b2.push(String::from(n.as_str()));
            collect_free(b, &b2, out);
        }
        E::LetT(n, _t, v, b, _l, _h) => {
            collect_free(v, bound, out);
            let mut b2: Vec<String> = Vec::new();
            let mut i = 0;
            while i < bound.len() {
                b2.push(String::from(bound[i].as_str()));
                i = i + 1;
            }
            b2.push(String::from(n.as_str()));
            collect_free(b, &b2, out);
        }
        E::Match(sc, arms, _l, _h) => {
            collect_free(sc, bound, out);
            {
                let mut gi = 0;
                while gi < arms.len() {
                    if arms[gi].has_guard {
                        collect_free(&arms[gi].guard, bound, out);
                    }
                    gi = gi + 1;
                }
            }
            let mut a = 0;
            while a < arms.len() {
                let mut b2: Vec<String> = Vec::new();
                let mut i = 0;
                while i < bound.len() {
                    b2.push(String::from(bound[i].as_str()));
                    i = i + 1;
                }
                let mut k = 0;
                while k < arms[a].binds.len() {
                    b2.push(String::from(arms[a].binds[k].as_str()));
                    k = k + 1;
                }
                collect_free(&arms[a].body, &b2, out);
                a = a + 1;
            }
        }
        E::ClosRef(_n, caps, _l, _h) => {
            let mut i = 0;
            while i < caps.len() {
                if !in_list(bound, &caps[i]) && !in_list(out, &caps[i]) {
                    out.push(String::from(caps[i].as_str()));
                }
                i = i + 1;
            }
        }
        E::Seq(a, b, _l, _h) => {
            collect_free(a, bound, out);
            collect_free(b, bound, out);
        }
        E::Bin(_o, a, b, _l, _h) => {
            collect_free(a, bound, out);
            collect_free(b, bound, out);
        }
        E::If(c, t, f, _l, _h) => {
            collect_free(c, bound, out);
            collect_free(t, bound, out);
            collect_free(f, bound, out);
        }
        E::While(c, b, _l, _h) => {
            collect_free(c, bound, out);
            collect_free(b, bound, out);
        }
        E::Call(_n, args, _l, _h) => {
            let mut k = 0;
            while k < args.len() {
                collect_free(&args[k], bound, out);
                k = k + 1;
            }
        }
        E::MethodCall(r, _m, args, _l, _h) => {
            collect_free(r, bound, out);
            let mut k = 0;
            while k < args.len() {
                collect_free(&args[k], bound, out);
                k = k + 1;
            }
        }
        E::Assign(n, v, _l, _h) => {
            if !in_list(bound, n) && !in_list(out, n) {
                out.push(String::from(n.as_str()));
            }
            collect_free(v, bound, out);
        }
        E::AssignTo(p, v, _l, _h) => {
            collect_free(p, bound, out);
            collect_free(v, bound, out);
        }
        E::Field(o, _f, _l, _h) => {
            collect_free(o, bound, out);
        }
        E::TupIdx(o, _i, _l, _h) => {
            collect_free(o, bound, out);
        }
        E::Index(b, i, _l, _h) => {
            collect_free(b, bound, out);
            collect_free(i, bound, out);
        }
        E::Not(x, _l, _h) => {
            collect_free(x, bound, out);
        }
        E::RetE(x, _l, _h) => {
            collect_free(x, bound, out);
        }
        E::Try(x, _l, _h) => {
            collect_free(x, bound, out);
        }
        E::Ref(x, _m, _l, _h) => {
            collect_free(x, bound, out);
        }
        E::Cast(x, _t, _l, _h) => {
            collect_free(x, bound, out);
        }
        E::TupLit(ps, _l, _h) => {
            let mut k = 0;
            while k < ps.len() {
                collect_free(&ps[k], bound, out);
                k = k + 1;
            }
        }
        E::ArrLit(ps, _l, _h) => {
            let mut k = 0;
            while k < ps.len() {
                collect_free(&ps[k], bound, out);
                k = k + 1;
            }
        }
        E::StructLit(_n, ps, _l, _h) => {
            let mut k = 0;
            while k < ps.len() {
                collect_free(&ps[k], bound, out);
                k = k + 1;
            }
        }
        E::EnumLit(_n, _v, ps, _l, _h) => {
            let mut k = 0;
            while k < ps.len() {
                collect_free(&ps[k], bound, out);
                k = k + 1;
            }
        }
        _ => {}
    }
}
pub fn fbody_ix(fs: &Vec<Fun>, i: usize) -> usize {
    if fs[i].dflt >= 0 {
        fs[i].dflt as usize
    } else {
        i
    }
}
pub fn sfields(ss: &Vec<Sdef>, n0: &str) -> i32 {
    let nr = sresolve(ss, n0);
    let n = nr.as_str();
    let mut i = 0;
    while i < ss.len() {
        if ss[i].name.as_str() == n {
            return ss[i].fields.len() as i32;
        }
        i = i + 1;
    }
    -1
}
pub fn find_method(fs: &Vec<Fun>, owner: &str, m: &str) -> i32 {
    let mut i = 0;
    while i < fs.len() {
        if fs[i].owner.as_str() == owner && fs[i].name.as_str() == m {
            return i as i32;
        }
        i = i + 1;
    }
    -1
}
pub fn fgparams(fs: &Vec<Fun>, n: &str) -> i32 {
    let mut i = 0;
    while i < fs.len() {
        if fs[i].name.as_str() == n {
            return fs[i].gparams.len() as i32;
        }
        i = i + 1;
    }
    -1
}
// `n` is a bare (unqualified, non-empty) name
pub fn sfx_bare(n: &str) -> bool {
    let b = n.as_bytes();
    if b.len() == 0 {
        return false;
    }
    let mut i = 0;
    while i < b.len() {
        if b[i] == 36 {
            return false;
        }
        i = i + 1;
    }
    true
}
// `m` is a module-qualified registration of the bare name `n` (`a$b$n`)
pub fn sfx_match(m: &str, n: &str) -> bool {
    let mb = m.as_bytes();
    let nb = n.as_bytes();
    if mb.len() <= nb.len() + 1 {
        return false;
    }
    let base = mb.len() - nb.len();
    if mb[base - 1] != 36 {
        return false;
    }
    let mut j = 0;
    while j < nb.len() {
        if mb[base + j] != nb[j] {
            return false;
        }
        j = j + 1;
    }
    true
}
// a bare type name matches a module-qualified registration when exactly one such registration exists
pub fn sfx_unique(names: &Vec<String>, n: &str) -> i32 {
    if !sfx_bare(n) {
        return 0 - 1;
    }
    let mut hit = 0 - 1;
    let mut seen = 0;
    let mut i = 0;
    while i < names.len() {
        if sfx_match(names[i].as_str(), n) {
            hit = i as i32;
            seen = seen + 1;
        }
        i = i + 1;
    }
    if seen == 1 {
        hit
    } else {
        0 - 1
    }
}
// MEM-LIT: sfx_unique over the struct / enum registrations in place (snames_of / enames_of copied every name per lookup)
pub fn sfx_unique_s(ss: &Vec<Sdef>, n: &str) -> i32 {
    if !sfx_bare(n) {
        return 0 - 1;
    }
    let mut hit = 0 - 1;
    let mut seen = 0;
    let mut i = 0;
    while i < ss.len() {
        if sfx_match(ss[i].name.as_str(), n) {
            hit = i as i32;
            seen = seen + 1;
        }
        i = i + 1;
    }
    if seen == 1 {
        hit
    } else {
        0 - 1
    }
}
pub fn sfx_unique_e(es: &Vec<Edef>, n: &str) -> i32 {
    if !sfx_bare(n) {
        return 0 - 1;
    }
    let mut hit = 0 - 1;
    let mut seen = 0;
    let mut i = 0;
    while i < es.len() {
        if sfx_match(es[i].name.as_str(), n) {
            hit = i as i32;
            seen = seen + 1;
        }
        i = i + 1;
    }
    if seen == 1 {
        hit
    } else {
        0 - 1
    }
}
// DBG-PAYLOAD: a payload-typed `Option$T` / `Result$T$E` (a let slot type) names the base enum everywhere it is looked up
pub fn pay_base(t: &str) -> String {
    if str_starts(t, "Option$") {
        return String::from("Option");
    }
    if str_starts(t, "Result$") {
        return String::from("Result");
    }
    String::from(t)
}
pub fn eidx(es: &Vec<Edef>, n0: &str) -> i32 {
    let nb = pay_base(n0);
    let n = nb.as_str();
    let mut i = 0;
    while i < es.len() {
        if es[i].name.as_str() == n {
            return i as i32;
        }
        i = i + 1;
    }
    {
        let k = sfx_unique_e(es, n);
        if k >= 0 {
            return k;
        }
    }
    -1
}
// the SUCCESS variants of the Try desugar: `Ok`/`Some` yield their payload, anything else is returned as-is
pub fn is_ok_variant(n: &str) -> bool {
    n == "Ok" || n == "Some"
}
pub fn variant_name(es: &Vec<Edef>, en: &str, k: usize) -> String {
    let mut i = 0;
    while i < es.len() {
        if es[i].name.as_str() == en {
            if k < es[i].variants.len() {
                return String::from(es[i].variants[k].name.as_str());
            }
            return String::from("");
        }
        i = i + 1;
    }
    String::from("")
}
pub fn variant_payload_n(es: &Vec<Edef>, en: &str, k: usize) -> u32 {
    let mut i = 0;
    while i < es.len() {
        if es[i].name.as_str() == en {
            if k < es[i].variants.len() {
                return es[i].variants[k].npayload;
            }
            return 0;
        }
        i = i + 1;
    }
    0
}
pub fn vidx(es: &Vec<Edef>, en: &str, v: &str) -> i32 {
    let en = &{
        let k = sfx_unique_e(es, en);
        if k >= 0 {
            String::from(es[k as usize].name.as_str())
        } else {
            String::from(en)
        }
    };
    let mut i = 0;
    while i < es.len() {
        if es[i].name.as_str() == en {
            let mut k = 0;
            while k < es[i].variants.len() {
                if es[i].variants[k].name.as_str() == v {
                    return k as i32;
                }
                k = k + 1;
            }
            return -1;
        }
        i = i + 1;
    }
    -1
}
// the declared TYPE of a variant's payload slot (empty = untyped), which is what makes a recursive ADT
// usable: a match binding gets the declared type, so `Add(a, b) => eval(a) + eval(b)` type-checks
pub fn vpayload_ty(es: &Vec<Edef>, en: &str, v: &str, k: usize) -> String {
    let en = &{
        let k = sfx_unique_e(es, en);
        if k >= 0 {
            String::from(es[k as usize].name.as_str())
        } else {
            String::from(en)
        }
    };
    let mut i = 0;
    while i < es.len() {
        if es[i].name.as_str() == en {
            let mut q = 0;
            while q < es[i].variants.len() {
                if es[i].variants[q].name.as_str() == v {
                    if k < es[i].variants[q].ptys.len() {
                        return String::from(es[i].variants[q].ptys[k].as_str());
                    }
                    return String::from("");
                }
                q = q + 1;
            }
            return String::from("");
        }
        i = i + 1;
    }
    String::from("")
}
pub fn max_payload(es: &Vec<Edef>, en: &str) -> u32 {
    let mut i = 0;
    while i < es.len() {
        if es[i].name.as_str() == en {
            let mut m = 0;
            let mut k = 0;
            while k < es[i].variants.len() {
                if es[i].variants[k].npayload > m {
                    m = es[i].variants[k].npayload;
                }
                k = k + 1;
            }
            return m;
        }
        i = i + 1;
    }
    0
}
// C-ENRESOLVE (R2a P5): vpayload resolves an UNQUALIFIED enum name (`E` for `l$E`) the way vpayload_ty / vidx / eidx
// do -- a struct-variant literal `l::E::B { n: 2 }` carries the bare owner, and its variant was "not found"
pub fn vpayload(es: &Vec<Edef>, en: &str, v: &str) -> i32 {
    let en = &{
        let k = sfx_unique_e(es, en);
        if k >= 0 {
            String::from(es[k as usize].name.as_str())
        } else {
            String::from(en)
        }
    };
    let mut i = 0;
    while i < es.len() {
        if es[i].name.as_str() == en.as_str() {
            let mut k = 0;
            while k < es[i].variants.len() {
                if es[i].variants[k].name.as_str() == v {
                    return es[i].variants[k].npayload as i32;
                }
                k = k + 1;
            }
            return -1;
        }
        i = i + 1;
    }
    -1
}
// C-ENRESOLVE: an enum name reaching us may be UNQUALIFIED (`E`) while the definition is module-
// qualified (`oast$E`). vidx and vpayload_ty resolved it, eidx resolved it, and nvariants did NOT --
// so a match whose scrutinee type was the bare name reported "not an enum", took the if-chain path
// with is_en false, emitted NO tag test (always-true arm) and typed no payload binding.
pub fn nvariants(es: &Vec<Edef>, en: &str) -> i32 {
    let i = eidx(es, en);
    if i < 0 {
        return -1;
    }
    es[i as usize].variants.len() as i32
}
// C-ENVARAMBIG: ONE variant name does not identify an enum. olex's token enum has a `Let` variant and
// so does oast::E, and the first-hit scan answered olex$Tk for every match on an E -- so the arm tests
// compared oast$E tags against Tk indices, no arm ever matched, and the wildcard swallowed the match.
// ALL the arms' names together do identify it: the enum must contain every one of them, uniquely.
pub fn enum_of_variants(es: &Vec<Edef>, vs: &Vec<String>) -> String {
    let mut found = String::from("");
    let mut n = 0;
    let mut i = 0;
    while i < es.len() {
        let mut all = true;
        let mut any = false;
        let mut q = 0;
        while q < vs.len() {
            if vs[q].len() > 0 && vs[q].as_str() != "_" {
                any = true;
                let mut hit = false;
                let mut k = 0;
                while k < es[i].variants.len() {
                    if es[i].variants[k].name.as_str() == vs[q].as_str() {
                        hit = true;
                    }
                    k = k + 1;
                }
                if !hit {
                    all = false;
                }
            }
            q = q + 1;
        }
        if any && all {
            n = n + 1;
            found = String::from(es[i].name.as_str());
        }
        i = i + 1;
    }
    if n == 1 {
        return found;
    }
    String::from("")
}
pub fn enum_of_variant(es: &Vec<Edef>, v: &str) -> String {
    let mut i = 0;
    while i < es.len() {
        let mut k = 0;
        while k < es[i].variants.len() {
            if es[i].variants[k].name.as_str() == v {
                return String::from(es[i].name.as_str());
            }
            k = k + 1;
        }
        i = i + 1;
    }
    String::from("")
}
pub fn sfield_ty(ss: &Vec<Sdef>, n: &str, f: &str) -> String {
    {
        let k = sfx_unique_s(ss, n);
        if k >= 0 {
            return sfield_ty_at(ss, k as usize, f);
        }
    }
    let mut i = 0;
    while i < ss.len() {
        if ss[i].name.as_str() == n {
            let mut k = 0;
            while k < ss[i].fields.len() {
                if ss[i].fields[k].as_str() == f {
                    return String::from(ss[i].ftys[k].as_str());
                }
                k = k + 1;
            }
            return String::from("");
        }
        i = i + 1;
    }
    String::from("")
}
// the type of a field by POSITION (the zero rule walks a struct's fields in declaration order)
pub fn sfield_ty_ix(ss: &Vec<Sdef>, n: &str, k: usize) -> String {
    let nr = sresolve(ss, n);
    let mut i = 0;
    while i < ss.len() {
        if ss[i].name.as_str() == nr.as_str() {
            if k < ss[i].ftys.len() {
                return String::from(ss[i].ftys[k].as_str());
            }
            return String::from("");
        }
        i = i + 1;
    }
    String::from("")
}
pub fn sfield_ty_at(ss: &Vec<Sdef>, i: usize, f: &str) -> String {
    let mut k = 0;
    while k < ss[i].fields.len() {
        if ss[i].fields[k].as_str() == f {
            if k < ss[i].ftys.len() {
                return String::from(ss[i].ftys[k].as_str());
            }
            return String::from("");
        }
        k = k + 1;
    }
    String::from("")
}
// C-TYRESOLVE: a struct declared in one module is registered under its module-qualified name (`a$S`),
// but a function in ANOTHER module records its return type as the bare `S` it wrote. Instrumented at the
// failing field read, the recorded type of the receiving local was EMPTY: fret_of answered `S`,
// is_known_ty said no, and the ety chain discarded it. So the resolution belongs where a type NAME is
// validated, not where a field is looked up (rung 70 put it in the field helpers and nothing changed).
pub fn sresolve(ss: &Vec<Sdef>, n: &str) -> String {
    let mut i = 0;
    while i < ss.len() {
        if ss[i].name.as_str() == n {
            return String::from(n);
        }
        i = i + 1;
    }
    let k = sfx_unique_s(ss, n);
    if k >= 0 {
        return String::from(ss[k as usize].name.as_str());
    }
    String::from(n)
}
pub fn sfield_idx(ss: &Vec<Sdef>, n0: &str, f: &str) -> i32 {
    let nr = sresolve(ss, n0);
    let n = nr.as_str();
    let mut i = 0;
    while i < ss.len() {
        if ss[i].name.as_str() == n {
            let mut k = 0;
            while k < ss[i].fields.len() {
                if ss[i].fields[k].as_str() == f {
                    return k as i32;
                }
                k = k + 1;
            }
            return -1;
        }
        i = i + 1;
    }
    -1
}
pub fn e_lo(e: &E) -> u32 {
    match e {
        E::ContN(_n, l, _h) => *l,
        E::BreakN(_n, l, _h) => *l,
        E::Cast(_x, _t, l, _h) => *l,
        E::Ref(_x, _m, l, _h) => *l,
        E::BreakE(l, _h) => *l,
        E::ContE(l, _h) => *l,
        E::Try(_x, l, _h) => *l,
        E::ClosRef(_n, _c, l, _h) => *l,
        E::LetT(_n, _t, _v, _b, l, _h) => *l,
        E::TupLit(_x, l, _h) => *l,
        E::TupIdx(_b, _i, l, _h) => *l,
        E::RetE(_x, l, _h) => *l,
        E::StrLit(_s, l, _h) => *l,
        E::MethodCall(_r, _n, _a, l, _h) => *l,
        E::AssignTo(_p, _v, l, _h) => *l,
        E::ArrLit(_x, l, _h) => *l,
        E::Index(_b, _i, l, _h) => *l,
        E::EnumLit(_e2, _v, _p, l, _h) => *l,
        E::Match(_s, _a, l, _h) => *l,
        E::Not(_x, l, _h) => *l,
        E::StructLit(_n, _f, l, _h) => *l,
        E::Field(_o, _f, l, _h) => *l,
        E::While(_c, _b, l, _h) => *l,
        E::Seq(_a, _b, l, _h) => *l,
        E::Assign(_n, _v, l, _h) => *l,
        E::Call(_n, _a, l, _h) => *l,
        E::Int(_v, l, _h) => *l,
        E::Bool(_v, l, _h) => *l,
        E::Var(_n, l, _h) => *l,
        E::Bin(_o, _a, _b, l, _h) => *l,
        E::If(_c, _t, _f, l, _h) => *l,
        E::Let(_n, _v, _b, l, _h) => *l,
    }
}
pub fn e_hi(e: &E) -> u32 {
    match e {
        E::ContN(_n, _l, h) => *h,
        E::BreakN(_n, _l, h) => *h,
        E::Cast(_x, _t, _l, h) => *h,
        E::Ref(_x, _m, _l, h) => *h,
        E::BreakE(_l, h) => *h,
        E::ContE(_l, h) => *h,
        E::Try(_x, _l, h) => *h,
        E::ClosRef(_n, _c, _l, h) => *h,
        E::LetT(_n, _t, _v, _b, _l, h) => *h,
        E::TupLit(_x, _l, h) => *h,
        E::TupIdx(_b, _i, _l, h) => *h,
        E::RetE(_x, _l, h) => *h,
        E::StrLit(_s, _l, h) => *h,
        E::MethodCall(_r, _n, _a, _l, h) => *h,
        E::AssignTo(_p, _v, _l, h) => *h,
        E::ArrLit(_x, _l, h) => *h,
        E::Index(_b, _i, _l, h) => *h,
        E::EnumLit(_e2, _v, _p, _l, h) => *h,
        E::Match(_s, _a, _l, h) => *h,
        E::Not(_x, _l, h) => *h,
        E::StructLit(_n, _f, _l, h) => *h,
        E::Field(_o, _f, _l, h) => *h,
        E::While(_c, _b, _l, h) => *h,
        E::Seq(_a, _b, _l, h) => *h,
        E::Assign(_n, _v, _l, h) => *h,
        E::Call(_n, _a, _l, h) => *h,
        E::Int(_v, _l, h) => *h,
        E::Bool(_v, _l, h) => *h,
        E::Var(_n, _l, h) => *h,
        E::Bin(_o, _a, _b, _l, h) => *h,
        E::If(_c, _t, _f, _l, h) => *h,
        E::Let(_n, _v, _b, _l, h) => *h,
    }
}

// shared with the typechecker and the driver: the name/type environment and two small string helpers
pub struct Env {
    pub n: Vec<String>,
    pub t: Vec<i32>,
}
pub fn vc(v: &Vec<String>) -> Vec<String> {
    let mut o: Vec<String> = Vec::new();
    let mut i = 0;
    while i < v.len() {
        o.push(String::from(v[i].as_str()));
        i = i + 1;
    }
    o
}
pub fn vci(v: &Vec<i32>) -> Vec<i32> {
    let mut o: Vec<i32> = Vec::new();
    let mut i = 0;
    while i < v.len() {
        o.push(v[i]);
        i = i + 1;
    }
    o
}
pub fn u32s2(n: u32) -> String {
    if n == 0 {
        return String::from("0");
    }
    let mut v = n;
    let mut s = String::from("");
    while v > 0 {
        let d = (v % 10) as u32;
        let c = (48 + d) as u8 as char;
        let mut t = String::from("");
        t.push(c);
        t.push_str(&s);
        s = t;
        v = v / 10;
    }
    s
}
// i64::MIN has no positive counterpart, so negating it to build digits overflows and produced a bare
// "-". The digits are built from the NEGATIVE side instead, which is total.
pub fn i64s(n: i64) -> String {
    if n == 0 {
        return String::from("0");
    }
    let neg = n < 0;
    if neg {
        let mut v = n;
        let mut s = String::from("");
        while v < 0 {
            let d = (0 - (v % 10)) as u32;
            let c = (48 + d) as u8 as char;
            let mut t = String::from("");
            t.push(c);
            t.push_str(&s);
            s = t;
            v = v / 10;
        }
        let mut t = String::from("-");
        t.push_str(&s);
        return t;
    }
    let mut u = if neg { 0 - n } else { n };
    let mut s = String::from("");
    while u > 0 {
        let d = (u % 10) as u32;
        let c = (48 + d) as u8 as char;
        let mut t = String::from("");
        t.push(c);
        t.push_str(&s);
        s = t;
        u = u / 10;
    }
    if neg {
        let mut t = String::from("-");
        t.push_str(&s);
        s = t;
    }
    s
}

// deep copies of the program tables, and name lookups over them (shared by every later phase)
pub fn copy_edefs(es: &Vec<Edef>) -> Vec<Edef> {
    let mut o: Vec<Edef> = Vec::new();
    let mut i = 0;
    while i < es.len() {
        let mut vs: Vec<Vdef> = Vec::new();
        let mut k = 0;
        while k < es[i].variants.len() {
            vs.push(Vdef {
                name: String::from(es[i].variants[k].name.as_str()),
                npayload: es[i].variants[k].npayload,
                ptys: copy_sgp(&es[i].variants[k].ptys),
                fnames: copy_sgp(&es[i].variants[k].fnames),
            });
            k = k + 1;
        }
        o.push(Edef {
            name: String::from(es[i].name.as_str()),
            variants: vs,
        });
        i = i + 1;
    }
    o
}
pub fn is_enum_name(es: &Vec<Edef>, n: &str) -> bool {
    let mut i = 0;
    while i < es.len() {
        if es[i].name.as_str() == n {
            return true;
        }
        i = i + 1;
    }
    false
}
pub fn is_struct_name(ss: &Vec<Sdef>, n: &str) -> bool {
    let mut i = 0;
    while i < ss.len() {
        if ss[i].name.as_str() == n {
            return true;
        }
        i = i + 1;
    }
    {
        if sfx_unique_s(ss, n) >= 0 {
            return true;
        }
    }
    false
}
pub fn copy_sgp(v: &Vec<String>) -> Vec<String> {
    let mut o: Vec<String> = Vec::new();
    let mut i = 0;
    while i < v.len() {
        o.push(String::from(v[i].as_str()));
        i = i + 1;
    }
    o
}
pub fn copy_sdefs(ss: &Vec<Sdef>) -> Vec<Sdef> {
    let mut o: Vec<Sdef> = Vec::new();
    let mut i = 0;
    while i < ss.len() {
        let mut fl: Vec<String> = Vec::new();
        let mut k = 0;
        while k < ss[i].fields.len() {
            fl.push(String::from(ss[i].fields[k].as_str()));
            k = k + 1;
        }
        let mut fy: Vec<String> = Vec::new();
        let mut k2 = 0;
        while k2 < ss[i].ftys.len() {
            fy.push(String::from(ss[i].ftys[k2].as_str()));
            k2 = k2 + 1;
        }
        o.push(Sdef {
            copy: ss[i].copy,
            sgp: copy_sgp(&ss[i].sgp),
            name: String::from(ss[i].name.as_str()),
            fields: fl,
            ftys: fy,
        });
        i = i + 1;
    }
    o
}
pub fn parse_i32(s: &str) -> i32 {
    let b = s.as_bytes();
    let mut v: i32 = 0;
    let mut i = 0;
    while i < b.len() {
        if b[i] >= 48 && b[i] <= 57 {
            v = v * 10 + ((b[i] - 48) as i32);
        }
        i = i + 1;
    }
    v
} // FN-ITEM-VAL
  // TUP-REST-END: a tuple-pattern slot AFTER a `..` is read from the END, but a plain tuple's arity is unknown where the
  // pattern is parsed. The parser marks such a TupIdx index (`tix_from_end(k)` = k-th from the end); every reader that knows
  // the arity resolves it with `tix_resolve(ix, arity)` -- typeck (tag 300+n), ety_of / omono (tup_elems), lowering.
pub fn tix_from_end(k: u32) -> u32 {
    2147483648 + k
}
pub fn tix_is_end(ix: u32) -> bool {
    ix >= 2147483648
}
// the 0-based slot for index ix in a tuple of `arity` elements; -1 when it cannot be (out of range)
pub fn tix_resolve(ix: u32, arity: usize) -> i64 {
    if !tix_is_end(ix) {
        if (ix as usize) < arity {
            return ix as i64;
        }
        return 0 - 1;
    }
    let k = (ix - 2147483648) as usize;
    if k == 0 || k > arity {
        return 0 - 1;
    }
    (arity - k) as i64
}
// DEREF-PLACE (R6): a copy of a FIXED place -- a local, its fields / tuple slots, a literal index. A variable index is not
// fixed (it may change after the borrow), so it is not a place for aliasing.
pub fn place_clone(e: &E) -> Option<E> {
    match e {
        E::Var(n, l, h) => Some(E::Var(String::from(n.as_str()), *l, *h)),
        E::Field(b, f, l, h) => match place_clone(b) {
            Some(c) => Some(E::Field(Box::new(c), String::from(f.as_str()), *l, *h)),
            None => None,
        },
        E::TupIdx(b, i, l, h) => match place_clone(b) {
            Some(c) => Some(E::TupIdx(Box::new(c), *i, *l, *h)),
            None => None,
        },
        E::Index(b, ix, l, h) => match ix.as_ref() {
            E::Int(v, a, c) => match place_clone(b) {
                Some(p) => Some(E::Index(Box::new(p), Box::new(E::Int(*v, *a, *c)), *l, *h)),
                None => None,
            },
            _ => None,
        },
        _ => None,
    }
}
// DEREF-PLACE: could expression e read the name n? CONSERVATIVE: any form not modelled counts as a mention.
pub fn e_mentions(e: &E, n: &str) -> bool {
    match e {
        E::Var(x, _l, _h) => x.as_str() == n,
        E::Int(_v, _l, _h) => false,
        E::Bool(_b, _l, _h) => false,
        E::StrLit(_s, _l, _h) => false,
        E::Bin(_o, a, b, _l, _h) => e_mentions(a, n) || e_mentions(b, n),
        E::Not(x, _l, _h) => e_mentions(x, n),
        E::Cast(x, _t, _l, _h) => e_mentions(x, n),
        E::Field(x, _f, _l, _h) => e_mentions(x, n),
        E::TupIdx(x, _i, _l, _h) => e_mentions(x, n),
        E::Index(x, i, _l, _h) => e_mentions(x, n) || e_mentions(i, n),
        E::Call(_f, a, _l, _h) => {
            let mut k = 0;
            while k < a.len() {
                if e_mentions(&a[k], n) {
                    return true;
                }
                k = k + 1;
            }
            false
        }
        _ => true,
    }
}
// DEREF-PLACE: the root local of a fixed place (`p` of `p.a[2].b`), "" if none
pub fn place_root(e: &E) -> String {
    match e {
        E::Var(n, _l, _h) => String::from(n.as_str()),
        E::Field(b, _f, _l, _h) => place_root(b),
        E::TupIdx(b, _i, _l, _h) => place_root(b),
        E::Index(b, _i, _l, _h) => place_root(b),
        _ => String::new(),
    }
}
// G1 G-P1: a fresh copy of a string list (Vec<String> has no Clone in this dialect)
pub fn copy_strs(v: &Vec<String>) -> Vec<String> {
    let mut o: Vec<String> = Vec::new();
    let mut i = 0;
    while i < v.len() {
        o.push(String::from(v[i].as_str()));
        i = i + 1;
    }
    o
}
