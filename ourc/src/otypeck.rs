// otypeck -- the TYPE SYSTEM: the tag lattice, type-name resolution, expression checking, call checks
pub fn env_get(e: &oast::Env, nm: &str) -> i32 {
    let mut i = e.n.len();
    while i > 0 {
        i = i - 1;
        if e.n[i].as_str() == nm {
            return e.t[i];
        }
    }
    -1
}
// the type NAME a tag stands for, for the one builtin-method table
// C-VECCOMPAT: `Vec::new()` carries no element type, so it answers the untyped Vec tag 201 and is
// compatible with ANY typed Vec (1000+i). The builtin-argument check already knew this and skipped a 201
// argument, but every other place that compares two tags for compatibility used a plain !=, so assigning
// Vec::new() into a Vec<T> place was rejected. The rule is stated once here and used at every comparison.
// C-BITNOT: rustc's `!` is the logical not on a bool and the BITWISE not on an integer. This tier has
// one scalar width (C1), so an integer `!` is faithful only where the high bits cannot escape -- the mask
// idiom `x & !m`. It is admitted under a bitwise operator and stays REFUSED elsewhere, so a directly
// printed `!7u32` keeps its clean refusal instead of answering -8 where rustc says 4294967288.
pub fn is_bitop(op: &str) -> bool {
    op == "&" || op == "|" || op == "^"
}
pub fn tyeq(a: i32, b: i32) -> bool {
    if a == b {
        return true;
    }
    if a == 201 && b >= 1000 {
        return true;
    }
    if b == 201 && a >= 1000 {
        return true;
    }
    // CLOS-VALUE: a closure literal (800..899) IS a value of the callable type Fn (900)
    if (a == 900 && b >= 800 && b < 1000) || (b == 900 && a >= 800 && a < 1000) {
        return true;
    }
    false
}
pub fn tagty(t: i32) -> String {
    if t == 4 {
        String::from("bstr")
    } else if t >= 2000 {
        String::from("arr")
    } else if t == 2 {
        String::from("str")
    } else if t == 200 {
        String::from("arr")
    } else if t == 201 || t >= 1000 {
        String::from("Vec")
    } else {
        String::from("")
    }
}
pub fn tyname2(t: i32, ss: &Vec<Sdef>) -> String {
    if t == 4 {
        return String::from("&[u8]");
    }
    if t >= 2000 {
        return String::from("[str]");
    }
    if t == 0 - 3 {
        String::from("<generic>")
    } else if t == 0 {
        String::from("int")
    } else if t == 1 {
        String::from("bool")
    } else if t == 2 {
        String::from("str")
    } else if t == 3 {
        String::from("f64")
    } else if t == 200 {
        String::from("[int]")
    } else if t == 201 {
        String::from("Vec")
    } else if t >= 1000 {
        String::from("Vec<..>")
    } else if t == 900 {
        String::from("Fn")
    } else if t >= 800 {
        String::from("closure")
    } else if t >= 400 {
        String::from("dyn")
    } else if t >= 300 {
        String::from("tuple")
    } else if t >= 100 {
        String::from("enum")
    } else if t >= 10 {
        let i = (t - 10) as usize;
        if i < ss.len() {
            String::from(ss[i].name.as_str())
        } else {
            String::from("struct")
        }
    } else {
        String::from("{unknown}")
    }
}
pub fn tyname(t: i32) -> String {
    if t == 0 {
        String::from("int")
    } else if t == 1 {
        String::from("bool")
    } else if t >= 10 {
        String::from("struct")
    } else {
        String::from("{unknown}")
    }
}
pub fn etag(ei: i32) -> i32 {
    100 + ei
}
pub fn is_gp(gps: &Vec<String>, n: &str) -> bool {
    let mut i = 0;
    while i < gps.len() {
        if gps[i].as_str() == n {
            return true;
        }
        i = i + 1;
    }
    false
}
pub fn tup_arity(tn: &str) -> i32 {
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
// `dyn Trait` is tag 400+<trait index>: a value whose concrete type is not known statically, only its trait.
pub fn dyn_name_trait(tn: &str) -> String {
    let b = tn.as_bytes();
    if b.len() < 5 {
        return String::from("");
    }
    if b[0] == 100 && b[1] == 121 && b[2] == 110 && b[3] == 36 {
        return String::from(&tn[4..]);
    }
    String::from("")
}
pub fn dyn_tag_of(tz: &Vec<Tdef>, tn: &str) -> i32 {
    let tr = dyn_name_trait(tn);
    if tr.len() == 0 {
        return 0 - 1;
    }
    let i = oast::tridx(tz, &tr);
    if i < 0 {
        return 0 - 1;
    }
    400 + i
}
// A typed vector is ONE rule: tag = 1000 + its ELEMENT's tag, so Vec<int>=1000, Vec<Sq>=1010+i,
// Vec<dyn Area>=1400+i, and the element is recovered by subtraction. (The first cut special-cased
// Vec<dyn T> at 600+i, which did not generalize to a vector of structs.)
pub fn elem_tag_of(ss: &Vec<Sdef>, ez: &Vec<Edef>, tz: &Vec<Tdef>, tn: &str) -> i32 {
    // RECURSIVE: a vector's element may itself be a vector (`Vec<Vec<int>>` = 1000 + (1000 + int))
    let v = vec_tag_of(ss, ez, tz, tn);
    if v >= 0 {
        return v;
    }
    let d = dyn_tag_of(tz, tn);
    if d >= 0 {
        return d;
    }
    let b = tag_of_tyname(ss, tn);
    if b >= 0 {
        return b;
    }
    let ei = oast::eidx(ez, tn);
    if ei >= 0 {
        return 100 + ei;
    }
    0 - 1
}
pub fn clos_idx_of_name(n: &str) -> i32 {
    let b = n.as_bytes();
    if b.len() < 6 {
        return 0 - 1;
    }
    if b[0] != 36 || b[1] != 99 || b[2] != 108 || b[3] != 111 || b[4] != 115 {
        return 0 - 1;
    }
    let mut v = 0;
    let mut i = 5;
    while i < b.len() {
        if b[i] == 36 && i > 5 {
            return v;
        }
        if b[i] < 48 || b[i] > 57 {
            return 0 - 1;
        }
        v = v * 10 + ((b[i] - 48) as i32);
        i = i + 1;
    }
    v
}
pub fn clos_name_of_tag(t: i32) -> String {
    let mut r = String::from("$clos");
    r.push_str(&oast::u32s2((t - 800) as u32));
    r
}
pub fn clos_ret_in(fz: &Vec<Fun>, n: &str) -> String {
    let mut i = 0;
    while i < fz.len() {
        if fz[i].name.as_str() == n {
            return String::from(fz[i].ret.as_str());
        }
        i = i + 1;
    }
    String::from("")
}
pub fn type_tag(ss: &Vec<Sdef>, ez: &Vec<Edef>, tz: &Vec<Tdef>, tn: &str) -> i32 {
    if tn.len() == 0 {
        return 0 - 1;
    }
    let v = vec_tag_of(ss, ez, tz, tn);
    if v >= 0 {
        return v;
    }
    let d = dyn_tag_of(tz, tn);
    if d >= 0 {
        return d;
    }
    let b = tag_of_tyname(ss, tn);
    if b >= 0 {
        return b;
    }
    let e = oast::eidx(ez, tn);
    if e >= 0 {
        return 100 + e;
    }
    0 - 1
}
pub fn vec_tag_of(ss: &Vec<Sdef>, ez: &Vec<Edef>, tz: &Vec<Tdef>, tn: &str) -> i32 {
    let el = olower::vec_elem_of(tn);
    if el.len() == 0 {
        return 0 - 1;
    }
    let e = elem_tag_of(ss, ez, tz, &el);
    if e < 0 {
        return 0 - 1;
    }
    if e >= 1000 {
        return 201;
    }
    1000 + e
}
pub fn tag_of_tyname(ss: &Vec<Sdef>, tn: &str) -> i32 {
    {
        // GEN-INST-TAG (generics): `T$i32` -- an instance of a GENERIC struct -- has its base struct's tag (the arguments are
        // recorded per binding by GEN-INST); `argument 1 of g expects T$i32, found T` was a false mismatch
        let tb = tn.as_bytes();
        let mut c = 0;
        while c < tb.len() && tb[c] != 36 {
            c = c + 1;
        }
        if c > 0 && c < tb.len() {
            let base = &tn[..c];
            let mut k = 0;
            while k < ss.len() {
                if ss[k].name.as_str() == base && ss[k].sgp.len() > 0 {
                    return tag_of_tyname(ss, base);
                }
                k = k + 1;
            }
        }
    }
    if tn == "bstr" {
        return 4;
    }
    let ta = tup_arity(tn);
    if ta >= 0 {
        return 300 + ta;
    }
    if tn == "Fn" {
        return 900;
    }
    if oast::is_int_width(tn) || tn == "char" {
        return 0;
    }
    if tn == "f64" {
        return 3;
    }
    if tn == "int" {
        0
    } else if tn == "str" {
        2
    } else if tn == "Vec" {
        201
    } else if tn == "bool" {
        1
    } else if tn == "arr" {
        200
    } else {
        let i = sidx_of(ss, tn);
        if i < 0 {
            -1
        } else {
            10 + i
        }
    }
}
pub fn sidx_of(ss: &Vec<Sdef>, n: &str) -> i32 {
    let mut i = 0;
    while i < ss.len() {
        if ss[i].name.as_str() == n {
            return i as i32;
        }
        i = i + 1;
    }
    {
        let k = oast::sfx_unique_s(ss, n);
        if k >= 0 {
            return k;
        }
    }
    -1
}
// typeck: returns the type tag (0=int,1=bool,-1=err) and PUSHES spanned diagnostics
pub fn check(e: &E, env: &mut oast::Env, ds: &mut Vec<Diag>) -> i32 {
    check_s(e, env, ds, &empty_sdefs())
}
pub fn empty_sdefs() -> Vec<Sdef> {
    let o: Vec<Sdef> = Vec::new();
    o
}
pub fn check_s(e: &E, env: &mut oast::Env, ds: &mut Vec<Diag>, ss: &Vec<Sdef>) -> i32 {
    check_f(e, env, ds, ss, &empty_funs())
}
pub fn empty_funs() -> Vec<Fun> {
    let o: Vec<Fun> = Vec::new();
    o
}
pub fn check_f(
    e: &E,
    env: &mut oast::Env,
    ds: &mut Vec<Diag>,
    ss: &Vec<Sdef>,
    fz: &Vec<Fun>,
) -> i32 {
    let ez: Vec<Edef> = Vec::new();
    let tz: Vec<Tdef> = Vec::new();
    check_e(e, env, ds, ss, fz, &ez, &tz)
}
pub fn check_e(
    e: &E,
    env: &mut oast::Env,
    ds: &mut Vec<Diag>,
    ss: &Vec<Sdef>,
    fz: &Vec<Fun>,
    ez: &Vec<Edef>,
    tz: &Vec<Tdef>,
) -> i32 {
    match e {
        E::StrLit(_v, _l, _h) => 2,
        E::BreakE(_l, _h) => 0 - 1,
        E::BreakN(_n, _l, _h) => 0 - 1,
        E::Ref(x, _m, _l, _h) => check_e(x, env, ds, ss, fz, ez, tz),
        E::Cast(x, t, l, h) => {
            let tx = check_e(x, env, ds, ss, fz, ez, tz);
            if t.as_str() == "f64" || t.as_str() == "f32" {
                if tx != 0 && tx != 1 && tx != 3 && tx >= 0 && tx != 0 - 3 {
                    let mut m = String::from("cannot cast ");
                    m.push_str(&tyname2(tx, ss));
                    m.push_str(" to ");
                    m.push_str(t);
                    ds.push(oerrors::err(m, *l, *h));
                }
                return 3;
            } // D1.1: int/f64 -> f64
            if oast::is_int_width(t) || t.as_str() == "int" || t.as_str() == "char" {
                if tx >= 100 && tx < 800 && ((tx - 100) as usize) < ez.len() {
                    let mut dn = String::from(ez[(tx - 100) as usize].name.as_str());
                    dn.push_str("$discr");
                    let mut k = 0;
                    while k < fz.len() {
                        if fz[k].name.as_str() == dn.as_str() {
                            return 0;
                        }
                        k = k + 1;
                    }
                } // ENUM-DISCR (R7): a FIELDLESS enum (it has a synthesized `Enum$discr`) casts to its discriminant
                if tx != 0 && tx != 1 && tx != 3 && tx >= 0 && tx != 0 - 3 {
                    let mut m = String::from("cannot cast ");
                    m.push_str(&tyname2(tx, ss));
                    m.push_str(" to ");
                    m.push_str(t);
                    ds.push(oerrors::err(m, *l, *h));
                }
                return 0;
            } // D1.1: f64 -> int allowed
              // only NUMERIC casts are implemented: an `as` to a trait object or struct would need a real
              // coercion, and pretending otherwise produced a garbage value and a segfault at runtime
            {
                let mut m = String::from("`as` to `");
                m.push_str(t);
                m.push_str("` is not implemented (only numeric casts)");
                ds.push(oerrors::err(m, *l, *h));
            }
            0
        }
        E::ContE(_l, _h) => 0 - 1,
        E::ContN(_n, _l, _h) => 0 - 1,
        E::Try(x, l, h) => {
            let tx = check_e(x, env, ds, ss, fz, ez, tz);
            if tx < 100 || tx >= 300 {
                if tx >= 0 {
                    let mut m = String::from("`?` needs an enum, found ");
                    m.push_str(&tyname2(tx, ss));
                    ds.push(oerrors::err(m, *l, *h));
                }
                return 0;
            }
            0
        }
        E::ClosRef(n, _c, _l, _h) => {
            let i = clos_idx_of_name(n);
            if i < 0 {
                0
            } else {
                800 + i
            }
        }
        E::LetT(n, t, v, b, l, h) => {
            let tv = check_e(v, env, ds, ss, fz, ez, tz);
            let mut td = type_tag(ss, ez, tz, t);
            if td < 0 && t.contains('$') {
                // GEN-ANN: `let t: Tag<i32> = ..` spells the INSTANCE `Tag$i32`; a let annotation erases its arguments
                // (GEN-INST records them from the value), so a generic struct's instance types as its base struct.
                // R4e: `PhantomData<T>` is the zero-sized unit -- it types as its value.
                let tb = t.as_bytes();
                let mut c = 0;
                while c < tb.len() && tb[c] != 36 {
                    c = c + 1;
                }
                let base = &t[..c];
                if base == "PhantomData" {
                    td = tv;
                } else {
                    let mut k = 0;
                    while k < ss.len() {
                        if ss[k].name.as_str() == base && ss[k].sgp.len() > 0 {
                            td = type_tag(ss, ez, tz, base);
                        }
                        k = k + 1;
                    }
                }
            }
            if td < 0 {
                let mut m = String::from("unknown type `");
                m.push_str(t);
                m.push_str("`");
                ds.push(oerrors::err(m, *l, *h));
                td = tv;
            }
            // a TRAIT OBJECT carries a tag plus a payload, so it can only be made from a value whose
            // concrete type we tag: a struct or an enum. `impl Trait for i32` then `&x as &dyn Trait`
            // handed the raw int to a tag dispatch and segfaulted -- refuse it instead.
            // P-BOX: a trait object is a {tag, payload} record, and with P-NOM the payload's concrete
            // type has a NAME, so a primitive can carry the tag too. What must still hold is that an
            // impl EXISTS for that type -- lowering refuses the untagged case, where no name matched.
            // DYN-IDENT (R2a P5): a value already of that trait-object type (`let b: Box<dyn Tr> = mk()`) needs no tag
            if td >= 400
                && td < 800
                && tv >= 0
                && !(tv >= 10 && tv < 200)
                && !(tv >= 0 && tv <= 2)
                && tv != td
            {
                let mut m = String::from("a trait object from ");
                m.push_str(&tyname2(tv, ss));
                m.push_str(" is not implemented (that type carries no tag)");
                ds.push(oerrors::err(m, *l, *h));
            }
            // TY-LET (E0308): `let x: T = v` needs v: T -- judged on the primitive tags (int 0, bool 1, str 2, f64 3) only
            if td >= 0 && td <= 3 && tv >= 0 && tv <= 3 && !tyeq(td, tv) {
                let mut m = String::from("mismatched types: expected ");
                m.push_str(&tyname2(td, ss));
                m.push_str(", found ");
                m.push_str(&tyname2(tv, ss));
                ds.push(oerrors::err(m, oast::e_lo(v), oast::e_hi(v)));
            }
            env.n.push(String::from(n.as_str()));
            env.t.push(td);
            let rb = check_e(b, env, ds, ss, fz, ez, tz);
            env.n.pop();
            env.t.pop();
            rb
        }
        E::TupLit(parts, _l, _h) => {
            let mut k = 0;
            while k < parts.len() {
                let _ = check_e(&parts[k], env, ds, ss, fz, ez, tz);
                k = k + 1;
            }
            300 + (parts.len() as i32)
        }
        E::TupIdx(b, ix, l, h) => {
            let tb = check_e(b, env, ds, ss, fz, ez, tz);
            // an UNKNOWN receiver (a lifted closure's parameter, say) cannot be checked statically; the index
            // is used directly by lowering, so permit it rather than reject valid code
            if tb == 0 - 3 {
                return 0;
            }
            if tb == 0 - 1 {
                return 0 - 1;
            } // TUP-UNKNOWN: an un-typed binding (a match payload typeck did not track) is not KNOWN to be a non-tuple -- defer
              // a TUPLE STRUCT's `.0` is a field read: its fields are named "0", "1", ...
            if tb >= 10 && tb < 100 {
                let si = (tb - 10) as usize;
                if si < ss.len() {
                    let sn = String::from(ss[si].name.as_str());
                    let fname = u32s2(*ix);
                    let fi = oast::sfield_idx(ss, &sn, &fname);
                    if fi < 0 {
                        let mut m = String::from("no field `");
                        m.push_str(&fname);
                        m.push_str("` on `");
                        m.push_str(&sn);
                        m.push_str("`");
                        ds.push(oerrors::err(m, *l, *h));
                        return 0;
                    }
                    let fty = oast::sfield_ty(ss, &sn, &fname);
                    let ft = type_tag(ss, ez, tz, &fty);
                    if ft >= 0 {
                        return ft;
                    }
                    return 0;
                }
            }
            if tb < 300 {
                let mut m = String::from("`.");
                m.push_str(&oast::u32s2(*ix));
                m.push_str("` needs a tuple, found ");
                m.push_str(&tyname2(tb, ss));
                ds.push(oerrors::err(m, *l, *h));
                return 0;
            }
            let n = tb - 300;
            if oast::tix_resolve(*ix, n as usize) < 0 {
                let mut m = String::from("tuple has ");
                m.push_str(&oast::u32s2(n as u32));
                m.push_str(" element(s), index ");
                m.push_str(&oast::u32s2(*ix));
                m.push_str(" is out of range");
                ds.push(oerrors::err(m, *l, *h));
            }
            0 - 3
        }
        E::RetE(x, _l, _h) => {
            let _ = check_e(x, env, ds, ss, fz, ez, tz);
            0 - 1
        }
        E::MethodCall(r, m, args, _lt, _ht)
            if (m.as_str() == "to_string" || m.as_str() == "to_owned") && args.len() == 0 =>
        {
            let _ = check_e(r, env, ds, ss, fz, ez, tz);
            2
        }
        // STR-INTO (R2a P5): `"m".into()` -- `From<&str> for String`, the identity in this value model (String == str).
        // Only on a str receiver: other `.into()`s need the TARGET type (int widening, a user From impl) and stay refused.
        E::MethodCall(r, m, args, lt, ht) if m.as_str() == "into" && args.len() == 0 => {
            let tr = check_e(r, env, ds, ss, fz, ez, tz);
            if tr == 2 {
                return 2;
            }
            if tr >= 0 {
                let mut msg = String::from("no method `into` on ");
                msg.push_str(&tyname2(tr, ss));
                ds.push(oerrors::err(msg, *lt, *ht));
            }
            -1
        }
        // the BUILT-IN method set lives in ONE place (`olower::builtin_method`); this arm asks it rather
        // than keeping a second copy of the names, and decides only the RESULT type
        E::MethodCall(r, m, args, l, h) => {
            let tr0 = check_e(r, env, ds, ss, fz, ez, tz);
            let tr = if tr0 == 200 { 1000 } else { tr0 }; // ARR-METH: a fixed array [int; N] has the slice methods of a Vec<int> (iter, len, contains, ...)
            let mut k = 0;
            while k < args.len() {
                let _ = check_e(&args[k], env, ds, ss, fz, ez, tz);
                k = k + 1;
            }
            // C-WRAPPING: an INT receiver has no name in the tag scheme (tagty answers ""), so the
            // wrapping methods are admitted here and the WIDTH gate lives in olower, which is the only
            // place that still knows the declared width. A narrower receiver is refused there.
            if m.as_str() == "unwrap" || m.as_str() == "expect" {
                match r.as_ref() {
                    E::MethodCall(v, cm, _va, _vl, _vh)
                        if (cm.as_str() == "first"
                            || cm.as_str() == "last"
                            || cm.as_str() == "nth"
                            || cm.as_str() == "max"
                            || cm.as_str() == "min"
                            || cm.as_str() == "pop") =>
                    {
                        let tv = check_e(v, env, ds, ss, fz, ez, tz);
                        if tv >= 1000 {
                            return tv - 1000;
                        }
                    }
                    _ => {}
                }
            } // D4.1 typed-Option chain
              // VEC-REMOVE-TY (R2a P4): `v.remove(i)` / `v.swap_remove(i)` answer the ELEMENT (a Vec<T> tag is 1000 + T) -- it was
              // int for every Vec, so `fn f(mut v: Vec<String>) -> String { v.remove(0) }` "returns str but body has int"
            if tr >= 1000
                && (m.as_str() == "remove" || m.as_str() == "swap_remove")
                && args.len() == 1
            {
                return tr - 1000;
            }
            if (m.as_str() == "wrapping_mul"
                || m.as_str() == "wrapping_add"
                || m.as_str() == "wrapping_sub")
                && args.len() == 1
                && tr == 0
            {
                return 0;
            }
            // ORD-CMP: `a.cmp(&b)` on an integer is the prelude Ordering
            if tr == 0 && m.as_str() == "cmp" && args.len() == 1 {
                let _ = check_e(&args[0], env, ds, ss, fz, ez, tz);
                let ei = oast::eidx(ez, "Ordering");
                if ei >= 0 {
                    return 100 + ei;
                }
                return 0;
            }
            // INT-METHODS: wrapping_neg, rem_euclid, wrapping_shl/shr, to_ascii_lowercase/uppercase (char is tag 0 too)
            if tr == 0
                && ((m.as_str() == "wrapping_neg" && args.len() == 0)
                    || (m.as_str() == "signum" && args.len() == 0)
                    || ((m.as_str() == "rem_euclid"
                        || m.as_str() == "div_euclid"
                        || m.as_str() == "wrapping_shl"
                        || m.as_str() == "wrapping_shr")
                        && args.len() == 1)
                    || ((m.as_str() == "to_ascii_lowercase" || m.as_str() == "to_ascii_uppercase")
                        && args.len() == 0))
            {
                let _ = check_e(r, env, ds, ss, fz, ez, tz);
                let mut k = 0;
                while k < args.len() {
                    let _ = check_e(&args[k], env, ds, ss, fz, ez, tz);
                    k = k + 1;
                }
                return 0;
            }
            if m.as_str() == "checked_div" && args.len() == 1 && tr == 0 {
                let oi = oast::eidx(ez, "Option");
                if oi >= 0 {
                    return 100 + oi;
                }
                return 0;
            } // P-CHECKEDDIV
            if (m.as_str() == "saturating_add"
                || m.as_str() == "saturating_sub"
                || m.as_str() == "saturating_mul")
                && args.len() == 1
                && tr == 0
            {
                return 0;
            } // SAT-OPS
            if (m.as_str() == "checked_add"
                || m.as_str() == "checked_sub"
                || m.as_str() == "checked_mul")
                && args.len() == 1
                && tr == 0
            {
                let oi = oast::eidx(ez, "Option");
                if oi >= 0 {
                    return 100 + oi;
                }
                return 0;
            } // D3.1
            if (m.as_str() == "count_ones"
                || m.as_str() == "leading_zeros"
                || m.as_str() == "trailing_zeros")
                && args.len() == 0
                && tr == 0
            {
                return 0;
            } // D3.2
            if m.as_str() == "to_le" && args.len() == 0 && tr == 0 {
                return 0;
            } // R4a
            if (m.as_str() == "max" || m.as_str() == "min") && args.len() == 1 && tr == 3 {
                return 3;
            } // D1.6 f64
            if (m.as_str() == "pow" || m.as_str() == "max" || m.as_str() == "min")
                && args.len() == 1
                && tr == 0
            {
                return 0;
            } // P-INT-METHODS
            if m.as_str() == "abs" && args.len() == 0 && tr == 0 {
                return 0;
            }
            if m.as_str() == "map" && tr >= 1000 {
                match &args[0] {
                    E::ClosRef(cn, _c, _cl, _ch) => {
                        let rr = clos_ret_in(fz, cn.as_str());
                        if rr.len() > 0 {
                            let t2 = type_tag(ss, ez, tz, &rr);
                            if t2 >= 0 {
                                return 1000 + t2;
                            }
                        }
                    }
                    _ => {}
                }
            } // D5
            if m.as_str() == "flat_map" && tr >= 1000 {
                match &args[0] {
                    E::ClosRef(cn, _c, _cl, _ch) => {
                        let rr = clos_ret_in(fz, cn.as_str());
                        if rr.len() > 0 {
                            let t2 = type_tag(ss, ez, tz, &rr);
                            if t2 >= 0 {
                                return t2;
                            }
                        }
                    }
                    _ => {}
                }
            } // D5   // P-INT-METHODS
            if tr == 3 && {
                let bm = olower::builtin_method("f64", m.as_str(), args.len());
                oast::str_starts(bm.as_str(), "f64m_") || oast::str_starts(bm.as_str(), "f64m2_")
            } {
                let mut k = 0;
                while k < args.len() {
                    let _ = check_e(&args[k], env, ds, ss, fz, ez, tz);
                    k = k + 1;
                }
                return 3;
            } // F64-LIBM
            if (m.as_str() == "is_nan" || m.as_str() == "is_infinite" || m.as_str() == "is_finite")
                && args.len() == 0
                && tr == 3
            {
                return 1;
            } // F64-PREDS: bool
            if m.as_str() == "signum" && args.len() == 0 && tr == 3 {
                return 3;
            } // F64-PREDS
            if (m.as_str() == "floor"
                || m.as_str() == "ceil"
                || m.as_str() == "round"
                || m.as_str() == "abs"
                || m.as_str() == "sqrt")
                && args.len() == 0
                && tr == 3
            {
                return 3;
            } // D1.2 f64
            if m.as_str() == "powi" && args.len() == 1 && tr == 3 {
                return 3;
            } // D1.4 f64
            if (m.as_str() == "is_ascii_digit"
                || m.as_str() == "is_alphabetic"
                || m.as_str() == "is_ascii_alphabetic"
                || m.as_str() == "is_alphanumeric"
                || m.as_str() == "is_ascii_alphanumeric"
                || m.as_str() == "is_whitespace"
                || m.as_str() == "is_ascii_whitespace"
                || m.as_str() == "is_numeric"
                || m.as_str() == "is_lowercase"
                || m.as_str() == "is_uppercase"
                || m.as_str() == "is_ascii_lowercase"
                || m.as_str() == "is_ascii_uppercase")
                && args.len() == 0
                && tr == 0
            {
                return 1;
            } // P-CHAR-PRED: -> bool
              // C-UNKNOWNRECV: a receiver whose type we could not name cannot name its method's
              // result either. This answered `int`, the arc's invented default, which then collided with
              // a str where a tuple element's type was genuinely unknown.
            if tr == 0 - 3 {
                return 0 - 3;
            }
            let nm2 = if tr >= 100 && tr < 200 {
                let i = (tr - 100) as usize;
                if i < ez.len() {
                    String::from(ez[i].name.as_str())
                } else {
                    tagty(tr)
                }
            } else {
                tagty(tr)
            };
            if m.as_str() == "binary_search" && args.len() == 1 && tr >= 1000 {
                let ri = oast::eidx(ez, "Result");
                if ri >= 0 {
                    return 100 + ri;
                }
            } // VEC-BSEARCH: Ok(usize) / Err(usize)
            if m.as_str() == "clone" && args.len() == 0 && tr >= 800 && tr < 900 {
                return tr;
            } // CLOS-CLONE: a closure capturing Copy values is Copy -- its clone is the same closure
            let eclone = m.as_str() == "clone" && args.len() == 0 && tr >= 100 && tr < 200; // C-ENUMCLONE
            if eclone || olower::builtin_method(&nm2, m, args.len()).len() > 0 {
                if (m.as_str() == "get" || m.as_str() == "borrow" || m.as_str() == "borrow_mut")
                    && args.len() == 0
                    && tr == 201
                {
                    return 0 - 3;
                } // R4 REFCELL: a cell of an AGGREGATE (Vec/String/struct) has no typed tag here (201) -- its value is UNKNOWN, not int
                if (m.as_str() == "get" || m.as_str() == "borrow" || m.as_str() == "borrow_mut")
                    && args.len() == 0
                    && tr >= 1000
                {
                    return tr - 1000;
                } // C-CELL (+ R4 REFCELL borrow / borrow_mut)
                if m.as_str() == "set" && args.len() == 1 {
                    return 0;
                } // C-CELL: answers ()
                if m.as_str() == "get" && tr >= 1000 {
                    return tr - 1000;
                }
                if m.as_str() == "clone" && args.len() == 0 && tr >= 100 && tr < 200 {
                    return tr;
                } // C-ENUMCLONE
                if (m.as_str() == "as_ref" || m.as_str() == "as_mut") && args.len() == 0 {
                    return tr;
                } // C-ASREF: Box is transparent
                if m.as_str() == "chars" && tr == 2 {
                    let t = type_tag(ss, ez, tz, "Vec$char");
                    if t >= 0 {
                        return t;
                    }
                } // STR-CHARS-UTF8: chars() is the DECODED chars, not the bytes
                if (m.as_str() == "as_bytes" || m.as_str() == "bytes" || m.as_str() == "chars")
                    && tr == 2
                {
                    return 4;
                } // a str read element-wise (C-CHARS)
                if m.as_str() == "len" && tr == 4 {
                    return 0;
                }

                // an adaptor answers the same sequence; a predicate answers a bool; everything else an int
                if m.as_str() == "iter"
                    || m.as_str() == "into_iter"
                    || m.as_str() == "as_slice"
                    || m.as_str() == "as_mut_slice"
                    || m.as_str() == "iter_mut"
                    || m.as_str() == "collect"
                    || m.as_str() == "cloned"
                    || m.as_str() == "copied"
                    || m.as_str() == "to_vec"
                    || m.as_str() == "clone"
                    || m.as_str() == "map"
                    || m.as_str() == "filter"
                    || m.as_str() == "rev"
                    || m.as_str() == "take"
                    || m.as_str() == "skip"
                    || m.as_str() == "take_while"
                    || m.as_str() == "skip_while"
                    || m.as_str() == "chain"
                    || m.as_str() == "flat_map"
                {
                    return tr;
                }
                {
                    let rn2 = if tr >= 100 && tr < 200 {
                        let i = (tr - 100) as usize;
                        if i < ez.len() {
                            String::from(ez[i].name.as_str())
                        } else {
                            String::from("")
                        }
                    } else {
                        String::from("")
                    };
                    if rn2.as_str() == "Option" || rn2.as_str() == "Result" {
                        if m.as_str() == "map" || m.as_str() == "map_err" {
                            return tr;
                        }
                        if m.as_str() == "map_or" {
                            return 0;
                        } // P-OPT-MAPOR
                        if m.as_str() == "and_then" {
                            return tr;
                        } // P-OPT-COMB: Option -> Option
                        if m.as_str() == "copied" || m.as_str() == "cloned" {
                            return tr;
                        } // P-OPT-COPIED: identity on Option
                        if m.as_str() == "filter" {
                            return tr;
                        } // P-OPT-FILTER: Option -> Option
                        if m.as_str() == "unwrap_or_else" {
                            return 0;
                        } // P-OPT-COMB: -> value: map_or(default,f) returns the mapped value type
                        if m.as_str() == "ok_or" {
                            let ri = oast::eidx(ez, "Result");
                            if ri >= 0 {
                                return 100 + ri;
                            }
                            return 0;
                        } // P-RES-COMB: Option -> Result
                        if m.as_str() == "unwrap_or" {
                            if args.len() == 1 {
                                let t0 = check_e(&args[0], env, ds, ss, fz, ez, tz);
                                if t0 >= 0 {
                                    return t0;
                                }
                            }
                            return 0;
                        } /* UNWRAP-OR-TY */
                        // P-RES-COMB: -> value
                        if m.as_str() == "ok" {
                            let oi = oast::eidx(ez, "Option");
                            if oi >= 0 {
                                return 100 + oi;
                            }
                            return 0;
                        } // P-RES-COMB: Result -> Option
                        if m.as_str() == "is_some"
                            || m.as_str() == "is_none"
                            || m.as_str() == "is_ok"
                            || m.as_str() == "is_err"
                        {
                            return 1;
                        }
                        return 0;
                    }
                }
                if tr == 2 {
                    // C-STRSPLIT: a str split answers a Vec OF STR (tag 1000+str), which is what makes
                    // the `.collect()` after it legal -- without this the chain was "no method collect on str"
                    if m.as_str() == "split" && args.len() == 1 {
                        return 1000 + 2;
                    }
                    if m.as_str() == "split_whitespace" && args.len() == 0 {
                        return 1000 + 2;
                    } // SPLIT-WS
                    if m.as_str() == "char_indices" && args.len() == 0 {
                        return 1000 + 302;
                    } // CHAR-IDX
                    if m.as_str() == "push" || m.as_str() == "push_str" {
                        return 0;
                    } // C-PUSHUNIT: these answer ()
                    if m.as_str() == "parse" {
                        let ri = oast::eidx(ez, "Result");
                        if ri >= 0 {
                            return 100 + ri;
                        }
                        return 0;
                    }
                    if m.as_str() == "find" || m.as_str() == "rfind" {
                        let oi = oast::eidx(ez, "Option");
                        if oi >= 0 {
                            return 100 + oi;
                        }
                        return 0;
                    }
                    if m.as_str() == "is_empty"
                        || m.as_str() == "contains"
                        || m.as_str() == "starts_with"
                        || m.as_str() == "ends_with"
                    {
                        return 1;
                    }
                    if m.as_str() == "len" {
                        return 0;
                    }
                    return 2;
                }
                if (m.as_str() == "windows" || m.as_str() == "chunks")
                    && args.len() == 1
                    && tr >= 1000
                {
                    return tr;
                } // SLICE-WIN
                if m.as_str() == "collect_string" {
                    return 2;
                } // COLLECT-TURBO
                if m.as_str() == "fold" && args.len() == 2 {
                    let t0 = check_e(&args[0], env, ds, ss, fz, ez, tz);
                    if t0 >= 0 {
                        return t0;
                    }
                } // FOLD-TY
                if m.as_str() == "enumerate" {
                    return 1000 + 302;
                }
                if m.as_str() == "partition" && args.len() == 1 {
                    return 302;
                }
                if m.as_str() == "zip" {
                    return 1000 + 302;
                } // P-ITER-ZIP: Vec<tup2>
                if (m.as_str() == "sum" || m.as_str() == "product")
                    && args.len() == 0
                    && tr >= 1000
                    && tr < 2000
                {
                    // R11 SUM-TY: `.sum()` of a sequence is its ELEMENT type (it answered int, so `let t: f64 = v.iter()
                    // .map(|s| s.area()).sum()` was a mismatch); a non-numeric element tag (an unannotated map closure
                    // keeps the receiver's tag) is GENERIC -- lowering types the value
                    let e = tr - 1000;
                    if e == 0 || e == 3 {
                        return e;
                    }
                    return 0 - 3;
                }
                if m.as_str() == "step_by" {
                    if tr >= 1000 {
                        return tr;
                    }
                    return 1000;
                } // P-ITER-STEPBY: range/Vec -> Vec<int>
                  // the Option-returning consumers answer the prelude's `Option`, which `{:?}` can print
                if m.as_str() == "max"
                    || m.as_str() == "min"
                    || m.as_str() == "last"
                    || m.as_str() == "first"
                    || m.as_str() == "next"
                    || m.as_str() == "position"
                    || m.as_str() == "find"
                    || m.as_str() == "pop"
                    || m.as_str() == "nth"
                    || m.as_str() == "reduce"
                    || m.as_str() == "max_by_key"
                    || m.as_str() == "min_by_key"
                {
                    let oi = oast::eidx(ez, "Option");
                    if oi >= 0 {
                        return 100 + oi;
                    }
                    return 0;
                }
                if m.as_str() == "any"
                    || m.as_str() == "all"
                    || m.as_str() == "is_empty"
                    || m.as_str() == "contains"
                {
                    return 1;
                }
                0
            } else {
                // the receiver's TYPE NAME, then look up Type$m
                if tr >= 400 {
                    let ti = (tr - 400) as usize;
                    if ti < tz.len() {
                        let td = &tz[ti];
                        let mut found = false;
                        let mut mi = 0;
                        while mi < td.mnames.len() {
                            if td.mnames[mi].as_str() == m.as_str() {
                                found = true;
                            }
                            mi = mi + 1;
                        }
                        if !found {
                            let mut msg = String::from("trait `");
                            msg.push_str(td.name.as_str());
                            msg.push_str("` has no method `");
                            msg.push_str(m);
                            msg.push_str("`");
                            ds.push(oerrors::err(msg, *l, *h));
                            return 0;
                        }
                        // the return type is the trait's own declaration, shared by every implementor
                        let mut tq = String::from(td.name.as_str());
                        tq.push_str("$");
                        tq.push_str(m);
                        let mut rr = String::from("");
                        let mut q = 0;
                        while q < fz.len() {
                            if fz[q].name.as_str() == tq.as_str() {
                                rr = String::from(fz[q].ret.as_str());
                            }
                            q = q + 1;
                        }
                        if rr.len() == 0 {
                            let mut q2 = 0;
                            while q2 < fz.len() {
                                if fz[q2].owner.len() > 0 && fz[q2].name.len() > m.len() {
                                    let mut cand = String::from(fz[q2].owner.as_str());
                                    cand.push_str("$");
                                    cand.push_str(m);
                                    if fz[q2].name.as_str() == cand.as_str() {
                                        rr = String::from(fz[q2].ret.as_str());
                                    }
                                }
                                q2 = q2 + 1;
                            }
                        }
                        let t3 = tag_of_tyname(ss, &rr);
                        if t3 >= 0 {
                            return t3;
                        }
                        let ei2 = oast::eidx(ez, &rr);
                        if ei2 >= 0 {
                            return 100 + ei2;
                        }
                        return 0;
                    }
                    return 0;
                }
                let rn = if tr >= 10 && tr < 100 {
                    let i = (tr - 10) as usize;
                    if i < ss.len() {
                        String::from(ss[i].name.as_str())
                    } else {
                        String::from("")
                    }
                } else if tr >= 100 {
                    let i = (tr - 100) as usize;
                    if i < ez.len() {
                        String::from(ez[i].name.as_str())
                    } else {
                        String::from("")
                    }
                } else {
                    String::from("")
                };
                if rn.len() > 0 && tr >= 10 && tr < 100 && m.as_str() == "clone" && args.len() == 0
                {
                    let mut mg = String::from(rn.as_str());
                    mg.push_str("$clone");
                    if oast::find_method(fz, &rn, &mg) < 0 {
                        return tr;
                    }
                } // CLONE-STRUCT: derive(Clone) -- a struct clone has the struct's type (olower builds the copy)
                if rn.len() == 0 && (m.as_str() == "clone" || m.as_str() == "to_owned") {
                    return tr;
                } // P-06-BOUNDMETHOD: clone()/to_owned() on a SCALAR (incl. a generic T:Clone monomorphized to int) is value-identity, type == receiver; olower lowers it to the receiver.
                  // TR-PRIM-IMPL: `impl Tr for i64` registers `i64$m`, but every integer width shares tag 0 -- try each width's
                  // mangled name (the impl's own type wins when the receiver width is known only as "int")
                if rn.len() == 0 && tr == 0 {
                    let ws = [
                        "int", "i64", "i32", "i16", "i8", "isize", "u64", "u32", "u16", "u8",
                        "usize",
                    ];
                    let mut wi = 0;
                    while wi < ws.len() {
                        let mut mg = String::from(ws[wi]);
                        mg.push_str("$");
                        mg.push_str(m);
                        let fi = oast::find_method(fz, ws[wi], &mg);
                        if fi >= 0 {
                            let rt = String::from(fz[fi as usize].ret.as_str());
                            if rt.len() == 0 {
                                return 0 - 1;
                            }
                            let t3 = tag_of_tyname(ss, &rt);
                            if t3 >= 0 {
                                return t3;
                            }
                            return 0;
                        }
                        wi = wi + 1;
                    }
                }
                if rn.len() == 0 {
                    if tr >= 0 {
                        let mut msg = String::from("no method `");
                        msg.push_str(m);
                        msg.push_str("` on ");
                        msg.push_str(&tyname2(tr, ss));
                        ds.push(oerrors::err(msg, *l, *h));
                    }
                    return -1;
                }
                // HASHMAP: the prelude map's methods take int OR str keys (lowering picks hms_/hmi_ by the key type), so they are typed
                // here without an argument-type check
                if rn.as_str() == "HashMap" {
                    let mut k = 0;
                    while k < args.len() {
                        let _ = check_e(&args[k], env, ds, ss, fz, ez, tz);
                        k = k + 1;
                    }
                    if m.as_str() == "insert" && args.len() == 2 {
                        return 0 - 1;
                    }
                    if (m.as_str() == "contains_key" && args.len() == 1)
                        || (m.as_str() == "is_empty" && args.len() == 0)
                    {
                        return 1;
                    }
                    if m.as_str() == "keys" && args.len() == 0 {
                        return 0 - 3;
                    } // HM-KEYS: a fresh Vec of the keys (its element type is the map's key type, known to lowering)
                    if m.as_str() == "len" && args.len() == 0 {
                        return 0;
                    }
                    if (m.as_str() == "iter" || m.as_str() == "into_iter") && args.len() == 0 {
                        return 1000 + 302;
                    } /* HM-ITER: a Vec of (key, value) */
                    if (m.as_str() == "get" || m.as_str() == "remove") && args.len() == 1 {
                        let ei = oast::eidx(ez, "Option");
                        if ei >= 0 {
                            return 100 + ei;
                        }
                    }
                }
                let mut mangled = String::from(rn.as_str());
                mangled.push_str("$");
                mangled.push_str(m);
                let fi = oast::find_method(fz, &rn, &mangled);
                // ITER-DFLT: `sum`/`count` are PROVIDED Iterator methods -- any type with its own `next` has them
                if fi < 0 && (m.as_str() == "sum" || m.as_str() == "count") && args.len() == 0 {
                    let mut nx = String::from(rn.as_str());
                    nx.push_str("$next");
                    if oast::find_method(fz, &rn, &nx) >= 0 {
                        return 0;
                    }
                }
                if fi < 0 {
                    let mut msg = String::from("no method `");
                    msg.push_str(m);
                    msg.push_str("` on `");
                    msg.push_str(&rn);
                    msg.push_str("`");
                    ds.push(oerrors::err(msg, *l, *h));
                    return -1;
                }
                let f = &fz[fi as usize];
                let want = (f.params.len() as i32) - 1;
                if want != (args.len() as i32) {
                    let mut msg = String::from("method `");
                    msg.push_str(m);
                    msg.push_str("` takes ");
                    msg.push_str(&oast::u32s2(want as u32));
                    msg.push_str(" argument(s) but ");
                    msg.push_str(&oast::u32s2(args.len() as u32));
                    msg.push_str(" supplied");
                    ds.push(oerrors::err(msg, *l, *h));
                }
                let t2 = type_tag(ss, ez, tz, &f.ret);
                if t2 >= 0 {
                    t2
                } else {
                    0
                }
            }
        }
        E::AssignTo(pl, v, l, h) => {
            let tp = check_e(pl, env, ds, ss, fz, ez, tz);
            let tv2 = check_e(v, env, ds, ss, fz, ez, tz);
            if tp >= 0 && tv2 >= 0 && !tyeq(tp, tv2) {
                let mut m = String::from("cannot assign ");
                m.push_str(&tyname2(tv2, ss));
                m.push_str(" to ");
                m.push_str(&tyname2(tp, ss));
                ds.push(oerrors::err(m, *l, *h));
            }
            0
        }
        E::ArrLit(xs, l, h) => {
            // every element must agree; a str array is as valid as an int array (both are 8-byte slots)
            let mut et = 0 - 2;
            let mut k = 0;
            while k < xs.len() {
                let tx = check_e(&xs[k], env, ds, ss, fz, ez, tz);
                if tx >= 0 {
                    if et == 0 - 2 {
                        et = tx;
                    } else if et != tx {
                        let mut m = String::from("array elements must agree, found ");
                        m.push_str(&tyname2(tx, ss));
                        m.push_str(" after ");
                        m.push_str(&tyname2(et, ss));
                        ds.push(oerrors::err(m, *l, *h));
                    }
                }
                k = k + 1;
            }
            if et == 2 {
                return 2000 + 2;
            }
            if et == 0 - 2 || et == 0 {
                return 200;
            }
            if et > 0 {
                return 2000 + et;
            }
            200
        }
        E::Index(b, ix, l, h) => {
            let tb = check_e(b, env, ds, ss, fz, ez, tz);
            let ti = check_e(ix, env, ds, ss, fz, ez, tz);
            if tb >= 10
                && tb < 100
                && ((tb - 10) as usize) < ss.len()
                && ss[(tb - 10) as usize].name.as_str() == "HashMap"
            {
                return 0;
            } // HASHMAP: m[k] is the value
            if ti != 0 && ti >= 0 {
                let mut m = String::from("index must be int, found ");
                m.push_str(&tyname2(ti, ss));
                ds.push(oerrors::err(m, *l, *h));
            }
            if tb < 0 {
                return tb;
            } // UNKNOWN stays unknown -- never invent `int`
            if tb == 4 {
                return 0;
            } // a BYTE VIEW yields a byte
            if tb >= 2000 {
                return tb - 2000;
            } // a typed ARRAY yields its element type
            if tb == 201 {
                return 0 - 3;
            }
            if tb >= 1000 {
                return tb - 1000;
            } // a typed vector yields its ELEMENT type
            if tb != 200 && tb >= 0 {
                let mut m = String::from("cannot index ");
                m.push_str(&tyname2(tb, ss));
                ds.push(oerrors::err(m, *l, *h));
                return -1;
            }
            0
        }
        E::EnumLit(en, v, ps, l, h) => {
            let ei = oast::eidx(ez, en);
            if ei < 0 {
                let mut m = String::from("cannot find enum `");
                m.push_str(en);
                m.push_str("` in this scope");
                ds.push(oerrors::err(m, *l, *h));
                return -1;
            }
            let want = oast::vpayload(ez, en, v);
            if want < 0 {
                let mut m = String::from("no variant `");
                m.push_str(v);
                m.push_str("` on enum `");
                m.push_str(en);
                m.push_str("`");
                ds.push(oerrors::err(m, *l, *h));
                return -1;
            }
            if want != (ps.len() as i32) {
                let mut m = String::from("variant `");
                m.push_str(v);
                m.push_str("` takes ");
                m.push_str(&oast::u32s2(want as u32));
                m.push_str(" payload(s) but ");
                m.push_str(&oast::u32s2(ps.len() as u32));
                m.push_str(" given");
                ds.push(oerrors::err(m, *l, *h));
            }
            let mut k = 0;
            while k < ps.len() {
                let _ = check_e(&ps[k], env, ds, ss, fz, ez, tz);
                k = k + 1;
            }
            etag(ei)
        }
        E::Match(sc, arms, l, h) => {
            let ts = check_e(sc, env, ds, ss, fz, ez, tz);
            let structural = {
                let mut f = false;
                let mut k = 0;
                while k < arms.len() {
                    if arms[k].has_lit || arms[k].has_slit || arms[k].has_guard {
                        f = true;
                    }
                    k = k + 1;
                }
                f
            };
            if ts < 100 || ts >= 300 || structural {
                let mut r = 0;
                let mut k = 0;
                let mut rty = 0 - 2;
                // GUARD-ARM-BINDS (R2a P4): an enum match that took this path (some arm has a guard / literal) still binds each arm's
                // PAYLOAD with its declared type, for the guard and the body -- unbound, `ty` in `Op::CastOp(t, sx, ty) => &ty[5..]`
                // defaulted to a Vec slice ("expects str, found Vec") whenever ANY arm of the match had a guard
                let sen = if ts >= 100 && ts < 300 && ((ts - 100) as usize) < ez.len() {
                    String::from(ez[(ts - 100) as usize].name.as_str())
                } else {
                    String::from("")
                };
                while k < arms.len() {
                    let mut snb = 0;
                    // only a PLAIN positional arm (one bind per payload slot, no literal / str-literal position): a rewritten nested
                    // pattern (`Some((s, n))`) or a struct-variant arm with literal fields does not line up with the payload by
                    // position (heavy pass 5 caught both regressing)
                    if sen.len() > 0
                        && !arms[k].wild
                        && arms[k].variant.len() > 0
                        && !arms[k].has_lit
                        && !arms[k].has_slit
                        && oast::vpayload(ez, &sen, &arms[k].variant)
                            == (arms[k].binds.len() as i32)
                        && !arm_has_rewrite_bind(&arms[k].binds)
                        && !variant_is_struct(ez, &sen, &arms[k].variant)
                    {
                        let mut bi = 0;
                        while bi < arms[k].binds.len() {
                            env.n.push(String::from(arms[k].binds[bi].as_str()));
                            let pt = oast::vpayload_ty(ez, &sen, &arms[k].variant, bi);
                            let tg = type_tag(ss, ez, tz, &pt);
                            env.t.push(if tg >= 0 { tg } else { 0 - 3 });
                            snb = snb + 1;
                            bi = bi + 1;
                        }
                    }
                    if arms[k].has_guard {
                        let tg = check_e(&arms[k].guard, env, ds, ss, fz, ez, tz);
                        if tg != 1 && tg >= 0 {
                            let mut m = String::from("match guard must be bool, found ");
                            m.push_str(&tyname2(tg, ss));
                            ds.push(oerrors::err(m, *l, *h));
                        }
                    }
                    r = check_e(&arms[k].body, env, ds, ss, fz, ez, tz);
                    while snb > 0 {
                        env.n.pop();
                        env.t.pop();
                        snb = snb - 1;
                    }
                    // TY-ARMS (E0308): literal/guard arms must agree too (only the enum path checked); primitive tags only
                    if r >= 0 && r <= 3 {
                        if rty == 0 - 2 {
                            rty = r;
                        } else if rty >= 0 && !tyeq(r, rty) {
                            let mut m = String::from("match arms have incompatible types: ");
                            m.push_str(&tyname2(rty, ss));
                            m.push_str(" vs ");
                            m.push_str(&tyname2(r, ss));
                            ds.push(oerrors::err(
                                m,
                                oast::e_lo(&arms[k].body),
                                oast::e_hi(&arms[k].body),
                            ));
                        }
                    }
                    k = k + 1;
                }
                // PAT-EXHAUST (E0004) on int/char/str/bool scrutinees: without a catch-all arm (no literal, no guard) a literal
                // match is non-exhaustive -- except a bool match whose unguarded arms cover both true and false
                if ts >= 0 && ts <= 2 {
                    let mut catchall = false;
                    let mut st = false;
                    let mut sf = false;
                    let mut q = 0;
                    while q < arms.len() {
                        let a = &arms[q];
                        if !a.has_lit && !a.has_slit && !a.has_guard {
                            catchall = true;
                        }
                        if a.has_lit && !a.has_guard {
                            if a.lit == 0 {
                                sf = true;
                            } else if a.lit == 1 {
                                st = true;
                            }
                        }
                        q = q + 1;
                    }
                    let ok = catchall || (ts == 1 && st && sf);
                    if !ok && arms.len() > 0 {
                        let mut m = String::from("non-exhaustive patterns: `");
                        m.push_str(&tyname2(ts, ss));
                        m.push_str("` values not covered (add a `_` arm)");
                        ds.push(oerrors::err(m, *l, *h));
                    }
                }
                r
            } else {
                if ts < 100 {
                    if ts >= 0 {
                        let mut m = String::from("`match` needs an enum, found ");
                        m.push_str(&tyname2(ts, ss));
                        ds.push(oerrors::err(m, *l, *h));
                    }
                    return -1;
                }
                let ei = ts - 100;
                let en = if (ei as usize) < ez.len() {
                    String::from(ez[ei as usize].name.as_str())
                } else {
                    String::from("")
                };
                let nv = oast::nvariants(ez, &en);
                let mut covered = 0;
                let mut haswild = false;
                let mut rty = -2;
                let mut k = 0;
                while k < arms.len() {
                    let a = &arms[k];
                    if a.wild {
                        haswild = true;
                    } else {
                        let vi = oast::vidx(ez, &en, &a.variant);
                        if vi < 0 {
                            let mut m = String::from("no variant `");
                            m.push_str(&a.variant);
                            m.push_str("` on enum `");
                            m.push_str(&en);
                            m.push_str("`");
                            ds.push(oerrors::err(m, *l, *h));
                        } else {
                            covered = covered + 1;
                            let np = oast::vpayload(ez, &en, &a.variant);
                            if np != (a.binds.len() as i32) {
                                let mut m = String::from("variant `");
                                m.push_str(&a.variant);
                                m.push_str("` binds ");
                                m.push_str(&oast::u32s2(np as u32));
                                m.push_str(" payload(s) but ");
                                m.push_str(&oast::u32s2(a.binds.len() as u32));
                                m.push_str(" named");
                                ds.push(oerrors::err(m, *l, *h));
                            }
                        }
                    }
                    let mut nb = 0;
                    let mut bi = 0;
                    while bi < a.binds.len() {
                        env.n.push(String::from(a.binds[bi].as_str()));
                        let pt = oast::vpayload_ty(ez, &en, &a.variant, bi);
                        let tg = type_tag(ss, ez, tz, &pt);
                        env.t.push(if tg >= 0 { tg } else { 0 - 3 });
                        nb = nb + 1;
                        bi = bi + 1;
                    }
                    let at = check_e(&a.body, env, ds, ss, fz, ez, tz);
                    {
                        let mut q = 0;
                        while q < nb {
                            env.n.pop();
                            env.t.pop();
                            q = q + 1;
                        }
                    }
                    if rty == -2 {
                        rty = at;
                    } else if at >= 0 && rty >= 0 && !tyeq(at, rty) {
                        let mut m = String::from("match arms have incompatible types: ");
                        m.push_str(&tyname2(rty, ss));
                        m.push_str(" vs ");
                        m.push_str(&tyname2(at, ss));
                        ds.push(oerrors::err(m, *l, *h));
                    }
                    k = k + 1;
                }
                if !haswild && nv >= 0 && covered < nv {
                    let mut m = String::from("non-exhaustive `match` on `");
                    m.push_str(&en);
                    m.push_str("`: ");
                    m.push_str(&oast::u32s2((nv - covered) as u32));
                    m.push_str(" variant(s) not covered");
                    ds.push(oerrors::err(m, *l, *h));
                }
                if rty == -2 {
                    0
                } else {
                    rty
                }
            }
        }
        E::Not(x, l, h) => {
            let tx = check_e(x, env, ds, ss, fz, ez, tz);
            if tx == 0 {
                return 0;
            } // NOT-INT: `!` on an integer is bitwise NOT (rustc: `!0u8` == 255); lowering already emits it
            if tx != 1 && tx >= 0 {
                let mut m = String::from("`!` needs a bool, found ");
                m.push_str(&tyname2(tx, ss));
                ds.push(oerrors::err(m, *l, *h));
                return -1;
            }
            1
        }
        E::StructLit(n, fs, l, h) => {
            let si = sidx_of(ss, n);
            if si < 0 {
                let mut m = String::from("cannot find struct `");
                m.push_str(n);
                m.push_str("` in this scope");
                ds.push(oerrors::err(m, *l, *h));
                let mut k = 0;
                while k < fs.len() {
                    let _ = check_e(&fs[k], env, ds, ss, fz, ez, tz);
                    k = k + 1;
                }
                return -1;
            }
            let want = oast::sfields(ss, n);
            if want >= 0 && want != (fs.len() as i32) {
                let mut m = String::from("struct `");
                m.push_str(n);
                m.push_str("` has ");
                m.push_str(&oast::u32s2(want as u32));
                m.push_str(" field(s) but ");
                m.push_str(&oast::u32s2(fs.len() as u32));
                m.push_str(" given");
                ds.push(oerrors::err(m, *l, *h));
            }
            let mut k = 0;
            while k < fs.len() {
                let tf = check_e(&fs[k], env, ds, ss, fz, ez, tz);
                if k < ss[si as usize].ftys.len()
                    && !oast::in_list(&ss[si as usize].sgp, ss[si as usize].ftys[k].as_str())
                {
                    // a field declared with the struct's own TYPE PARAMETER (`x: A`) takes the value's type -- not checked here
                    let want = tag_of_tyname(ss, &ss[si as usize].ftys[k]);
                    if want >= 0 && tf >= 0 && !tyeq(tf, want) {
                        let mut m = String::from("field `");
                        m.push_str(&ss[si as usize].fields[k]);
                        m.push_str("` expects ");
                        m.push_str(&tyname2(want, ss));
                        m.push_str(", found ");
                        m.push_str(&tyname2(tf, ss));
                        ds.push(oerrors::err(m, *l, *h));
                    }
                }
                k = k + 1;
            }
            10 + si
        }
        E::Field(o, f, l, h) => {
            let to = check_e(o, env, ds, ss, fz, ez, tz);
            if to < 10 {
                if to >= 0 {
                    let mut m = String::from("no field `");
                    m.push_str(f);
                    m.push_str("` on ");
                    m.push_str(&tyname2(to, ss));
                    ds.push(oerrors::err(m, *l, *h));
                }
                return -1;
            }
            let si = (to - 10) as usize;
            if si < ss.len() {
                let sn = String::from(ss[si].name.as_str());
                let fi = oast::sfield_idx(ss, &sn, f);
                if fi < 0 {
                    let mut m = String::from("no field `");
                    m.push_str(f);
                    m.push_str("` on struct `");
                    m.push_str(&sn);
                    m.push_str("`");
                    ds.push(oerrors::err(m, *l, *h));
                    return -1;
                }
                let fty = oast::sfield_ty(ss, &sn, f);
                if oast::in_list(&ss[si].sgp, fty.as_str()) {
                    return 0 - 3; // GEN-FIELD (typeck): a field declared with the struct's TYPE PARAMETER is typed by lowering's GEN-INST (was int: `t.x.len()` on a T<String> refused)
                }
                let ft = type_tag(ss, ez, tz, &fty);
                if ft >= 0 {
                    return ft;
                }
            }
            0
        }
        E::While(c, b, _l, _h) => {
            let tc = check_e(c, env, ds, ss, fz, ez, tz);
            if tc != 1 && tc >= 0 {
                let mut m = String::from("`while` condition must be bool, found ");
                m.push_str(&tyname(tc));
                ds.push(oerrors::err(m, oast::e_lo(c), oast::e_hi(c)));
            }
            let _ = check_e(b, env, ds, ss, fz, ez, tz);
            0
        }
        E::Seq(a, b, _l, _h) => {
            // STMT-IF (R2a P4): an `if` in STATEMENT position has no value -- its branches are not compared (rustc: both are `()`).
            // The parser keeps a trailing `expr;` as the block value (the prelude relies on that), so `if c { v.remove(0); } else
            // { w.push(1); }` compared str with int once remove was typed (SEMI-UNIT, making `{ x; }` unit, broke the prelude)
            if let E::If(c, th, el, _li, _hi) = a.as_ref() {
                check_stmt_if(c, th, el, env, ds, ss, fz, ez, tz);
            } else {
                let _ = check_e(a, env, ds, ss, fz, ez, tz);
            }
            check_e(b, env, ds, ss, fz, ez, tz)
        }
        E::Assign(n, v, l, h) => {
            let tv = check_e(v, env, ds, ss, fz, ez, tz);
            let tn = env_get(env, n);
            if tn >= 0 && tv >= 0 && !tyeq(tn, tv) {
                let mut m = String::from("cannot assign ");
                m.push_str(&tyname2(tv, ss));
                m.push_str(" to ");
                m.push_str(&tyname2(tn, ss));
                ds.push(oerrors::err(m, *l, *h));
            }
            0
        }
        // C-COPYISCOPY: str_copy answers a fresh str; one argument, which must be a str
        // C-MEMTAKE: $zeroof answers the same type as its argument
        E::Call(n2, args, _l, _h) if n2.as_str() == "$zeroof" && args.len() == 1 => {
            check_e(&args[0], env, ds, ss, fz, ez, tz)
        }
        // R6 ZERO-DFLT-USE: an unannotated `Default::default()` is typed by lowering from its use; UNKNOWN here (permissive)
        E::Call(n2, args, _l, _h) if n2.as_str() == "$zerodflt" && args.len() == 0 => 0 - 3,
        // C-CELL: a Cell is a one-slot vector OF the value's type, so the tag is a TYPED Vec -- which is
        // what makes `c.get()` answer the value's own type instead of an unknown.
        // RC-IDENTITY (R4c): an Rc IS its referent here (identity handle), so it types as the referent; strong_count is an int
        E::Call(n2, args, _l, _h)
            if (n2.as_str() == "$rcnew"
                || n2.as_str() == "$rcclone"
                || n2.as_str() == "$boxnew"
                || n2.as_str() == "$cellhandle")
                && args.len() == 1 =>
        {
            check_e(&args[0], env, ds, ss, fz, ez, tz)
        }
        E::Call(n2, args, _l, _h) if n2.as_str() == "$rccount" && args.len() == 1 => {
            let _ = check_e(&args[0], env, ds, ss, fz, ez, tz);
            0
        }
        E::Call(n2, args, _l, _h) if n2.as_str() == "$cellnew" && args.len() == 1 => {
            let t = check_e(&args[0], env, ds, ss, fz, ez, tz);
            if t >= 0 && t < 1000 {
                1000 + t
            } else {
                201
            }
        }
        E::Call(n2, args, _l, _h) if n2.as_str() == "$chanint" && args.len() == 1 => {
            check_e(&args[0], env, ds, ss, fz, ez, tz)
        }
        E::Call(n2, _args, _l, _h) if n2.as_str() == "$dropfield" => 0,
        E::Call(n2, _args, _l, _h)
            if n2.as_str() == "$partialmove" || n2.as_str() == "$updfromdrop" =>
        {
            0
        }
        E::Call(n2, args, _l, _h) if n2.as_str() == "$ptrnull" && args.len() == 0 => 0, // R4b PTR: a null pointer word
        // R4f CHAN: `$alias(&x)` is x (a second name for one shared handle)
        E::Call(n2, args, _l, _h) if n2.as_str() == "$alias" && args.len() == 1 => {
            check_e(&args[0], env, ds, ss, fz, ez, tz)
        }
        // R4a LAYOUT: size_of / align_of answer a usize
        E::Call(n2, args, _l, _h) if n2.as_str() == "$sizeofval" && args.len() == 1 => {
            let _ = check_e(&args[0], env, ds, ss, fz, ez, tz);
            0
        }
        E::Call(n2, _args, _l, _h)
            if oast::str_starts(n2.as_str(), "$sizeof$")
                || oast::str_starts(n2.as_str(), "$alignof$") =>
        {
            0
        }
        // R4a XMUTE: a transmute has its TARGET type
        E::Call(n2, args, _l, _h)
            if oast::str_starts(n2.as_str(), "$xmute$") && args.len() == 1 =>
        {
            let _ = check_e(&args[0], env, ds, ss, fz, ez, tz);
            tag_of_tyname(ss, &n2[7..])
        }
        E::Call(n2, _args, _l, _h) if olower::zeroty_name(n2).len() > 0 => {
            tag_of_tyname(ss, &olower::zeroty_name(n2))
        }
        // ARR-REP-LOOP (C1): `[x; n]` with a large or non-literal n is `arr_fill(n, x)` -- an ARRAY like the literal it replaces
        E::Call(n2, args, _l, _h) if n2.as_str() == "arr_fill" && args.len() == 2 => {
            let _ = check_e(&args[0], env, ds, ss, fz, ez, tz);
            let tx = check_e(&args[1], env, ds, ss, fz, ez, tz);
            if tx == 2 {
                return 2000 + 2;
            }
            if tx > 0 {
                return 2000 + tx;
            }
            200
        }
        E::Call(n2, args, l, h) if n2.as_str() == "str_copy" => {
            if args.len() != 1 {
                let m = String::from("`str_copy` takes 1 argument(s)");
                ds.push(oerrors::err(m, *l, *h));
            }
            // the ARGUMENT is not judged here: str_copy is synthesized from String::from, whose argument
            // is whatever the user wrote, and claiming it must be a str added a FALSE diagnostic wherever
            // the argument's type is merely unresolved (it refused our own composed unit).
            let mut k = 0;
            while k < args.len() {
                let _ta = check_e(&args[k], env, ds, ss, fz, ez, tz);
                k = k + 1;
            }
            2
        }
        E::Call(n2, args, l, h)
            if n2.as_str() == "str_at"
                || n2.as_str() == "str_cat"
                || n2.as_str() == "str_sub"
                || n2.as_str() == "chr1" =>
        {
            let want = if n2.as_str() == "str_sub" { 3 } else { 2 };
            if (args.len() as i32) != want {
                let mut m = String::from("`");
                m.push_str(n2);
                m.push_str("` takes ");
                m.push_str(&oast::u32s2(want as u32));
                m.push_str(" argument(s)");
                ds.push(oerrors::err(m, *l, *h));
            }
            let mut k = 0;
            while k < args.len() {
                let ta = check_e(&args[k], env, ds, ss, fz, ez, tz);
                let need = if k == 0 || (n2.as_str() == "str_cat" && k == 1) {
                    2
                } else {
                    0
                };
                if ta != need && ta != 0 - 3 {
                    let mut m = String::from("`");
                    m.push_str(n2);
                    m.push_str("` argument ");
                    m.push_str(&oast::u32s2(k as u32));
                    m.push_str(" must be ");
                    m.push_str(if need == 2 { "a str" } else { "an int" });
                    m.push_str(", found ");
                    m.push_str(&tyname2(ta, ss));
                    ds.push(oerrors::err(m, *l, *h));
                }
                k = k + 1;
            }
            if n2.as_str() == "str_at" {
                0
            } else {
                2
            }
        }
        E::Call(n2, args, l, h)
            if n2.as_str() == "vec_new"
                || n2.as_str() == "vec_push"
                || n2.as_str() == "vec_get"
                || n2.as_str() == "vec_set"
                || n2.as_str() == "vec_len" =>
        {
            let want = if n2.as_str() == "vec_new" {
                0
            } else if n2.as_str() == "vec_set" {
                3
            } else if n2.as_str() == "vec_push" || n2.as_str() == "vec_get" {
                2
            } else {
                1
            };
            if (args.len() as i32) != want {
                let mut m = String::from("`");
                m.push_str(n2);
                m.push_str("` takes ");
                m.push_str(&oast::u32s2(want as u32));
                m.push_str(" argument(s)");
                ds.push(oerrors::err(m, *l, *h));
                let mut k = 0;
                while k < args.len() {
                    let _ = check_e(&args[k], env, ds, ss, fz, ez, tz);
                    k = k + 1;
                }
                return if n2.as_str() == "vec_new" { 201 } else { 0 };
            }
            let mut k = 0;
            while k < args.len() {
                let ta = check_e(&args[k], env, ds, ss, fz, ez, tz);
                let need = if k == 0 && n2.as_str() != "vec_new" {
                    201
                } else {
                    0
                };
                if k == 0 && ta >= 1000 {
                    k = k + 1;
                    continue;
                }
                if k == 1 && n2.as_str() == "vec_push" {
                    let t0 = check_e(&args[0], env, ds, ss, fz, ez, tz);
                    if t0 >= 1000 {
                        let el = t0 - 1000;
                        if el >= 400 {
                            if ta >= 10 && ta < 100 {
                                k = k + 1;
                                continue;
                            }
                        }
                        if tyeq(ta, el) {
                            k = k + 1;
                            continue;
                        } // CLOS-VALUE: a closure pushed into a Vec<Fn>
                        let mut m = String::from("`vec_push` value must be ");
                        m.push_str(&tyname2(el, ss));
                        m.push_str(", found ");
                        m.push_str(&tyname2(ta, ss));
                        ds.push(oerrors::err(m, *l, *h));
                        k = k + 1;
                        continue;
                    }
                    if t0 == 201 {
                        k = k + 1;
                        continue;
                    }
                }
                if ta != need && ta != 0 - 3 {
                    let mut m = String::from("`");
                    m.push_str(n2);
                    m.push_str("` argument ");
                    m.push_str(&oast::u32s2(k as u32));
                    m.push_str(" must be ");
                    m.push_str(if need == 201 { "a Vec" } else { "an int" });
                    m.push_str(", found ");
                    m.push_str(&tyname2(ta, ss));
                    ds.push(oerrors::err(m, *l, *h));
                }
                k = k + 1;
            }
            if n2.as_str() == "vec_new" {
                return 201;
            }
            if n2.as_str() == "vec_get" && args.len() == 2 {
                let t0 = check_e(&args[0], env, ds, ss, fz, ez, tz);
                if t0 >= 1000 {
                    return t0 - 1000;
                }
            }
            0
        }
        E::Call(n2, args, _lf1, _hf1) if n2.as_str() == "f64_parse" && args.len() == 1 => {
            let _ = check_e(&args[0], env, ds, ss, fz, ez, tz);
            3
        }
        E::Call(n2, args, _lf2, _hf2)
            if (n2.as_str() == "f64_snprintf" || n2.as_str() == "f64_snprintf_e")
                && args.len() == 2 =>
        {
            let _ = check_e(&args[0], env, ds, ss, fz, ez, tz);
            let _ = check_e(&args[1], env, ds, ss, fz, ez, tz);
            2
        }
        // a global load/store: an int in, an int out
        E::Call(n2, args, _lg, _hg) if olower::is_gname(n2) => {
            let mut k = 0;
            while k < args.len() {
                let _ = check_e(&args[k], env, ds, ss, fz, ez, tz);
                k = k + 1;
            }
            0
        }
        E::Call(n2, args, _lf3, _hf3)
            if (n2.as_str() == "f64_bits" || n2.as_str() == "f64_isnan") && args.len() == 1 =>
        {
            let _ = check_e(&args[0], env, ds, ss, fz, ez, tz);
            0
        }
        E::Call(n2, args, _lq, _hq) if n2.as_str() == "seq_slice" && args.len() == 3 => {
            let t0 = check_e(&args[0], env, ds, ss, fz, ez, tz);
            let mut k = 1;
            while k < args.len() {
                let _ = check_e(&args[k], env, ds, ss, fz, ez, tz);
                k = k + 1;
            }
            if t0 == 0 - 3 {
                0 - 3
            } else if t0 == 2 {
                2
            } else if t0 == 4 {
                4 // BSTR-SLICE (R2a P4): a slice of a byte string is a byte string (was a Vec slice: crash at run time)
            } else {
                201
            }
        }
        E::Call(n2, args, _ls, _hs)
            if n2.as_str() == "seq_len" || n2.as_str() == "seq_get" || n2.as_str() == "seq_set" =>
        {
            let t0 = if args.len() > 0 {
                check_e(&args[0], env, ds, ss, fz, ez, tz)
            } else {
                0
            };
            let mut k = 1;
            while k < args.len() {
                let _ = check_e(&args[k], env, ds, ss, fz, ez, tz);
                k = k + 1;
            }
            if n2.as_str() == "seq_len" {
                return 0;
            }
            if t0 >= 1000 {
                return t0 - 1000;
            }
            0
        }
        E::Call(n2, args, _lp, _hp) if n2.as_str() == "panic_exit" && args.len() == 0 => 0 - 1,
        E::Call(n2, args, _lp, _hp) if n2.as_str() == "print_nl" && args.len() == 0 => 0,
        E::Call(n2, args, l, h) if n2.as_str() == "argc" => {
            // C-ARGV: argc is the one builtin with NO argument, so it cannot ride the one-argument family
            if args.len() != 0 {
                let mut m = String::from("`argc` takes no argument");
                ds.push(oerrors::err(m, *l, *h));
            }
            0
        }
        E::Call(n2, args, _lf, _hf) if n2.as_str() == "fmtpad_call" => {
            let mut k = 0;
            while k < args.len() {
                let _ = check_e(&args[k], env, ds, ss, fz, ez, tz);
                k = k + 1;
            }
            2
        } // FMT-PAD: (text, width, align, fill) -> str
        E::Call(n2, args, _l, _h)
            if (n2.as_str() == "$shovf" || n2.as_str() == "$ovf1") && args.len() == 2 =>
        {
            let _ = check_e(&args[0], env, ds, ss, fz, ez, tz);
            let _ = check_e(&args[1], env, ds, ss, fz, ez, tz);
            1
        } // OVF-SHIFT: bool
        E::Call(n2, args, _l, _h) if n2.as_str() == "drop" && args.len() == 1 => {
            let _ = check_e(&args[0], env, ds, ss, fz, ez, tz);
            0 - 1
        } // DROP-FN: any value, answers ()
        E::Call(n2, args, l, h)
            if n2.as_str() == "str_dbg"
                || n2.as_str() == "print"
                || n2.as_str() == "len"
                || n2.as_str() == "print_int"
                || n2.as_str() == "print_raw"
                || n2.as_str() == "print_int_raw"
                || n2.as_str() == "print_char"
                || n2.as_str() == "print_any"
                || n2.as_str() == "print_dbg"
                || n2.as_str() == "print_nl"
                || n2.as_str() == "panic_exit"
                || n2.as_str() == "arreq"
                || n2.as_str() == "arr_len"
                || n2.as_str() == "veceq"
                || n2.as_str() == "enumeq"
                || n2.as_str() == "proc_exit"
                || n2.as_str() == "vec_popraw"
                || n2.as_str() == "str_of"
                || n2.as_str() == "readfile"
                || n2.as_str() == "fileok"
                || n2.as_str() == "stdio_swap"
                || n2.as_str() == "argv_at" =>
        {
            if args.len() != 1 {
                let mut m = String::from("`");
                m.push_str(n2);
                m.push_str("` takes 1 argument");
                ds.push(oerrors::err(m, *l, *h));
                return 0;
            }
            let ta = check_e(&args[0], env, ds, ss, fz, ez, tz);
            if n2.as_str() == "print_any"
                || n2.as_str() == "print_int_raw"
                || n2.as_str() == "print_char"
                || n2.as_str() == "arreq"
                || n2.as_str() == "arr_len"
                || n2.as_str() == "veceq"
                || n2.as_str() == "enumeq"
                || n2.as_str() == "proc_exit"
                || n2.as_str() == "vec_popraw"
            {
                return 0;
            }
            if n2.as_str() == "str_of"
                || n2.as_str() == "str_dbg"
                || n2.as_str() == "fmtpad_call"
                || n2.as_str() == "argv_at"
                || n2.as_str() == "readfile"
            {
                return 2;
            }
            if n2.as_str() == "fileok" || n2.as_str() == "stdio_swap" {
                return 0;
            }
            if n2.as_str() == "print_dbg" {
                // `{:?}` has no derived Debug here: a scalar or str prints like `{}`, anything else is refused
                // D2-D6: a SEQUENCE, a STRUCT and an ENUM now have a Debug printer emitted from their
                // static shape; what is still unsupported is caught in lowering, where the shape is known
                // a TUPLE is admitted here and decided in lowering: a tuple LITERAL carries its element
                // types and is emittable, a tuple VARIABLE does not and is refused there
                let dbgable = ta == 200
                    || ta == 201
                    || ta >= 1000
                    || (ta >= 10 && ta < 200)
                    || (ta >= 300 && ta < 400);
                if !dbgable && ta != 0 && ta != 1 && ta != 2 && ta != 3 && ta != 0 - 3 && ta >= 0 {
                    let m = String::from(
                        "`{:?}` on a non-scalar needs a derived Debug (not implemented)",
                    );
                    ds.push(oerrors::err(m, *l, *h));
                }
                return 0;
            }
            if n2.as_str() == "print_raw" {
                if ta != 2 && ta != 0 - 3 {
                    let mut m = String::from("`print_raw` needs a str");
                    ds.push(oerrors::err(m, *l, *h));
                }
                return 0;
            }
            if n2.as_str() == "print_int" {
                if ta != 0 && ta != 0 - 3 {
                    let mut m = String::from("`print_int` needs an int, found ");
                    m.push_str(&tyname2(ta, ss));
                    ds.push(oerrors::err(m, *l, *h));
                }
                return 0;
            }
            if ta != 2 && ta != 0 - 3 {
                let mut m = String::from("`");
                m.push_str(n2);
                m.push_str("` needs a str, found ");
                m.push_str(&tyname2(ta, ss));
                ds.push(oerrors::err(m, *l, *h));
            }
            0
        }
        E::Call(n2, args, _lf, _hf)
            if {
                let mut tt = 0 - 1;
                let mut q = env.n.len();
                while q > 0 {
                    q = q - 1;
                    if env.n[q].as_str() == n2.as_str() {
                        tt = env.t[q];
                        q = 0;
                    }
                }
                tt == 900
            } =>
        {
            let mut k = 0;
            while k < args.len() {
                let _ = check_e(&args[k], env, ds, ss, fz, ez, tz);
                k = k + 1;
            }
            // a call through an `Fn`-typed parameter has an UNKNOWN result type: our closures carry no
            // signature. Answering `int` let our own dialect's `f(x) == 1` type-check and rejected a
            // bool-returning Rust closure in `if f(x)`; answering unknown admits both, which is what we know.
            0 - 3
        }
        E::Call(n2, args, l, h)
            if {
                let mut tt = 0 - 1;
                let mut q = env.n.len();
                while q > 0 {
                    q = q - 1;
                    if env.n[q].as_str() == n2.as_str() {
                        tt = env.t[q];
                        q = 0;
                    }
                }
                tt >= 800 && tt < 1000
            } =>
        {
            let mut tv = 0;
            {
                let mut q = env.n.len();
                while q > 0 {
                    q = q - 1;
                    if env.n[q].as_str() == n2.as_str() {
                        tv = env.t[q];
                        q = 0;
                    }
                }
            }
            let cn = clos_name_of_tag(tv);
            let mut k = 0;
            while k < args.len() {
                let _ = check_e(&args[k], env, ds, ss, fz, ez, tz);
                k = k + 1;
            }
            let mut want = 0 - 1;
            let mut rr = String::from("");
            let mut q2 = 0;
            while q2 < fz.len() {
                if fz[q2].name.as_str() == cn.as_str()
                    || clos_idx_of_name(fz[q2].name.as_str()) == tv - 800
                {
                    want = (fz[q2].params.len() as i32) - fz[q2].ncap;
                    rr = String::from(fz[q2].ret.as_str());
                }
                q2 = q2 + 1;
            }
            if want >= 0 && want != (args.len() as i32) {
                let mut m = String::from("closure takes ");
                m.push_str(&oast::u32s2(want as u32));
                m.push_str(" argument(s) but ");
                m.push_str(&oast::u32s2(args.len() as u32));
                m.push_str(" supplied");
                ds.push(oerrors::err(m, *l, *h));
            }
            let t2 = tag_of_tyname(ss, &rr);
            if t2 >= 0 {
                t2
            } else {
                let ei = oast::eidx(ez, &rr);
                if ei >= 0 {
                    100 + ei
                } else {
                    0
                }
            }
        }
        E::Call(n2, args, _l, _h) => {
            // C-ONEWALK: the per-argument TYPE check lives here, where every argument is already typed
            // exactly once, instead of in check_calls_t -- which typed each argument with check_e and then
            // recursed into the same subtree, making the pass quadratic in call-nesting depth. The NAME and
            // ARITY diagnostics stay in check_calls_t: they need no types, and check_e reaches Call nodes
            // that walk deliberately never reports (a trait associated const read through Self).
            let mut fx = 0 - 1;
            {
                let mut fi = 0;
                while fi < fz.len() {
                    if fz[fi].name.as_str() == n2 {
                        fx = fi as i32;
                        fi = fz.len();
                    } else {
                        fi = fi + 1;
                    }
                }
            }
            let mut k = 0;
            while k < args.len() {
                let ta = check_e(&args[k], env, ds, ss, fz, ez, tz);
                if fx >= 0 {
                    let fq = fx as usize;
                    if k < fz[fq].ptys.len() && !is_gp(&fz[fq].gparams, &fz[fq].ptys[k]) {
                        let want = tag_of_tyname(ss, &fz[fq].ptys[k]);
                        let ok_fn = want == 900 && ta >= 800 && ta < 1000;
                        if want >= 0 && ta >= 0 && !tyeq(ta, want) && !ok_fn {
                            let mut m = String::from("argument ");
                            m.push_str(&oast::u32s2((k + 1) as u32));
                            m.push_str(" of `");
                            m.push_str(n2);
                            m.push_str("` expects ");
                            m.push_str(&tyname2(want, ss));
                            m.push_str(", found ");
                            m.push_str(&tyname2(ta, ss));
                            ds.push(oerrors::err(m, *_l, *_h));
                        }
                    }
                }
                // a TRAIT OBJECT parameter can only take a struct or an enum (those carry the tag)
                {
                    let mut fj = 0;
                    while fj < fz.len() {
                        if fz[fj].name.as_str() == n2 && k < fz[fj].ptys.len() {
                            if dyn_name_trait(fz[fj].ptys[k].as_str()).len() > 0
                                && ta >= 0
                                && !(ta >= 10 && ta < 200)
                                && !(ta >= 0 && ta <= 2)
                            {
                                let mut m2 = String::from("a trait object from ");
                                m2.push_str(&tyname2(ta, ss));
                                m2.push_str(
                                    " is not implemented (only a struct or enum carries the tag)",
                                );
                                ds.push(oerrors::err(m2, *_l, *_h));
                            }
                            fj = fz.len();
                        } else {
                            fj = fj + 1;
                        }
                    }
                }
                k = k + 1;
            }
            let rt = fn_ret(fz, n2);
            {
                let mut fi = 0;
                while fi < fz.len() {
                    if fz[fi].name.as_str() == n2 {
                        if is_gp(&fz[fi].gparams, &rt) {
                            let mut q = 0;
                            while q < fz[fi].ptys.len() {
                                if fz[fi].ptys[q].as_str() == rt.as_str() && q < args.len() {
                                    return check_e(&args[q], env, ds, ss, fz, ez, tz);
                                }
                                q = q + 1;
                            }
                            return 0 - 3;
                        }
                    }
                    fi = fi + 1;
                }
            }
            let t2 = type_tag(ss, ez, tz, &rt);
            if t2 >= 0 {
                t2
            } else {
                0
            }
        }
        E::Int(_v, _l, _h) => 0,
        E::Bool(_v, _l, _h) => 1,
        E::Var(n, _l, _h) => env_get(env, n),
        E::Bin(op, a, b, l, h) => {
            let ta = if is_bitop(op) {
                match a.as_ref() {
                    E::Not(ax, _la, _ha) => {
                        let ti = check_e(ax, env, ds, ss, fz, ez, tz);
                        if ti == 0 {
                            0
                        } else {
                            check_e(a, env, ds, ss, fz, ez, tz)
                        }
                    }
                    _ => check_e(a, env, ds, ss, fz, ez, tz),
                }
            } else {
                check_e(a, env, ds, ss, fz, ez, tz)
            };
            let tb = if is_bitop(op) {
                match b.as_ref() {
                    E::Not(bx, _lb, _hb) => {
                        let ti = check_e(bx, env, ds, ss, fz, ez, tz);
                        if ti == 0 {
                            0
                        } else {
                            check_e(b, env, ds, ss, fz, ez, tz)
                        }
                    }
                    _ => check_e(b, env, ds, ss, fz, ez, tz),
                }
            } else {
                check_e(b, env, ds, ss, fz, ez, tz)
            };
            let ovl = {
                let mm = olower::op_method(op);
                ta >= 10 && ta < 100 && mm.len() > 0 && {
                    let si = (ta - 10) as usize;
                    if si < ss.len() {
                        let mut q = String::from(ss[si].name.as_str());
                        q.push('$');
                        q.push_str(&mm);
                        oast::find_method(fz, ss[si].name.as_str(), &q) >= 0
                    } else {
                        false
                    }
                }
            };
            let _ = tb;
            if ovl {
                let mm = olower::op_method(op);
                let si = (ta - 10) as usize;
                let mut q = String::from(ss[si].name.as_str());
                q.push('$');
                q.push_str(&mm);
                let fi = oast::find_method(fz, ss[si].name.as_str(), &q);
                if fi < 0 {
                    return 0;
                }
                let rr = String::from(fz[fi as usize].ret.as_str());
                let t2 = type_tag(ss, ez, tz, &rr);
                if t2 >= 0 {
                    t2
                } else {
                    0
                }
            } else {
                // F-1: an f64 operand (tag 3) makes the operation a FLOAT operation -- arithmetic answers an
                // f64, a comparison a bool. Mixing a float with an int is a type error in Rust and here.
                if ta == 3 || tb == 3 {
                    // the `0 - x` shape that a unary minus desugars to is a float negation when x is a float
                    {
                        let isz = match a.as_ref() {
                            E::Int(0, _lz, _hz) => true,
                            _ => false,
                        };
                        if isz && tb == 3 && op.as_str() == "-" {
                            return 3;
                        }
                    }
                    let both = (ta == 3 || ta == 0 - 3) && (tb == 3 || tb == 0 - 3);
                    if !both {
                        if ta >= 0 && tb >= 0 {
                            let mut m = String::from("cannot apply `");
                            m.push_str(op);
                            m.push_str("` to ");
                            m.push_str(&tyname2(ta, ss));
                            m.push_str(" and ");
                            m.push_str(&tyname2(tb, ss));
                            ds.push(oerrors::err(m, *l, *h));
                        }
                        return 3;
                    }
                    if op.as_str() == "=="
                        || op.as_str() == "!="
                        || op.as_str() == "<"
                        || op.as_str() == "<="
                        || op.as_str() == ">"
                        || op.as_str() == ">="
                    {
                        return 1;
                    }
                    return 3;
                }
                if op.as_str() == "+"
                    || op.as_str() == "-"
                    || op.as_str() == "*"
                    || op.as_str() == "/"
                    || op.as_str() == "%"
                    || op.as_str() == "&"
                    || op.as_str() == "|"
                    || op.as_str() == "^"
                    || op.as_str() == "<<"
                    || op.as_str() == ">>"
                {
                    if (ta == 0 || ta == 0 - 3) && (tb == 0 || tb == 0 - 3) {
                        0
                    } else if ta == 1
                        && tb == 1
                        && (op.as_str() == "&" || op.as_str() == "|" || op.as_str() == "^")
                    {
                        1
                    } else {
                        // BOOL-BITOPS: & | ^ on two bools are non-short-circuit logic
                        if ta >= 0 && tb >= 0 {
                            let mut m = String::from("cannot apply `");
                            m.push_str(op);
                            m.push_str("` to ");
                            m.push_str(&tyname(ta));
                            m.push_str(" and ");
                            m.push_str(&tyname(tb));
                            ds.push(oerrors::err(m, *l, *h));
                        }
                        -1
                    }
                } else if op.as_str() == "&&" || op.as_str() == "||" {
                    if ta == 1 && tb == 1 {
                        1
                    } else {
                        if ta >= 0 && tb >= 0 {
                            let mut m = String::from("`");
                            m.push_str(op);
                            m.push_str("` needs bool operands, found ");
                            m.push_str(&tyname2(ta, ss));
                            m.push_str(" and ");
                            m.push_str(&tyname2(tb, ss));
                            ds.push(oerrors::err(m, *l, *h));
                        }
                        -1
                    }
                } else if op.as_str() == "==" || op.as_str() == "!=" {
                    if (ta == tb && ta >= 0)
                        || ta == 0 - 3
                        || tb == 0 - 3
                        || (ta == 1000 && tb == 200)
                    {
                        1
                    } else {
                        if ta >= 0 && tb >= 0 {
                            let mut m = String::from("cannot compare ");
                            m.push_str(&tyname2(ta, ss));
                            m.push_str(" with ");
                            m.push_str(&tyname2(tb, ss));
                            ds.push(oerrors::err(m, *l, *h));
                        }
                        -1
                    }
                } else {
                    if (ta == 0 || ta == 0 - 3) && (tb == 0 || tb == 0 - 3) {
                        1
                    } else {
                        if ta == 2 && tb == 2 {
                            1
                        } else if ta == tb && ta >= 100 && ta < 200 && {
                            // ORD-UNIT-ENUM: a unit-only enum orders by declaration (derive(PartialOrd)); tags compare
                            let ei = (ta - 100) as usize;
                            let mut unit = ei < ez.len();
                            if unit {
                                let mut k = 0;
                                while k < ez[ei].variants.len() {
                                    if ez[ei].variants[k].npayload > 0 {
                                        unit = false;
                                    }
                                    k = k + 1;
                                }
                            }
                            unit
                        } {
                            1
                        } else {
                            if ta >= 0 && tb >= 0 {
                                let mut m = String::from("cannot order ");
                                m.push_str(&tyname2(ta, ss));
                                m.push_str(" with ");
                                m.push_str(&tyname2(tb, ss));
                                ds.push(oerrors::err(m, *l, *h));
                            }
                            -1
                        }
                    }
                }
            }
        }
        E::If(c, th, el, _l, _h) => {
            let tc = check_e(c, env, ds, ss, fz, ez, tz);
            let tt = check_e(th, env, ds, ss, fz, ez, tz);
            let te = check_e(el, env, ds, ss, fz, ez, tz);
            if tc != 1 && tc >= 0 {
                let mut m = String::from("`if` condition must be bool, found ");
                m.push_str(&tyname(tc));
                ds.push(oerrors::err(m, oast::e_lo(c), oast::e_hi(c)));
            }
            if !tyeq(tt, te) && tt >= 0 && te >= 0 {
                let mut m = String::from("`if` and `else` have incompatible types: ");
                m.push_str(&tyname(tt));
                m.push_str(" vs ");
                m.push_str(&tyname(te));
                ds.push(oerrors::err(m, oast::e_lo(el), oast::e_hi(el)));
                return -1;
            }
            if tc != 1 {
                -1
            } else {
                tt
            }
        }
        E::Let(n, v, b, _l, _h) => {
            let tv = check_e(v, env, ds, ss, fz, ez, tz);
            // TY-IF-NOELSE (E0317): `let x = if c { v };` -- an `if` with no `else` has type (), so a VALUE in its then-arm
            // is a mismatch. The parser marks the missing else as Int(0) with a zero-width span.
            match v.as_ref() {
                E::If(_c, _t, el, _l2, _h2) => match el.as_ref() {
                    E::Int(z, zl, zh) => {
                        if *z == 0 && *zl == *zh && tv >= 0 && tv <= 3 {
                            ds.push(oerrors::err(String::from("`if` may be missing an `else` clause (the `if` has a value but no `else`)"), oast::e_lo(v), oast::e_hi(v)));
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
            env.n.push(String::from(n.as_str()));
            env.t.push(tv);
            let rb = check_e(b, env, ds, ss, fz, ez, tz);
            env.n.pop();
            env.t.pop();
            rb
        }
    }
}
pub fn fenv_of(f: &Fun, ss: &Vec<Sdef>) -> oast::Env {
    let mut nn: Vec<String> = Vec::new();
    let mut tt: Vec<i32> = Vec::new();
    let mut i = 0;
    while i < f.params.len() {
        nn.push(String::from(f.params[i].as_str()));
        let t0 = if i < f.ptys.len() {
            tag_of_tyname(ss, &f.ptys[i])
        } else {
            0
        };
        let t = if t0 < 0 { 0 - 3 } else { t0 };
        tt.push(t);
        i = i + 1;
    }
    oast::Env { n: nn, t: tt }
}
pub fn fenv_of_t(f: &Fun, ss: &Vec<Sdef>, ez: &Vec<Edef>, tz: &Vec<Tdef>) -> oast::Env {
    let mut nn: Vec<String> = Vec::new();
    let mut tt: Vec<i32> = Vec::new();
    let mut i = 0;
    while i < f.params.len() {
        nn.push(String::from(f.params[i].as_str()));
        let t0 = if i < f.ptys.len() {
            type_tag(ss, ez, tz, &f.ptys[i])
        } else {
            0
        };
        let t = if t0 < 0 { 0 - 3 } else { t0 };
        tt.push(t);
        i = i + 1;
    }
    oast::Env { n: nn, t: tt }
}
pub fn fn_arity(funs: &Vec<Fun>, n: &str) -> i32 {
    let mut i = 0;
    while i < funs.len() {
        if funs[i].name.as_str() == n {
            return funs[i].params.len() as i32;
        }
        i = i + 1;
    }
    -1
}
pub fn fn_ret(funs: &Vec<Fun>, n: &str) -> String {
    let mut i = 0;
    while i < funs.len() {
        if funs[i].name.as_str() == n {
            return String::from(funs[i].ret.as_str());
        }
        i = i + 1;
    }
    String::from("int")
}
pub fn check_calls(e: &E, funs: &Vec<Fun>, ds: &mut Vec<Diag>) {
    let ez: Vec<Edef> = Vec::new();
    let tz: Vec<Tdef> = Vec::new();
    let mut ev = oast::Env {
        n: Vec::new(),
        t: Vec::new(),
    };
    check_calls_t(e, funs, ds, &empty_sdefs(), &mut ev, &ez, &tz)
}
pub fn check_calls_t(
    e: &E,
    funs: &Vec<Fun>,
    ds: &mut Vec<Diag>,
    ss: &Vec<Sdef>,
    env: &mut oast::Env,
    ez: &Vec<Edef>,
    tz: &Vec<Tdef>,
) {
    match e {
        E::Call(n, args, _lg2, _hg2) if olower::is_gname(n) => {
            let mut k = 0;
            while k < args.len() {
                check_calls_t(&args[k], funs, ds, ss, env, ez, tz);
                k = k + 1;
            }
        }
        E::Call(n, args, _l2, _h2)
            if n.as_str() == "$shovf"
                || n.as_str() == "$ovf1"
                || n.as_str() == "drop"
                || n.as_str() == "print"
                || n.as_str() == "len"
                || n.as_str() == "print_int"
                || n.as_str() == "streq"
                || n.as_str() == "vec_new"
                || n.as_str() == "vec_push"
                || n.as_str() == "vec_push1"
                || n.as_str() == "$cellnew"
                || n.as_str() == "$rcnew"
                || n.as_str() == "$boxnew"
                || n.as_str() == "$cellhandle"
                || n.as_str() == "$rcclone"
                || n.as_str() == "$rccount"
                || n.as_str() == "vec_get"
                || n.as_str() == "vec_set"
                || n.as_str() == "vec_len"
                || n.as_str() == "str_at"
                || n.as_str() == "str_cat"
                || n.as_str() == "str_copy"
                || n.as_str() == "$zeroof"
                || n.as_str() == "$zerodflt"
                || olower::zeroty_name(n).len() > 0
                || oast::str_starts(n.as_str(), "$xmute$")
                || oast::str_starts(n.as_str(), "$sizeof$")
                || n.as_str() == "$sizeofval"
                || n.as_str() == "$alias"
                || n.as_str() == "$ptrnull"
                || n.as_str() == "$dropfield"
                || n.as_str() == "$partialmove"
                || n.as_str() == "$updfromdrop"
                || n.as_str() == "$chanint"
                || oast::str_starts(n.as_str(), "$alignof$")
                || n.as_str() == "str_sub"
                || n.as_str() == "print_any"
                || n.as_str() == "print_dbg"
                || n.as_str() == "print_raw"
                || n.as_str() == "fmtpad_call"
                || n.as_str() == "print_int_raw"
                || n.as_str() == "print_char"
                || n.as_str() == "print_nl"
                || n.as_str() == "panic_exit"
                || n.as_str() == "arreq"
                || n.as_str() == "arr_len"
                || n.as_str() == "veceq"
                || n.as_str() == "enumeq"
                || n.as_str() == "proc_exit"
                || n.as_str() == "vec_popraw"
                || n.as_str() == "arr_fill"
                || n.as_str() == "rc_new"
                || n.as_str() == "rc_inc"
                || n.as_str() == "rc_dec"
                || n.as_str() == "rc_count"
                || n.as_str() == "str_of"
                || n.as_str() == "str_dbg"
                || n.as_str() == "seq_len"
                || n.as_str() == "seq_get"
                || n.as_str() == "seq_set"
                || n.as_str() == "seq_slice"
                || n.as_str() == "f64_parse"
                || n.as_str() == "f64_snprintf"
                || n.as_str() == "f64_snprintf_e"
                || n.as_str() == "f64_bits"
                || n.as_str() == "f64_isnan"
                || n.as_str() == "readfile"
                || n.as_str() == "fileok"
                || n.as_str() == "stdio_swap"
                || n.as_str() == "argc"
                || n.as_str() == "argv_at"
                || n.as_str() == "load_at"
                || n.as_str() == "store_at"
                || n.as_str() == "vec_elem_addr"
                || n.as_str() == "fld_addr" =>
        {
            let mut k = 0;
            while k < args.len() {
                check_calls_t(&args[k], funs, ds, ss, env, ez, tz);
                k = k + 1;
            }
        }
        E::Call(n, args, _l9, _h9)
            if {
                let mut t = 0 - 1;
                let mut q = env.n.len();
                while q > 0 {
                    q = q - 1;
                    if env.n[q].as_str() == n.as_str() {
                        t = env.t[q];
                        q = 0;
                    }
                }
                t == 900
            } =>
        {
            let mut k = 0;
            while k < args.len() {
                check_calls_t(&args[k], funs, ds, ss, env, ez, tz);
                k = k + 1;
            }
        }
        E::Call(n, args, l, h)
            if {
                let mut t = 0 - 1;
                let mut q = env.n.len();
                while q > 0 {
                    q = q - 1;
                    if env.n[q].as_str() == n.as_str() {
                        t = env.t[q];
                        q = 0;
                    }
                }
                t >= 800 && t < 1000
            } =>
        {
            let mut k = 0;
            while k < args.len() {
                check_calls_t(&args[k], funs, ds, ss, env, ez, tz);
                k = k + 1;
            }
        }
        E::Call(n, args, l, h) => {
            let a = fn_arity(funs, n);
            if a < 0 {
                let mut m = String::from("cannot find function `");
                m.push_str(n);
                m.push_str("` in this scope");
                ds.push(oerrors::err(m, *l, *h));
            } else if a != (args.len() as i32) {
                let mut m = String::from("function `");
                m.push_str(n);
                m.push_str("` takes ");
                m.push_str(&oast::u32s2(a as u32));
                m.push_str(" argument(s) but ");
                m.push_str(&oast::u32s2(args.len() as u32));
                m.push_str(" supplied");
                ds.push(oerrors::err(m, *l, *h));
            }
            let mut k = 0;
            while k < args.len() {
                check_calls_t(&args[k], funs, ds, ss, env, ez, tz);
                k = k + 1;
            }
        }
        E::Bin(_o, a, b, _l, _h) => {
            check_calls_t(a, funs, ds, ss, env, ez, tz);
            check_calls_t(b, funs, ds, ss, env, ez, tz);
        }
        E::While(c, b, _l, _h) => {
            check_calls_t(c, funs, ds, ss, env, ez, tz);
            check_calls_t(b, funs, ds, ss, env, ez, tz);
        }
        E::Not(x, _l, _h) => {
            check_calls_t(x, funs, ds, ss, env, ez, tz);
        }
        E::StrLit(_v, _l, _h) => {}
        E::ClosRef(_n, _c, _l, _h) => {}
        E::Try(x, _l, _h) => {
            check_calls_t(x, funs, ds, ss, env, ez, tz);
        }
        E::BreakE(_l, _h) => {}
        E::BreakN(_n, _l, _h) => {}
        E::Ref(x, _m, _l, _h) => {
            check_calls_t(x, funs, ds, ss, env, ez, tz);
        }
        E::Cast(x, _t, _l, _h) => {
            check_calls_t(x, funs, ds, ss, env, ez, tz);
        }
        E::ContE(_l, _h) => {}
        E::ContN(_n, _l, _h) => {}
        E::LetT(n, _t, v, b, _l, _h) => {
            check_calls_t(v, funs, ds, ss, env, ez, tz);
            match v.as_ref() {
                E::ClosRef(cn, _c, _l2, _h2) => {
                    let ci = clos_idx_of_name(cn);
                    env.n.push(String::from(n.as_str()));
                    env.t.push(800 + ci);
                    check_calls_t(b, funs, ds, ss, env, ez, tz);
                    env.n.pop();
                    env.t.pop();
                }
                _ => {
                    let td = type_tag(ss, ez, tz, _t);
                    env.n.push(String::from(n.as_str()));
                    env.t.push(td);
                    check_calls_t(b, funs, ds, ss, env, ez, tz);
                    env.n.pop();
                    env.t.pop();
                }
            }
        }
        E::TupLit(parts, _l, _h) => {
            let mut k = 0;
            while k < parts.len() {
                check_calls_t(&parts[k], funs, ds, ss, env, ez, tz);
                k = k + 1;
            }
        }
        E::TupIdx(b, _i, _l, _h) => {
            check_calls_t(b, funs, ds, ss, env, ez, tz);
        }
        E::RetE(x, _l, _h) => {
            check_calls_t(x, funs, ds, ss, env, ez, tz);
        }
        E::MethodCall(rv, _m, args, _l, _h) => {
            check_calls_t(rv, funs, ds, ss, env, ez, tz);
            let mut k = 0;
            while k < args.len() {
                check_calls_t(&args[k], funs, ds, ss, env, ez, tz);
                k = k + 1;
            }
        }
        E::AssignTo(pl, v, _l, _h) => {
            check_calls_t(pl, funs, ds, ss, env, ez, tz);
            check_calls_t(v, funs, ds, ss, env, ez, tz);
        }
        E::ArrLit(xs, _l, _h) => {
            let mut k = 0;
            while k < xs.len() {
                check_calls_t(&xs[k], funs, ds, ss, env, ez, tz);
                k = k + 1;
            }
        }
        E::Index(b, ix, _l, _h) => {
            check_calls_t(b, funs, ds, ss, env, ez, tz);
            check_calls_t(ix, funs, ds, ss, env, ez, tz);
        }
        E::EnumLit(_e2, _v, ps, _l, _h) => {
            let mut k = 0;
            while k < ps.len() {
                check_calls_t(&ps[k], funs, ds, ss, env, ez, tz);
                k = k + 1;
            }
        }
        // C-ARGENUM: an arm's payload BINDINGS must be in scope while its body is checked -- the main type
        // rule binds them, this walk did not, so any argument expression over a binding fell to the
        // untyped fallback and reported a type error at the call instead.
        E::Match(sc, arms, _l, _h) => {
            check_calls_t(sc, funs, ds, ss, env, ez, tz);
            let en = {
                let mut dsx: Vec<Diag> = Vec::new();
                let ts = check_e(sc, env, &mut dsx, ss, funs, ez, tz);
                if ts >= 100 && ts < 200 && ((ts - 100) as usize) < ez.len() {
                    String::from(ez[(ts - 100) as usize].name.as_str())
                } else {
                    String::from("")
                }
            };
            let mut k = 0;
            while k < arms.len() {
                if en.len() > 0 && arms[k].binds.len() > 0 {
                    let mut nb = 0;
                    let mut bi = 0;
                    while bi < arms[k].binds.len() {
                        env.n.push(String::from(arms[k].binds[bi].as_str()));
                        let pt = oast::vpayload_ty(ez, &en, &arms[k].variant, bi);
                        let tg = type_tag(ss, ez, tz, &pt);
                        env.t.push(if tg >= 0 { tg } else { 0 - 3 });
                        nb = nb + 1;
                        bi = bi + 1;
                    }
                    check_calls_t(&arms[k].body, funs, ds, ss, env, ez, tz);
                    {
                        let mut q = 0;
                        while q < nb {
                            env.n.pop();
                            env.t.pop();
                            q = q + 1;
                        }
                    }
                } else {
                    check_calls_t(&arms[k].body, funs, ds, ss, env, ez, tz);
                }
                k = k + 1;
            }
        }
        E::StructLit(_n, fs, _l, _h) => {
            let mut k = 0;
            while k < fs.len() {
                check_calls_t(&fs[k], funs, ds, ss, env, ez, tz);
                k = k + 1;
            }
        }
        E::Field(o, _f, _l, _h) => {
            check_calls_t(o, funs, ds, ss, env, ez, tz);
        }
        E::Seq(a, b, _l, _h) => {
            check_calls_t(a, funs, ds, ss, env, ez, tz);
            check_calls_t(b, funs, ds, ss, env, ez, tz);
        }
        E::Assign(_n, v, _l, _h) => {
            check_calls_t(v, funs, ds, ss, env, ez, tz);
        }
        E::If(c, t, f, _l, _h) => {
            // IF-ENV (R2a P4): the branches see the enclosing bindings -- a FRESH env dropped every closure binding, so
            // `let sq = |x| x * x; if c { sq(n) }` was "cannot find function `sq`" (Let pushes/pops, so env is balanced)
            check_calls_t(c, funs, ds, ss, env, ez, tz);
            check_calls_t(t, funs, ds, ss, env, ez, tz);
            check_calls_t(f, funs, ds, ss, env, ez, tz);
        }
        E::Let(n, v, b, _l, _h) => {
            check_calls_t(v, funs, ds, ss, env, ez, tz);
            match v.as_ref() {
                E::ClosRef(cn, _c, _l2, _h2) => {
                    let ci = clos_idx_of_name(cn);
                    env.n.push(String::from(n.as_str()));
                    env.t.push(800 + ci);
                    check_calls_t(b, funs, ds, ss, env, ez, tz);
                    env.n.pop();
                    env.t.pop();
                }
                _ => {
                    let mut dsx: Vec<Diag> = Vec::new();
                    let tv = check_e(v, env, &mut dsx, ss, funs, ez, tz);
                    env.n.push(String::from(n.as_str()));
                    env.t.push(tv);
                    check_calls_t(b, funs, ds, ss, env, ez, tz);
                    env.n.pop();
                    env.t.pop();
                }
            }
        }
        _ => {}
    }
}
// GUARD-ARM-BINDS: an arm arm_rewrite_all reshaped (a nested / literal sub-pattern became a synthetic `__pmN` + guard + let) is
// not a plain positional arm -- binding `__pmN` with its payload's coarse tag (a tuple tag carries no element types) typed the
// destructured names as int (`Some((s, n))` then `s.len()`: "no method len on int"); such arms stay unbound as before
fn arm_has_rewrite_bind(bs: &Vec<String>) -> bool {
    let mut i = 0;
    while i < bs.len() {
        if oast::str_starts(bs[i].as_str(), "__pm") {
            return true;
        }
        i = i + 1;
    }
    false
}
// GUARD-ARM-BINDS: a STRUCT-variant arm (named fields) is rewritten with field literals as synthetic guards that compare a bool
// field against an int-encoded literal -- binding the field with its bool payload type made that guard "cannot compare bool with
// int" (heavy pass 5); struct-variant arms stay unbound as before (only tuple-variant arms are bound)
fn variant_is_struct(ez: &Vec<Edef>, en: &str, v: &str) -> bool {
    let mut i = 0;
    while i < ez.len() {
        if ez[i].name.as_str() == en {
            let mut k = 0;
            while k < ez[i].variants.len() {
                if ez[i].variants[k].name.as_str() == v {
                    return ez[i].variants[k].fnames.len() > 0;
                }
                k = k + 1;
            }
        }
        i = i + 1;
    }
    false
}
// STMT-IF (R2a P4): check a statement-position if (and its else-if chain): the condition must be bool; the branches are checked
// for their own errors but their types are NOT compared (the if has no value)
fn check_stmt_if(
    c: &E,
    th: &E,
    el: &E,
    env: &mut oast::Env,
    ds: &mut Vec<Diag>,
    ss: &Vec<Sdef>,
    fz: &Vec<Fun>,
    ez: &Vec<Edef>,
    tz: &Vec<Tdef>,
) {
    let tc = check_e(c, env, ds, ss, fz, ez, tz);
    if tc != 1 && tc >= 0 {
        let mut m = String::from("`if` condition must be bool, found ");
        m.push_str(&tyname(tc));
        ds.push(oerrors::err(m, oast::e_lo(c), oast::e_hi(c)));
    }
    let _ = check_e(th, env, ds, ss, fz, ez, tz);
    if let E::If(c2, th2, el2, _l2, _h2) = el {
        check_stmt_if(c2, th2, el2, env, ds, ss, fz, ez, tz);
    } else {
        let _ = check_e(el, env, ds, ss, fz, ez, tz);
    }
}
