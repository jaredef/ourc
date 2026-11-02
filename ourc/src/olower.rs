pub struct LCx {
    pub gsub: Vec<String>, // G1 G-P1: the instance being lowered: its parameter -> type map ("T=u16")
    pub gops: Vec<String>,
    pub ginst: Vec<String>,
    pub fpays: Vec<String>,
    pub cellvars: Vec<String>,
    pub captys: Vec<String>,
    pub fnpar: Vec<i32>,
    pub selfty: String,
    pub names: Vec<String>,
    pub next_id: u32,
    pub ar: oarena::IdArena,
    pub sdefs: Vec<Sdef>,
    pub stypes: Vec<String>,
    pub fnames: Vec<String>,
    pub frets: Vec<String>,
    pub fptys: Vec<String>,
    pub edefs: Vec<Edef>,
    pub fgps: Vec<String>,
    pub fgbs: Vec<String>,
    pub demands: Vec<String>,
    pub dimpls: Vec<String>,
    pub fncaps: Vec<i32>,
    pub addr_fns: Vec<String>,
    pub pass_fns: Vec<String>,
    pub dvars: Vec<String>,
    pub pdrops: Vec<String>, // DROP-JUMP: pending scope drops (`n\tvt\tslot`), "|" marks a loop body start
    pub fresh_fns: Vec<String>,
    pub gluestk: Vec<String>,
    pub movecaps: Vec<String>,
    pub moveclos: Vec<String>, // CAP-DROP: the `move` closures (they own their captures)
    pub capuse: Vec<String>, // CAP-DROP: "<closure>|<capture>=*" (used whole) or "=f1,f2" (the fields its body reads)
    pub rcvars: Vec<String>, // RC-IDENTITY (R4c): the bindings in scope that hold an Rc handle (clone shares, drop decrements)
    pub borrowcaps: Vec<String>, // CLOS-BORROW: "<closure>|v" -- a non-move closure that only reads v through fields borrows it
    pub refself: Vec<String>, // REF-SELF (W1): methods `Ty$m` whose receiver is `&self` / `&mut self` (a call borrows, never moves)
    pub rangevars: Vec<String>, // R8 RANGE-DBG: let-bound names whose value is a range (its Debug is `a..b`, not the Vec)
    pub dbginst: String, // R12d DBG-GEN: the generic instance ("P=val,Q=val") of the value `{:?}` is printing
    pub curfn: String,   // FN-BOUND-RET: the fn being lowered (its "@fnret" bound records)
    pub tupowned: Vec<String>, // TUP-GLUE: bindings that hold a tuple of OWNED values (a literal with no `&` element, or a move of one)
    pub tupglue: bool, // TUP-GLUE: the drop being built may glue a tuple (set only for a tupowned binding)
    pub recv_top: bool,
    pub borrow_bind: bool,
    // (curfn, above, is also VIS-FIELD-READ (R2a P6)'s qualified fn name: its module is the prefix before the last `$`) // FOR-BORROW: the let being lowered binds a by-reference loop element (never dropped)
}
pub fn lcx_new() -> LCx {
    LCx {
        gsub: Vec::new(),
        gops: Vec::new(),
        ginst: Vec::new(),
        fpays: Vec::new(),
        cellvars: Vec::new(),
        captys: Vec::new(),
        fnpar: Vec::new(),
        selfty: String::from(""),
        names: Vec::new(),
        next_id: 0,
        ar: oarena::ar_new(),
        sdefs: Vec::new(),
        stypes: Vec::new(),
        fnames: Vec::new(),
        frets: Vec::new(),
        fptys: Vec::new(),
        edefs: Vec::new(),
        fgps: Vec::new(),
        fgbs: Vec::new(),
        demands: Vec::new(),
        dimpls: Vec::new(),
        fncaps: Vec::new(),
        addr_fns: Vec::new(),
        pass_fns: Vec::new(),
        dvars: Vec::new(),
        pdrops: Vec::new(),
        fresh_fns: Vec::new(),
        gluestk: Vec::new(),
        movecaps: Vec::new(),
        moveclos: Vec::new(),
        capuse: Vec::new(),
        rcvars: Vec::new(),
        borrowcaps: Vec::new(),
        refself: Vec::new(),
        rangevars: Vec::new(),
        dbginst: String::new(),
        curfn: String::new(),
        tupowned: Vec::new(),
        tupglue: false,
        recv_top: false,
        borrow_bind: false,
    }
}
// C-LCXREBUILD: the nine table fields (sdefs, fnames, frets, fptys, edefs, fgps, fgbs, dimpls, fncaps) are
// WHOLE-PROGRAM and identical for every function lowered against the same Prog; only selfty/names/stypes/
// next_id/ar/demands are per-function. Rebuilding the tables per function made the signature table the
// arena dominator (measured: ~84% of it), so a caller that lowers many functions builds ONE context and
// resets just the per-function half between them. This is exactly lcx_new() minus the tables.
// C-PSFILTER: the recorded type of a PARAMETER (what lowering later reads back as its "static type"), in
// ONE place. It was written out four times -- odrv's emit loop and three sites in omono -- which is why
// nothing could widen it: a change had to be made in four copies to take effect. Scalars are recorded as
// the empty string DELIBERATELY: for a struct, enum, str, Fn, dyn object or Vec the NAME is the dispatch
// key, and a scalar needed none. That blindness is what blocks an integer `!` (see C-BITNOT, rung 44).
pub fn local_sty(ss: &Vec<Sdef>, ez: &Vec<Edef>, pt: &str) -> String {
    if oast::is_struct_name(ss, pt)
        || oast::is_enum_name(ez, pt)
        || pt == "str"
        || pt == "Fn"
        || dyn_trait_of(pt).len() > 0
        || vec_elem_of(pt).len() > 0
    {
        return String::from(pt);
    }
    // C-ARRLOCAL: `arr` and `bstr` were the two sequence types this rule DROPPED, so a parameter declared
    // either recorded no type at all -- and then seq_len/seq_get, which dispatch on the recorded type,
    // fell back to the VECTOR helpers and read an array's buffer as a vector header. That is why `{:?}`
    // on an array was accepted and SEGFAULTED: the prelude's dbg_ai takes an `arr`.
    if is_arr_ty(pt) || pt == "bstr" {
        return String::from(pt);
    }
    if oparse::tup_elems(pt).len() > 0 {
        return String::from(pt); // a TUPLE local (a tuple param or capture) keeps its element types -- `t.0` of a captured (String, _) was untyped
    }
    // widened: a SCALAR's name is recorded too, normalised to the three this tier actually distinguishes.
    // Nothing dispatches on it, but every rule that must tell an int local from a bool local was blind
    // without it (C-BITNOT, and the width-dependent arithmetic in the C1 ledger).
    if pt == "bool" {
        return String::from("bool");
    }
    if pt == "f64" || pt == "f32" {
        return String::from("f64");
    }
    // C-INTWIDTH: the DECLARED width is kept, not normalised away. Every rule that only wants
    // "is this an integer" already pairs its == "int" test with is_int_width, and the rules that need the
    // width -- wrapping arithmetic, and the C1 ledger's narrow-width arithmetic -- have no other source
    // for it.
    // V4.4 CHAR-PARAM: a char keeps "char" -- a `char` local already does, and answering "int" made `{}` / `{:?}` of a
    // char PARAMETER print its code point (`fn za(v: char) { println!("{}", v) }` printed 99)
    if pt == "char" {
        return String::from("char");
    }
    if pt == "int" {
        return String::from("int");
    }
    if oast::is_int_width(pt) {
        return String::from(pt);
    }
    String::from("")
}
pub fn lcx_reset(lc: &mut LCx) {
    while lc.cellvars.len() > 0 {
        lc.cellvars.pop();
    }
    lc.selfty = String::from("");
    while lc.names.len() > 0 {
        lc.names.pop();
    }
    while lc.stypes.len() > 0 {
        lc.stypes.pop();
    }
    while lc.demands.len() > 0 {
        lc.demands.pop();
    }
    while lc.dvars.len() > 0 {
        lc.dvars.pop();
    }
    while lc.pdrops.len() > 0 {
        lc.pdrops.pop();
    }
    while lc.tupowned.len() > 0 {
        lc.tupowned.pop();
    }
    lc.next_id = 0;
    lc.ar = oarena::ar_new();
}
fn fret_of(cx: &LCx, n: &str) -> String {
    let mut i = 0;
    while i < cx.fnames.len() {
        if cx.fnames[i].as_str() == n {
            return String::from(cx.frets[i].as_str());
        }
        i = i + 1;
    }
    String::from("")
}
fn fidx_of(cx: &LCx, n: &str) -> i32 {
    let mut i = 0;
    while i < cx.fnames.len() {
        if cx.fnames[i].as_str() == n {
            return i as i32;
        }
        i = i + 1;
    }
    -1
}
// the index of the parameter whose DECLARED type name equals `ty` (-1 if none) -- how a generic return `T` is
// tied back to the argument that supplies T.
// A `dyn Trait` value is a FAT PAIR {type_tag, data}: the tag indexes the trait's implementing types in
// declaration order (cx.dimpls holds "Trait|Type" in that order), and `data` is the ordinary struct handle.
// A method call on it emits a tag-dispatch CHAIN of direct calls -- a vtable whose entries are resolved
// statically, which is what this tier can express without function pointers.
// `Vec$dyn$Trait` is the type NAME of a heterogeneous vector; its ELEMENT type is the `dyn$Trait` suffix.
// the method name each overloadable operator dispatches to (Rust's std::ops names)
// the builtin a method call on a BUILT-IN type routes to (so programs can read like Rust)
// `$gload$NAME` / `$gstore$NAME`
pub fn is_gname(n: &str) -> bool {
    let b = n.as_bytes();
    b.len() > 7 && b[0] == 36 && b[1] == 103 && (b[2] == 108 || b[3] == 116)
}
pub fn gname_of(n: &str) -> String {
    let b = n.as_bytes();
    let mut i = 1;
    let mut seen = 0;
    while i < b.len() {
        if b[i] == 36 {
            seen = seen + 1;
            if seen == 1 {
                return String::from(&n[i + 1..]);
            }
        }
        i = i + 1;
    }
    String::from("")
}
pub fn self_fix(cx: &LCx, n: &str) -> String {
    let b = n.as_bytes();
    if b.len() > 5
        && b[0] == 83
        && b[1] == 101
        && b[2] == 108
        && b[3] == 102
        && b[4] == 36
        && cx.selfty.len() > 0
    {
        let mut r = String::from(cx.selfty.as_str());
        r.push('$');
        r.push_str(&n[5..]);
        return r;
    }
    String::from(n)
}
// can this type's Debug be emitted from its static shape? (and which shape)
pub fn dbg_emit_ty(ty: &str) -> String {
    if ty == "arr" {
        return String::from("ai");
    }
    // C-ARRELEM: an array of ints prints with the array printer; an array of anything else is printed as
    // the VECTOR of that element, which is exactly what arr_slice turns it into at the call site.
    {
        let ae = arr_elem_of(ty);
        if ae.len() > 0 {
            if ae.as_str() == "int" || oast::is_int_width(&ae) {
                return String::from("ai");
            } // DBG-VCHAR: a char array goes the Vec<char> way below -- the int printer showed [97, 98] for ['a', 'b']
            let mut vt = String::from("Vec$");
            vt.push_str(ae.as_str());
            return dbg_emit_ty(&vt);
        }
    }
    {
        let el = vec_elem_of(ty);
        if el.len() > 0 {
            if el.as_str() == "str" {
                return String::from("vs");
            }
            if el.as_str() == "char" {
                return String::from("vc");
            } // DBG-VCHAR
            if el.as_str() == "f64" {
                return String::from("vf");
            } // DBG-VF64: floats print Debug-style (1.0, -0.0)
            if el.as_str() == "bool" {
                return String::from("vb");
            }
            if vec_elem_of(&el).len() > 0 {
                if vec_elem_of(&el).as_str() == "int" || oast::is_int_width(&vec_elem_of(&el)) {
                    return String::from("vvi");
                }
                return String::from("");
            }
            if el.as_str() == "int" || oast::is_int_width(&el) {
                return String::from("vi");
            }
            return String::from("");
        }
    }
    String::from("")
}
pub fn dbg_emit(cx: &LCx, e: &E, ty: &str) -> String {
    if ty == "unit" {
        return String::from("unit");
    }
    // a tuple LITERAL carries its elements, so its Debug is emittable even though a tuple VARIABLE's is
    // not (our tuple type name records only the arity)
    match e {
        E::TupLit(p, _l, _h) => {
            if p.len() > 0 {
                return String::from("tuplit");
            }
        }
        _ => {}
    }
    // ... and a tuple VARIABLE is printable too once its name carries them (oparse::tup_name)
    if oparse::tup_elems(ty).len() > 0 {
        return String::from("tupty");
    }
    {
        let k = dbg_emit_ty(ty);
        if k.len() > 0 {
            return k;
        }
    }
    {
        let el = vec_elem_of(ty);
        if el.len() > 0 {
            if el.as_str() == "str" {
                return String::from("vs");
            }
            if el.as_str() == "char" {
                return String::from("vc");
            } // DBG-VCHAR
            if el.as_str() == "f64" {
                return String::from("vf");
            } // DBG-VF64: floats print Debug-style (1.0, -0.0)
            if el.as_str() == "bool" {
                return String::from("vb");
            }
            if vec_elem_of(&el).len() > 0 {
                if vec_elem_of(&el).as_str() == "int" || oast::is_int_width(&vec_elem_of(&el)) {
                    return String::from("vvi");
                }
                return String::from("");
            }
            if el.as_str() == "int" || oast::is_int_width(&el) {
                return String::from("vi");
            }
            {
                let mut i = 0;
                while i < cx.sdefs.len() {
                    if cx.sdefs[i].name.as_str() == el.as_str() {
                        return String::from("vstruct");
                    }
                    i = i + 1;
                }
            } // P-FMT-NEST: Vec<struct> via the dbg_build loop
            if oast::eidx(&cx.edefs, &el) >= 0 {
                return String::from("vstruct");
            } // P-FMT-NEST r3: Vec<enum> too (the loop dispatches via dbg_scalar)
            if oparse::tup_elems(&el).len() > 0 {
                return String::from("vstruct");
            } // DBG-VTUP: Vec<(T, U)> loops too (dbg_scalar prints a typed tuple)
            return String::from("");
        }
    }
    // a bare `Vec` carries no element type: refuse rather than assume (a str printed as an int is a
    // pointer value, which is the worst kind of answer)
    if ty == "Vec" {
        return String::from("");
    }
    if oast::eidx(&cx.edefs, ty) >= 0 {
        return String::from("enum");
    }
    {
        let mut i = 0;
        while i < cx.sdefs.len() {
            if cx.sdefs[i].name.as_str() == ty {
                return String::from("struct");
            }
            i = i + 1;
        }
    }
    String::from("")
}
fn dbg_str(cx: &mut LCx, t: &str) -> H {
    let i = fresh(cx);
    H::StrH(String::from(t), i)
}
fn dbg_print(cx: &mut LCx, t: &str) -> H {
    let sh = dbg_str(cx, t);
    let mut a: Vec<H> = Vec::new();
    a.push(sh);
    let i = fresh(cx);
    H::CallH(String::from("print_raw"), a, i)
}
fn dbg_seq(cx: &mut LCx, a: H, b: H) -> H {
    let i = fresh(cx);
    H::SeqH(Box::new(a), Box::new(b), i)
}
// the Debug printer for a value of a known type, as a sequence of prints
fn dbg_enum_val(cx: &mut LCx, lv: H, ty: &str) -> H {
    let tyb = {
        let v = split_dollar_l(ty);
        String::from(v[0].as_str())
    };
    let ei = {
        let e0 = oast::eidx(&cx.edefs, ty);
        if e0 >= 0 {
            e0
        } else {
            oast::eidx(&cx.edefs, tyb.as_str())
        }
    };
    let ed = &cx.edefs[ei as usize]; /* DBG-PAYLOAD: Option$T / Result$T$E look up their base enum */
    let mut vn: Vec<String> = Vec::new();
    let mut vnp: Vec<u32> = Vec::new();
    let mut vpt: Vec<String> = Vec::new();
    let mut vfn: Vec<String> = Vec::new();
    {
        let mut i = 0;
        while i < ed.variants.len() {
            vn.push(String::from(ed.variants[i].name.as_str()));
            vnp.push(ed.variants[i].npayload);
            let mut j = String::from("");
            let mut k = 0;
            while k < ed.variants[i].ptys.len() {
                if k > 0 {
                    j.push(',');
                }
                j.push_str(ed.variants[i].ptys[k].as_str());
                k = k + 1;
            }
            vpt.push(j);
            {
                let mut j2 = String::from("");
                let mut k2 = 0;
                while k2 < ed.variants[i].fnames.len() {
                    if k2 > 0 {
                        j2.push(',');
                    }
                    j2.push_str(ed.variants[i].fnames[k2].as_str());
                    k2 = k2 + 1;
                }
                vfn.push(j2);
            }
            i = i + 1;
        }
    }
    cx.names.push(String::from("$dbge"));
    cx.stypes.push(String::from(ty));
    let sl = (cx.names.len() - 1) as u32;
    let i0 = fresh(cx);
    let mut chain = H::Lit(0, i0);
    let mut q = vn.len();
    while q > 0 {
        q = q - 1;
        let mut arm = dbg_print(cx, &vn[q]);
        if vnp[q] > 0 {
            let fns = split_c(vfn[q].as_str());
            let named = fns.len() > 0 && fns[0].len() > 0;
            let o = dbg_print(cx, if named { " { " } else { "(" });
            arm = dbg_seq(cx, arm, o);
            let pts = split_c(vpt[q].as_str());
            let mut k = 0;
            while k < (vnp[q] as usize) {
                if k > 0 {
                    let c = dbg_print(cx, ", ");
                    arm = dbg_seq(cx, arm, c);
                }
                if named && k < fns.len() {
                    let mut lab = String::from(fns[k].as_str());
                    lab.push_str(": ");
                    let l3 = dbg_print(cx, &lab);
                    arm = dbg_seq(cx, arm, l3);
                }
                let ri = fresh(cx);
                let rl = H::Local(sl, ri);
                let fi2 = fresh(cx);
                let fh = H::FieldH(Box::new(rl), (k + 1) as u32, fi2);
                let mut pt = if k < pts.len() {
                    String::from(pts[k].as_str())
                } else {
                    String::from("int")
                };
                // DBG-PAYLOAD: a payload-typed Option$T / Result$T$E prints Some/Ok/Err's payload as T / T / E
                {
                    let ta = split_dollar_l(ty);
                    if ta.len() >= 2 && k == 0 {
                        if ta[0].as_str() == "Option" && vn[q].as_str() == "Some" {
                            pt = String::from(&ty[7..]);
                        } /* the WHOLE rest: a tuple payload `tup2$int$char` has its own `$`s */
                        if ta[0].as_str() == "Result" && ta.len() >= 3 {
                            if vn[q].as_str() == "Ok" {
                                pt = String::from(ta[1].as_str());
                            } else if vn[q].as_str() == "Err" {
                                pt = String::from(ta[2].as_str());
                            }
                        }
                    }
                }
                // V4.3f DBG-GEN-PAYLOAD (refusal > WRONG): a payload typed by the enum's own type PARAMETER (`K(A, B)`) has no
                // instance here: the parser records it as "" (a type parameter), which printed as a word -- right for an int by
                // accident, a handle for `()` / an enum / a str (macro-tt-followed-by-seq printed `Keep(1, 202547280)`). Refused.
                // (Option / Result are typed by DBG-PAYLOAD above; sweep-valid: trivial-message's `Ok(42)` was right)
                if pt.len() == 0 && tyb.as_str() != "Option" && tyb.as_str() != "Result" {
                    let mut a: Vec<H> = Vec::new();
                    let z = fresh(cx);
                    a.push(H::Lit(0, z));
                    let iu = fresh(cx);
                    return H::CallH(String::from("$dbg_unsupported"), a, iu);
                }
                let pp = dbg_scalar(cx, fh, &pt);
                arm = dbg_seq(cx, arm, pp);
                k = k + 1;
            }
            let c2 = dbg_print(cx, if named { " }" } else { ")" });
            arm = dbg_seq(cx, arm, c2);
        }
        let ri2 = fresh(cx);
        let rl2 = H::Local(sl, ri2);
        let ti = fresh(cx);
        let tagh = H::FieldH(Box::new(rl2), 0, ti);
        let li = fresh(cx);
        let lith = H::Lit(q as i64, li);
        let bi = fresh(cx);
        let test = H::BinH(String::from("=="), Box::new(tagh), Box::new(lith), bi);
        let ii = fresh(cx);
        chain = H::IfH(Box::new(test), Box::new(arm), Box::new(chain), ii);
    }
    let fi3 = fresh(cx);
    return H::LetH(sl, Box::new(lv), Box::new(chain), fi3);
}
fn dbg_struct_val(cx: &mut LCx, lv: H, ty: &str) -> H {
    // `Point { x: 1, y: 2 }`, or `P(1, 2)` for a positional one, or `U` for a unit struct
    let mut fl: Vec<String> = Vec::new();
    let mut ft: Vec<String> = Vec::new();
    {
        let mut i = 0;
        while i < cx.sdefs.len() {
            if cx.sdefs[i].name.as_str() == ty {
                let mut k = 0;
                while k < cx.sdefs[i].fields.len() {
                    fl.push(String::from(cx.sdefs[i].fields[k].as_str()));
                    let mut t2 = if k < cx.sdefs[i].ftys.len() {
                        String::from(cx.sdefs[i].ftys[k].as_str())
                    } else {
                        String::from("")
                    };
                    if t2.as_str() == "Option" || t2.as_str() == "Result" {
                        // DBG-FPAY (W7): the field's recorded payload makes it `Option$T` / `Result$T$E` for the payload printer
                        let mut pre = String::from("@fpay:");
                        pre.push_str(ty);
                        pre.push('.');
                        pre.push_str(cx.sdefs[i].fields[k].as_str());
                        pre.push('=');
                        let mut q = 0;
                        while q < cx.fpays.len() {
                            if oast::str_starts(cx.fpays[q].as_str(), pre.as_str()) {
                                let pay = String::from(&cx.fpays[q][pre.len()..]);
                                t2.push('$');
                                t2.push_str(pay.replace(",", "$").as_str());
                                q = cx.fpays.len();
                            }
                            q = q + 1;
                        }
                    }
                    if oast::in_list(&cx.sdefs[i].sgp, t2.as_str()) && cx.dbginst.len() > 0 {
                        // R12d DBG-GEN: a field typed by the struct's parameter takes the printed value's instance
                        let parts = split_commas_l(cx.dbginst.as_str());
                        let mut q = 0;
                        while q < parts.len() {
                            let pb = parts[q].as_bytes();
                            let mut c = 0;
                            while c < pb.len() && pb[c] != 61 {
                                c = c + 1;
                            }
                            if c < pb.len() && &parts[q][..c] == t2.as_str() {
                                t2 = String::from(&parts[q][c + 1..]);
                                q = parts.len();
                            }
                            q = q + 1;
                        }
                    }
                    ft.push(t2);
                    k = k + 1;
                }
            }
            i = i + 1;
        }
    }
    if fl.len() == 0 {
        let dn = dbg_tyname(cx, ty);
        return dbg_print(cx, dn.as_str());
    }
    let positional = fl[0].as_str() == "0";
    // bind the value once; every field read goes through the binding
    cx.names.push(String::from("$dbgv"));
    cx.stypes.push(String::from(ty));
    let sl = (cx.names.len() - 1) as u32;
    let dn = dbg_tyname(cx, ty);
    let mut out = dbg_print(cx, dn.as_str());
    {
        let open = if positional {
            String::from("(")
        } else {
            String::from(" { ")
        };
        let o = dbg_print(cx, &open);
        out = dbg_seq(cx, out, o);
    }
    let mut k = 0;
    while k < fl.len() {
        if k > 0 {
            let c = dbg_print(cx, ", ");
            out = dbg_seq(cx, out, c);
        }
        if !positional {
            let mut lab = String::from(fl[k].as_str());
            lab.push_str(": ");
            let l2 = dbg_print(cx, &lab);
            out = dbg_seq(cx, out, l2);
        }
        let ri = fresh(cx);
        let rl = H::Local(sl, ri);
        let fi2 = fresh(cx);
        let fh = H::FieldH(Box::new(rl), k as u32, fi2);
        let fp = dbg_scalar(cx, fh, &ft[k]);
        out = dbg_seq(cx, out, fp);
        k = k + 1;
    }
    {
        let close = if positional {
            String::from(")")
        } else {
            String::from(" }")
        };
        let c2 = dbg_print(cx, &close);
        out = dbg_seq(cx, out, c2);
    }
    let fi3 = fresh(cx);
    return H::LetH(sl, Box::new(lv), Box::new(out), fi3);
}
pub fn dbg_build(cx: &mut LCx, e: &E, ty: &str) -> H {
    {
        // R12d DBG-GEN: the instance of a generic struct value (`W { v: 5 }` -> "V=int") for dbg_struct_val's field types
        let gi = match e {
            E::Var(n, _l, _h) => {
                let mut pre = String::from(n.as_str());
                pre.push('=');
                let mut r = String::new();
                let mut i = cx.ginst.len();
                while i > 0 && r.len() == 0 {
                    i = i - 1;
                    if oast::str_starts(cx.ginst[i].as_str(), pre.as_str()) {
                        r = String::from(&cx.ginst[i][pre.len()..]);
                    }
                }
                r
            }
            _ => ginst_of(cx, e),
        };
        let gb = gi.as_bytes();
        let mut c = 0;
        while c < gb.len() && gb[c] != 58 {
            c = c + 1;
        }
        cx.dbginst = if c < gb.len() {
            String::from(&gi[c + 1..])
        } else {
            String::new()
        };
    }
    {
        // R8 RANGE-DBG: a range is a Vec of its elements here, so `{:?}` printed `[2, 3, 4, 5]` for `2..6` (WRONG) -- refused
        let isr = match e {
            E::Call(f, _a, _l, _h) => f.as_str() == "range2" || f.as_str() == "range",
            E::Var(n, _l, _h) => oast::in_list(&cx.rangevars, n.as_str()),
            _ => false,
        };
        if isr {
            let i0 = fresh(cx);
            return H::CallH(
                String::from("$debug_of_a_range_value_is_not_modelled"),
                Vec::new(),
                i0,
            );
        }
    }
    let kind = dbg_emit(cx, e, ty);
    // P-FMT-NEST: a Vec<struct> prints `[e, e]`, each element via dbg_struct_val (vec_get/vec_len are the H primitives).
    {
        let el = vec_elem_of(ty);
        if el.len() > 0 {
            let mut isst = false;
            {
                let mut i = 0;
                while i < cx.sdefs.len() {
                    if cx.sdefs[i].name.as_str() == el.as_str() {
                        isst = true;
                    }
                    i = i + 1;
                }
            }
            if oast::eidx(&cx.edefs, &el) >= 0 {
                isst = true;
            } // P-FMT-NEST r3: Vec<enum> loops too (dbg_scalar dispatches)
            if oparse::tup_elems(&el).len() > 0 {
                isst = true;
            } // DBG-VTUP
            if isst {
                let lv = lower(e, cx);
                cx.names.push(String::from("$dbgvec"));
                cx.stypes.push(String::from(ty));
                let vs = (cx.names.len() - 1) as u32;
                cx.names.push(String::from("$dbgvi"));
                cx.stypes.push(String::from("int"));
                let is = (cx.names.len() - 1) as u32;
                let r1 = fresh(cx);
                let r2 = fresh(cx);
                let mut ga: Vec<H> = Vec::new();
                ga.push(H::Local(vs, r1));
                ga.push(H::Local(is, r2));
                let gi = fresh(cx);
                let getc = H::CallH(String::from("vec_get"), ga, gi);
                let elemdbg = dbg_scalar(cx, getc, el.as_str()); // P-FMT-NEST r3: dbg_scalar dispatches struct/enum/scalar per element
                let r3 = fresh(cx);
                let z0 = fresh(cx);
                let ci = fresh(cx);
                let cond = H::BinH(
                    String::from(">"),
                    Box::new(H::Local(is, r3)),
                    Box::new(H::Lit(0, z0)),
                    ci,
                );
                let comma = dbg_print(cx, ", ");
                let z1 = fresh(cx);
                let ifi = fresh(cx);
                let sep = H::IfH(
                    Box::new(cond),
                    Box::new(comma),
                    Box::new(H::Lit(0, z1)),
                    ifi,
                );
                let body1 = dbg_seq(cx, sep, elemdbg);
                let r4 = fresh(cx);
                let o1 = fresh(cx);
                let ai = fresh(cx);
                let inc = H::BinH(
                    String::from("+"),
                    Box::new(H::Local(is, r4)),
                    Box::new(H::Lit(1, o1)),
                    ai,
                );
                let asi = fresh(cx);
                let body = dbg_seq(cx, body1, H::AssignH(is, Box::new(inc), asi));
                let r5 = fresh(cx);
                let r6 = fresh(cx);
                let mut la: Vec<H> = Vec::new();
                la.push(H::Local(vs, r6));
                let sli = fresh(cx);
                let slen = H::CallH(String::from("vec_len"), la, sli);
                let wc = fresh(cx);
                let wcond = H::BinH(
                    String::from("<"),
                    Box::new(H::Local(is, r5)),
                    Box::new(slen),
                    wc,
                );
                let wi = fresh(cx);
                let wh = H::WhileH(Box::new(wcond), Box::new(body), wi);
                let open = dbg_print(cx, "[");
                let s1 = dbg_seq(cx, open, wh);
                let close = dbg_print(cx, "]");
                let whole = dbg_seq(cx, s1, close);
                let iz = fresh(cx);
                let li = fresh(cx);
                let letidx = H::LetH(is, Box::new(H::Lit(0, iz)), Box::new(whole), li);
                let lv2 = fresh(cx);
                return H::LetH(vs, Box::new(lv), Box::new(letidx), lv2);
            }
        }
    }
    if kind.as_str() == "vi"
        || kind.as_str() == "vs"
        || kind.as_str() == "vb"
        || kind.as_str() == "vvi"
        || kind.as_str() == "ai"
        || kind.as_str() == "vc"
        || kind.as_str() == "vf"
    {
        let mut nm = String::from("dbg_");
        nm.push_str(kind.as_str());
        // C-ARRELEM: an ARRAY printed by a VECTOR printer is copied into a vector first -- the same
        // conversion C-ARRTOSLICE makes for a slice parameter, and the only one arr_slice knows.
        let val = if is_arr_ty(ty) && kind.as_str() != "ai" {
            let a0 = lower(e, cx);
            let a1 = lower(e, cx);
            let mut na: Vec<H> = Vec::new();
            na.push(a1);
            let il = fresh(cx);
            let ln = H::CallH(String::from("arr_len"), na, il);
            let z = fresh(cx);
            let mut sa: Vec<H> = Vec::new();
            sa.push(a0);
            sa.push(H::Lit(0, z));
            sa.push(ln);
            let i2 = fresh(cx);
            H::CallH(String::from("arr_slice"), sa, i2)
        } else {
            lower(e, cx)
        };
        let mut a: Vec<H> = Vec::new();
        a.push(val);
        let i = fresh(cx);
        return H::CallH(nm, a, i);
    }
    if kind.as_str() == "unit" {
        return dbg_print(cx, "()");
    }
    if kind.as_str() == "tuplit" {
        match e {
            E::TupLit(parts, _l, _h) => {
                let mut out = dbg_print(cx, "(");
                let mut k = 0;
                while k < parts.len() {
                    if k > 0 {
                        let c = dbg_print(cx, ", ");
                        out = dbg_seq(cx, out, c);
                    }
                    let pt = ety_of(cx, &parts[k]);
                    // a NESTED tuple element is a tuple, not a scalar: print it with the same printer
                    let pv = lower(&parts[k], cx);
                    let pp = dbg_scalar(cx, pv, &pt);
                    out = dbg_seq(cx, out, pp);
                    k = k + 1;
                }
                // rustc prints a 1-tuple as `(5,)` -- the comma is how it stays a tuple
                if parts.len() == 1 {
                    let cm = dbg_print(cx, ",");
                    out = dbg_seq(cx, out, cm);
                }
                let c2 = dbg_print(cx, ")");
                return dbg_seq(cx, out, c2);
            }
            _ => {}
        }
    }
    if kind.as_str() == "tupty" {
        let lv = lower(e, cx);
        return dbg_scalar(cx, lv, ty);
    }
    if kind.as_str() == "struct" {
        let lv = lower(e, cx);
        return dbg_struct_val(cx, lv, ty);
    }
    if kind.as_str() == "enum" {
        let lv = lower(e, cx);
        return dbg_enum_val(cx, lv, ty);
    }
    lower(e, cx)
}
// a already-lowered SCALAR field, printed the Debug way
fn dbg_scalar(cx: &mut LCx, v: H, ty: &str) -> H {
    // `{:?}` on a str/char ESCAPES (rustc: `"a\"b"`, `'\''`); the quotes are the printer's, not the value's
    if ty == "str" {
        let mut a: Vec<H> = Vec::new();
        a.push(v);
        let i = fresh(cx);
        return H::CallH(String::from("dbgq"), a, i);
    }
    if ty == "char" {
        let mut a: Vec<H> = Vec::new();
        a.push(v);
        let i = fresh(cx);
        return H::CallH(String::from("dbgc"), a, i);
    }
    if ty == "f64" {
        let mut a: Vec<H> = Vec::new();
        a.push(v);
        let z = fresh(cx);
        a.push(H::Lit(1, z));
        let i = fresh(cx);
        let st = H::CallH(String::from("f64_str"), a, i);
        let mut p: Vec<H> = Vec::new();
        p.push(st);
        let i2 = fresh(cx);
        return H::CallH(String::from("print_raw"), p, i2);
    }
    if ty == "bool" {
        let i1 = fresh(cx);
        let t1 = H::StrH(String::from("true"), i1);
        let i2 = fresh(cx);
        let f1 = H::StrH(String::from("false"), i2);
        let i3 = fresh(cx);
        let pick = H::IfH(Box::new(v), Box::new(t1), Box::new(f1), i3);
        let mut p: Vec<H> = Vec::new();
        p.push(pick);
        let i4 = fresh(cx);
        return H::CallH(String::from("print_raw"), p, i4);
    }
    {
        let k = dbg_emit_ty(ty);
        if k.as_str() == "vi"
            || k.as_str() == "vs"
            || k.as_str() == "vvi"
            || k.as_str() == "ai"
            || k.as_str() == "vc"
            || k.as_str() == "vf"
        {
            let mut nm = String::from("dbg_");
            nm.push_str(k.as_str());
            let mut a: Vec<H> = Vec::new();
            a.push(v);
            let i = fresh(cx);
            return H::CallH(nm, a, i);
        }
    }
    {
        let es = oparse::tup_elems(ty);
        if es.len() > 0 {
            cx.names.push(String::from("$dbgt"));
            cx.stypes.push(String::from(ty));
            let sl = (cx.names.len() - 1) as u32;
            let mut out = dbg_print(cx, "(");
            let mut k = 0;
            while k < es.len() {
                if k > 0 {
                    let c = dbg_print(cx, ", ");
                    out = dbg_seq(cx, out, c);
                }
                let ri = fresh(cx);
                let rl = H::Local(sl, ri);
                let fi = fresh(cx);
                let fh = H::FieldH(Box::new(rl), k as u32, fi);
                let el = String::from(es[k].as_str());
                let pp = dbg_scalar(cx, fh, el.as_str());
                out = dbg_seq(cx, out, pp);
                k = k + 1;
            }
            // rustc prints a 1-tuple as `(5,)` -- the comma is how it stays a tuple
            if es.len() == 1 {
                let cm = dbg_print(cx, ",");
                out = dbg_seq(cx, out, cm);
            }
            let c2 = dbg_print(cx, ")");
            out = dbg_seq(cx, out, c2);
            let fi3 = fresh(cx);
            return H::LetH(sl, Box::new(v), Box::new(out), fi3);
        }
    }
    {
        let mut si = 0;
        while si < cx.sdefs.len() {
            if cx.sdefs[si].name.as_str() == ty {
                return dbg_struct_val(cx, v, ty);
            }
            si = si + 1;
        }
    } // P-FMT-NEST: a struct-typed field prints structurally (recurse), not as its handle
    if oast::eidx(&cx.edefs, ty) >= 0 {
        return dbg_enum_val(cx, v, ty);
    } // P-FMT-NEST r3: an enum-typed field prints structurally (recurse)
    if ty.len() > 0 && ty != "int" && !oast::is_int_width(ty) {
        // a field we cannot print must not be shown as its handle
        let mut a: Vec<H> = Vec::new();
        let z = fresh(cx);
        a.push(H::Lit(0, z));
        let i = fresh(cx);
        return H::CallH(String::from("$dbg_unsupported"), a, i);
    }
    let mut a: Vec<H> = Vec::new();
    a.push(v);
    let i = fresh(cx);
    H::CallH(String::from("print_int_raw"), a, i)
}
pub fn unique_method(cx: &LCx, m: &str) -> String {
    let mut hit = String::from("");
    let mut n = 0;
    let mut i = 0;
    while i < cx.fnames.len() {
        let fname = &cx.fnames[i];
        let b = fname.as_bytes();
        let mb = m.as_bytes();
        if b.len() > mb.len() + 1 {
            let base = b.len() - mb.len();
            if b[base - 1] == 36 {
                let mut j = 0;
                let mut same = true;
                while j < mb.len() {
                    if b[base + j] != mb[j] {
                        same = false;
                    }
                    j = j + 1;
                }
                if same {
                    n = n + 1;
                    hit = String::from(fname.as_str());
                }
            }
        }
        i = i + 1;
    }
    if n == 1 {
        return hit;
    }
    // a `Trait$m` entry is a DECLARATION or a default, not an implementation target. When exactly one
    // candidate is owned by a non-trait type, that one is the method -- which is what `impl Trait` in an
    // argument position needs, since the parameter carries no concrete type name.
    {
        let mut hit2 = String::from("");
        let mut n2 = 0;
        let mut k = 0;
        while k < cx.fnames.len() {
            let fname = &cx.fnames[k];
            let b = fname.as_bytes();
            let mb = m.as_bytes();
            if b.len() > mb.len() + 1 {
                let base = b.len() - mb.len();
                if b[base - 1] == 36 {
                    let mut j = 0;
                    let mut same = true;
                    while j < mb.len() {
                        if b[base + j] != mb[j] {
                            same = false;
                        }
                        j = j + 1;
                    }
                    if same {
                        let owner = String::from(&fname[0..base - 1]);
                        if !is_trait_name(cx, &owner) {
                            n2 = n2 + 1;
                            hit2 = String::from(fname.as_str());
                        }
                    }
                }
            }
            k = k + 1;
        }
        if n2 == 1 {
            return hit2;
        }
    }
    String::from("")
}
pub fn is_trait_name(cx: &LCx, n: &str) -> bool {
    let mut i = 0;
    while i < cx.dimpls.len() {
        let p = split_bar(cx.dimpls[i].as_str());
        if p.len() == 2 && p[0].as_str() == n {
            return true;
        }
        i = i + 1;
    }
    false
}
// C-IDENTMETHOD: the methods that are the IDENTITY in a handle-based value model, stated once. A str and
// its bytes are the same handle (C8), a Box and its contents are the same handle (C-BOX), and a clone of
// an immutable value is the value -- so every one of these is a no-op whatever the receiver is. The str
// branch of builtin_method listed most of them and the Box branch listed two more; an UNNAMED receiver
// had no list at all, which is how `$as_str`, `$as_bytes` and friends reached the emitter undefined.
pub fn ident_method(m: &str) -> bool {
    m == "as_str"
        || m == "as_ref"
        || m == "as_mut"
        || m == "as_bytes"
        || m == "chars"
        || m == "to_string"
        || m == "to_owned"
        || m == "clone"
        || m == "iter"
        || m == "into_iter"
        || m == "borrow"
        || m == "borrow_mut"
        || m == "into_inner"
}
// ORD-AGG: a type whose < is the builtin value order (ints of any width, char, bool, f64, str)
fn ord_scalar(t: &str) -> bool {
    t == "int" || oast::is_int_width(t) || t == "char" || t == "bool" || t == "f64" || t == "str"
}
pub fn builtin_method(rt0: &str, m: &str, na: usize) -> String {
    let rtb = oast::pay_base(rt0);
    let rt = rtb.as_str(); /* DBG-PAYLOAD */
    if m == "to_string" && na == 0 && rt == "char" {
        return String::from("chr_str");
    } // CHR-STR: 'q'.to_string() is "q", not "113"
    if m == "cmp" && na == 1 && (rt == "int" || rt == "char" || rt == "" || oast::is_int_width(rt))
    {
        return String::from("int_cmp");
    } // an UNTYPED receiver (a comparator closure's param) defaults to the integer cmp   // ORD-CMP: Ord::cmp on integers -> prelude Ordering
    if m == "sort_by" && na == 1 {
        return String::from("vsort_by");
    }
    if m == "join" && na == 1 && (rt == "Vec$str" || rt == "arr$str") {
        return String::from("vstr_join");
    } // STR-JOIN: [String].join(sep) / .concat()
    if m == "concat" && na == 0 && (rt == "Vec$str" || rt == "arr$str") {
        return String::from("vstr_concat");
    } // ORD-CMP: sort by a comparator closure (in place, stable)
    if m == "checked_div" && na == 1 && rt != "f64" {
        return String::from("ichecked_div");
    } // P-CHECKEDDIV
    if (m == "saturating_add" || m == "saturating_sub")
        && na == 1
        && oast::is_int_width(rt)
        && oast::width_bits(rt) < 64
    {
        if m == "saturating_add" {
            return String::from("$satadd");
        }
        return String::from("$satsub");
    } // SAT-OPS: clamp to the width (64-bit widths refused: the i64 sum itself could overflow)
    if m == "saturating_mul" && na == 1 && oast::is_int_width(rt) && oast::width_bits(rt) <= 16 {
        return String::from("$satmul");
    } // SAT-OPS: a <=16-bit product cannot overflow i64
    if m == "checked_add" && na == 1 && oast::is_int_width(rt) && oast::width_bits(rt) < 64 {
        return String::from("$chkadd");
    } // D3.1
    if m == "to_le" && na == 0 && oast::is_int_width(rt) {
        return String::from("$id");
    } // R4a: the target is little-endian, so to_le is the value itself
    if m == "count_ones" && na == 0 && oast::is_int_width(rt) {
        return String::from("$cntones");
    } // D3.2
    if m == "leading_zeros" && na == 0 && oast::is_int_width(rt) {
        return String::from("$leadz");
    } // D3.2
    if m == "trailing_zeros" && na == 0 && oast::is_int_width(rt) {
        return String::from("$trailz");
    } // D3.2
    if m == "checked_sub" && na == 1 && oast::is_int_width(rt) && oast::width_bits(rt) < 64 {
        return String::from("$chksub");
    } // D3.1
    if m == "checked_mul" && na == 1 && oast::is_int_width(rt) && oast::width_bits(rt) < 64 {
        return String::from("$chkmul");
    } // D3.1
    if m == "pow" && na == 1 && rt != "f64" {
        return String::from("ipow");
    } // P-INT-METHODS
    if m == "abs" && na == 0 && rt != "f64" {
        return String::from("iabs");
    } // P-F64-METHODS: f64.abs is sign-bit clear, not int abs -- refuse here until the f64 arc lands (refuse > the raw-bits wrong answer)
    if rt == "f64" && na == 0 && m == "floor" {
        return String::from("ffloor");
    } // D1.2
    if rt == "f64" && na == 0 && m == "ceil" {
        return String::from("fceil");
    } // D1.2
    if rt == "f64" && na == 0 && m == "round" {
        return String::from("fround");
    } // D1.2
    if rt == "f64" && na == 0 && m == "abs" {
        return String::from("fabs");
    } // D1.3
    if rt == "f64" && na == 1 && m == "powi" {
        return String::from("fpowi");
    } // D1.4
    if rt == "f64"
        && na == 0
        && (m == "is_nan" || m == "is_infinite" || m == "is_finite" || m == "signum")
    {
        if m == "is_nan" {
            return String::from("fis_nan");
        }
        if m == "is_infinite" {
            return String::from("fis_inf");
        }
        if m == "is_finite" {
            return String::from("fis_fin");
        }
        return String::from("fsignum");
    } // F64-PREDS
    if rt == "f64" && na == 0 {
        let c = match m {
            "ln" => "log",
            "exp" => "exp",
            "exp2" => "exp2",
            "sin" => "sin",
            "cos" => "cos",
            "log10" => "log10",
            "log2" => "log2",
            _ => "",
        };
        if c.len() > 0 {
            let mut f = String::from("f64m_");
            f.push_str(c);
            return f;
        }
    } // F64-LIBM: unary libm (rustc lowers these to the same glibc calls)
    if rt == "f64" && na == 1 {
        let c = match m {
            "powf" => "pow",
            _ => "",
        };
        if c.len() > 0 {
            let mut f = String::from("f64m2_");
            f.push_str(c);
            return f;
        }
    } // F64-LIBM: binary
    if rt == "f64" && na == 0 && m == "sqrt" {
        return String::from("f64_sqrt");
    } // D1.5
    if rt == "f64" && na == 1 && m == "max" {
        return String::from("fmax");
    } // D1.6
    if rt == "f64" && na == 1 && m == "min" {
        return String::from("fmin");
    } // D1.6
    if (m == "max" || m == "min") && na == 1 && rt.len() > 0 && !ord_scalar(rt) {
        return String::from("$ord_of_aggregate_unsupported");
    } // ORD-AGG: imax/imin on an enum/struct compared the HANDLES (P::A.max(P::B) printed a pointer)
    if m == "max" && na == 1 && rt != "f64" {
        return String::from("imax");
    }
    if m == "min" && na == 1 && rt != "f64" {
        return String::from("imin");
    }
    if m == "is_ascii_digit" && na == 0 {
        return String::from("cis_digit");
    } // P-CHAR-PRED
    if (m == "is_alphabetic" || m == "is_ascii_alphabetic") && na == 0 {
        return String::from("cis_alpha");
    }
    if (m == "is_lowercase" || m == "is_ascii_lowercase") && na == 0 {
        return String::from("cis_lower");
    } // CHAR-CASE
    if (m == "is_uppercase" || m == "is_ascii_uppercase") && na == 0 {
        return String::from("cis_upper");
    }
    if (m == "is_alphanumeric" || m == "is_ascii_alphanumeric") && na == 0 {
        return String::from("cis_alnum");
    }
    if (m == "is_whitespace" || m == "is_ascii_whitespace") && na == 0 {
        return String::from("cis_ws");
    }
    if m == "is_numeric" && na == 0 {
        return String::from("cis_digit");
    }
    // C-ASREF: `Box<T>` is TRANSPARENT in this value model (C-BOX) -- a boxed value and the value itself
    // are the same handle -- so `b.as_ref()`, `b.as_mut()` and `b.borrow()` are the identity on ANY
    // receiver, exactly as `as_str` already is on a str. Without this, matching on `pl.as_ref()` (the
    // ordinary way to look inside a Box<E>) was refused.
    if (m == "as_ref" || m == "as_mut") && na == 0 {
        return String::from("$id");
    }
    // C-WRAPPING: at 64 bits the wrapping methods ARE the plain operators, exactly -- probed against
    // rustc -O, a u64 and an i64 running the same wrapping chain give identical bits. BELOW 64 bits they
    // are not: 0u8.wrapping_sub(1) is 255 and we would answer -1 (rung 51 reverted the unguarded form
    // after the rc-topology gate produced three such witnesses). So they are admitted only at a declared
    // 64-bit width, which C-INTWIDTH keeps, and stay REFUSED at every narrower one.
    // C-WRAPPING (widened): admit at ANY known int width. 64-bit widths are the plain operator; a
    // NARROWER width masks via CastH in the lowering (the same trunc/ext the P-WIDTH arithmetic path uses),
    // so 250u8.wrapping_add(10)==4, not -1 (rung 51's unguarded gap). An unknown-width receiver (a bare
    // suffixed literal whose suffix we drop) is NOT is_int_width -> stays refused (value-safe).
    if (m == "wrapping_mul" || m == "wrapping_add" || m == "wrapping_sub")
        && na == 1
        && (oast::is_int_width(rt) || rt == "i64" || rt == "u64" || rt == "isize" || rt == "usize")
    {
        return String::from("$wrap");
    }

    // see C-ENUMCLONE: an enum value is immutable here, so cloning it is the identity. The caller passes
    // the receiver's TYPE NAME, and an enum name is anything the enum table knows -- which the caller
    // checks before asking, so the name test here is the enum-shaped prefix the tagger uses.

    if oast::str_starts(rt, "clos$") && m == "clone" && na == 0 {
        return String::from("$id");
    } // CLOS-CLONE: a Copy closure clones to itself
    if oast::str_starts(rt, "tup") && (m == "clone" || m == "to_owned") && na == 0 {
        return String::from("$id");
    } // TUP-CLONE: a tuple value is immutable here -- its clone is itself
    if rt == "arr" || arr_elem_of(rt).len() > 0 {
        if m == "len" && na == 0 {
            return String::from("arr_len");
        }
        if (m == "iter" || m == "into_iter") && na == 0 {
            return String::from("arr_vec");
        } /* ARR-METH */
        return String::from("");
    }
    // O1-O9: the Option/Result surface, as prelude functions over the prelude's own enums
    if rt == "Option" {
        if (m == "unwrap" || m == "expect") && (na == 0 || na == 1) {
            return String::from("ounwrap");
        }
        if m == "unwrap_or" && na == 1 {
            return String::from("ounwrap_or");
        }
        if m == "is_some" && na == 0 {
            return String::from("ois_some");
        }
        if m == "is_none" && na == 0 {
            return String::from("ois_none");
        }
        if m == "map" && na == 1 {
            return String::from("omap");
        }
        if m == "map_or" && na == 2 {
            return String::from("omap_or");
        } // P-OPT-MAPOR
        if m == "unwrap_or_else" && na == 1 {
            return String::from("ounwrap_or_else");
        } // P-OPT-COMB
        if m == "and_then" && na == 1 {
            return String::from("oand_then");
        } // P-OPT-COMB
        if m == "ok_or" && na == 1 {
            return String::from("ook_or");
        } // P-RES-COMB
        if m == "filter" && na == 1 {
            return String::from("ofilter");
        } // P-OPT-FILTER
        if (m == "copied" || m == "cloned") && na == 0 {
            return String::from("$id");
        } // P-OPT-COPIED: no refs -> identity
        if m == "unwrap_or_default" && na == 0 {
            return String::from("ounwrap_or_default");
        } // P-OPT-UNWRAPDEFAULT
        return String::from("");
    }
    if rt == "Result" {
        if (m == "unwrap" || m == "expect") && (na == 0 || na == 1) {
            return String::from("runwrap");
        }
        if m == "is_ok" && na == 0 {
            return String::from("rok");
        }
        if m == "is_err" && na == 0 {
            return String::from("rerr");
        }
        if m == "unwrap_or" && na == 1 {
            return String::from("runwrap_or");
        } // P-RES-COMB
        if m == "ok" && na == 0 {
            return String::from("rok_opt");
        } // P-RES-COMB
          // C-RESULTMAP: Result.map / .map_err, the symmetric siblings of Option.map (omap). map transforms
          // the Ok payload and passes Err through; map_err transforms Err and passes Ok through. Prelude fns,
          // dispatched the same way omap is -- one rule, both halves.
        if m == "map" && na == 1 {
            return String::from("rmap");
        }
        if m == "map_err" && na == 1 {
            return String::from("rmaperr");
        }
        return String::from("");
    }
    if rt == "bstr" {
        if (m == "len" || m == "count") && na == 0 {
            return String::from("len");
        }
        if m == "next" && na == 0 {
            return String::from("bnext");
        }
        return String::from("");
    }
    if rt == "str" {
        if m == "len" && na == 0 {
            return String::from("len");
        }
        if m == "at" && na == 1 {
            return String::from("str_at");
        }
        // S1-S12: the str surface our C-string model expresses EXACTLY. `chars()` is absent on
        // purpose: a byte walk diverges on non-ASCII (the s13 probe), and mutation needs a place.
        // push/push_str ARE expressible now: the growth is a re-assignment of the receiver's
        // slot (see the MethodCall arm in `lower`), so the "mutation needs a place" reason the
        // chars() note gives no longer holds for them
        // `push(char)` joins push_str now that `__chr1` exists: the refusal named a missing
        // one-byte-string runtime, and the runtime is ours to add
        if (m == "push_str" || m == "push") && na == 1 {
            return String::from("$strgrow");
        }
        // a str IS its bytes here, so as_bytes is the identity plus a name (`bstr`) that makes
        // `b[i]` read with str_at and `b.len()` read with len
        if m == "chars" && na == 0 {
            return String::from("str_chars_vec");
        } /* STR-CHARS-UTF8: iterate DECODED chars -- the byte view made `for c in "héy".chars()` yield 4 bytes (a wrong answer) */
        if (m == "as_bytes" || m == "bytes") && na == 0 {
            return String::from("$id");
        } /* STR-BYTES: bytes() iterates the byte view */
        if m == "is_empty" && na == 0 {
            return String::from("sempty");
        }
        if (m == "as_str" || m == "trim_matches") && na == 0 {
            return String::from("$id");
        }
        // C-COPYISCOPY: these three OWN their result, and in-place growth makes the
        // difference observable; as_str above is a borrow and stays the identity.
        if (m == "to_string" || m == "to_owned" || m == "clone") && na == 0 {
            return String::from("str_copy");
        }
        if m == "into" && na == 0 {
            return String::from("str_copy"); // STR-INTO: `From<&str> for String` -- an owned copy, like to_owned
        }
        // C-STRSPLIT: `s.split(x)` answers a Vec of str, which our prelude can build out of
        // str_find and str_sub -- no new runtime, and `.collect()` after it is already the
        // identity. A CHAR separator is widened to a one-byte string at the call (below).
        if m == "split" && na == 1 {
            return String::from("str_split");
        }
        if m == "split_whitespace" && na == 0 {
            return String::from("ssplit_ws");
        } // SPLIT-WS
        if m == "char_indices" && na == 0 {
            return String::from("str_char_idx");
        } // CHAR-IDX: (byte index, DECODED char) pairs -- UTF-8 aware (it was one pair per BYTE: a silent wrong answer on non-ASCII)
        if m == "trim_end_matches" && na == 1 {
            return String::from("strim_endc");
        } // TRIM-MATCHES: a char pattern
        if m == "trim_start_matches" && na == 1 {
            return String::from("strim_startc");
        }
        if m == "trim_matches" && na == 1 {
            return String::from("strim_c");
        }
        if m == "contains" && na == 1 {
            return String::from("shas");
        }
        if m == "find" && na == 1 {
            return String::from("sfind");
        }
        if m == "rfind" && na == 1 {
            return String::from("srfind");
        } // STR-RFIND: the LAST match's byte index
        if m == "starts_with" && na == 1 {
            return String::from("sstarts");
        }
        if m == "ends_with" && na == 1 {
            return String::from("sends");
        }
        if m == "trim" && na == 0 {
            return String::from("strim");
        }
        if m == "trim_start" && na == 0 {
            return String::from("strims");
        }
        if m == "trim_end" && na == 0 {
            return String::from("strime");
        }
        if m == "to_uppercase" && na == 0 {
            return String::from("supper");
        }
        if m == "to_lowercase" && na == 0 {
            return String::from("slower");
        }
        if m == "repeat" && na == 1 {
            return String::from("srepeat");
        }
        if m == "replace" && na == 2 {
            return String::from("sreplace");
        }
        if m == "replacen" && na == 3 {
            return String::from("sreplacen");
        } // REPLACEN (R2a P4): replace the first n matches (prelude sreplacen)
        if m == "parse" && na == 0 {
            return String::from("sparse");
        }
        return String::from("");
    }
    if vec_elem_of(rt).len() > 0 || rt == "Vec" {
        if m == "len" && na == 0 {
            return String::from("vec_len");
        }
        if m == "push" && na == 1 {
            return String::from("vec_push");
        }
        if m == "get" && na == 1 {
            return String::from("vec_get");
        }
        if m == "set" && na == 2 {
            return String::from("vec_set");
        }
        if m == "remove" && na == 1 {
            return String::from("vremove");
        } // C-VECREMOVE: a prelude function, like max and pop
        if m == "swap_remove" && na == 1 {
            return String::from("vswapremove");
        } // P-VEC-SWAPREMOVE
        if m == "insert" && na == 2 {
            return String::from("vinsert");
        } // P-ITER-INSERT: in-place shift-right insert, like vremove
          // P-ITER: an iterator IS a vector here (eager), so `.iter()` is the identity and each adaptor is
          // a prelude function written in our own language
          // C-CELL: `get()` with NO index and `set(v)` with no index are the Cell accessors over the
          // one-slot vector the marker built; the indexed get/set keep their own meaning.
        if (m == "get" || m == "borrow" || m == "borrow_mut") && na == 0 {
            return String::from("$cellget");
        } // R4 REFCELL: borrow()/borrow_mut() hand out the cell's value (a handle for Vec/String/struct: mutation through it is shared)
        if m == "set" && na == 1 {
            return String::from("$cellset");
        }
        if (m == "iter"
            || m == "into_iter"
            || m == "as_slice"
            || m == "as_mut_slice"
            || m == "iter_mut"
            || m == "collect"
            || m == "cloned"
            || m == "copied")
            && na == 0
        {
            return String::from("$id");
        }
        if m == "sort" && na == 0 && oast::str_starts(rt0, "Vec$tup2$") {
            if oast::str_starts(rt0, "Vec$tup2$str") {
                return String::from("vsort_t2s");
            }
            return String::from("vsort_t2i");
        } // SORT-TUP: lexicographic, str by content
        if (m == "windows" || m == "chunks") && na == 1 {
            if m == "windows" {
                return String::from("vwindows");
            }
            return String::from("vchunks");
        } // SLICE-WIN
        if m == "collect_string" && na == 0 {
            if rt == "Vec$char" {
                return String::from("vchar_string");
            }
            return String::from("vstr_concat");
        } // COLLECT-TURBO
        if (m == "to_vec" || m == "clone") && na == 0 {
            return String::from("vclone");
        } // VEC-CLONE: a cloned Vec is a NEW buffer -- the identity aliased it, so sorting the clone reordered the original
        if m == "sum" && na == 0 {
            if rt == "Vec$f64" {
                return String::from("vsum_f"); // R11 SUM-F64: a float sum (vsum_any adds ints: `cannot apply + to int and f64`)
            }
            return String::from("sum");
        }
        if m == "product" && na == 0 {
            if rt == "Vec$f64" {
                return String::from("vproduct_f");
            }
            return String::from("oproduct");
        }
        if m == "count" && na == 0 {
            return String::from("vec_len");
        }
        if m == "is_empty" && na == 0 {
            return String::from("vempty");
        }
        if (m == "max" || m == "min") && na == 0 && {
            let el = vec_elem_of(rt);
            el.len() > 0
                && (el.as_str() == "str"
                    || (!ord_scalar(el.as_str()) && !oast::str_starts(el.as_str(), "tup")))
        } {
            return String::from("$ord_of_aggregate_unsupported");
        } // ORD-AGG: omax/omin compare element HANDLES -- wrong for enums/structs (Some(A) vs Some(B)) and for strs
        if m == "max" && na == 0 {
            return String::from("omax");
        }
        if m == "min" && na == 0 {
            return String::from("omin");
        }
        if m == "rev" && na == 0 {
            return String::from("vrev");
        }
        if m == "last" && na == 0 {
            return String::from("olast");
        }
        if (m == "first" || m == "next") && na == 0 {
            return String::from("ofirst");
        }
        if m == "map" && na == 1 {
            return String::from("map");
        }
        if m == "filter" && na == 1 {
            return String::from("filter");
        }
        if m == "fold" && na == 2 {
            return String::from("fold");
        }
        if m == "any" && na == 1 {
            return String::from("vany");
        }
        if m == "all" && na == 1 {
            return String::from("vall");
        }
        if m == "position" && na == 1 {
            return String::from("opos");
        }
        if m == "binary_search"
            && na == 1
            && (oast::str_starts(rt, "Vec$") || oast::str_starts(rt, "arr$"))
            && {
                let el = &rt[4..];
                el == "int" || oast::is_int_width(el)
            }
        {
            return String::from("vbsearch");
        } // VEC-BSEARCH: rustc 1.95 binary_search_by exactly (which index a duplicate run answers is part of the result)
        if m == "find" && na == 1 {
            return String::from("ofind");
        } // VEC-FIND: iter().find(p) -> Option
        if m == "partition" && na == 1 {
            return String::from("opartition");
        }
        if m == "take" && na == 1 {
            return String::from("vtake");
        }
        if m == "skip" && na == 1 {
            return String::from("vskip");
        }
        if m == "step_by" && na == 1 {
            return String::from("vstepby");
        } // P-ITER-STEPBY
        if m == "take_while" && na == 1 {
            return String::from("vtakewhile");
        }
        if m == "skip_while" && na == 1 {
            return String::from("vskipwhile");
        }
        if m == "contains" && na == 1 {
            return String::from("vhas");
        }
        if m == "retain" && na == 1 {
            return String::from("vretain");
        } // P-VEC-RETAIN
        if (m == "sort" || m == "sort_unstable") && na == 0 && vec_elem_of(rt).as_str() == "str" {
            return String::from("vsort_s");
        } // STR-SORT: strs sort by CONTENT (vsort ordered the string POINTERS -- ["b", "a"] stayed unsorted)
        if (m == "sort" || m == "sort_unstable") && na == 0 && {
            let el = vec_elem_of(rt);
            el.len() > 0 && !ord_scalar(el.as_str()) && !oast::str_starts(el.as_str(), "tup")
        } {
            return String::from("$ord_of_aggregate_unsupported");
        } // ORD-AGG: vsort ordered enum/struct HANDLES (a Vec of derived-Ord enums came back unsorted -- a wrong answer)
        if m == "sort" && na == 0 {
            return String::from("vsort");
        } // P-VEC-SORT
        if m == "sort_unstable" && na == 0 {
            return String::from("vsort");
        } // P-VEC-SORT: unstable == stable for a total order
        if m == "reverse" && na == 0 {
            return String::from("vreverse");
        } // P-VEC-REVERSE
        if m == "swap" && na == 2 {
            return String::from("vswap");
        } // P-VEC-SWAP
        if m == "truncate" && na == 1 {
            return String::from("vtruncate");
        } // P-VEC-TRUNCATE
        if m == "dedup" && na == 0 {
            return String::from("vdedup");
        } // P-VEC-DEDUP
        if m == "append" && na == 1 {
            return String::from("vappend");
        } // P-VEC-APPEND
        if m == "extend" && na == 1 {
            return String::from("vextend");
        } // P-VEC-EXTEND
        if m == "clear" && na == 0 {
            return String::from("vclear");
        } // P-VEC-CLEAR
        if m == "sort_by_key" && na == 1 {
            return String::from("vsortbykey");
        } // P-VEC-SORTBYKEY
        if m == "max_by_key" && na == 1 {
            return String::from("vmaxbykey");
        } // P-VEC-MAXBYKEY
        if m == "min_by_key" && na == 1 {
            return String::from("vminbykey");
        } // P-VEC-MAXBYKEY
        if m == "nth" && na == 1 {
            return String::from("vnth");
        } // P-VEC-NTH
        if m == "reduce" && na == 1 {
            return String::from("vreduce");
        } // P-VEC-REDUCE
        if m == "enumerate" && na == 0 {
            return String::from("venum");
        }
        if m == "zip" && na == 1 {
            return String::from("vzip");
        } // P-ITER-ZIP
        if m == "chain" && na == 1 {
            return String::from("vchain");
        } // D2.1
        if m == "flat_map" && na == 1 {
            return String::from("vflatmap");
        } // D2.2
        if m == "pop" && na == 0 {
            return String::from("vpop");
        }
    }
    String::from("")
}
pub fn arms_have_lit_or_guard(arms: &Vec<Arm>) -> bool {
    let mut i = 0;
    while i < arms.len() {
        if arms[i].has_lit || arms[i].has_slit || arms[i].has_guard {
            return true;
        }
        i = i + 1;
    }
    false
}
// OP-PRIM: an operator on a PRIMITIVE is never user-overloaded (orphan rule: no user `impl Add for i32`) -- a user TRAIT method that happens to be named `add` must not capture `+`
pub fn prim_ty(t: &str) -> bool {
    t == "i8"
        || t == "i16"
        || t == "i32"
        || t == "i64"
        || t == "i128"
        || t == "isize"
        || t == "u8"
        || t == "u16"
        || t == "u32"
        || t == "u64"
        || t == "u128"
        || t == "usize"
        || t == "f32"
        || t == "f64"
        || t == "bool"
        || t == "char"
        || t == "int"
        || t == "str"
}
pub fn op_method(o: &str) -> String {
    if o == "+" {
        String::from("add")
    } else if o == "-" {
        String::from("sub")
    } else if o == "*" {
        String::from("mul")
    } else if o == "/" {
        String::from("div")
    } else if o == "==" {
        String::from("eq")
    } else {
        String::from("")
    }
}
pub fn clos_name_ix(n: &str) -> i32 {
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
pub fn itoa_u(x: u32) -> String {
    if x == 0 {
        return String::from("0");
    }
    let mut v = x;
    let mut t: Vec<char> = Vec::new();
    while v > 0 {
        t.push((48 + (v % 10)) as u8 as char);
        v = v / 10;
    }
    let mut r = String::from("");
    let mut i = t.len();
    while i > 0 {
        r.push(t[i - 1]);
        i = i - 1;
    }
    r
}
pub fn clos_ix_of(n: &str) -> i32 {
    let b = n.as_bytes();
    if b.len() < 6 {
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
pub fn clos_of(t: &str) -> String {
    let b = t.as_bytes();
    if b.len() < 6 {
        return String::from("");
    }
    if b[0] == 99 && b[1] == 108 && b[2] == 111 && b[3] == 115 && b[4] == 36 {
        return String::from(&t[5..]);
    }
    String::from("")
}
pub fn is_vec_ty(t: &str) -> bool {
    t == "Vec" || vec_elem_of(t).len() > 0
}
// `arr$T` is the typed-array name, the array twin of `Vec$T`
pub fn arr_elem_of(t: &str) -> String {
    let b = t.as_bytes();
    if b.len() < 5 {
        return String::from("");
    }
    if b[0] == 97 && b[1] == 114 && b[2] == 114 && b[3] == 36 {
        return String::from(&t[4..]);
    }
    String::from("")
}
// C-ARRELEM: the one array test -- the bare name (element unknown) and the element-carrying name are
// both arrays, and every rule that used to compare with "arr" asks this instead.
pub fn is_arr_ty(t: &str) -> bool {
    t == "arr" || arr_elem_of(t).len() > 0
}
pub fn elem_of(t: &str) -> String {
    let v = vec_elem_of(t);
    if v.len() > 0 {
        return v;
    }
    arr_elem_of(t)
}
// BIN-W64: a declared 64-bit integer type name
fn is_w64(t: &str) -> bool {
    t == "i64" || t == "u64" || t == "isize" || t == "usize"
}
// TUP-BIND-PAY: the declared payload type of arm binding `bn` ("" when not a binding / unknown enum)
fn bind_payload_ty(cx: &LCx, sc: &E, arm: &Arm, bn: &str) -> String {
    let t = ety_of(cx, sc);
    let en = if oast::eidx(&cx.edefs, t.as_str()) >= 0 {
        t
    } else {
        sole_enum_of_variant(&cx.edefs, arm.variant.as_str())
    };
    if en.len() == 0 || oast::eidx(&cx.edefs, en.as_str()) < 0 {
        return String::new();
    }
    let mut bi = 0;
    while bi < arm.binds.len() {
        if arm.binds[bi].as_str() == bn {
            return oast::vpayload_ty(&cx.edefs, en.as_str(), arm.variant.as_str(), bi);
        }
        bi = bi + 1;
    }
    String::new()
}
// MATCH-BIND-PAY: the one enum that declares variant `v` ("" when none or ambiguous)
fn sole_enum_of_variant(es: &Vec<Edef>, v: &str) -> String {
    let mut found = String::new();
    let mut n = 0;
    let mut i = 0;
    while i < es.len() {
        let mut k = 0;
        while k < es[i].variants.len() {
            if es[i].variants[k].name.as_str() == v {
                found = String::from(es[i].name.as_str());
                n = n + 1;
            }
            k = k + 1;
        }
        i = i + 1;
    }
    if n == 1 {
        found
    } else {
        String::new()
    }
}
// MATCH-BIND-PAY: does some arm return one of its own bindings whose payload type is str?
fn match_returns_str_bind(cx: &LCx, sc: &E, arms: &Vec<Arm>) -> bool {
    let st1 = ety_of(cx, sc);
    let mut k = 0;
    while k < arms.len() {
        let st = if oast::eidx(&cx.edefs, st1.as_str()) >= 0 {
            String::from(st1.as_str())
        } else {
            sole_enum_of_variant(&cx.edefs, arms[k].variant.as_str())
        };
        if st.len() == 0 {
            k = k + 1;
            continue;
        }
        if let E::Var(bv, _l, _h) = &arms[k].body {
            let mut bi = 0;
            while bi < arms[k].binds.len() {
                if arms[k].binds[bi].as_str() == bv.as_str()
                    && oast::vpayload_ty(&cx.edefs, st.as_str(), arms[k].variant.as_str(), bi)
                        .as_str()
                        == "str"
                {
                    return true;
                }
                bi = bi + 1;
            }
        }
        k = k + 1;
    }
    false
}
// MATCH-BIND-PAY: an arm body that is pure control flow (return / break / continue)
fn ctl_arm(e: &E) -> bool {
    match e {
        E::RetE(_x, _l, _h) => true,
        E::BreakE(_l, _h) => true,
        E::BreakN(_n, _l, _h) => true,
        E::ContE(_l, _h) => true,
        E::ContN(_n, _l, _h) => true,
        _ => false,
    }
}
// EN-AMBIG: does some non-wild arm name a variant that SOME enum declares?
fn arms_name_a_known_variant(es: &Vec<Edef>, arms: &Vec<Arm>) -> bool {
    let mut a = 0;
    while a < arms.len() {
        if !arms[a].wild && arms[a].variant.len() > 0 && arms[a].variant.as_str() != "_" {
            let mut i = 0;
            while i < es.len() {
                let mut k = 0;
                while k < es[i].variants.len() {
                    if es[i].variants[k].name.as_str() == arms[a].variant.as_str() {
                        return true;
                    }
                    k = k + 1;
                }
                i = i + 1;
            }
        }
        a = a + 1;
    }
    false
}
// EN-ARITY: the ONE enum in which every non-wild arm's variant exists WITH that arm's payload count ("" when none / several)
fn enum_by_variant_arity(es: &Vec<Edef>, arms: &Vec<Arm>) -> String {
    let mut found = String::new();
    let mut n = 0;
    let mut i = 0;
    while i < es.len() {
        let mut ok = true;
        let mut any = false;
        let mut a = 0;
        while a < arms.len() {
            if !arms[a].wild && arms[a].variant.len() > 0 && arms[a].variant.as_str() != "_" {
                any = true;
                let mut hit = false;
                let mut k = 0;
                while k < es[i].variants.len() {
                    if es[i].variants[k].name.as_str() == arms[a].variant.as_str()
                        && (es[i].variants[k].npayload as usize) == arms[a].binds.len()
                    {
                        hit = true;
                    }
                    k = k + 1;
                }
                if !hit {
                    ok = false;
                }
            }
            a = a + 1;
        }
        if ok && any {
            found = String::from(es[i].name.as_str());
            n = n + 1;
        }
        i = i + 1;
    }
    if n == 1 {
        found
    } else {
        String::new()
    }
}
// VIS-FIELD-READ: is field `f` of struct `sn` private (an `@priv:m$S|f` fact from the parser) and the current fn outside module m?
fn field_read_private(cx: &LCx, sn: &str, f: &str) -> bool {
    if sn.len() == 0 || cx.curfn.len() == 0 {
        return false;
    }
    let mut i = 0;
    while i < cx.fpays.len() {
        let e = cx.fpays[i].as_str();
        if oast::str_starts(e, "@priv:") {
            let body = &e[6..];
            let bb = body.as_bytes();
            let mut bar = 0;
            while bar < bb.len() && bb[bar] != 124 {
                bar = bar + 1;
            }
            if bar < bb.len() && &body[bar + 1..] == f {
                let st = &body[..bar];
                let sb = st.as_bytes();
                // the struct as written may be the bare name (`S`) or qualified (`m$S`)
                let same = st == sn
                    || (sb.len() > sn.len()
                        && &st[sb.len() - sn.len()..] == sn
                        && sb[sb.len() - sn.len() - 1] == 36);
                if same {
                    let mut cut = 0;
                    let mut q = 0;
                    while q < sb.len() {
                        if sb[q] == 36 {
                            cut = q + 1;
                        }
                        q = q + 1;
                    }
                    let cb = cx.curfn.as_bytes();
                    let inside = cb.len() >= cut && &cb[..cut] == &sb[..cut];
                    if !inside {
                        return true;
                    }
                }
            }
        }
        i = i + 1;
    }
    false
}
pub fn vec_elem_of(t: &str) -> String {
    let b = t.as_bytes();
    if b.len() < 5 {
        return String::from("");
    }
    if b[0] == 86 && b[1] == 101 && b[2] == 99 && b[3] == 36 {
        return String::from(&t[4..]);
    }
    String::from("")
}
pub fn dyn_trait_of(t: &str) -> String {
    let b = t.as_bytes();
    if b.len() < 5 {
        return String::from("");
    }
    if b[0] == 100 && b[1] == 121 && b[2] == 110 && b[3] == 36 {
        return String::from(&t[4..]);
    }
    String::from("")
}
pub fn dyn_types(cx: &LCx, tr: &str) -> Vec<String> {
    let mut o: Vec<String> = Vec::new();
    let mut i = 0;
    while i < cx.dimpls.len() {
        let p = split_bar(cx.dimpls[i].as_str());
        if p.len() == 2 && p[0].as_str() == tr {
            o.push(String::from(p[1].as_str()));
        }
        i = i + 1;
    }
    o
}
pub fn dyn_tag(cx: &LCx, tr: &str, ty: &str) -> i32 {
    let ts = dyn_types(cx, tr);
    let mut i = 0;
    while i < ts.len() {
        if ts[i].as_str() == ty {
            return i as i32;
        }
        i = i + 1;
    }
    0 - 1
}
pub fn split_bar(s: &str) -> Vec<String> {
    let b = s.as_bytes();
    let mut out: Vec<String> = Vec::new();
    let mut start = 0;
    let mut i = 0;
    while i <= b.len() {
        if i == b.len() || b[i] == 124 {
            out.push(String::from(&s[start..i]));
            start = i + 1;
        }
        i = i + 1;
    }
    out
}
pub fn split_c(s: &str) -> Vec<String> {
    let b = s.as_bytes();
    let mut out: Vec<String> = Vec::new();
    let mut start = 0;
    let mut i = 0;
    while i <= b.len() {
        if i == b.len() || b[i] == 44 {
            out.push(String::from(&s[start..i]));
            start = i + 1;
        }
        i = i + 1;
    }
    out
}
fn pty_pos(cx: &LCx, fi: i32, ty: &str) -> i32 {
    if fi < 0 {
        return -1;
    }
    let joined = String::from(cx.fptys[fi as usize].as_str());
    let b = joined.as_bytes();
    let mut start = 0;
    let mut idx = 0;
    let mut i = 0;
    while i <= b.len() {
        if i == b.len() || b[i] == 44 {
            let seg = String::from(&joined[start..i]);
            if seg.as_str() == ty {
                return idx;
            }
            idx = idx + 1;
            start = i + 1;
        }
        i = i + 1;
    }
    -1
}
fn is_known_ty(cx: &LCx, n: &str) -> bool {
    let nr = oast::sresolve(&cx.sdefs, n);
    let n = nr.as_str();
    // COMPOSITE type names count as known: without this a function returning `Vec<int>` propagated no type,
    // so a generic call could not unify its element and fell back to the un-monomorphized bare name
    if n == "str" || n == "Fn" || n == "bool" || n == "unit" {
        return true;
    }
    if is_arr_ty(n) {
        return true;
    } // STATIC-ARR: a const/static of array type (`static F: [i64; 4]`) keeps it -- its == compared ADDRESSES
      // K2 (probed from RC): a WIDTH is a property of the value and must survive a return, a parameter and
      // a let -- `let x = id(250u8); x + 10` is 4, and the suffix is three steps from the arithmetic. A
      // width name that is not "known" evaporates the moment it leaves the annotation that introduced it.
    if oast::is_int_width(n) || n == "char" || n == "f64" {
        return true;
    }
    if vec_elem_of(n).len() > 0 {
        return true;
    }
    if arr_elem_of(n).len() > 0 {
        return true;
    }
    if oparse::tup_elems(n).len() > 0 {
        return true;
    }
    if dyn_trait_of(n).len() > 0 {
        return true;
    }
    if oast::eidx(&cx.edefs, n) >= 0 {
        return true;
    }
    let mut i = 0;
    while i < cx.sdefs.len() {
        if cx.sdefs[i].name.as_str() == n {
            return true;
        }
        i = i + 1;
    }
    false
}
// the STRUCT TYPE NAME of an arbitrary expression ("" if not a struct) -- the lowering's own small inference,
// needed so `p.field` resolves to the right INDEX for params, call results, nested fields and literals.
// the type a `$zeroty$T` marker names, or "" when the name is not one
pub fn zeroty_name(n: &str) -> String {
    let b = n.as_bytes();
    let pb = "$zeroty$".as_bytes();
    if b.len() <= pb.len() {
        return String::from("");
    }
    let mut i = 0;
    while i < pb.len() {
        if b[i] != pb[i] {
            return String::from("");
        }
        i = i + 1;
    }
    String::from(&n[pb.len()..])
}
pub fn zero_of(cx: &mut LCx, t: &str) -> H {
    if t == "str" {
        let i = fresh(cx);
        return H::StrH(String::from(""), i);
    }
    // ARR-DEFAULT (refusal > WRONG; mir/issue-77002): an array's default is N elements' defaults, but an array type here
    // carries no length (`arr$int`) -- the empty sequence it answered made `b[0]` segfault / panic. Refused.
    if is_arr_ty(t) {
        let a5: Vec<H> = Vec::new();
        let i = fresh(cx);
        return H::CallH(String::from("$dflt_arr_unsupported"), a5, i);
    }
    if is_vec_ty(t) || t == "Vec" {
        let a2: Vec<H> = Vec::new();
        let i = fresh(cx);
        return H::CallH(String::from("vec_new"), a2, i);
    }
    // C-ZEROAGG: a STRUCT's default is a struct whose every field holds that field's default -- the same
    // question again, one level down, which is why this asks itself rather than guessing a word. Answering
    // the integer zero for an aggregate handed back a NULL HANDLE: `#[derive(Default)]` on an enum was
    // accepted and the match on it SEGFAULTED.
    {
        let rn = oast::sresolve(&cx.sdefs, t);
        if oast::is_struct_name(&cx.sdefs, &rn) {
            let nf = oast::sfields(&cx.sdefs, &rn);
            let mut lf: Vec<H> = Vec::new();
            let mut k = 0;
            while k < nf {
                let ft = oast::sfield_ty_ix(&cx.sdefs, &rn, k as usize);
                lf.push(zero_of(cx, &ft));
                k = k + 1;
            }
            let i = fresh(cx);
            return H::StructLitH(nf as u32, lf, i);
        }
    }
    // an ENUM has no derivable default here: rustc needs a `#[default]` variant and we do not read that
    // attribute, so this is REFUSED by name rather than answered with a null handle.
    if oast::is_enum_name(&cx.edefs, t) || oast::eidx(&cx.edefs, t) >= 0 {
        {
            let mut dn = String::from(t);
            dn.push_str("$default");
            if fn_known(cx, &dn) {
                let a4: Vec<H> = Vec::new();
                let i = fresh(cx);
                return H::CallH(dn, a4, i);
            }
        } // DFLT-ENUM: the `#[default]` variant
        let a3: Vec<H> = Vec::new();
        let i = fresh(cx);
        return H::CallH(String::from("$dflt_enum_unsupported"), a3, i);
    }
    let i = fresh(cx);
    H::Lit(0, i)
}
pub fn seq_tail_is(e: &E, n: &str) -> bool {
    match e {
        E::Var(bn, _l, _h) => bn.as_str() == n,
        E::Seq(_a, b, _l, _h) => seq_tail_is(b, n),
        _ => false,
    }
}
// C-BODYTY: the type a function BODY answers, which is its TAIL's type. ety_of cannot answer this for an
// annotated let, because the annotation is a channel the desugars read back out of it (rung 95 refuted two
// attempts to re-interpret it, one caught by the vec!/Debug probes and one by the durable progs gate). So
// the consumer that wants the tail gets its own rule instead: walk past a let whose tail is NOT its own
// binding, and otherwise defer to ety_of, which handles the `let x = v; x` shape already.
pub fn body_ty(cx: &LCx, e: &E) -> String {
    match e {
        E::Let(n, _v, b, _l, _h) => {
            if seq_tail_is(b, n) {
                return ety_of(cx, e);
            }
            body_ty(cx, b)
        }
        E::LetT(n, _t, _v, b, _l, _h) => {
            if seq_tail_is(b, n) {
                return ety_of(cx, e);
            }
            body_ty(cx, b)
        }
        E::Seq(_a, b, _l, _h) => body_ty(cx, b),
        _ => ety_of(cx, e),
    }
}
// GEN-RET-SUBST: a generic call's RETURN type with each type parameter inferred from its argument (`fn sw<T>(p:(T,T))->(T,T)` on (str,str) -> tup2$str$str); "" unless every parameter in it resolves
pub fn gen_ret_subst(cx: &LCx, fi: i32, rt: &str, cargs: &Vec<E>) -> String {
    if fi < 0 || (fi as usize) >= cx.fgps.len() {
        return String::from("");
    }
    let gps = split_c(&cx.fgps[fi as usize]);
    let ptys = split_c(&cx.fptys[fi as usize]);
    if gps.len() == 0 {
        return String::from("");
    }
    let mut r = String::from(rt);
    let mut hit = false;
    let mut g = 0;
    while g < gps.len() {
        if gps[g].len() == 0 {
            return String::from("");
        }
        if oast::has_seg(r.as_str(), gps[g].as_str()) || r.as_str() == gps[g].as_str() {
            let mut c = String::from("");
            let mut k = 0;
            while k < ptys.len() && k < cargs.len() && c.len() == 0 {
                if oast::has_seg(ptys[k].as_str(), gps[g].as_str())
                    || ptys[k].as_str() == gps[g].as_str()
                {
                    c = oast::unify_seg(
                        ptys[k].as_str(),
                        ety_of(cx, &cargs[k]).as_str(),
                        gps[g].as_str(),
                    );
                }
                k = k + 1;
            }
            if c.len() == 0 {
                return String::from("");
            }
            r = oast::subst_seg(r.as_str(), gps[g].as_str(), c.as_str());
            hit = true;
        }
        g = g + 1;
    }
    if hit {
        r
    } else {
        String::from("")
    }
}
pub fn ety_of(cx: &LCx, e: &E) -> String {
    if let E::MethodCall(r, _m, _a, _l, _h) = e {
        let rt = ety_of(cx, r); // ETY-ONCE: the receiver is typed once per call
        return ety_mcall(cx, e, rt.as_str());
    }
    match e {
        E::Var(n, _a, _b) => sty_of(cx, n),
        E::StructLit(sn, _f, _a, _b) => String::from(sn.as_str()),
        E::EnumLit(en, _v, _p, _a, _b) => String::from(en.as_str()),
        E::StrLit(_v, _l, _h) => String::from("str"),
        // an integer literal's type was left UNKNOWN here (the engine reads "" as int anyway), which was
        // invisible until a tuple's name had to be minted from its elements: one unknown element made the
        // whole name fall back to the bare arity
        E::Int(_v, _l, _h) => String::from("int"),
        E::Bool(_v, _l, _h) => String::from("bool"),
        E::Not(x, _l, _h) => {
            let tx = ety_of(cx, x);
            if tx.as_str() == "int" || oast::is_int_width(&tx) {
                if oast::is_int_width(&tx) {
                    tx
                } else {
                    String::from("int")
                }
            } else {
                String::from("bool")
            }
        }
        E::RetE(x, _l, _h) => ety_of(cx, x),
        // ETY-EXP (REGRESS-E): ONE arm for binary operators, which types each operand ONCE (ety_bin). Seven guarded arms each
        // re-typed the operands, and a failing guard fell through to the next, so an n-operator chain cost k^n: a 20-operator
        // `1 * 1 * ..` was OOM-killed at compile time.
        E::Bin(o, a, b2, _l, _h) => ety_bin(cx, o, a, b2),
        E::MethodCall(r, m, _a2, _l, _h)
            if (m.as_str() == "join" || m.as_str() == "concat") && {
                let t = ety_of(cx, r);
                t.as_str() == "Vec$str" || t.as_str() == "arr$str"
            } =>
        {
            String::from("str")
        } // STR-JOIN result type
        E::MethodCall(r, m, a2, _l, _h)
            if oast::str_starts(ety_of(cx, r).as_str(), "HashMap")
                && (m.as_str() == "iter" || m.as_str() == "into_iter")
                && a2.len() == 0 =>
        {
            if ety_of(cx, r).as_str() == "HashMap$str" {
                String::from("Vec$tup2$str$int")
            } else {
                String::from("Vec$tup2$int$int")
            }
        } // HM-ITER
        E::MethodCall(r, m, a2, _l, _h)
            if oast::str_starts(ety_of(cx, r).as_str(), "HashMap")
                && m.as_str() == "keys"
                && a2.len() == 0 =>
        {
            if ety_of(cx, r).as_str() == "HashMap$str" {
                String::from("Vec$str")
            } else {
                String::from("Vec$int")
            }
        } // HM-KEYS
        E::MethodCall(r, m, a2, _l, _h)
            if oast::str_starts(ety_of(cx, r).as_str(), "HashMap")
                && m.as_str() == "is_empty"
                && a2.len() == 0 =>
        {
            String::from("bool")
        } // HM-KEYS
        E::MethodCall(r, m, _a2, _l, _h)
            if oast::str_starts(ety_of(cx, r).as_str(), "HashMap")
                && (m.as_str() == "get"
                    || m.as_str() == "remove"
                    || m.as_str() == "contains_key"
                    || m.as_str() == "len") =>
        {
            if m.as_str() == "get" || m.as_str() == "remove" {
                String::from("Option")
            } else if m.as_str() == "contains_key" {
                String::from("bool")
            } else {
                String::from("int")
            }
        } // HASHMAP method types
        E::MethodCall(r, m, _a2, _l, _h)
            if m.as_str() == "to_ascii_lowercase"
                || m.as_str() == "to_ascii_uppercase"
                || m.as_str() == "wrapping_neg"
                || m.as_str() == "rem_euclid"
                || m.as_str() == "div_euclid"
                || m.as_str() == "signum"
                || m.as_str() == "wrapping_shl"
                || m.as_str() == "wrapping_shr" =>
        {
            ety_of(cx, r)
        } // INT-METHODS keep the receiver type
        // C-ETYARITH: arithmetic over integers ANSWERS an integer. ety_of used to fall through to "" for
        // it, so any rule that needed the kind of `a-1` was blind -- which is why `!(a-1)` reached the
        // bool lowering and computed a comparison with zero (C-BITNOT, rungs 44-45).
        E::Try(_x, _l, _h) => String::from(""),
        E::Ref(x, _m, _l, _h) => ety_of(cx, x),
        E::Cast(_x, t, _l, _h) => String::from(t.as_str()),
        E::ClosRef(n, _c, _l, _h) => {
            let mut r = String::from("clos$");
            r.push_str(n.as_str());
            r
        }
        E::TupLit(p, _l, _h) => {
            if p.len() == 0 {
                String::from("unit")
            } else {
                let mut es: Vec<String> = Vec::new();
                let mut k = 0;
                while k < p.len() {
                    es.push(ety_of(cx, &p[k]));
                    k = k + 1;
                }
                oparse::tup_name(p.len() as u32, &es)
            }
        }
        E::TupIdx(b, ix, _l, _h) => {
            let bt = ety_of(cx, b);
            // a tuple STRUCT field has a declared type; a plain tuple does not
            if bt.len() > 0 {
                let t = oast::sfield_ty(&cx.sdefs, &bt, oast::u32s2(*ix).as_str());
                if t.len() > 0 {
                    return t;
                }
            }
            // a plain tuple's element type now comes from its NAME
            {
                let es = oparse::tup_elems(&bt);
                let r = oast::tix_resolve(*ix, es.len()); // TUP-REST-END: a from-the-end slot resolves here too
                if r >= 0 {
                    return String::from(es[r as usize].as_str());
                }
            }
            String::from("")
        }
        E::MethodCall(r, m, a2, _l, _h)
            if m.as_str() == "clone"
                && a2.len() == 0
                && (oast::eidx(&cx.edefs, &ety_of(cx, r)) >= 0
                    || oast::sfields(&cx.sdefs, &ety_of(cx, r)) > 0) =>
        {
            ety_of(cx, r)
        } // CLONE-STRUCT: a struct clone keeps the type
        // C-STRASRESULT: as_str, to_string and to_owned are the IDENTITY in value but their TYPE is
        // always str, whatever the receiver. ety_of was answering the receiver's type, so a String built
        // from an unnameable receiver came out unnameable itself -- and String::from is erased at parse
        // time (oparse maps it to the identity), so nothing downstream restored the name. That chain is
        // how six push_str calls in oparse reached the emitter as `$push_str`: trn unnamed, then
        // trn.as_str() unnamed, then dn unnamed.
        E::MethodCall(r, m, _a2, _l, _h) if gen_ret(cx, r, m).len() > 0 => gen_ret(cx, r, m), // GEN-INST: a generic impl method returning T on a recorded instance
        E::MethodCall(r, m, _a2, _l, _h)
            if dyn_trait_of(&ety_of(cx, r)).len() > 0
                && dyn_ret(cx, &dyn_trait_of(&ety_of(cx, r)), m).len() > 0 =>
        {
            dyn_ret(cx, &dyn_trait_of(&ety_of(cx, r)), m)
        } // DYN-RET: a dyn call answers its implementations' declared return type
        E::MethodCall(r, m, a2, _l, _h)
            if (m.as_str() == "get" || m.as_str() == "borrow" || m.as_str() == "borrow_mut")
                && a2.len() == 0 =>
        {
            let vt = ety_of(cx, r);
            vec_elem_of(&vt)
        } // C-CELL (+ R4 REFCELL)
        E::MethodCall(_r, m, a2, _l, _h) if m.as_str() == "split" && a2.len() == 1 => {
            String::from("Vec$str")
        }
        E::MethodCall(_r, m, a2, _l, _h) if m.as_str() == "split_whitespace" && a2.len() == 0 => {
            String::from("Vec$str")
        } // SPLIT-WS
        E::MethodCall(_r, m, a2, _l, _h) if m.as_str() == "collect_string" && a2.len() == 0 => {
            String::from("str")
        } // COLLECT-TURBO
        E::MethodCall(r, m, a2, _l, _h)
            if (m.as_str() == "unwrap_or" || m.as_str() == "unwrap_or_default")
                && a2.len() == 1
                && {
                    let e0 = ety_of(cx, r);
                    e0.as_str() == "Option" || e0.as_str() == "Result"
                }
                && ety_of(cx, &a2[0]).len() > 0
                && ety_of(cx, &a2[0]).as_str() != "int" =>
        {
            ety_of(cx, &a2[0])
        } // UNWRAP-OR-TY: the default has the payload type
        E::MethodCall(_r, m, a2, _l, _h)
            if m.as_str() == "fold" && a2.len() == 2 && ety_of(cx, &a2[0]).len() > 0 =>
        {
            ety_of(cx, &a2[0])
        } // FOLD-TY: fold answers its accumulator's type (a String accumulator printed as an address)
        E::MethodCall(r, m, a2, _l, _h)
            if (m.as_str() == "windows" || m.as_str() == "chunks") && a2.len() == 1 =>
        {
            let mut t = String::from("Vec$");
            t.push_str(&ety_of(cx, r));
            t
        } // SLICE-WIN: a Vec of sub-slices
        E::MethodCall(_r, m, a2, _l, _h) if m.as_str() == "char_indices" && a2.len() == 0 => {
            String::from("Vec$tup2$int$char")
        } // CHAR-IDX
        E::MethodCall(_r, m, a2, _l, _h) if m.as_str() == "parse" && a2.len() == 0 => {
            String::from("Result")
        } // PARSE-UNNAMED: str::parse answers a Result
        E::MethodCall(_r, m, a2, _l, _h)
            if (m.as_str() == "as_str"
                || m.as_str() == "to_string"
                || m.as_str() == "to_owned"
                || (m.as_str() == "into" && ety_of(cx, _r).as_str() == "str"))
                && a2.len() == 0 =>
        {
            String::from("str")
        }
        E::MethodCall(r, m, a2, _l, _h)
            if (m.as_str() == "as_ref" || m.as_str() == "as_mut") && a2.len() == 0 =>
        {
            ety_of(cx, r)
        }
        E::MethodCall(r, m, a2, _l, _h)
            if (m.as_str() == "get" || m.as_str() == "remove" || m.as_str() == "swap_remove")
                && a2.len() == 1
                && vec_elem_of(&ety_of(cx, r)).len() > 0 =>
        {
            vec_elem_of(&ety_of(cx, r))
        }
        E::MethodCall(r, m, a2, _l, _h)
            if (m.as_str() == "wrapping_mul"
                || m.as_str() == "wrapping_add"
                || m.as_str() == "wrapping_sub")
                && a2.len() == 1 =>
        {
            ety_of(cx, r)
        }
        E::MethodCall(r, m, _a2, _l, _h)
            if m.as_str() == "chars" && ety_of(cx, r).as_str() == "str" =>
        {
            String::from("Vec$char")
        } // STR-CHARS-UTF8
        E::MethodCall(r, m, _a2, _l, _h)
            if (m.as_str() == "as_bytes" || m.as_str() == "bytes")
                && ety_of(cx, r).as_str() == "str" =>
        {
            String::from("bstr")
        }
        E::MethodCall(r, m, a2, _l, _h)
            if (m.as_str() == "is_nan"
                || m.as_str() == "is_infinite"
                || m.as_str() == "is_finite")
                && a2.len() == 0
                && ety_of(cx, r).as_str() == "f64" =>
        {
            String::from("bool")
        } // F64-PREDS
        E::MethodCall(r, m, a2, _l, _h)
            if (m.as_str() == "floor"
                || m.as_str() == "ceil"
                || m.as_str() == "round"
                || m.as_str() == "abs"
                || m.as_str() == "sqrt"
                || m.as_str() == "signum")
                && a2.len() == 0
                && ety_of(cx, r).as_str() == "f64" =>
        {
            String::from("f64")
        } // D1.2
        E::MethodCall(r, m, a2, _l, _h)
            if m.as_str() == "powi" && a2.len() == 1 && ety_of(cx, r).as_str() == "f64" =>
        {
            String::from("f64")
        } // D1.4
        E::MethodCall(r, m, a2, _l, _h)
            if ety_of(cx, r).as_str() == "f64" && {
                let bm = builtin_method("f64", m.as_str(), a2.len());
                oast::str_starts(bm.as_str(), "f64m_") || oast::str_starts(bm.as_str(), "f64m2_")
            } =>
        {
            String::from("f64")
        } // F64-LIBM
        E::MethodCall(r, m, a2, _l, _h)
            if (m.as_str() == "max" || m.as_str() == "min")
                && a2.len() == 1
                && ety_of(cx, r).as_str() == "f64" =>
        {
            String::from("f64")
        } // D1.6
        E::MethodCall(r, m, _a2, _l, _h)
            if m.as_str() == "count" && ety_of(cx, r).as_str() == "bstr" =>
        {
            String::from("int")
        }
        E::MethodCall(_r, m, _a2, _l, _h)
            if m.as_str() == "to_string" || m.as_str() == "to_owned" =>
        {
            String::from("str")
        }
        E::MethodCall(r, m, _a2, _l, _h) if m.as_str() == "get" => {
            let vt = ety_of(cx, r);
            vec_elem_of(&vt)
        }
        // a BUILT-IN method's result type, by the same rule the typechecker uses: an adaptor answers the
        // same sequence, a predicate a bool, the rest an int
        E::MethodCall(r, m, _a2, _l2, _h2)
            if (m.as_str() == "unwrap"
                || m.as_str() == "expect"
                || m.as_str() == "unwrap_or"
                || m.as_str() == "unwrap_or_default"
                || m.as_str() == "unwrap_or_else")
                && unwrap_pay(cx, r).len() > 0 =>
        {
            unwrap_pay(cx, r)
        } // UNWRAP-PAY
        E::MethodCall(r, m, _a2, _l2, _h2)
            if (m.as_str() == "unwrap" || m.as_str() == "expect") && {
                match r.as_ref() {
                    E::MethodCall(v, cm, _va, _vl, _vh) => {
                        (cm.as_str() == "first"
                            || cm.as_str() == "last"
                            || cm.as_str() == "nth"
                            || cm.as_str() == "max"
                            || cm.as_str() == "min"
                            || cm.as_str() == "pop")
                            && vec_elem_of(&ety_of(cx, v)).len() > 0
                    }
                    _ => false,
                }
            } =>
        {
            match r.as_ref() {
                E::MethodCall(v, _cm, _va, _vl, _vh) => vec_elem_of(&ety_of(cx, v)),
                _ => String::from(""),
            }
        } // D4.1 typed-Option chain
        E::MethodCall(r, m, a2, _ab, _bb)
            if builtin_method(&ety_of(cx, r), m, a2.len()).len() > 0 =>
        {
            let rt = ety_of(cx, r);
            if m.as_str() == "checked_div" {
                return String::from("Option");
            } // P-CHECKEDDIV
            if m.as_str() == "checked_add"
                || m.as_str() == "checked_sub"
                || m.as_str() == "checked_mul"
            {
                return String::from("Option");
            } // D3.1
            if m.as_str() == "saturating_add"
                || m.as_str() == "saturating_sub"
                || m.as_str() == "to_le"
                || m.as_str() == "saturating_mul"
            {
                return String::from(rt.as_str());
            } // SAT-OPS: keeps the width
            if m.as_str() == "count_ones"
                || m.as_str() == "leading_zeros"
                || m.as_str() == "trailing_zeros"
            {
                return String::from("int");
            } // D3.2
            if (m.as_str() == "max" || m.as_str() == "min")
                && a2.len() == 1
                && rt.as_str() == "char"
            {
                return String::from("char");
            } // ORD-CHAR: 'a'.min('c') is a char (it printed 97)
            if m.as_str() == "pow"
                || m.as_str() == "abs"
                || (m.as_str() == "max" && a2.len() == 1)
                || (m.as_str() == "min" && a2.len() == 1)
            {
                return String::from("int");
            } // P-INT-METHODS
            if m.as_str() == "is_ascii_digit"
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
                || m.as_str() == "is_ascii_uppercase"
            {
                return String::from("bool");
            } // P-CHAR-PRED
            if oast::pay_base(rt.as_str()).as_str() == "Option" {
                if m.as_str() == "map" || m.as_str() == "and_then" {
                    return String::from("Option");
                }
                if m.as_str() == "ok_or" {
                    return String::from("Result");
                } // P-RES-COMB
                if m.as_str() == "filter" {
                    return String::from("Option");
                } // P-OPT-FILTER
                if m.as_str() == "is_some" || m.as_str() == "is_none" {
                    return String::from("bool");
                }
                if m.as_str() == "copied" || m.as_str() == "cloned" {
                    return String::from("Option");
                } // P-OPT-COPIED
                return String::from("int");
            }
            if oast::pay_base(rt.as_str()).as_str() == "Result" {
                if m.as_str() == "is_ok" || m.as_str() == "is_err" {
                    return String::from("bool");
                }
                if m.as_str() == "ok" {
                    return String::from("Option");
                } // P-RES-COMB
                if m.as_str() == "map" || m.as_str() == "map_err" {
                    return String::from("Result");
                }
                return String::from("int");
            }
            if rt.as_str() == "str" {
                if m.as_str() == "push" || m.as_str() == "push_str" {
                    return String::from("int");
                } // C-PUSHUNIT: () here
                if m.as_str() == "parse" {
                    return String::from("Result");
                }
                if m.as_str() == "find" || m.as_str() == "rfind" {
                    return String::from("Option");
                }
                if m.as_str() == "is_empty"
                    || m.as_str() == "contains"
                    || m.as_str() == "starts_with"
                    || m.as_str() == "ends_with"
                {
                    return String::from("bool");
                }
                if m.as_str() == "len" {
                    return String::from("int");
                }
                return String::from("str");
            }
            if (m.as_str() == "iter" || m.as_str() == "into_iter")
                && oast::str_starts(rt.as_str(), "arr$")
            {
                let mut vt = String::from("Vec$");
                vt.push_str(&rt[4..]);
                return vt;
            } // ARR-METH
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
                || m.as_str() == "filter"
                || m.as_str() == "rev"
                || m.as_str() == "take"
                || m.as_str() == "skip"
                || m.as_str() == "step_by"
                || m.as_str() == "take_while"
                || m.as_str() == "skip_while"
                || m.as_str() == "chain"
            {
                return rt;
            }
            if m.as_str() == "map" {
                match &a2[0] {
                    E::ClosRef(cn, _c, _cl, _ch) => {
                        let rr = fret_of(cx, cn);
                        if rr.len() > 0 {
                            let mut vt = String::from("Vec$");
                            vt.push_str(rr.as_str());
                            return vt;
                        }
                    }
                    _ => {}
                }
                return rt;
            } // D5
            if m.as_str() == "flat_map" {
                match &a2[0] {
                    E::ClosRef(cn, _c, _cl, _ch) => {
                        let rr = fret_of(cx, cn);
                        if rr.len() > 0 {
                            return rr;
                        }
                    }
                    _ => {}
                }
                return rt;
            } // D5
            if m.as_str() == "partition" {
                let el = vec_elem_of(&rt);
                if el.len() > 0 {
                    let mut vt = String::from("Vec$");
                    vt.push_str(el.as_str());
                    let mut es: Vec<String> = Vec::new();
                    es.push(String::from(vt.as_str()));
                    es.push(vt);
                    return oparse::tup_name(2, &es);
                }
                return rt;
            }
            if m.as_str() == "enumerate" {
                let el = vec_elem_of(&rt);
                if el.len() > 0 && {
                    let eb = el.as_bytes();
                    let mut d = false;
                    let mut z = 0;
                    while z < eb.len() {
                        if eb[z] == 36 {
                            d = true;
                        }
                        z = z + 1;
                    }
                    !d
                } {
                    let mut t = String::from("Vec$tup2$int$");
                    t.push_str(el.as_str());
                    return t;
                }
            } // ENUM-ELEM: (index, element) keeps the element type -- untyped, a char printed as its number
            if m.as_str() == "enumerate" || m.as_str() == "zip" {
                return String::from("Vec$tup2");
            } // P-ITER-ZIP
            if m.as_str() == "binary_search" {
                return String::from("Result$int$int");
            } // VEC-BSEARCH: Ok(index) / Err(insertion point), both usize
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
                return String::from("Option");
            }
            if m.as_str() == "get" {
                return vec_elem_of(&rt);
            }
            // a PREDICATE answers a bool, and `{}` must spell it `true`/`false`
            if m.as_str() == "any"
                || m.as_str() == "all"
                || m.as_str() == "is_empty"
                || m.as_str() == "contains"
            {
                return String::from("bool");
            }
            String::from("int")
        }
        E::MethodCall(r, m, _a2, _a, _b) => {
            let rt = ety_of(cx, r);
            let mut nm = String::from(rt.as_str());
            nm.push_str("$");
            nm.push_str(m);
            if fidx_of(cx, &nm) < 0 {
                let u = unique_method(cx, m);
                if u.len() > 0 {
                    nm = u;
                }
            }
            let r2 = fret_of(cx, &nm);
            // G1 G-P3: a generic impl's method returning `Self` (`Vec$T$tw`) answers the RECEIVER's type
            let r2 = if r2.as_str() == "Self" && rt.len() > 0 {
                String::from(rt.as_str())
            } else {
                r2
            };
            let r2 = gen_meth_ret(cx, &nm, &rt, r2); // G1 G-P3: impl parameters substituted from the receiver
            if r2.len() > 0 && is_known_ty(cx, &r2) {
                r2
            } else {
                String::from("")
            }
        }
        E::Call(fnm, _fa, _a, _b)
            if clos_of(&sty_of(cx, fnm.as_str())).len() > 0
                && fret_of(cx, clos_of(&sty_of(cx, fnm.as_str())).as_str()).len() > 0 =>
        {
            fret_of(cx, clos_of(&sty_of(cx, fnm.as_str())).as_str())
        } // FN-ITEM-RET: a call through a closure-valued local answers the lifted closure's return type (it answered nothing)
        E::Call(fnm, _c2, _a, _b) if fnm.as_str() == "readfile" || fnm.as_str() == "argv_at" => {
            String::from("str")
        }
        E::Call(fnm, cz, _a, _b) if fnm.as_str() == "$zeroof" && cz.len() == 1 => {
            ety_of(cx, &cz[0])
        }
        E::Call(fnm, _cz, _a, _b) if oast::str_starts(fnm.as_str(), "$xmute$") => {
            String::from(&fnm[7..])
        } // R4a XMUTE
        E::Call(fnm, _cz, _a, _b)
            if oast::str_starts(fnm.as_str(), "$sizeof$")
                || oast::str_starts(fnm.as_str(), "$alignof$") =>
        {
            String::from("usize")
        } // R4a LAYOUT
        E::Call(fnm, _cz, _a, _b) if fnm.as_str() == "$sizeofval" => String::from("usize"), // R4a LAYOUT
        E::Call(fnm, cz, _a, _b) if fnm.as_str() == "$alias" && cz.len() == 1 => {
            ety_of(cx, unref(&cz[0]))
        } // R4f CHAN
        E::Call(fnm, _cz, _a, _b) if fnm.as_str() == "$ptrnull" => String::from("int"), // R4b PTR
        E::Call(fnm, cz, _a, _b) if fnm.as_str() == "$chanint" && cz.len() == 1 => {
            ety_of(cx, &cz[0])
        }
        E::Call(fnm, _cz, _a, _b) if zeroty_name(fnm).len() > 0 => zeroty_name(fnm),
        // RC-IDENTITY (R4c): an Rc handle has its referent's type (identity handle)
        E::Call(fnm, cz, _a, _b)
            if (fnm.as_str() == "$rcnew"
                || fnm.as_str() == "$rcclone"
                || fnm.as_str() == "$boxnew"
                || fnm.as_str() == "$cellhandle")
                && cz.len() == 1 =>
        {
            ety_of(cx, unref(&cz[0]))
        }
        E::Call(fnm, cz, _a, _b) if fnm.as_str() == "$rccount" && cz.len() == 1 => {
            String::from("int")
        }
        E::Call(fnm, cz, _a, _b) if fnm.as_str() == "$cellnew" && cz.len() == 1 => {
            let t = ety_of(cx, &cz[0]);
            let mut r = String::from("Vec$");
            r.push_str(t.as_str());
            r
        }
        E::Call(fnm, cz, _a, _b) if fnm.as_str() == "vec_push1" && cz.len() == 2 => {
            let t = ety_of(cx, &cz[1]);
            let mut r = String::from("Vec$");
            r.push_str(t.as_str());
            r
        }
        E::Call(fnm, _cz, _a, _b) if fnm.as_str() == "str_split" => String::from("Vec$str"),
        E::Call(fnm, _cz, _a, _b) if fnm.as_str() == "str_chars_vec" => String::from("Vec$char"), // CHARS-VEC
        E::Call(fnm, _cz, _a, _b) if fnm.as_str() == "str_bytes_vec" => String::from("Vec$u8"), // STR-BYTES
        E::Call(fnm, _c2, _a, _b)
            if fnm.as_str() == "str_dbg"
                || fnm.as_str() == "dbgq_s"
                || fnm.as_str() == "dbgc_s" =>
        {
            String::from("str")
        } // FMT-DBG-STR
        E::Call(fnm, _c2, _a, _b) if fnm.as_str() == "$shovf" || fnm.as_str() == "$ovf1" => {
            String::from("bool")
        } // OVF-SHIFT + OVF-NEGABS
        E::Call(fnm, _c2, _a, _b)
            if fnm.as_str() == "str_cat"
                || fnm.as_str() == "str_sub"
                || fnm.as_str() == "str_of"
                || fnm.as_str() == "str_from_int"
                || fnm.as_str() == "str_copy" =>
        {
            String::from("str")
        }
        E::Call(fnm, _cg, _a, _b) if fnm.as_str() == "vec_new" => String::from("Vec"),
        E::Call(fnm, fa, _a, _b) if fnm.as_str() == "arr_fill" && fa.len() == 2 => {
            let et = ety_of(cx, &fa[1]);
            let mut r = String::from("arr");
            if et.len() > 0 {
                r.push('$');
                r.push_str(et.as_str());
            }
            r
        } // ARR-REP-LOOP: an array of its fill value
        E::Call(fnm, _cg, _a, _b)
            if fnm.as_str() == "f64_parse" || fnm.as_str() == "f64_of_int" =>
        {
            String::from("f64")
        }
        E::Call(fnm, _cg, _a, _b)
            if fnm.as_str() == "f64_snprintf" || fnm.as_str() == "f64_snprintf_e" =>
        {
            String::from("str")
        }
        E::Call(fnm, _cg, _a, _b) if fnm.as_str() == "f64_isnan" => String::from("int"),
        E::Call(fnm, _cg, _a, _b) if fnm.as_str() == "f64_bits" => String::from("int"),
        E::Call(fnm, _cg, _a, _b) if fnm.as_str() == "f64_sqrt" => String::from("f64"), // D1.5
        E::Call(fnm, _cg, _a, _b)
            if oast::str_starts(fnm.as_str(), "f64m_")
                || oast::str_starts(fnm.as_str(), "f64m2_") =>
        {
            String::from("f64")
        } // F64-LIBM
        E::Call(fnm, _cg, _a, _b) if is_gname(fnm) => String::from("int"),
        E::Call(fnm, cg, _a, _b)
            if fnm.as_str() == "seq_slice"
                && cg.len() == 3
                && (ety_of(cx, &cg[0]).as_str() == "str"
                    || ety_of(cx, &cg[0]).as_str() == "bstr") =>
        {
            // BSTR-SLICE (R2a P4): a range slice of a byte string is a byte string (str_sub on the same buffer)
            ety_of(cx, &cg[0])
        }
        E::Call(fnm, cg, _a, _b) if fnm.as_str() == "seq_slice" && cg.len() == 3 => {
            let st = ety_of(cx, &cg[0]);
            // C-ARRSLICE: a slice OF AN ARRAY is an array -- arr_slice answers one -- and without this the
            // slice had no type at all, so `{:?}` on it was refused while the array itself printed.
            if is_arr_ty(&st) {
                let ae = arr_elem_of(&st);
                let mut r = String::from("Vec$");
                if ae.len() > 0 {
                    r.push_str(ae.as_str());
                } else {
                    r.push_str("int");
                }
                return r;
            }
            let el = vec_elem_of(&st);
            if el.len() > 0 {
                let mut r = String::from("Vec$");
                r.push_str(el.as_str());
                r
            } else {
                String::from("Vec")
            }
        }
        // C-LETTAIL: a `let` whose body ENDS in the bound name answers the bound value's type. It used to
        // require the body to BE that name, so a desugar that writes something before handing the binding
        // back -- mem::replace is exactly that shape -- fell through to the unknown type and printed a str
        // as a number. The tail is walked through Seq, where every such desugar puts its statements.
        E::Let(n, v, b, _l, _h) => {
            if seq_tail_is(b, n) {
                let t = ety_of(cx, v);
                if t.as_str() == "Vec" {
                    let et = vpush_elem_ty(cx, b, n);
                    if et.len() > 0 && et.as_str() != "int" {
                        let mut r = String::from("Vec$");
                        r.push_str(et.as_str());
                        return r;
                    }
                } // VEC-LIT-ELEM: `vec![D(1)]` is `let $vecl = vec_new(); vec_push($vecl, D(1)); $vecl` -- name the element
                return t;
            }
            ety_of(cx, b)
        }
        // C-ARRELEM: an array's type name carries its ELEMENT, the way a vector's does. It said so only
        // for strings, and the bare `arr` meant "array of something" -- so the Debug printer, which picks
        // by element, showed an array of bools as [1, 0] and an array of arrays as its inner POINTERS.
        E::ArrLit(x, _l, _h) => {
            if x.len() == 0 {
                return String::from("arr");
            }
            let e0 = ety_of(cx, &x[0]);
            if e0.len() == 0 {
                return String::from("arr");
            }
            let mut same = true;
            let mut k = 1;
            while k < x.len() {
                if ety_of(cx, &x[k]).as_str() != e0.as_str() {
                    same = false;
                }
                k = k + 1;
            }
            if !same {
                return String::from("arr");
            }
            let mut r = String::from("arr$");
            r.push_str(e0.as_str());
            r
        }
        E::Index(b, _ix, _l, _h) => {
            let bt = ety_of(cx, b);
            if bt.as_str() == "bstr" {
                return String::from("int");
            }
            if oast::str_starts(bt.as_str(), "HashMap") {
                return String::from("int");
            }
            let el = elem_of(&bt);
            if el.len() > 0 {
                el
            } else {
                String::from("")
            }
        }
        E::Match(sc, arms, _l, _h) => {
            // W6 MATCH-TY: all arms have ONE type in Rust -- the first arm that can be typed answers. Arm 0 alone was
            // `Var(v)` for `Some(v) => v`, a binding not in scope here, so the match was untyped and println printed a
            // String pointer as an int. A body that IS its own Some(v) binding is the scrutinee's payload type.
            // MATCH-BIND-PAY guard: a str payload returned through its binding keeps the OLD typing (refused downstream) --
            // typed str, `.len()` on the String value reads its bytes as a length (IC-7 / W6 string-payload-len: WRONG)
            let strbind = match_returns_str_bind(cx, sc, arms);
            let mut ctlt = String::new(); // the first control arm's type: the OLD answer, kept when no other arm types the match
            let mut k = 0;
            while k < arms.len() {
                // MATCH-BIND-PAY (R2a P4): a control-flow arm (`_ => return false`, break, continue) never answers the match type --
                // it typed `let xs = match e { E::StructLit(_, xs, ..) => xs, .., _ => return false }` as bool (ext_any: bool$len).
                // (panic arms are NOT skipped: ARM-DIVERGE showed their type carries other shapes.)
                if !strbind && ctl_arm(&arms[k].body) {
                    if ctlt.len() == 0 {
                        ctlt = ety_of(cx, &arms[k].body);
                    }
                    k = k + 1;
                    continue;
                }
                // TUP-BIND-PAY (R2a P4): an arm whose body is a TUPLE of its own bindings (`E::C(cn, caps, ..) => (cn, caps)`)
                // answers the tuple of their payload types -- untyped, `let (cn, caps) = match ..` left caps untyped and
                // `caps[0]` read it as an ARRAY (__arr_get) on a Vec handle (ourc compiled by ourc: SIGSEGV in cap_drops)
                if let E::TupLit(xs0, _tl0, _th0) = &arms[k].body {
                    let mut tys0: Vec<String> = Vec::new();
                    let mut all0 = xs0.len() > 0;
                    let mut xi0 = 0;
                    while xi0 < xs0.len() {
                        let t = match &xs0[xi0] {
                            E::Var(xn, _a, _b) => bind_payload_ty(cx, sc, &arms[k], xn.as_str()),
                            _ => String::new(),
                        };
                        if t.len() == 0 {
                            all0 = false;
                        }
                        tys0.push(t);
                        xi0 = xi0 + 1;
                    }
                    if all0 {
                        return oparse::tup_name(xs0.len() as u32, &tys0);
                    }
                }
                // ... and an arm that returns one of its own bindings answers that payload's declared type (generalizes the
                // `Some(v) => v` case below to any enum variant / position)
                if let E::Var(bv0, _bl0, _bh0) = &arms[k].body {
                    let mut bi0 = 0;
                    while bi0 < arms[k].binds.len() {
                        if arms[k].binds[bi0].as_str() == bv0.as_str() {
                            // the scrutinee is often P-PAT's synthetic `$msN` (not in scope here: untyped) -- then the enum is the
                            // ONE that declares this variant (ambiguous -> no answer)
                            let st0 = {
                                let t = ety_of(cx, sc);
                                if oast::eidx(&cx.edefs, t.as_str()) >= 0 {
                                    t
                                } else {
                                    sole_enum_of_variant(&cx.edefs, arms[k].variant.as_str())
                                }
                            };
                            if st0.len() > 0 && oast::eidx(&cx.edefs, st0.as_str()) >= 0 {
                                let pt0 = oast::vpayload_ty(
                                    &cx.edefs,
                                    st0.as_str(),
                                    arms[k].variant.as_str(),
                                    bi0,
                                );
                                if pt0.len() > 0
                                    && pt0.as_str() != "str"
                                    && pt0.as_str() != "String"
                                {
                                    return pt0;
                                }
                            }
                        }
                        bi0 = bi0 + 1;
                    }
                }
                let t = ety_of(cx, &arms[k].body);
                if t.len() > 0 {
                    return t;
                }
                {
                    // LETELSE-PAYLOAD-TY (W7): a body that IS one of its own bindings of a USER enum variant (`V2(b) => b`, which
                    // is what `let V::V2(b) = v else ..` rewrites to) is that variant's payload type at the binding's position --
                    // untyped, a bool printed as 1
                    let pt = arm_bind_payload_ty(cx, sc, &arms[k]);
                    if pt.len() > 0 {
                        return pt;
                    }
                }
                if arms[k].binds.len() == 1 && arms[k].variant.as_str() == "Some" {
                    if let E::Var(bv, _bl, _bh) = &arms[k].body {
                        if bv.as_str() == arms[k].binds[0].as_str() {
                            let st = ety_of(cx, sc);
                            if oast::str_starts(st.as_str(), "Option$") {
                                let b = st.as_bytes();
                                let mut p = String::new();
                                let mut j = 7;
                                while j < b.len() {
                                    p.push(b[j] as char);
                                    j = j + 1;
                                }
                                return p;
                            }
                        }
                    }
                }
                k = k + 1;
            }
            ctlt // MATCH-BIND-PAY: no other arm typed the match -> the control arm's (old) answer
        }
        E::If(_c, t, _f, _l, _h) => ety_of(cx, t),
        E::LetT(n, t, _v, b, _l, _h) => {
            if (t.as_str() == "Vec"
                || (n.as_str() == "$vecl" && oast::str_starts(t.as_str(), "Vec$")))
                && seq_tail_is(b, n)
            {
                let et = vpush_elem_ty(cx, b, n); // (a `$vecl` literal's lit_ty spells a nested vec! as bare `Vec`: `Vec$Vec`)
                if et.len() > 0 && et.as_str() != "int" {
                    let mut r = String::from("Vec$");
                    r.push_str(et.as_str());
                    return r;
                }
            } // VEC-LIT-ELEM: `vec![D(1)]` expands to `let $vecl: Vec = vec_new(); vec_push($vecl, D(1)); $vecl` (lit_ty cannot type D(1))
            String::from(t.as_str())
        }
        E::Seq(_a, b, _l, _h) => ety_of(cx, b),
        E::Call(fnm, cg, _a, _b)
            if (fnm.as_str() == "vec_get" || fnm.as_str() == "seq_get") && cg.len() == 2 =>
        {
            let vt = ety_of(cx, &cg[0]);
            let el = vec_elem_of(&vt);
            if el.len() > 0 {
                el
            } else {
                arr_elem_of(&vt)
            }
        } // ARR-ELEM: `for s in ["a", "b"]` binds a str, not an int
        E::Call(fnm0, cargs, _a, _b) => {
            {
                // LOCAL-SHADOWS-FN: a call through a LOCAL binding (a `f: Fn` param, a closure value) is not the global fn of
                // that name -- the prelude's `f(x) == 1` (ofilter, vany, ...) typed as the USER fn `f<A>`'s return `Pt$A`
                let si = slot_of(cx, fnm0) as usize;
                if si < cx.names.len() && cx.names[si].as_str() == fnm0.as_str() {
                    // FN-BOUND-RET (W7): a local `f: F` with `F: Fn*(..) -> R` answers R (it answered nothing: a String result printed
                    // as an address)
                    let g = sty_of(cx, fnm0);
                    let mut key = String::from("@fnret:");
                    key.push_str(cx.curfn.as_str());
                    key.push('#');
                    key.push_str(fnm0.as_str());
                    key.push('=');
                    let mut q = 0;
                    while q < cx.fpays.len() {
                        if g.len() > 0 && oast::str_starts(cx.fpays[q].as_str(), key.as_str()) {
                            return String::from(&cx.fpays[q][key.len()..]);
                        }
                        q = q + 1;
                    }
                    return String::new();
                }
            }
            let fnm = &self_fix(cx, fnm0);
            let rt = oast::sresolve(&cx.sdefs, &fret_of(cx, fnm));
            let gs = if oast::str_starts(rt.as_str(), "tup")
                || (rt.len() == 1 && rt.as_bytes()[0] >= 65 && rt.as_bytes()[0] <= 90)
            {
                gen_ret_subst(cx, fidx_of(cx, fnm), rt.as_str(), cargs)
            } else {
                String::from("")
            };
            if gs.len() > 0 {
                gs
            } else if rt.len() > 0 && is_known_ty(cx, &rt) {
                rt
            } else if oast::str_starts(rt.as_str(), "tup")
                && oparse::tup_elems(rt.as_str()).len() == 0
                && rt.len() > 3
            {
                rt // TUP-RET-ARITY (W7): a NESTED tuple return is named by arity only (`tup2`); answering nothing made `x == y` an
                   // integer compare of two addresses (false). As `tup2`, AGG-EQ refuses the untyped tuple instead
            } else {
                let fi = fidx_of(cx, fnm);
                let pp = pty_pos(cx, fi, &rt);
                if pp >= 0 && (pp as usize) < cargs.len() {
                    ety_of(cx, &cargs[pp as usize])
                } else {
                    let mut vt = String::from("Vec$");
                    vt.push_str(&rt);
                    let pv = pty_pos(cx, fi, &vt);
                    if pv >= 0 && (pv as usize) < cargs.len() {
                        let at = ety_of(cx, &cargs[pv as usize]);
                        let ve = vec_elem_of(&at);
                        if ve.len() > 0 {
                            ve
                        } else {
                            arr_elem_of(&at)
                        }
                    } else {
                        String::from("")
                    }
                }
            }
        } // GEN-RET-ELEM: `fn f<T>(v: &[T]) -> T` answers the element type of its slice argument
        E::Field(o, f, _a, _b) => {
            let os = ety_of(cx, o);
            if os.len() == 0 {
                String::from("")
            } else {
                let os = gen_base_of(cx, os); // GEN-INST-PASS: `pick::<T, U>(..) -> Triple<U>` typed `Triple$U`, which missed the binding's GEN-INST
                let ft = oast::sfield_ty(&cx.sdefs, &os, f);
                // G1 G-P3: inside an instance of a generic impl's method, a field declared with the type parameter reads the
                // instance's concrete type (`self.v: T` in `W$show$int` is int)
                let ft = if cx.gsub.len() > 0 {
                    gsub_text(cx, ft.as_str())
                } else {
                    ft
                };
                // GEN-FIELD: a field declared with the struct's type parameter (`v: T`) takes the binding's GEN-INST
                // argument (`let t = Tag { v: String::from("x") }` -> T = str); it typed as `T` -> int, and a String
                // field printed as its address
                if let E::Var(n, _vl, _vh) = o.as_ref() {
                    // GEN-MULTI (R12): a MULTI-parameter instance (`Pair:A=int,B=char`) -- ginst_vt reads one parameter only, so a
                    // `b: B` field typed as the bare `B` (int): `p.b` of `Pair { a: 1, b: 'x' }` printed 120 (WRONG)
                    let mp = ginst_param(cx, n, os.as_str(), ft.as_str());
                    if mp.len() > 0 {
                        return mp;
                    }
                    if n.as_str() != "self" && multi_param_of(cx, os.as_str(), ft.as_str()) {
                        // (not `self`: inside the generic method body a field read only MOVES the value -- refusing it refused
                        // every generic multi-parameter method)
                        // GEN-MULTI-UNRES: a multi-parameter field with NO recorded instance (`let q = p.swap()` -> Pair<B, A>) is
                        // unknown -- typing it as the bare parameter (int) printed a char field as 120. The $proj$ marker is refused.
                        return String::from("$proj$unresolved|");
                    }
                }
                if let E::Var(n, _vl, _vh) = o.as_ref() {
                    let g = ginst_vt(cx, n, os.as_str());
                    if g.as_str() != os.as_str() {
                        let mut si = 0;
                        while si < cx.sdefs.len() {
                            if cx.sdefs[si].name.as_str() == os.as_str() {
                                return struct_tsub(cx, si, g.as_str(), ft.as_str());
                            }
                            si = si + 1;
                        }
                    }
                }
                ft
            }
        }
        _ => String::from(""),
    }
}
// ETY-ONCE (C1): ety_of of a METHOD CALL with the receiver type computed ONCE (rt) -- the arms below are ety_of's own, with
// every `ety_of(cx, r)` replaced by rt. Its MethodCall guards each re-typed the receiver, so a chain of n calls cost k^n
// (six chained `.map`s exhausted 20+ GB: iterators/invalid-iterator-chain was killed).
fn ety_mcall(cx: &LCx, e: &E, rt: &str) -> String {
    match e {
        E::Var(n, _a, _b) => sty_of(cx, n),
        E::StructLit(sn, _f, _a, _b) => String::from(sn.as_str()),
        E::EnumLit(en, _v, _p, _a, _b) => String::from(en.as_str()),
        E::StrLit(_v, _l, _h) => String::from("str"),
        // an integer literal's type was left UNKNOWN here (the engine reads "" as int anyway), which was
        // invisible until a tuple's name had to be minted from its elements: one unknown element made the
        // whole name fall back to the bare arity
        E::Int(_v, _l, _h) => String::from("int"),
        E::Bool(_v, _l, _h) => String::from("bool"),
        E::Not(x, _l, _h) => {
            let tx = ety_of(cx, x);
            if tx.as_str() == "int" || oast::is_int_width(&tx) {
                if oast::is_int_width(&tx) {
                    tx
                } else {
                    String::from("int")
                }
            } else {
                String::from("bool")
            }
        }
        E::RetE(x, _l, _h) => ety_of(cx, x),
        // an overloaded operator's result type is its method's return type, so `(p + q).x` resolves
        E::Bin(o, a, b2, _l, _h)
            if (ety_of(cx, a).as_str() == "f64" || ety_of(cx, b2).as_str() == "f64")
                && arith_op(o) =>
        {
            String::from("f64")
        }
        E::Bin(o, _a, b2, _l, _h) if o.as_str() == "^" && ety_of(cx, b2).as_str() == "f64" => {
            String::from("f64")
        }
        E::Bin(o, _a, _b, _l, _h) if cmp_op(o) => String::from("bool"),
        // BIN-W64 (R2a P4): integer arithmetic with an operand of a declared 64-bit width has that width (rustc: `(half / c) << 1`
        // with c: i64 is i64) -- it answered the width-less "int", so `q.wrapping_mul(c)` on such a q was refused as unknown-width
        // (`int$wrapping_mul` never defined). A shift takes the LEFT operand's width. Narrower widths are left to P-WIDTH.
        E::Bin(o, a, b2, _l, _h)
            if (arith_op(o)
                || o.as_str() == "&"
                || o.as_str() == "|"
                || o.as_str() == "^"
                || o.as_str() == "<<"
                || o.as_str() == ">>")
                && is_w64(bin_w64_ty(cx, o, a, b2).as_str()) =>
        {
            bin_w64_ty(cx, o, a, b2)
        }
        E::Bin(o, a, _b, _l, _h)
            if (o.as_str() == "&" || o.as_str() == "|" || o.as_str() == "^")
                && ety_of(cx, a).as_str() == "bool" =>
        {
            String::from("bool")
        } // BOOL-BITOPS
        E::MethodCall(r, m, _a2, _l, _h)
            if (m.as_str() == "join" || m.as_str() == "concat") && {
                let t = String::from(rt);
                t.as_str() == "Vec$str" || t.as_str() == "arr$str"
            } =>
        {
            String::from("str")
        } // STR-JOIN result type
        E::MethodCall(r, m, a2, _l, _h)
            if oast::str_starts(String::from(rt).as_str(), "HashMap")
                && (m.as_str() == "iter" || m.as_str() == "into_iter")
                && a2.len() == 0 =>
        {
            if String::from(rt).as_str() == "HashMap$str" {
                String::from("Vec$tup2$str$int")
            } else {
                String::from("Vec$tup2$int$int")
            }
        } // HM-ITER
        E::MethodCall(r, m, a2, _l, _h)
            if oast::str_starts(String::from(rt).as_str(), "HashMap")
                && m.as_str() == "keys"
                && a2.len() == 0 =>
        {
            if String::from(rt).as_str() == "HashMap$str" {
                String::from("Vec$str")
            } else {
                String::from("Vec$int")
            }
        } // HM-KEYS
        E::MethodCall(r, m, a2, _l, _h)
            if oast::str_starts(String::from(rt).as_str(), "HashMap")
                && m.as_str() == "is_empty"
                && a2.len() == 0 =>
        {
            String::from("bool")
        } // HM-KEYS
        E::MethodCall(r, m, _a2, _l, _h)
            if oast::str_starts(String::from(rt).as_str(), "HashMap")
                && (m.as_str() == "get"
                    || m.as_str() == "remove"
                    || m.as_str() == "contains_key"
                    || m.as_str() == "len") =>
        {
            if m.as_str() == "get" || m.as_str() == "remove" {
                String::from("Option")
            } else if m.as_str() == "contains_key" {
                String::from("bool")
            } else {
                String::from("int")
            }
        } // HASHMAP method types
        E::MethodCall(r, m, _a2, _l, _h)
            if m.as_str() == "to_ascii_lowercase"
                || m.as_str() == "to_ascii_uppercase"
                || m.as_str() == "wrapping_neg"
                || m.as_str() == "rem_euclid"
                || m.as_str() == "div_euclid"
                || m.as_str() == "signum"
                || m.as_str() == "wrapping_shl"
                || m.as_str() == "wrapping_shr" =>
        {
            String::from(rt)
        } // INT-METHODS keep the receiver type
        // C-ETYARITH: arithmetic over integers ANSWERS an integer. ety_of used to fall through to "" for
        // it, so any rule that needed the kind of `a-1` was blind -- which is why `!(a-1)` reached the
        // bool lowering and computed a comparison with zero (C-BITNOT, rungs 44-45).
        // BITOP-WIDTH: `&` `|` `^` cannot overflow, so the result keeps its typed operand's width (`0b1010_u8 | 0x0F` is u8);
        // collapsing it to "int" lost the width that count_ones / leading_zeros / `!` depend on. Mixed widths stay "int".
        E::Bin(o, a, b2, _l, _h)
            if (o.as_str() == "&" || o.as_str() == "|" || o.as_str() == "^") && {
                let ta = ety_of(cx, a);
                let tb = ety_of(cx, b2);
                (oast::is_int_width(&ta) && (tb.as_str() == "int" || tb.as_str() == ta.as_str()))
                    || (oast::is_int_width(&tb) && ta.as_str() == "int")
            } =>
        {
            let ta = ety_of(cx, a);
            if oast::is_int_width(&ta) {
                ta
            } else {
                ety_of(cx, b2)
            }
        }
        E::Bin(o, a, b2, _l, _h)
            if arith_op(o) && {
                let ta = ety_of(cx, a);
                let tb = ety_of(cx, b2);
                ta.as_str() == "int"
                    || oast::is_int_width(&ta)
                    || tb.as_str() == "int"
                    || oast::is_int_width(&tb)
            } =>
        {
            // C-UPRINT: an arithmetic result over a 64-bit UNSIGNED operand is itself u64/usize, so `{}`
            // prints it unsigned (u64::MAX - 1 is 18446744073709551614, not -2). Narrower widths + signed
            // stay "int" (they fit a positive i64 or are genuinely signed) -- minimal, no tag-range touch.
            let ta = ety_of(cx, a);
            let tb = ety_of(cx, b2);
            if ta.as_str() == "u64" || tb.as_str() == "u64" {
                String::from("u64")
            } else if ta.as_str() == "usize" || tb.as_str() == "usize" {
                String::from("usize")
            } else {
                String::from("int")
            }
        }
        E::Bin(o, a, _b, _l, _h) => {
            let at = ety_of(cx, a);
            let m = op_method(o);
            if at.len() > 0 && m.len() > 0 && !prim_ty(at.as_str()) {
                let mut mn = String::from(at.as_str());
                mn.push('$');
                mn.push_str(&m);
                if fidx_of(cx, &mn) >= 0 {
                    let r = oast::sresolve(&cx.sdefs, &fret_of(cx, &mn));
                    if r.len() > 0 && is_known_ty(cx, &r) {
                        return r;
                    }
                }
            }
            String::from("")
        }
        E::Try(_x, _l, _h) => String::from(""),
        E::Ref(x, _m, _l, _h) => ety_of(cx, x),
        E::Cast(_x, t, _l, _h) => String::from(t.as_str()),
        E::ClosRef(n, _c, _l, _h) => {
            let mut r = String::from("clos$");
            r.push_str(n.as_str());
            r
        }
        E::TupLit(p, _l, _h) => {
            if p.len() == 0 {
                String::from("unit")
            } else {
                let mut es: Vec<String> = Vec::new();
                let mut k = 0;
                while k < p.len() {
                    es.push(ety_of(cx, &p[k]));
                    k = k + 1;
                }
                oparse::tup_name(p.len() as u32, &es)
            }
        }
        E::TupIdx(b, ix, _l, _h) => {
            let bt = ety_of(cx, b);
            // a tuple STRUCT field has a declared type; a plain tuple does not
            if bt.len() > 0 {
                let t = oast::sfield_ty(&cx.sdefs, &bt, oast::u32s2(*ix).as_str());
                if t.len() > 0 {
                    return t;
                }
            }
            // a plain tuple's element type now comes from its NAME
            {
                let es = oparse::tup_elems(&bt);
                let r = oast::tix_resolve(*ix, es.len()); // TUP-REST-END: a from-the-end slot resolves here too
                if r >= 0 {
                    return String::from(es[r as usize].as_str());
                }
            }
            String::from("")
        }
        E::MethodCall(r, m, a2, _l, _h)
            if m.as_str() == "clone"
                && a2.len() == 0
                && (oast::eidx(&cx.edefs, &String::from(rt)) >= 0
                    || oast::sfields(&cx.sdefs, &String::from(rt)) > 0) =>
        {
            String::from(rt)
        } // CLONE-STRUCT: a struct clone keeps the type
        // C-STRASRESULT: as_str, to_string and to_owned are the IDENTITY in value but their TYPE is
        // always str, whatever the receiver. ety_of was answering the receiver's type, so a String built
        // from an unnameable receiver came out unnameable itself -- and String::from is erased at parse
        // time (oparse maps it to the identity), so nothing downstream restored the name. That chain is
        // how six push_str calls in oparse reached the emitter as `$push_str`: trn unnamed, then
        // trn.as_str() unnamed, then dn unnamed.
        E::MethodCall(r, m, _a2, _l, _h) if gen_ret(cx, r, m).len() > 0 => gen_ret(cx, r, m), // GEN-INST: a generic impl method returning T on a recorded instance
        E::MethodCall(r, m, _a2, _l, _h)
            if dyn_trait_of(&String::from(rt)).len() > 0
                && dyn_ret(cx, &dyn_trait_of(&String::from(rt)), m).len() > 0 =>
        {
            dyn_ret(cx, &dyn_trait_of(&String::from(rt)), m)
        } // DYN-RET: a dyn call answers its implementations' declared return type
        E::MethodCall(r, m, a2, _l, _h)
            if (m.as_str() == "get" || m.as_str() == "borrow" || m.as_str() == "borrow_mut")
                && a2.len() == 0 =>
        {
            let vt = String::from(rt);
            vec_elem_of(&vt)
        } // C-CELL (+ R4 REFCELL)
        E::MethodCall(_r, m, a2, _l, _h) if m.as_str() == "split" && a2.len() == 1 => {
            String::from("Vec$str")
        }
        E::MethodCall(_r, m, a2, _l, _h) if m.as_str() == "split_whitespace" && a2.len() == 0 => {
            String::from("Vec$str")
        } // SPLIT-WS
        E::MethodCall(_r, m, a2, _l, _h) if m.as_str() == "collect_string" && a2.len() == 0 => {
            String::from("str")
        } // COLLECT-TURBO
        E::MethodCall(r, m, a2, _l, _h)
            if (m.as_str() == "unwrap_or" || m.as_str() == "unwrap_or_default")
                && a2.len() == 1
                && {
                    let e0 = String::from(rt);
                    e0.as_str() == "Option" || e0.as_str() == "Result"
                }
                && ety_of(cx, &a2[0]).len() > 0
                && ety_of(cx, &a2[0]).as_str() != "int" =>
        {
            ety_of(cx, &a2[0])
        } // UNWRAP-OR-TY: the default has the payload type
        E::MethodCall(_r, m, a2, _l, _h)
            if m.as_str() == "fold" && a2.len() == 2 && ety_of(cx, &a2[0]).len() > 0 =>
        {
            ety_of(cx, &a2[0])
        } // FOLD-TY: fold answers its accumulator's type (a String accumulator printed as an address)
        E::MethodCall(r, m, a2, _l, _h)
            if (m.as_str() == "windows" || m.as_str() == "chunks") && a2.len() == 1 =>
        {
            let mut t = String::from("Vec$");
            t.push_str(&String::from(rt));
            t
        } // SLICE-WIN: a Vec of sub-slices
        E::MethodCall(_r, m, a2, _l, _h) if m.as_str() == "char_indices" && a2.len() == 0 => {
            String::from("Vec$tup2$int$char")
        } // CHAR-IDX
        E::MethodCall(_r, m, a2, _l, _h) if m.as_str() == "parse" && a2.len() == 0 => {
            String::from("Result")
        } // PARSE-UNNAMED: str::parse answers a Result
        E::MethodCall(_r, m, a2, _l, _h)
            if (m.as_str() == "as_str"
                || m.as_str() == "to_string"
                || m.as_str() == "to_owned"
                || (m.as_str() == "into" && ety_of(cx, _r).as_str() == "str"))
                && a2.len() == 0 =>
        {
            String::from("str")
        }
        E::MethodCall(r, m, a2, _l, _h)
            if (m.as_str() == "as_ref" || m.as_str() == "as_mut") && a2.len() == 0 =>
        {
            String::from(rt)
        }
        E::MethodCall(r, m, a2, _l, _h)
            if (m.as_str() == "get" || m.as_str() == "remove" || m.as_str() == "swap_remove")
                && a2.len() == 1
                && vec_elem_of(&String::from(rt)).len() > 0 =>
        {
            vec_elem_of(&String::from(rt))
        }
        E::MethodCall(r, m, a2, _l, _h)
            if (m.as_str() == "wrapping_mul"
                || m.as_str() == "wrapping_add"
                || m.as_str() == "wrapping_sub")
                && a2.len() == 1 =>
        {
            String::from(rt)
        }
        E::MethodCall(r, m, _a2, _l, _h)
            if m.as_str() == "chars" && String::from(rt).as_str() == "str" =>
        {
            String::from("Vec$char")
        } // STR-CHARS-UTF8
        E::MethodCall(r, m, _a2, _l, _h)
            if (m.as_str() == "as_bytes" || m.as_str() == "bytes")
                && String::from(rt).as_str() == "str" =>
        {
            String::from("bstr")
        }
        E::MethodCall(r, m, a2, _l, _h)
            if (m.as_str() == "is_nan"
                || m.as_str() == "is_infinite"
                || m.as_str() == "is_finite")
                && a2.len() == 0
                && String::from(rt).as_str() == "f64" =>
        {
            String::from("bool")
        } // F64-PREDS
        E::MethodCall(r, m, a2, _l, _h)
            if (m.as_str() == "floor"
                || m.as_str() == "ceil"
                || m.as_str() == "round"
                || m.as_str() == "abs"
                || m.as_str() == "sqrt"
                || m.as_str() == "signum")
                && a2.len() == 0
                && String::from(rt).as_str() == "f64" =>
        {
            String::from("f64")
        } // D1.2
        E::MethodCall(r, m, a2, _l, _h)
            if m.as_str() == "powi" && a2.len() == 1 && String::from(rt).as_str() == "f64" =>
        {
            String::from("f64")
        } // D1.4
        E::MethodCall(r, m, a2, _l, _h)
            if String::from(rt).as_str() == "f64" && {
                let bm = builtin_method("f64", m.as_str(), a2.len());
                oast::str_starts(bm.as_str(), "f64m_") || oast::str_starts(bm.as_str(), "f64m2_")
            } =>
        {
            String::from("f64")
        } // F64-LIBM
        E::MethodCall(r, m, a2, _l, _h)
            if (m.as_str() == "max" || m.as_str() == "min")
                && a2.len() == 1
                && String::from(rt).as_str() == "f64" =>
        {
            String::from("f64")
        } // D1.6
        E::MethodCall(r, m, _a2, _l, _h)
            if m.as_str() == "count" && String::from(rt).as_str() == "bstr" =>
        {
            String::from("int")
        }
        E::MethodCall(_r, m, _a2, _l, _h)
            if m.as_str() == "to_string" || m.as_str() == "to_owned" =>
        {
            String::from("str")
        }
        E::MethodCall(r, m, _a2, _l, _h) if m.as_str() == "get" => {
            let vt = String::from(rt);
            vec_elem_of(&vt)
        }
        // a BUILT-IN method's result type, by the same rule the typechecker uses: an adaptor answers the
        // same sequence, a predicate a bool, the rest an int
        E::MethodCall(r, m, _a2, _l2, _h2)
            if (m.as_str() == "unwrap"
                || m.as_str() == "expect"
                || m.as_str() == "unwrap_or"
                || m.as_str() == "unwrap_or_default"
                || m.as_str() == "unwrap_or_else")
                && unwrap_pay(cx, r).len() > 0 =>
        {
            unwrap_pay(cx, r)
        } // UNWRAP-PAY
        E::MethodCall(r, m, _a2, _l2, _h2)
            if (m.as_str() == "unwrap" || m.as_str() == "expect") && {
                match r.as_ref() {
                    E::MethodCall(v, cm, _va, _vl, _vh) => {
                        (cm.as_str() == "first"
                            || cm.as_str() == "last"
                            || cm.as_str() == "nth"
                            || cm.as_str() == "max"
                            || cm.as_str() == "min"
                            || cm.as_str() == "pop")
                            && vec_elem_of(&ety_of(cx, v)).len() > 0
                    }
                    _ => false,
                }
            } =>
        {
            match r.as_ref() {
                E::MethodCall(v, _cm, _va, _vl, _vh) => vec_elem_of(&ety_of(cx, v)),
                _ => String::from(""),
            }
        } // D4.1 typed-Option chain
        E::MethodCall(r, m, a2, _ab, _bb)
            if builtin_method(&String::from(rt), m, a2.len()).len() > 0 =>
        {
            let rt = String::from(rt);
            if m.as_str() == "checked_div" {
                return String::from("Option");
            } // P-CHECKEDDIV
            if m.as_str() == "checked_add"
                || m.as_str() == "checked_sub"
                || m.as_str() == "checked_mul"
            {
                return String::from("Option");
            } // D3.1
            if m.as_str() == "saturating_add"
                || m.as_str() == "saturating_sub"
                || m.as_str() == "to_le"
                || m.as_str() == "saturating_mul"
            {
                return String::from(rt.as_str());
            } // SAT-OPS: keeps the width
            if m.as_str() == "count_ones"
                || m.as_str() == "leading_zeros"
                || m.as_str() == "trailing_zeros"
            {
                return String::from("int");
            } // D3.2
            if (m.as_str() == "max" || m.as_str() == "min")
                && a2.len() == 1
                && rt.as_str() == "char"
            {
                return String::from("char");
            } // ORD-CHAR: 'a'.min('c') is a char (it printed 97)
            if m.as_str() == "pow"
                || m.as_str() == "abs"
                || (m.as_str() == "max" && a2.len() == 1)
                || (m.as_str() == "min" && a2.len() == 1)
            {
                return String::from("int");
            } // P-INT-METHODS
            if m.as_str() == "is_ascii_digit"
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
                || m.as_str() == "is_ascii_uppercase"
            {
                return String::from("bool");
            } // P-CHAR-PRED
            if oast::pay_base(rt.as_str()).as_str() == "Option" {
                if m.as_str() == "map" || m.as_str() == "and_then" {
                    return String::from("Option");
                }
                if m.as_str() == "ok_or" {
                    return String::from("Result");
                } // P-RES-COMB
                if m.as_str() == "filter" {
                    return String::from("Option");
                } // P-OPT-FILTER
                if m.as_str() == "is_some" || m.as_str() == "is_none" {
                    return String::from("bool");
                }
                if m.as_str() == "copied" || m.as_str() == "cloned" {
                    return String::from("Option");
                } // P-OPT-COPIED
                return String::from("int");
            }
            if oast::pay_base(rt.as_str()).as_str() == "Result" {
                if m.as_str() == "is_ok" || m.as_str() == "is_err" {
                    return String::from("bool");
                }
                if m.as_str() == "ok" {
                    return String::from("Option");
                } // P-RES-COMB
                if m.as_str() == "map" || m.as_str() == "map_err" {
                    return String::from("Result");
                }
                return String::from("int");
            }
            if rt.as_str() == "str" {
                if m.as_str() == "push" || m.as_str() == "push_str" {
                    return String::from("int");
                } // C-PUSHUNIT: () here
                if m.as_str() == "parse" {
                    return String::from("Result");
                }
                if m.as_str() == "find" || m.as_str() == "rfind" {
                    return String::from("Option");
                }
                if m.as_str() == "is_empty"
                    || m.as_str() == "contains"
                    || m.as_str() == "starts_with"
                    || m.as_str() == "ends_with"
                {
                    return String::from("bool");
                }
                if m.as_str() == "len" {
                    return String::from("int");
                }
                return String::from("str");
            }
            if (m.as_str() == "iter" || m.as_str() == "into_iter")
                && oast::str_starts(rt.as_str(), "arr$")
            {
                let mut vt = String::from("Vec$");
                vt.push_str(&rt[4..]);
                return vt;
            } // ARR-METH
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
                || m.as_str() == "filter"
                || m.as_str() == "rev"
                || m.as_str() == "take"
                || m.as_str() == "skip"
                || m.as_str() == "step_by"
                || m.as_str() == "take_while"
                || m.as_str() == "skip_while"
                || m.as_str() == "chain"
            {
                return rt;
            }
            if m.as_str() == "map" {
                match &a2[0] {
                    E::ClosRef(cn, _c, _cl, _ch) => {
                        let rr = fret_of(cx, cn);
                        if rr.len() > 0 {
                            let mut vt = String::from("Vec$");
                            vt.push_str(rr.as_str());
                            return vt;
                        }
                    }
                    _ => {}
                }
                return rt;
            } // D5
            if m.as_str() == "flat_map" {
                match &a2[0] {
                    E::ClosRef(cn, _c, _cl, _ch) => {
                        let rr = fret_of(cx, cn);
                        if rr.len() > 0 {
                            return rr;
                        }
                    }
                    _ => {}
                }
                return rt;
            } // D5
            if m.as_str() == "partition" {
                let el = vec_elem_of(&rt);
                if el.len() > 0 {
                    let mut vt = String::from("Vec$");
                    vt.push_str(el.as_str());
                    let mut es: Vec<String> = Vec::new();
                    es.push(String::from(vt.as_str()));
                    es.push(vt);
                    return oparse::tup_name(2, &es);
                }
                return rt;
            }
            if m.as_str() == "enumerate" {
                let el = vec_elem_of(&rt);
                if el.len() > 0 && {
                    let eb = el.as_bytes();
                    let mut d = false;
                    let mut z = 0;
                    while z < eb.len() {
                        if eb[z] == 36 {
                            d = true;
                        }
                        z = z + 1;
                    }
                    !d
                } {
                    let mut t = String::from("Vec$tup2$int$");
                    t.push_str(el.as_str());
                    return t;
                }
            } // ENUM-ELEM: (index, element) keeps the element type -- untyped, a char printed as its number
            if m.as_str() == "enumerate" || m.as_str() == "zip" {
                return String::from("Vec$tup2");
            } // P-ITER-ZIP
            if m.as_str() == "binary_search" {
                return String::from("Result$int$int");
            } // VEC-BSEARCH: Ok(index) / Err(insertion point), both usize
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
                return String::from("Option");
            }
            if m.as_str() == "get" {
                return vec_elem_of(&rt);
            }
            // a PREDICATE answers a bool, and `{}` must spell it `true`/`false`
            if m.as_str() == "any"
                || m.as_str() == "all"
                || m.as_str() == "is_empty"
                || m.as_str() == "contains"
            {
                return String::from("bool");
            }
            String::from("int")
        }
        E::MethodCall(r, m, _a2, _a, _b) => {
            let rt = String::from(rt);
            let mut nm = String::from(rt.as_str());
            nm.push_str("$");
            nm.push_str(m);
            if fidx_of(cx, &nm) < 0 {
                let u = unique_method(cx, m);
                if u.len() > 0 {
                    nm = u;
                }
            }
            let r2 = fret_of(cx, &nm);
            // G1 G-P3: a generic impl's method returning `Self` (`Vec$T$tw`) answers the RECEIVER's type
            let r2 = if r2.as_str() == "Self" && rt.len() > 0 {
                String::from(rt.as_str())
            } else {
                r2
            };
            let r2 = gen_meth_ret(cx, &nm, &rt, r2); // G1 G-P3: impl parameters substituted from the receiver
            if r2.len() > 0 && is_known_ty(cx, &r2) {
                r2
            } else {
                String::from("")
            }
        }
        E::Call(fnm, _fa, _a, _b)
            if clos_of(&sty_of(cx, fnm.as_str())).len() > 0
                && fret_of(cx, clos_of(&sty_of(cx, fnm.as_str())).as_str()).len() > 0 =>
        {
            fret_of(cx, clos_of(&sty_of(cx, fnm.as_str())).as_str())
        } // FN-ITEM-RET: a call through a closure-valued local answers the lifted closure's return type (it answered nothing)
        E::Call(fnm, _c2, _a, _b) if fnm.as_str() == "readfile" || fnm.as_str() == "argv_at" => {
            String::from("str")
        }
        E::Call(fnm, cz, _a, _b) if fnm.as_str() == "$zeroof" && cz.len() == 1 => {
            ety_of(cx, &cz[0])
        }
        E::Call(fnm, _cz, _a, _b) if oast::str_starts(fnm.as_str(), "$xmute$") => {
            String::from(&fnm[7..])
        } // R4a XMUTE
        E::Call(fnm, _cz, _a, _b)
            if oast::str_starts(fnm.as_str(), "$sizeof$")
                || oast::str_starts(fnm.as_str(), "$alignof$") =>
        {
            String::from("usize")
        } // R4a LAYOUT
        E::Call(fnm, _cz, _a, _b) if fnm.as_str() == "$sizeofval" => String::from("usize"), // R4a LAYOUT
        E::Call(fnm, cz, _a, _b) if fnm.as_str() == "$alias" && cz.len() == 1 => {
            ety_of(cx, unref(&cz[0]))
        } // R4f CHAN
        E::Call(fnm, _cz, _a, _b) if fnm.as_str() == "$ptrnull" => String::from("int"), // R4b PTR
        E::Call(fnm, cz, _a, _b) if fnm.as_str() == "$chanint" && cz.len() == 1 => {
            ety_of(cx, &cz[0])
        }
        E::Call(fnm, _cz, _a, _b) if zeroty_name(fnm).len() > 0 => zeroty_name(fnm),
        // RC-IDENTITY (R4c): an Rc handle has its referent's type (identity handle)
        E::Call(fnm, cz, _a, _b)
            if (fnm.as_str() == "$rcnew"
                || fnm.as_str() == "$rcclone"
                || fnm.as_str() == "$boxnew"
                || fnm.as_str() == "$cellhandle")
                && cz.len() == 1 =>
        {
            ety_of(cx, unref(&cz[0]))
        }
        E::Call(fnm, cz, _a, _b) if fnm.as_str() == "$rccount" && cz.len() == 1 => {
            String::from("int")
        }
        E::Call(fnm, cz, _a, _b) if fnm.as_str() == "$cellnew" && cz.len() == 1 => {
            let t = ety_of(cx, &cz[0]);
            let mut r = String::from("Vec$");
            r.push_str(t.as_str());
            r
        }
        E::Call(fnm, cz, _a, _b) if fnm.as_str() == "vec_push1" && cz.len() == 2 => {
            let t = ety_of(cx, &cz[1]);
            let mut r = String::from("Vec$");
            r.push_str(t.as_str());
            r
        }
        E::Call(fnm, _cz, _a, _b) if fnm.as_str() == "str_split" => String::from("Vec$str"),
        E::Call(fnm, _cz, _a, _b) if fnm.as_str() == "str_chars_vec" => String::from("Vec$char"), // CHARS-VEC
        E::Call(fnm, _cz, _a, _b) if fnm.as_str() == "str_bytes_vec" => String::from("Vec$u8"), // STR-BYTES
        E::Call(fnm, _c2, _a, _b)
            if fnm.as_str() == "str_dbg"
                || fnm.as_str() == "dbgq_s"
                || fnm.as_str() == "dbgc_s" =>
        {
            String::from("str")
        } // FMT-DBG-STR
        E::Call(fnm, _c2, _a, _b) if fnm.as_str() == "$shovf" || fnm.as_str() == "$ovf1" => {
            String::from("bool")
        } // OVF-SHIFT + OVF-NEGABS
        E::Call(fnm, _c2, _a, _b)
            if fnm.as_str() == "str_cat"
                || fnm.as_str() == "str_sub"
                || fnm.as_str() == "str_of"
                || fnm.as_str() == "str_from_int"
                || fnm.as_str() == "str_copy" =>
        {
            String::from("str")
        }
        E::Call(fnm, _cg, _a, _b) if fnm.as_str() == "vec_new" => String::from("Vec"),
        E::Call(fnm, fa, _a, _b) if fnm.as_str() == "arr_fill" && fa.len() == 2 => {
            let et = ety_of(cx, &fa[1]);
            let mut r = String::from("arr");
            if et.len() > 0 {
                r.push('$');
                r.push_str(et.as_str());
            }
            r
        } // ARR-REP-LOOP: an array of its fill value
        E::Call(fnm, _cg, _a, _b)
            if fnm.as_str() == "f64_parse" || fnm.as_str() == "f64_of_int" =>
        {
            String::from("f64")
        }
        E::Call(fnm, _cg, _a, _b)
            if fnm.as_str() == "f64_snprintf" || fnm.as_str() == "f64_snprintf_e" =>
        {
            String::from("str")
        }
        E::Call(fnm, _cg, _a, _b) if fnm.as_str() == "f64_isnan" => String::from("int"),
        E::Call(fnm, _cg, _a, _b) if fnm.as_str() == "f64_bits" => String::from("int"),
        E::Call(fnm, _cg, _a, _b) if fnm.as_str() == "f64_sqrt" => String::from("f64"), // D1.5
        E::Call(fnm, _cg, _a, _b)
            if oast::str_starts(fnm.as_str(), "f64m_")
                || oast::str_starts(fnm.as_str(), "f64m2_") =>
        {
            String::from("f64")
        } // F64-LIBM
        E::Call(fnm, _cg, _a, _b) if is_gname(fnm) => String::from("int"),
        E::Call(fnm, cg, _a, _b)
            if fnm.as_str() == "seq_slice"
                && cg.len() == 3
                && (ety_of(cx, &cg[0]).as_str() == "str"
                    || ety_of(cx, &cg[0]).as_str() == "bstr") =>
        {
            // BSTR-SLICE (R2a P4): a range slice of a byte string is a byte string (str_sub on the same buffer)
            ety_of(cx, &cg[0])
        }
        E::Call(fnm, cg, _a, _b) if fnm.as_str() == "seq_slice" && cg.len() == 3 => {
            let st = ety_of(cx, &cg[0]);
            // C-ARRSLICE: a slice OF AN ARRAY is an array -- arr_slice answers one -- and without this the
            // slice had no type at all, so `{:?}` on it was refused while the array itself printed.
            if is_arr_ty(&st) {
                let ae = arr_elem_of(&st);
                let mut r = String::from("Vec$");
                if ae.len() > 0 {
                    r.push_str(ae.as_str());
                } else {
                    r.push_str("int");
                }
                return r;
            }
            let el = vec_elem_of(&st);
            if el.len() > 0 {
                let mut r = String::from("Vec$");
                r.push_str(el.as_str());
                r
            } else {
                String::from("Vec")
            }
        }
        // C-LETTAIL: a `let` whose body ENDS in the bound name answers the bound value's type. It used to
        // require the body to BE that name, so a desugar that writes something before handing the binding
        // back -- mem::replace is exactly that shape -- fell through to the unknown type and printed a str
        // as a number. The tail is walked through Seq, where every such desugar puts its statements.
        E::Let(n, v, b, _l, _h) => {
            if seq_tail_is(b, n) {
                let t = ety_of(cx, v);
                if t.as_str() == "Vec" {
                    let et = vpush_elem_ty(cx, b, n);
                    if et.len() > 0 && et.as_str() != "int" {
                        let mut r = String::from("Vec$");
                        r.push_str(et.as_str());
                        return r;
                    }
                } // VEC-LIT-ELEM: `vec![D(1)]` is `let $vecl = vec_new(); vec_push($vecl, D(1)); $vecl` -- name the element
                return t;
            }
            ety_of(cx, b)
        }
        // C-ARRELEM: an array's type name carries its ELEMENT, the way a vector's does. It said so only
        // for strings, and the bare `arr` meant "array of something" -- so the Debug printer, which picks
        // by element, showed an array of bools as [1, 0] and an array of arrays as its inner POINTERS.
        E::ArrLit(x, _l, _h) => {
            if x.len() == 0 {
                return String::from("arr");
            }
            let e0 = ety_of(cx, &x[0]);
            if e0.len() == 0 {
                return String::from("arr");
            }
            let mut same = true;
            let mut k = 1;
            while k < x.len() {
                if ety_of(cx, &x[k]).as_str() != e0.as_str() {
                    same = false;
                }
                k = k + 1;
            }
            if !same {
                return String::from("arr");
            }
            let mut r = String::from("arr$");
            r.push_str(e0.as_str());
            r
        }
        E::Index(b, _ix, _l, _h) => {
            let bt = ety_of(cx, b);
            if bt.as_str() == "bstr" {
                return String::from("int");
            }
            if oast::str_starts(bt.as_str(), "HashMap") {
                return String::from("int");
            }
            let el = elem_of(&bt);
            if el.len() > 0 {
                el
            } else {
                String::from("")
            }
        }
        E::Match(sc, arms, _l, _h) => {
            // W6 MATCH-TY: all arms have ONE type in Rust -- the first arm that can be typed answers. Arm 0 alone was
            // `Var(v)` for `Some(v) => v`, a binding not in scope here, so the match was untyped and println printed a
            // String pointer as an int. A body that IS its own Some(v) binding is the scrutinee's payload type.
            // MATCH-BIND-PAY guard: a str payload returned through its binding keeps the OLD typing (refused downstream) --
            // typed str, `.len()` on the String value reads its bytes as a length (IC-7 / W6 string-payload-len: WRONG)
            let strbind = match_returns_str_bind(cx, sc, arms);
            let mut ctlt = String::new(); // the first control arm's type: the OLD answer, kept when no other arm types the match
            let mut k = 0;
            while k < arms.len() {
                // MATCH-BIND-PAY (R2a P4): a control-flow arm (`_ => return false`, break, continue) never answers the match type --
                // it typed `let xs = match e { E::StructLit(_, xs, ..) => xs, .., _ => return false }` as bool (ext_any: bool$len).
                // (panic arms are NOT skipped: ARM-DIVERGE showed their type carries other shapes.)
                if !strbind && ctl_arm(&arms[k].body) {
                    if ctlt.len() == 0 {
                        ctlt = ety_of(cx, &arms[k].body);
                    }
                    k = k + 1;
                    continue;
                }
                // TUP-BIND-PAY (R2a P4): an arm whose body is a TUPLE of its own bindings (`E::C(cn, caps, ..) => (cn, caps)`)
                // answers the tuple of their payload types -- untyped, `let (cn, caps) = match ..` left caps untyped and
                // `caps[0]` read it as an ARRAY (__arr_get) on a Vec handle (ourc compiled by ourc: SIGSEGV in cap_drops)
                if let E::TupLit(xs0, _tl0, _th0) = &arms[k].body {
                    let mut tys0: Vec<String> = Vec::new();
                    let mut all0 = xs0.len() > 0;
                    let mut xi0 = 0;
                    while xi0 < xs0.len() {
                        let t = match &xs0[xi0] {
                            E::Var(xn, _a, _b) => bind_payload_ty(cx, sc, &arms[k], xn.as_str()),
                            _ => String::new(),
                        };
                        if t.len() == 0 {
                            all0 = false;
                        }
                        tys0.push(t);
                        xi0 = xi0 + 1;
                    }
                    if all0 {
                        return oparse::tup_name(xs0.len() as u32, &tys0);
                    }
                }
                // ... and an arm that returns one of its own bindings answers that payload's declared type (generalizes the
                // `Some(v) => v` case below to any enum variant / position)
                if let E::Var(bv0, _bl0, _bh0) = &arms[k].body {
                    let mut bi0 = 0;
                    while bi0 < arms[k].binds.len() {
                        if arms[k].binds[bi0].as_str() == bv0.as_str() {
                            // the scrutinee is often P-PAT's synthetic `$msN` (not in scope here: untyped) -- then the enum is the
                            // ONE that declares this variant (ambiguous -> no answer)
                            let st0 = {
                                let t = ety_of(cx, sc);
                                if oast::eidx(&cx.edefs, t.as_str()) >= 0 {
                                    t
                                } else {
                                    sole_enum_of_variant(&cx.edefs, arms[k].variant.as_str())
                                }
                            };
                            if st0.len() > 0 && oast::eidx(&cx.edefs, st0.as_str()) >= 0 {
                                let pt0 = oast::vpayload_ty(
                                    &cx.edefs,
                                    st0.as_str(),
                                    arms[k].variant.as_str(),
                                    bi0,
                                );
                                if pt0.len() > 0
                                    && pt0.as_str() != "str"
                                    && pt0.as_str() != "String"
                                {
                                    return pt0;
                                }
                            }
                        }
                        bi0 = bi0 + 1;
                    }
                }
                let t = ety_of(cx, &arms[k].body);
                if t.len() > 0 {
                    return t;
                }
                {
                    // LETELSE-PAYLOAD-TY (W7): a body that IS one of its own bindings of a USER enum variant (`V2(b) => b`, which
                    // is what `let V::V2(b) = v else ..` rewrites to) is that variant's payload type at the binding's position --
                    // untyped, a bool printed as 1
                    let pt = arm_bind_payload_ty(cx, sc, &arms[k]);
                    if pt.len() > 0 {
                        return pt;
                    }
                }
                if arms[k].binds.len() == 1 && arms[k].variant.as_str() == "Some" {
                    if let E::Var(bv, _bl, _bh) = &arms[k].body {
                        if bv.as_str() == arms[k].binds[0].as_str() {
                            let st = ety_of(cx, sc);
                            if oast::str_starts(st.as_str(), "Option$") {
                                let b = st.as_bytes();
                                let mut p = String::new();
                                let mut j = 7;
                                while j < b.len() {
                                    p.push(b[j] as char);
                                    j = j + 1;
                                }
                                return p;
                            }
                        }
                    }
                }
                k = k + 1;
            }
            ctlt // MATCH-BIND-PAY: no other arm typed the match -> the control arm's (old) answer
        }
        E::If(_c, t, _f, _l, _h) => ety_of(cx, t),
        E::LetT(n, t, _v, b, _l, _h) => {
            if (t.as_str() == "Vec"
                || (n.as_str() == "$vecl" && oast::str_starts(t.as_str(), "Vec$")))
                && seq_tail_is(b, n)
            {
                let et = vpush_elem_ty(cx, b, n); // (a `$vecl` literal's lit_ty spells a nested vec! as bare `Vec`: `Vec$Vec`)
                if et.len() > 0 && et.as_str() != "int" {
                    let mut r = String::from("Vec$");
                    r.push_str(et.as_str());
                    return r;
                }
            } // VEC-LIT-ELEM: `vec![D(1)]` expands to `let $vecl: Vec = vec_new(); vec_push($vecl, D(1)); $vecl` (lit_ty cannot type D(1))
            String::from(t.as_str())
        }
        E::Seq(_a, b, _l, _h) => ety_of(cx, b),
        E::Call(fnm, cg, _a, _b)
            if (fnm.as_str() == "vec_get" || fnm.as_str() == "seq_get") && cg.len() == 2 =>
        {
            let vt = ety_of(cx, &cg[0]);
            let el = vec_elem_of(&vt);
            if el.len() > 0 {
                el
            } else {
                arr_elem_of(&vt)
            }
        } // ARR-ELEM: `for s in ["a", "b"]` binds a str, not an int
        E::Call(fnm0, cargs, _a, _b) => {
            let fnm = &self_fix(cx, fnm0);
            let rt = oast::sresolve(&cx.sdefs, &fret_of(cx, fnm));
            let gs = if oast::str_starts(rt.as_str(), "tup")
                || (rt.len() == 1 && rt.as_bytes()[0] >= 65 && rt.as_bytes()[0] <= 90)
            {
                gen_ret_subst(cx, fidx_of(cx, fnm), rt.as_str(), cargs)
            } else {
                String::from("")
            };
            if gs.len() > 0 {
                gs
            } else if rt.len() > 0 && is_known_ty(cx, &rt) {
                rt
            } else if oast::str_starts(rt.as_str(), "tup")
                && oparse::tup_elems(rt.as_str()).len() == 0
                && rt.len() > 3
            {
                rt // TUP-RET-ARITY (W7): a NESTED tuple return is named by arity only (`tup2`); answering nothing made `x == y` an
                   // integer compare of two addresses (false). As `tup2`, AGG-EQ refuses the untyped tuple instead
            } else {
                let fi = fidx_of(cx, fnm);
                let pp = pty_pos(cx, fi, &rt);
                if pp >= 0 && (pp as usize) < cargs.len() {
                    ety_of(cx, &cargs[pp as usize])
                } else {
                    let mut vt = String::from("Vec$");
                    vt.push_str(&rt);
                    let pv = pty_pos(cx, fi, &vt);
                    if pv >= 0 && (pv as usize) < cargs.len() {
                        let at = ety_of(cx, &cargs[pv as usize]);
                        let ve = vec_elem_of(&at);
                        if ve.len() > 0 {
                            ve
                        } else {
                            arr_elem_of(&at)
                        }
                    } else {
                        String::from("")
                    }
                }
            }
        } // GEN-RET-ELEM: `fn f<T>(v: &[T]) -> T` answers the element type of its slice argument
        E::Field(o, f, _a, _b) => {
            let os = ety_of(cx, o);
            if os.len() == 0 {
                String::from("")
            } else {
                let os = gen_base_of(cx, os); // GEN-INST-PASS: `pick::<T, U>(..) -> Triple<U>` typed `Triple$U`, which missed the binding's GEN-INST
                let ft = oast::sfield_ty(&cx.sdefs, &os, f);
                // G1 G-P3: inside an instance of a generic impl's method, a field declared with the type parameter reads the
                // instance's concrete type (`self.v: T` in `W$show$int` is int)
                let ft = if cx.gsub.len() > 0 {
                    gsub_text(cx, ft.as_str())
                } else {
                    ft
                };
                // GEN-FIELD: a field declared with the struct's type parameter (`v: T`) takes the binding's GEN-INST
                // argument (`let t = Tag { v: String::from("x") }` -> T = str); it typed as `T` -> int, and a String
                // field printed as its address
                if let E::Var(n, _vl, _vh) = o.as_ref() {
                    // GEN-MULTI (R12): a MULTI-parameter instance (`Pair:A=int,B=char`) -- ginst_vt reads one parameter only, so a
                    // `b: B` field typed as the bare `B` (int): `p.b` of `Pair { a: 1, b: 'x' }` printed 120 (WRONG)
                    let mp = ginst_param(cx, n, os.as_str(), ft.as_str());
                    if mp.len() > 0 {
                        return mp;
                    }
                    if n.as_str() != "self" && multi_param_of(cx, os.as_str(), ft.as_str()) {
                        // (not `self`: inside the generic method body a field read only MOVES the value -- refusing it refused
                        // every generic multi-parameter method)
                        // GEN-MULTI-UNRES: a multi-parameter field with NO recorded instance (`let q = p.swap()` -> Pair<B, A>) is
                        // unknown -- typing it as the bare parameter (int) printed a char field as 120. The $proj$ marker is refused.
                        return String::from("$proj$unresolved|");
                    }
                }
                if let E::Var(n, _vl, _vh) = o.as_ref() {
                    let g = ginst_vt(cx, n, os.as_str());
                    if g.as_str() != os.as_str() {
                        let mut si = 0;
                        while si < cx.sdefs.len() {
                            if cx.sdefs[si].name.as_str() == os.as_str() {
                                return struct_tsub(cx, si, g.as_str(), ft.as_str());
                            }
                            si = si + 1;
                        }
                    }
                }
                ft
            }
        }
        _ => String::from(""),
    }
}
fn sty_of(cx: &LCx, n: &str) -> String {
    let mut i = cx.names.len();
    while i > 0 {
        i = i - 1;
        if cx.names[i].as_str() == n {
            return String::from(cx.stypes[i].as_str());
        }
    }
    String::from("")
}
pub fn fresh(cx: &mut LCx) -> u32 {
    let i = oarena::ar_alloc(&mut cx.ar, 1);
    cx.next_id = i + 1;
    oindex::hidx(&oindex::hid(i))
} // ids come from the ARENA and are typed through oindex::HirId
fn slot_of(cx: &LCx, n: &str) -> u32 {
    let mut i = cx.names.len();
    while i > 0 {
        i = i - 1;
        if cx.names[i].as_str() == n {
            return i as u32;
        }
    }
    0
}
// route a call to a BOUND-generic fn to its monomorphization: the concrete type comes from the argument whose
// declared type IS the bounded parameter. No concrete type recoverable -> the original name (no silent mis-route).
pub fn mono_name(cx: &mut LCx, n: &str, args: &Vec<E>) -> String {
    let mut atys: Vec<String> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        atys.push(ety_of(cx, &args[i]));
        i = i + 1;
    }
    mono_name_tys(cx, n, &atys)
}
// the same rule over argument TYPES rather than expressions -- a builtin method already knows its
// receiver's type and cannot hand over the expression twice (`E` has no Clone), and the DEMAND must be
// recorded identically or the instance is never materialised
pub fn mono_name_tys(cx: &mut LCx, n: &str, atys: &Vec<String>) -> String {
    let fi = fidx_of(cx, n);
    if fi < 0 {
        return String::from(n);
    }
    let gi = fi as usize;
    if gi >= cx.fgbs.len() {
        return String::from(n);
    }
    let gps = split_c(&cx.fgps[gi]);
    let ptys = split_c(&cx.fptys[gi]);
    if gps.len() == 0 {
        return String::from(n);
    }
    // resolve EVERY type parameter, in declaration order: for each one find a parameter whose declared type
    // mentions it (as the whole type, or inside a composite like `Vec<T>`) and unify against that argument's
    // type. All of them must resolve, which is what lets `fn pair<A, B>(a: A, b: B)` monomorphize -- the
    // single-parameter version returned at the first match and could not name a two-parameter instance.
    let mut concs: Vec<String> = Vec::new();
    let mut g = 0;
    while g < gps.len() {
        if gps[g].len() == 0 {
            return String::from(n);
        }
        let mut found = String::from("");
        let mut k = 0;
        while k < ptys.len() && k < atys.len() {
            if ptys[k].as_str() == gps[g].as_str()
                || oast::ends_with_seg(ptys[k].as_str(), gps[g].as_str())
                || oast::has_seg(ptys[k].as_str(), gps[g].as_str())
            {
                let at = if oast::str_starts(atys[k].as_str(), "arr$")
                    && oast::str_starts(ptys[k].as_str(), "Vec$")
                {
                    let mut v = String::from("Vec$");
                    v.push_str(&atys[k][4..]);
                    v
                } else {
                    String::from(atys[k].as_str())
                }; // ARR-GEN: an array argument to a `&[T]` (Vec$T) parameter binds T to its element
                let c = oast::unify_seg(ptys[k].as_str(), at.as_str(), gps[g].as_str());
                if c.len() > 0 {
                    found = c;
                    k = ptys.len();
                }
            }
            k = k + 1;
        }
        if found.len() == 0 {
            let mut comp2 = false;
            let mut k3 = 0;
            while k3 < ptys.len() && k3 < atys.len() {
                if oast::ends_with_seg(ptys[k3].as_str(), gps[g].as_str()) {
                    let base = &ptys[k3][0..ptys[k3].len() - gps[g].len()];
                    let mut ab = String::from(atys[k3].as_str());
                    ab.push('$');
                    if ab.as_str() == base {
                        comp2 = true;
                    }
                }
                k3 = k3 + 1;
            }
            if comp2 {
                found = String::from("int");
            }
        }
        if found.len() == 0 {
            return String::from(n);
        }
        concs.push(found);
        g = g + 1;
    }
    // an UNBOUNDED single parameter used as a whole parameter type keeps the old untyped path, so nothing
    // that already worked is re-routed
    if cx.fgbs[gi].len() == 0 {
        let mut comp = false;
        let mut q = 0;
        while q < gps.len() {
            let mut k2 = 0;
            while k2 < ptys.len() {
                if oast::ends_with_seg(ptys[k2].as_str(), gps[q].as_str()) {
                    comp = true;
                }
                k2 = k2 + 1;
            }
            q = q + 1;
        }
        // G1 G-P2 MONO-ALL (keeper: full monomorphization): an unbounded generic is instanced too -- the erased path kept
        // ONE body, so a type written inside it (`size_of::<T>()`) could never resolve. The erased original is still emitted
        // as the fallback for call sites whose T does not resolve.
        let _ = comp;
    }
    let mut r = String::from(n);
    let mut d = String::from(n);
    d.push('|');
    let mut c2 = 0;
    while c2 < concs.len() {
        r.push('$');
        r.push_str(concs[c2].as_str());
        if c2 > 0 {
            d.push(',');
        }
        d.push_str(concs[c2].as_str());
        c2 = c2 + 1;
    }
    cx.demands.push(d);
    r
}
// lower `v`, wrapping it into a dyn fat pair when the TARGET type is `dyn Trait` and v's static type is an
// implementing struct. One place for the coercion, used by a dyn-annotated `let` and by a Vec<dyn T> push.
// the operators whose RESULT has the operands' type (a comparison answers a bool, so it never wraps)
pub fn cmp_op(o: &str) -> bool {
    o == "=="
        || o == "!="
        || o == "<"
        || o == "<="
        || o == ">"
        || o == ">="
        || o == "&&"
        || o == "||"
}
pub fn arith_op(o: &str) -> bool {
    o == "+"
        || o == "-"
        || o == "*"
        || o == "/"
        || o == "%"
        || o == "<<"
        || o == ">>"
        || o == "&"
        || o == "|"
        || o == "^"
}
pub fn lower_coerce(v: &E, cx: &mut LCx, target: &str) -> H {
    let tr = dyn_trait_of(target);
    if tr.len() > 0 {
        let at = ety_of(cx, v);
        // DYN-IDENT (R2a P5): a value that is ALREADY a `dyn Tr` (a `fn mk() -> Box<dyn Tr>` result) is the {tag, payload}
        // pair itself -- wrapping it again found no `impl Tr for dyn Tr` and refused `vec![mk()]` / `let b: Box<dyn Tr> = mk()`
        if dyn_trait_of(&at).as_str() == tr.as_str() {
            return lower(v, cx);
        }
        let tg = dyn_tag(cx, &tr, &at);
        if tg >= 0 {
            let z = fresh(cx);
            let tagh = H::Lit(tg as i64, z);
            let dat = lower(v, cx);
            let mut pr: Vec<H> = Vec::new();
            pr.push(tagh);
            pr.push(dat);
            let i = fresh(cx);
            return H::StructLitH(2, pr, i);
        }
        // no impl of this trait names this concrete type: the value cannot carry a tag, so dispatching
        // through it would read a raw word as a {tag, payload} record. Emit a call the driver's own IR
        // verification cannot resolve, which turns a segfault into a diagnostic.
        {
            let dat = lower(v, cx);
            let mut pr: Vec<H> = Vec::new();
            pr.push(dat);
            let i = fresh(cx);
            return H::CallH(String::from("$dyn_no_impl"), pr, i);
        }
    }
    // BSTR-TO-SLICE (R2a P4): a `&[u8]` parameter is a Vec of bytes in our value model, and a byte string (`s.as_bytes()`, a
    // slice of one) is a raw byte BUFFER -- the callee read it with the vector helpers and SEGFAULTED (ourc compiled by ourc
    // died in olex::cfg_pre). Convert it the way `.bytes()` does.
    if target == "Vec$u8" {
        let at = ety_of(cx, v);
        if at.as_str() == "bstr" {
            let mut a: Vec<H> = Vec::new();
            a.push(lower(v, cx));
            let i = fresh(cx);
            return H::CallH(String::from("str_bytes_vec"), a, i);
        }
    }
    // C-ARRTOSLICE: a `&[T]` parameter is a VECTOR in our value model (sl_param says so), and an ARRAY is
    // a different layout -- so passing `&a` for an array a handed the callee an array to read with the
    // vector helpers, which SEGFAULTED. rustc's own coercion is array-to-slice; ours is a copy into a
    // vector, which arr_slice already does. Only a NAME is converted: anything else would be evaluated
    // twice by this shape.
    if vec_elem_of(target).len() > 0 || target == "Vec" {
        let at = ety_of(cx, v);
        if is_arr_ty(&at) {
            // the argument is `&a` as often as `a`, so the NAME test is the one that already knows both
            if oparse::place_name(v).len() > 0 {
                let a0 = lower(v, cx);
                let a1 = lower(v, cx);
                let mut na: Vec<H> = Vec::new();
                na.push(a1);
                let il = fresh(cx);
                let ln = H::CallH(String::from("arr_len"), na, il);
                let z = fresh(cx);
                let mut sa: Vec<H> = Vec::new();
                sa.push(a0);
                sa.push(H::Lit(0, z));
                sa.push(ln);
                let i = fresh(cx);
                return H::CallH(String::from("arr_slice"), sa, i);
            }
            // ARR-TEMP: any OTHER array argument (`f(&[1, 2, 3])`) was passed RAW -- an array read with the vector helpers,
            // which segfaulted (slice params, slice patterns, `&s[1..]` of it). Bind it to a temporary first, so the
            // conversion reads the temporary twice instead of evaluating the expression twice.
            let lv = lower(v, cx);
            cx.stypes.push(String::from(at.as_str()));
            cx.names.push(String::from("$arrarg"));
            let s = (cx.names.len() - 1) as u32;
            let l0 = fresh(cx);
            let l1 = fresh(cx);
            let mut na: Vec<H> = Vec::new();
            na.push(H::Local(s, l1));
            let il = fresh(cx);
            let ln = H::CallH(String::from("arr_len"), na, il);
            let z = fresh(cx);
            let mut sa: Vec<H> = Vec::new();
            sa.push(H::Local(s, l0));
            sa.push(H::Lit(0, z));
            sa.push(ln);
            let ic = fresh(cx);
            let conv = H::CallH(String::from("arr_slice"), sa, ic);
            let _ = cx.names.pop();
            let _ = cx.stypes.pop();
            let i = fresh(cx);
            return H::LetH(s, Box::new(lv), Box::new(conv), i);
        }
    }
    lower(v, cx)
}
pub fn lower(e: &E, cx: &mut LCx) -> H {
    // DYN-RET (R2a P5): the driver's return-site coercion marker for a trait-object-returning fn
    if let E::Call(n, args, _l, _h) = e {
        if args.len() == 1 && oast::str_starts(n.as_str(), "$dyncoerce$") {
            let tgt = String::from(&n[11..]);
            return lower_coerce(&args[0], cx, &tgt);
        }
    }
    match e {
        // R8 RANGE-TO (refusal > WRONG): a `let x = ..b` binding is its bare bound here, so `&v[x]` indexed v[b] (an
        // element, not the slice `..b`) -- refused. Other uses (`x + 1`, `for i in x`) are rustc errors.
        E::Index(_ib, ix, _l, _h) if rangeto_index(cx, ix) => {
            let i0 = fresh(cx);
            H::CallH(
                String::from("$a_range_to_value_is_only_modelled_for_debug_print"),
                Vec::new(),
                i0,
            )
        }
        // R8 RANGE-TO: `rangeto(b)` / `rangetoinc(b)` is the int bound; only its Debug differs (dbg_build)
        E::Call(f, a, _l, _h)
            if (f.as_str() == "rangeto" || f.as_str() == "rangetoinc") && a.len() == 1 =>
        {
            lower(&a[0], cx)
        }
        // V1-STR-REF (refusal > WRONG): `let r = &mut s;` with s a String -- a String is a reallocating pointer, so a
        // `r.push(..)` stored its grown buffer in r's slot and `s` kept the old one (rustc 3, ourc 1). Refused until a
        // `&mut` local of a String is modelled as a place (a `&mut String` PARAMETER is: it passes the slot).
        E::Let(_n, v, _b, _l, _h) if let_mut_ref_str(cx, v) => {
            let i0 = fresh(cx);
            H::CallH(
                String::from("$a_local_mut_ref_to_a_String_is_not_modelled"),
                Vec::new(),
                i0,
            )
        }
        E::LetT(_n, _t, v, _b, _l, _h) if let_mut_ref_str(cx, v) => {
            let i0 = fresh(cx);
            H::CallH(
                String::from("$a_local_mut_ref_to_a_String_is_not_modelled"),
                Vec::new(),
                i0,
            )
        }
        // the closure VALUE is a heap record of its captured values; its identity is in its TYPE
        E::ClosRef(n, caps, _l, _h) => {
            let mut lp: Vec<H> = Vec::new();
            let ci = clos_ix_of(n);
            let z = fresh(cx);
            lp.push(H::Lit(ci as i64, z));
            let mut k = 0;
            while k < caps.len() {
                let sl = slot_of(cx, &caps[k]);
                let i2 = fresh(cx);
                let mut mk = String::from(n.as_str());
                mk.push('|');
                mk.push_str(caps[k].as_str());
                if oast::in_list(&cx.movecaps, &mk) {
                    // W3 MOVE-CELL: `move || { c += 1; c }` -- the closure gets its OWN cell, seeded with c's current value
                    // (it wrote through the creator's variable: rustc `0 2`, ourc `2 2`)
                    let cur = if is_cellvar(cx, &caps[k]) {
                        let mut ga: Vec<H> = Vec::new();
                        ga.push(H::Local(sl, i2));
                        ga.push(H::Lit(0, fresh(cx)));
                        H::CallH(String::from("vec_get"), ga, fresh(cx))
                    } else {
                        H::Local(sl, i2)
                    };
                    let mut pa: Vec<H> = Vec::new();
                    pa.push(H::CallH(String::from("vec_new"), Vec::new(), fresh(cx)));
                    pa.push(cur);
                    lp.push(H::CallH(String::from("vec_push1"), pa, fresh(cx)));
                } else if is_cellvar(cx, &caps[k]) && oast::in_list(&cx.moveclos, n) {
                    // MOVE-SNAPSHOT (W3): a `move` closure that only READS a variable the creator keeps in a cell captures its
                    // CURRENT VALUE -- it was handed the cell handle and computed with the address (`move || z * 100`)
                    let mut ga: Vec<H> = Vec::new();
                    ga.push(H::Local(sl, i2));
                    ga.push(H::Lit(0, fresh(cx)));
                    lp.push(H::CallH(String::from("vec_get"), ga, fresh(cx)));
                } else {
                    lp.push(H::Local(sl, i2));
                }
                // CLOS-CAPTY: record the capture's TYPE for the lifted closure (its capture params are untyped), so `p.x` resolves there
                if (sl as usize) < cx.stypes.len() && cx.stypes[sl as usize].len() > 0 {
                    let mut key = String::from(n.as_str());
                    key.push('#');
                    key.push_str(&oast::u32s2(k as u32));
                    key.push('=');
                    key.push_str(cx.stypes[sl as usize].as_str());
                    cx.captys.push(key);
                }
                k = k + 1;
            }
            let n2 = lp.len() as u32;
            let i = fresh(cx);
            H::StructLitH(n2, lp, i)
        }
        E::TupLit(parts, _l, _h) if parts.len() == 0 => {
            let i = fresh(cx);
            H::Lit(0, i)
        }
        E::TupLit(parts, _l, _h) => {
            let mut lp: Vec<H> = Vec::new();
            let mut k = 0;
            while k < parts.len() {
                lp.push(lower(&parts[k], cx));
                k = k + 1;
            }
            let i = fresh(cx);
            H::StructLitH(parts.len() as u32, lp, i)
        }
        E::TupIdx(b, ix, _l, _h) => {
            if const_drop_temp(cx, b) {
                let a0: Vec<H> = Vec::new();
                let i0 = fresh(cx);
                return H::CallH(
                    String::from("$a_field_of_a_drop_const_is_not_modelled"),
                    a0,
                    i0,
                );
            }
            if let E::Call(cn, _ca, _cl, _ch) = b.as_ref() {
                // DROP-FIELD-TEMP on a TUPLE struct (`BAR.0`, `mk().0`): the temporary must drop after the read; the named-field
                // path (field_temp) crashes ourc here (open node) -- REFUSED until then, never answered without the drop
                let sn = ety_of(cx, b);
                let nost: Vec<String> = Vec::new();
                if !cn.starts_with("$") && drop_glue("$tq", &nost, sn.as_str(), cx).len() > 0 {
                    let a0: Vec<H> = Vec::new();
                    let i0 = fresh(cx);
                    return H::CallH(
                        String::from("$a_field_of_a_drop_temporary_tuple_struct_is_not_modelled"),
                        a0,
                        i0,
                    );
                }
            }
            let mut slot = *ix;
            if oast::tix_is_end(*ix) {
                // TUP-REST-END: `(.., b, c)` -- the slot counts from the END; the arity comes from the tuple's type name
                let n = oparse::tup_elems(&ety_of(cx, b)).len();
                let r = oast::tix_resolve(*ix, n);
                if r < 0 {
                    let a0: Vec<H> = Vec::new();
                    let i0 = fresh(cx);
                    return H::CallH(String::from("$tuple_rest_with_unknown_arity"), a0, i0);
                } // unknown arity: a named refusal, never a guessed slot
                slot = r as u32;
            }
            let lb = lower(b, cx);
            let i = fresh(cx);
            H::FieldH(Box::new(lb), slot, i)
        }
        E::Ref(x, m, _l, _h) => {
            if *m {
                match &**x {
                    E::Index(b, ix, _a, _b2) if is_vec_ty(&ety_of(cx, b)) => {
                        let lb = lower(b, cx);
                        let li = lower(ix, cx);
                        let mut a: Vec<H> = Vec::new();
                        a.push(lb);
                        a.push(li);
                        let i = fresh(cx);
                        return H::CallH(String::from("vec_elem_addr"), a, i);
                    }
                    E::Field(o, f, _a, _b2) => {
                        let sn = ety_of(cx, o);
                        let idx = oast::sfield_idx(&cx.sdefs, &sn, f);
                        // REF-FIELD-AGG (R2a P4): `&mut s.f` with f an AGGREGATE (Vec / String / struct / enum / map / tuple) is the
                        // field's HANDLE -- a handle already behaves as a reference. Passing the slot ADDRESS (fld_addr) made the
                        // callee read the address as the handle: `bump(&mut p.v)` SEGFAULTED (ourc compiled by ourc died in
                        // fill_capture_tys(&mut p.clos, ..)). Scalar fields keep their address (the callee writes through it).
                        let fty = oast::sfield_ty(&cx.sdefs, &sn, f);
                        let agg = is_vec_ty(&fty)
                            || fty.as_str() == "str"
                            || fty.as_str() == "String"
                            || oast::str_starts(fty.as_str(), "HashMap")
                            || oast::str_starts(fty.as_str(), "tup")
                            || oast::is_struct_name(&cx.sdefs, fty.as_str())
                            || oast::eidx(&cx.edefs, fty.as_str()) >= 0;
                        if idx >= 0 && agg {
                            return lower(x, cx);
                        }
                        if idx >= 0 {
                            let lo2 = lower(o, cx);
                            let z = fresh(cx);
                            let mut a: Vec<H> = Vec::new();
                            a.push(lo2);
                            a.push(H::Lit(idx as i64, z));
                            let i = fresh(cx);
                            return H::CallH(String::from("fld_addr"), a, i);
                        }
                    }
                    _ => {}
                }
            }
            lower(x, cx)
        }
        E::Cast(x, t, _l, _h) => {
            let st = ety_of(cx, x);
            {
                let mut dn = String::from(st.as_str());
                dn.push_str("$discr");
                if st.len() > 0 && fn_known(cx, &dn) && t.as_str() != "f64" && t.as_str() != "f32" {
                    // ENUM-DISCR (R7): a fieldless enum casts to its DISCRIMINANT, not its variant index
                    let mut da: Vec<H> = Vec::new();
                    da.push(lower(x, cx));
                    let di = fresh(cx);
                    return H::CallH(dn, da, di);
                }
            }
            let lx = lower(x, cx);
            let i = fresh(cx);
            let tag = if t.as_str() == "f64" || t.as_str() == "f32" {
                if st.as_str() == "f64" {
                    String::from("$idcast")
                } else {
                    String::from("$i2f")
                }
            }
            // D1.1 int->f64
            else if (oast::is_int_width(t) || t.as_str() == "int") && st.as_str() == "f64" {
                let mut tg = String::from("$f2i$");
                tg.push_str(t.as_str());
                tg
            }
            // D1.1 f64->int (width-aware, saturating)
            else {
                String::from(t.as_str())
            };
            H::CastH(Box::new(lx), tag, i)
        }
        E::BreakE(_l, _h) => {
            let i = fresh(cx);
            pd_jump(0, H::BreakH(i), cx)
        }
        E::BreakN(n, _l, _h) => {
            let i = fresh(cx);
            pd_jump(*n, H::BreakNH(*n, i), cx)
        }
        E::ContE(_l, _h) => {
            let i = fresh(cx);
            pd_jump(0, H::ContH(i), cx)
        }
        E::ContN(n, _l, _h) => {
            let i = fresh(cx);
            pd_jump(*n, H::ContNH(*n, i), cx)
        }
        E::RetE(x, _l, _h) => {
            // DROP-RET (heavy pass 2): `return v` evaluates v FIRST, then drops every pending binding of the function
            // (innermost first, the DROP-JUMP stack across all loop markers), then returns -- an early return skipped them
            let lx = lower(x, cx);
            let bt = ety_of(cx, x);
            cx.stypes.push(bt);
            cx.names.push(String::from("$retv"));
            let s2 = (cx.names.len() - 1) as u32;
            let ri = fresh(cx);
            let ret = H::RetH(Box::new(H::Local(s2, ri)), fresh(cx));
            let body = pd_jump(1000000, ret, cx); // 1000000 = the function exit: every pending binding
            let _ = cx.names.pop();
            let _ = cx.stypes.pop();
            let i = fresh(cx);
            H::LetH(s2, Box::new(lx), Box::new(body), i)
        }
        E::StrLit(v, _l, _h) => {
            let i = fresh(cx);
            H::StrH(String::from(v.as_str()), i)
        }
        E::MethodCall(r, m, args, _l, _h) => {
            if (m.as_str() == "sort" || m.as_str() == "sort_unstable") && args.len() == 0 {
                // R13 DERIVE-ORD: `v.sort()` of a Vec of a derived-Ord struct sorts by the synthesized comparator `$sortcmp$S`
                let el = vec_elem_of(ety_of(cx, r).as_str());
                let mut cn = String::from("$sortcmp$");
                cn.push_str(el.as_str());
                if el.len() > 0 && fn_known(cx, &cn) {
                    if let E::Var(vn, _vl, _vh) = r.as_ref() {
                        // an insertion sort as ordinary AST (vsort_by's algorithm), calling the comparator DIRECTLY -- a closure
                        // value synthesized here did not match the lifted-closure convention (it segfaulted)
                        let v = || E::Var(String::from(vn.as_str()), 0, 0);
                        let var = |s: &str| E::Var(String::from(s), 0, 0);
                        let int = |k: i64| E::Int(k, 0, 0);
                        let bin = |o: &str, a: E, b: E| {
                            E::Bin(String::from(o), Box::new(a), Box::new(b), 0, 0)
                        };
                        let idx = |a: E, b: E| E::Index(Box::new(a), Box::new(b), 0, 0);
                        let mut cargs: Vec<E> = Vec::new();
                        cargs.push(idx(v(), bin("-", var("$sj"), int(1))));
                        cargs.push(var("$sx"));
                        let mut oa: Vec<E> = Vec::new();
                        oa.push(E::Call(String::from(cn.as_str()), cargs, 0, 0));
                        let gt = E::Call(String::from("ord_is_gt"), oa, 0, 0);
                        let cond = bin("&&", bin(">", var("$sj"), int(0)), gt);
                        let shift = E::Seq(
                            Box::new(E::AssignTo(
                                Box::new(idx(v(), var("$sj"))),
                                Box::new(idx(v(), bin("-", var("$sj"), int(1)))),
                                0,
                                0,
                            )),
                            Box::new(E::Assign(
                                String::from("$sj"),
                                Box::new(bin("-", var("$sj"), int(1))),
                                0,
                                0,
                            )),
                            0,
                            0,
                        );
                        let inner = E::While(Box::new(cond), Box::new(shift), 0, 0);
                        let place =
                            E::AssignTo(Box::new(idx(v(), var("$sj"))), Box::new(var("$sx")), 0, 0);
                        let step = E::Assign(
                            String::from("$si"),
                            Box::new(bin("+", var("$si"), int(1))),
                            0,
                            0,
                        );
                        let body = E::Let(
                            String::from("$sx"),
                            Box::new(idx(v(), var("$si"))),
                            Box::new(E::Let(
                                String::from("$sj"),
                                Box::new(var("$si")),
                                Box::new(E::Seq(
                                    Box::new(inner),
                                    Box::new(E::Seq(Box::new(place), Box::new(step), 0, 0)),
                                    0,
                                    0,
                                )),
                                0,
                                0,
                            )),
                            0,
                            0,
                        );
                        let lenc =
                            E::MethodCall(Box::new(v()), String::from("len"), Vec::new(), 0, 0);
                        let outer =
                            E::While(Box::new(bin("<", var("$si"), lenc)), Box::new(body), 0, 0);
                        let prog =
                            E::Let(String::from("$si"), Box::new(int(1)), Box::new(outer), 0, 0);
                        return lower(&prog, cx);
                    }
                }
            }
            if gen_unsound(cx, r, m) {
                let a0: Vec<H> = Vec::new();
                let i0 = fresh(cx);
                return H::CallH(
                    String::from(
                        "$generic_impl_method_on_a_non_integer_instance_is_not_implemented",
                    ),
                    a0,
                    i0,
                );
            } // GEN-INST-GUARD
            let rt = ety_of(cx, r);
            if m.as_str() == "clone" && args.len() == 0 {
                if let E::Var(x, _vl, _vh) = unref(r.as_ref()) {
                    if is_rcvar(cx, x) {
                        // RC-IDENTITY (R4c): `a.clone()` of an Rc SHARES (count + 1, same handle) -- never CLONE-DEEP
                        let mut la: Vec<H> = Vec::new();
                        la.push(rc_raw(r.as_ref(), cx));
                        let i = fresh(cx);
                        return H::CallH(String::from("rc_inc"), la, i);
                    }
                }
            }
            {
                let mut ad = adapt_owned(r, m.as_str(), args, rt.as_str(), cx); // DROP-ADAPT (W1)
                if ad.len() > 0 {
                    return ad.pop().unwrap();
                }
            }
            {
                // DROP-RECV (W1): an owned TEMPORARY receiver (`mk(i).ok()`, `T(1).len()`) drops when the call's expression
                // ends -- in a `while` condition, every iteration. Only for a user method whose return type does not mention
                // the receiver type (a Drop type's fields cannot be moved out, E0509, so the receiver can only survive by
                // being returned).
                let mut fnm = String::from(rt.as_str());
                fnm.push('$');
                fnm.push_str(m.as_str());
                let built = match r.as_ref() {
                    E::StructLit(_n, _f, _l2, _h2) => true,
                    E::EnumLit(_t, _v, _p, _l2, _h2) => true,
                    E::Call(f, _a, _l2, _h2) => {
                        rt.as_str() == f.as_str() || is_fresh_fn(cx, f.as_str())
                    }
                    _ => false,
                };
                let ret = fret_of(cx, fnm.as_str());
                let nost: Vec<String> = Vec::new();
                let top = cx.recv_top;
                cx.recv_top = false; // only THIS call is top; its receiver and arguments are not
                if top
                    && built
                    && rt.len() > 0
                    && fn_known(cx, &fnm)
                    && !ty_owns(cx, ret.as_str(), rt.as_str(), 0)
                    && drop_glue("$rcv", &nost, rt.as_str(), cx).len() > 0
                {
                    cx.stypes.push(String::from(rt.as_str()));
                    cx.names.push(String::from("$rcv"));
                    let s = (cx.names.len() - 1) as u32;
                    let lr = lower(r, cx);
                    let mut ca: Vec<H> = Vec::new();
                    let li = fresh(cx);
                    ca.push(H::Local(s, li));
                    let mut k = 0;
                    while k < args.len() {
                        ca.push(lower(&args[k], cx));
                        k = k + 1;
                    }
                    let ci = fresh(cx);
                    let call = H::CallH(fnm, ca, ci);
                    cx.stypes.push(String::from(ret.as_str()));
                    cx.names.push(String::from("$rres"));
                    let s2 = (cx.names.len() - 1) as u32;
                    let mut g = drop_glue("$rcv", &nost, rt.as_str(), cx);
                    let ge = g.pop().unwrap();
                    let dh = lower(&ge, cx);
                    let _ = cx.names.pop();
                    let _ = cx.stypes.pop();
                    let ri = fresh(cx);
                    let sq = H::SeqH(Box::new(dh), Box::new(H::Local(s2, ri)), fresh(cx));
                    let inner = H::LetH(s2, Box::new(call), Box::new(sq), fresh(cx));
                    let _ = cx.names.pop();
                    let _ = cx.stypes.pop();
                    let li2 = fresh(cx);
                    return H::LetH(s, Box::new(lr), Box::new(inner), li2);
                }
            }
            // ITER-DFLT: `x.sum()`/`x.count()` on a type with its own `next` (and no own sum/count) is the prelude's
            // provided method `iter_dflt_<m>(x)`. The receiver is bound to a temporary first, so the call is an ordinary
            // generic call that the monomorphizer sees with the receiver's concrete type.
            if (m.as_str() == "sum" || m.as_str() == "count") && args.len() == 0 && rt.len() > 0 {
                let mut nx = String::from(rt.as_str());
                nx.push_str("$next");
                let mut own = String::from(rt.as_str());
                own.push('$');
                own.push_str(m.as_str());
                let mut hn = false;
                let mut ho = false;
                let mut q = 0;
                while q < cx.fnames.len() {
                    if cx.fnames[q].as_str() == nx.as_str() {
                        hn = true;
                    }
                    if cx.fnames[q].as_str() == own.as_str() {
                        ho = true;
                    }
                    q = q + 1;
                }
                if hn && !ho {
                    let lv = lower(r, cx);
                    cx.stypes.push(String::from(rt.as_str()));
                    cx.names.push(String::from("$itrecv"));
                    let s = (cx.names.len() - 1) as u32;
                    let mut fname = String::from("iter_dflt_");
                    fname.push_str(m.as_str());
                    let mut ca: Vec<E> = Vec::new();
                    ca.push(E::Var(String::from("$itrecv"), 0, 0));
                    let call = E::Call(fname, ca, 0, 0);
                    let lb = lower(&call, cx);
                    let _ = cx.names.pop();
                    let _ = cx.stypes.pop();
                    let i = fresh(cx);
                    return H::LetH(s, Box::new(lv), Box::new(lb), i);
                }
            }
            // HASHMAP: the prelude map, routed by the KEY type the local was inferred with (HashMap$str / HashMap$int)
            if oast::str_starts(rt.as_str(), "HashMap") {
                let fam = if rt.as_str() == "HashMap$str" {
                    "hms_"
                } else {
                    "hmi_"
                };
                let fname = if m.as_str() == "insert" && args.len() == 2 {
                    let mut f = String::from(fam);
                    f.push_str("insert");
                    f
                } else if m.as_str() == "get" && args.len() == 1 {
                    let mut f = String::from(fam);
                    f.push_str("get");
                    f
                } else if m.as_str() == "contains_key" && args.len() == 1 {
                    let mut f = String::from(fam);
                    f.push_str("contains");
                    f
                } else if m.as_str() == "len" && args.len() == 0 {
                    String::from("hm_len")
                } else if m.as_str() == "is_empty" && args.len() == 0 {
                    String::from("hm_empty")
                } else if m.as_str() == "remove" && args.len() == 1 {
                    let mut f = String::from(fam);
                    f.push_str("remove"); // HM-REMOVE: swap-remove from both columns (HashMap order is unspecified)
                    f
                } else if m.as_str() == "keys" && args.len() == 0 {
                    String::from("hm_keys") // HM-KEYS: a COPY of the key column (k.sort() must not reorder the map)
                } else if (m.as_str() == "iter" || m.as_str() == "into_iter") && args.len() == 0 {
                    String::from("hm_pairs")
                }
                // HM-ITER: (key, value) pairs
                else {
                    String::from("")
                };
                if fname.len() > 0 {
                    let mut ca: Vec<H> = Vec::new();
                    ca.push(lower(r, cx));
                    let mut k = 0;
                    while k < args.len() {
                        ca.push(lower(&args[k], cx));
                        k = k + 1;
                    }
                    let i = fresh(cx);
                    return H::CallH(fname, ca, i);
                }
            }
            // INT-METHODS: built as ordinary expressions over temporaries ($im0 = receiver, $im1 = argument), then lowered
            {
                let mn = m.as_str();
                let lo = 0;
                let hi = 0;
                let v0 = || E::Var(String::from("$im0"), 0, 0);
                let v1 = || E::Var(String::from("$im1"), 0, 0);
                let bits: i64 = if rt.as_str() == "u8" || rt.as_str() == "i8" {
                    8
                } else if rt.as_str() == "u16" || rt.as_str() == "i16" {
                    16
                } else if rt.as_str() == "u32" || rt.as_str() == "i32" {
                    32
                } else {
                    64
                };
                let mut e2 = E::Int(0, lo, hi);
                let mut ok = false;
                if mn == "wrapping_neg" && args.len() == 0 {
                    e2 = E::Bin(
                        String::from("-"),
                        Box::new(E::Int(0, lo, hi)),
                        Box::new(v0()),
                        lo,
                        hi,
                    );
                    ok = true;
                }
                if mn == "rem_euclid" && args.len() == 1 {
                    // r = x % m; r < 0 ? r + |m| : r
                    let r = || {
                        E::Bin(
                            String::from("%"),
                            Box::new(E::Var(String::from("$im0"), 0, 0)),
                            Box::new(E::Var(String::from("$im1"), 0, 0)),
                            0,
                            0,
                        )
                    };
                    let am = E::If(
                        Box::new(E::Bin(
                            String::from("<"),
                            Box::new(v1()),
                            Box::new(E::Int(0, lo, hi)),
                            lo,
                            hi,
                        )),
                        Box::new(E::Bin(
                            String::from("-"),
                            Box::new(E::Int(0, lo, hi)),
                            Box::new(v1()),
                            lo,
                            hi,
                        )),
                        Box::new(v1()),
                        lo,
                        hi,
                    );
                    e2 = E::If(
                        Box::new(E::Bin(
                            String::from("<"),
                            Box::new(r()),
                            Box::new(E::Int(0, lo, hi)),
                            lo,
                            hi,
                        )),
                        Box::new(E::Bin(
                            String::from("+"),
                            Box::new(r()),
                            Box::new(am),
                            lo,
                            hi,
                        )),
                        Box::new(r()),
                        lo,
                        hi,
                    );
                    ok = true;
                }
                // INT-SIGNUM / INT-DIVEUCLID: signum is -1/0/1; div_euclid rounds toward the euclidean quotient
                // (q = x / m, then one step toward -inf when the remainder is negative and m > 0, toward +inf when m < 0)
                if mn == "signum" && args.len() == 0 && rt.as_str() != "f64" {
                    /* int only: f64 signum is fsignum (+-1.0, NaN) */
                    e2 = E::If(
                        Box::new(E::Bin(
                            String::from(">"),
                            Box::new(v0()),
                            Box::new(E::Int(0, lo, hi)),
                            lo,
                            hi,
                        )),
                        Box::new(E::Int(1, lo, hi)),
                        Box::new(E::If(
                            Box::new(E::Bin(
                                String::from("<"),
                                Box::new(v0()),
                                Box::new(E::Int(0, lo, hi)),
                                lo,
                                hi,
                            )),
                            Box::new(E::Int(0 - 1, lo, hi)),
                            Box::new(E::Int(0, lo, hi)),
                            lo,
                            hi,
                        )),
                        lo,
                        hi,
                    );
                    ok = true;
                }
                if mn == "div_euclid" && args.len() == 1 {
                    let q = || {
                        E::Bin(
                            String::from("/"),
                            Box::new(E::Var(String::from("$im0"), 0, 0)),
                            Box::new(E::Var(String::from("$im1"), 0, 0)),
                            0,
                            0,
                        )
                    };
                    let rneg = E::Bin(
                        String::from("<"),
                        Box::new(E::Bin(
                            String::from("%"),
                            Box::new(v0()),
                            Box::new(v1()),
                            lo,
                            hi,
                        )),
                        Box::new(E::Int(0, lo, hi)),
                        lo,
                        hi,
                    );
                    let adj = E::If(
                        Box::new(E::Bin(
                            String::from(">"),
                            Box::new(v1()),
                            Box::new(E::Int(0, lo, hi)),
                            lo,
                            hi,
                        )),
                        Box::new(E::Bin(
                            String::from("-"),
                            Box::new(q()),
                            Box::new(E::Int(1, lo, hi)),
                            lo,
                            hi,
                        )),
                        Box::new(E::Bin(
                            String::from("+"),
                            Box::new(q()),
                            Box::new(E::Int(1, lo, hi)),
                            lo,
                            hi,
                        )),
                        lo,
                        hi,
                    );
                    e2 = E::If(Box::new(rneg), Box::new(adj), Box::new(q()), lo, hi);
                    ok = true;
                }
                if (mn == "wrapping_shl" || mn == "wrapping_shr") && args.len() == 1 {
                    // the shift amount is masked to the width
                    let amt = E::Bin(
                        String::from("&"),
                        Box::new(v1()),
                        Box::new(E::Int(bits - 1, lo, hi)),
                        lo,
                        hi,
                    );
                    let op = if mn == "wrapping_shl" {
                        String::from("<<")
                    } else {
                        String::from(">>")
                    };
                    e2 = E::Bin(op, Box::new(v0()), Box::new(amt), lo, hi);
                    ok = true;
                }
                if (mn == "to_ascii_lowercase" || mn == "to_ascii_uppercase") && args.len() == 0 {
                    let (a, z, d) = if mn == "to_ascii_lowercase" {
                        (65, 90, 32)
                    } else {
                        (97, 122, 0 - 32)
                    };
                    let inr = E::Bin(
                        String::from("&&"),
                        Box::new(E::Bin(
                            String::from(">="),
                            Box::new(E::Cast(Box::new(v0()), String::from("i64"), lo, hi)),
                            Box::new(E::Int(a, lo, hi)),
                            lo,
                            hi,
                        )),
                        Box::new(E::Bin(
                            String::from("<="),
                            Box::new(E::Cast(Box::new(v0()), String::from("i64"), lo, hi)),
                            Box::new(E::Int(z, lo, hi)),
                            lo,
                            hi,
                        )),
                        lo,
                        hi,
                    );
                    e2 = E::If(
                        Box::new(inr),
                        Box::new(E::Bin(
                            String::from("+"),
                            Box::new(v0()),
                            Box::new(E::Int(d, lo, hi)),
                            lo,
                            hi,
                        )),
                        Box::new(v0()),
                        lo,
                        hi,
                    );
                    ok = true;
                }
                if ok {
                    if mn != "to_ascii_lowercase"
                        && mn != "to_ascii_uppercase"
                        && rt.as_str() != "i64"
                        && rt.as_str() != "int"
                        && oast::is_int_width(&rt)
                    {
                        e2 = E::Cast(Box::new(e2), String::from(rt.as_str()), lo, hi);
                    }
                    let l0 = lower(r, cx);
                    cx.stypes.push(String::from(rt.as_str()));
                    cx.names.push(String::from("$im0"));
                    let s0 = (cx.names.len() - 1) as u32;
                    let mut s1: u32 = 0;
                    let mut l1 = H::Lit(0, 0);
                    let has1 = args.len() == 1;
                    if has1 {
                        l1 = lower(&args[0], cx);
                        let at = ety_of(cx, &args[0]);
                        cx.stypes.push(at);
                        cx.names.push(String::from("$im1"));
                        s1 = (cx.names.len() - 1) as u32;
                    }
                    let lb = lower(&e2, cx);
                    let mut out = lb;
                    if has1 {
                        let _ = cx.names.pop();
                        let _ = cx.stypes.pop();
                        let i1 = fresh(cx);
                        out = H::LetH(s1, Box::new(l1), Box::new(out), i1);
                    }
                    let _ = cx.names.pop();
                    let _ = cx.stypes.pop();
                    let i0 = fresh(cx);
                    return H::LetH(s0, Box::new(l0), Box::new(out), i0);
                }
            }
            if m.as_str() == "to_string" && args.len() == 0 && rt.as_str() == "char" {
                let mut ca: Vec<H> = Vec::new();
                ca.push(lower(r, cx));
                let i = fresh(cx);
                return H::CallH(String::from("chr_str"), ca, i);
            } // CHR-STR
            if (m.as_str() == "to_string" || m.as_str() == "to_owned") && args.len() == 0 {
                if rt.as_str() == "str" {
                    let mut cp: Vec<H> = Vec::new();
                    cp.push(lower(r, cx));
                    let i = fresh(cx);
                    return H::CallH(String::from("str_copy"), cp, i);
                }
                if rt.as_str() == "f64" {
                    let mut fa: Vec<H> = Vec::new();
                    fa.push(lower(r, cx));
                    let z = fresh(cx);
                    fa.push(H::Lit(0, z));
                    let i = fresh(cx);
                    return H::CallH(String::from("f64_str"), fa, i);
                }
                // TOSTRING-DISPLAY: a user `impl Display` type's to_string is its fmt text; a bool is true/false; any other
                // aggregate would print its POINTER (it fell to str_from_int) -- refused
                if m.as_str() == "to_string" && rt.len() > 0 {
                    let mut fm = String::from(rt.as_str());
                    fm.push_str("$fmt");
                    if fidx_of(cx, &fm) >= 0 {
                        let mut fa: Vec<H> = Vec::new();
                        fa.push(lower(r, cx));
                        let z = fresh(cx);
                        fa.push(H::Lit(0, z));
                        let ci = fresh(cx);
                        return H::CallH(fm, fa, ci);
                    }
                }
                if rt.as_str() == "bool" {
                    let c = lower(r, cx);
                    let t1 = fresh(cx);
                    let f1 = fresh(cx);
                    let i = fresh(cx);
                    return H::IfH(
                        Box::new(c),
                        Box::new(H::StrH(String::from("true"), t1)),
                        Box::new(H::StrH(String::from("false"), f1)),
                        i,
                    );
                }
                if oast::sfields(&cx.sdefs, &rt) > 0
                    || oast::eidx(&cx.edefs, &rt) >= 0
                    || oast::str_starts(rt.as_str(), "Vec")
                    || oast::str_starts(rt.as_str(), "tup")
                {
                    let a0: Vec<H> = Vec::new();
                    let i = fresh(cx);
                    return H::CallH(String::from("$to_string_of_aggregate_unsupported"), a0, i);
                }
                let mut la: Vec<H> = Vec::new();
                la.push(lower(r, cx));
                let i = fresh(cx);
                return H::CallH(String::from("str_from_int"), la, i);
            }
            if m.as_str() == "partition" && args.len() == 1 {
                let a0ty = ety_of(cx, &args[0]);
                let mut at1: Vec<String> = Vec::new();
                at1.push(String::from(rt.as_str()));
                at1.push(String::from(a0ty.as_str()));
                let nmt = mono_name_tys(cx, "opart_t", &at1);
                let nmg = mono_name_tys(cx, "opart_g", &at1);
                let mut l1: Vec<H> = Vec::new();
                l1.push(lower(r, cx));
                l1.push(lower(&args[0], cx));
                let i1 = fresh(cx);
                let c1 = H::CallH(nmt, l1, i1);
                let mut l2: Vec<H> = Vec::new();
                l2.push(lower(r, cx));
                l2.push(lower(&args[0], cx));
                let i2 = fresh(cx);
                let c2 = H::CallH(nmg, l2, i2);
                let mut tp: Vec<H> = Vec::new();
                tp.push(c1);
                tp.push(c2);
                let i3 = fresh(cx);
                return H::StructLitH(2, tp, i3);
            }
            // STR-PAT-CHAR: a str method's PATTERN argument may be a char (`s.find('l')`, `s.replace('l', "L")`, `s.split('é')`);
            // a char is an int here, so it is encoded to its UTF-8 string first (chr_str). It crashed (find/replace: the char
            // was read as a string pointer) or matched one byte only (split's chr1 -- wrong for any non-ASCII char).
            // A CLOSURE pattern (`s.find(|c| ..)`) has no lowering and answered a wrong index -- refused.
            if rt.as_str() == "str"
                && args.len() >= 1
                && (m.as_str() == "find"
                    || m.as_str() == "rfind"
                    || m.as_str() == "contains"
                    || m.as_str() == "starts_with"
                    || m.as_str() == "ends_with"
                    || m.as_str() == "split"
                    || m.as_str() == "replace")
            {
                if (match &args[0] {
                    E::ClosRef(_a, _b, _c, _d) => true,
                    _ => false,
                }) {
                    let a0: Vec<H> = Vec::new();
                    let i = fresh(cx);
                    return H::CallH(String::from("$str_pattern_closure_unsupported"), a0, i);
                }
                if ety_of(cx, &args[0]).as_str() == "char" {
                    let bi2 = builtin_method(&rt, m, args.len());
                    let lr = lower(r, cx);
                    let mut c1: Vec<H> = Vec::new();
                    c1.push(lower(&args[0], cx));
                    let ic = fresh(cx);
                    let pat = H::CallH(String::from("chr_str"), c1, ic);
                    let mut ca: Vec<H> = Vec::new();
                    ca.push(lr);
                    ca.push(pat);
                    let mut k = 1;
                    while k < args.len() {
                        ca.push(lower(&args[k], cx));
                        k = k + 1;
                    }
                    let i = fresh(cx);
                    return H::CallH(bi2, ca, i);
                }
            }
            if m.as_str() == "clone" && args.len() == 0 && is_vec_ty(&rt) {
                let lr = lower(r, cx);
                return deep_clone_h(cx, lr, rt.as_str());
            } // CLONE-DEEP-ELEM: a top-level Vec clone (scalar elements still go to vclone)
            {
                let bi = builtin_method(&rt, m, args.len());
                if bi.as_str() == "$id" {
                    return lower(r, cx);
                }
                // C-STRSPLIT: a char separator is an int in our model, so it is widened with chr1 -- the
                // same one-byte-string helper `push(char)` already uses.
                if bi.as_str() == "str_split" && args.len() == 1 {
                    let lr = lower(r, cx);
                    let la = lower(&args[0], cx);
                    let at = ety_of(cx, &args[0]);
                    let sep = if at.as_str() == "str" {
                        la
                    } else {
                        let mut p1: Vec<H> = Vec::new();
                        p1.push(la);
                        let i0 = fresh(cx);
                        H::CallH(String::from("chr1"), p1, i0)
                    };
                    let mut ca: Vec<H> = Vec::new();
                    ca.push(lr);
                    ca.push(sep);
                    let i = fresh(cx);
                    return H::CallH(String::from("str_split"), ca, i);
                }
                // CLONE-STRUCT (derive(Clone)): a struct value is a pointer to its field cells, so clone must build a NEW
                // record -- else the clone aliases the original and a write through one shows in the other. A field-wise
                // (shallow) copy; strings are immutable here, so only a nested Vec still shares its buffer.
                if m.as_str() == "clone" && args.len() == 0 && oast::sfields(&cx.sdefs, &rt) > 0 {
                    match r.as_ref() {
                        E::Var(_vn, _a, _b) => {
                            let lr = lower(r, cx);
                            return deep_clone_h(cx, lr, rt.as_str());
                        }
                        // CLONE-TEMP: a FRESH value (`S(1, ()).clone()`, `S { .. }.clone()`) has no other owner -- its clone is itself
                        E::StructLit(_n, _p, _a, _b) => {
                            return lower(r, cx);
                        }
                        E::Call(cn, _p, _a, _b) if oast::sfields(&cx.sdefs, cn) > 0 => {
                            return lower(r, cx);
                        }
                        _ => {}
                    }
                }
                // C-ENUMCLONE: cloning an enum is the identity (an enum value is immutable here)
                // P-06-BOUNDMETHOD: clone() on a SCALAR (incl. a generic T:Clone monomorphized to int) is the
                // identity too -- rustc's Clone for Copy scalars returns the value. ourc resolved clone by the
                // receiver type name and found no inherent `clone` on int; make scalar clone the value itself.
                if m.as_str() == "clone"
                    && args.len() == 0
                    && (oast::eidx(&cx.edefs, &rt) >= 0
                        || rt.as_str() == "int"
                        || rt.as_str() == "i32"
                        || rt.as_str() == "i64"
                        || rt.as_str() == "bool"
                        || rt.as_str() == "char"
                        || rt.as_str() == "u8"
                        || rt.as_str() == "u16"
                        || rt.as_str() == "u32"
                        || rt.as_str() == "u64"
                        || rt.as_str() == "i8"
                        || rt.as_str() == "i16"
                        || rt.as_str() == "usize"
                        || rt.as_str() == "isize"
                        || rt.as_str() == "f64"
                        || rt.as_str() == "f32")
                {
                    return lower(r, cx);
                }
                // `s.push_str(x)` / `s.push(c)`: rustc's String is a growable owned buffer; ours is an
                // immutable value (ledger C8), so the growth is a re-assignment of the receiver's own slot --
                // which is what rustc's semantics come to for a local. Dispatched on the receiver TYPE,
                // because `v.push(x)` on a Vec is a different method entirely.
                // C-WRAPPING: the wrapping methods ARE the plain operators at a 64-bit width
                if bi.as_str() == "$cntones" || bi.as_str() == "$leadz" || bi.as_str() == "$trailz"
                {
                    let rv = lower(r, cx);
                    let w = ety_of(cx, r);
                    let bits = oast::width_bits(&w);
                    let fnm = if bi.as_str() == "$leadz" {
                        String::from("ileading_zeros")
                    } else if bi.as_str() == "$trailz" {
                        String::from("itrailing_zeros")
                    } else {
                        String::from("icount_ones")
                    };
                    let ib = fresh(cx);
                    let ir = fresh(cx);
                    let mut ca: Vec<H> = Vec::new();
                    ca.push(rv);
                    ca.push(H::Lit(bits as i64, ib));
                    return H::CallH(fnm, ca, ir);
                }
                if bi.as_str() == "$chkadd"
                    || bi.as_str() == "$chksub"
                    || bi.as_str() == "$chkmul"
                    || bi.as_str() == "$satadd"
                    || bi.as_str() == "$satsub"
                    || bi.as_str() == "$satmul"
                {
                    let rv = lower(r, cx);
                    let av = lower(&args[0], cx);
                    let w = ety_of(cx, r);
                    let bits = oast::width_bits(&w);
                    let sg = oast::width_signed(&w);
                    let mut pw = 1;
                    let mut k = 0;
                    while k < bits {
                        pw = pw * 2;
                        k = k + 1;
                    }
                    let lo = if sg { 0 - pw / 2 } else { 0 };
                    let hi = if sg { pw / 2 - 1 } else { pw - 1 };
                    let fnm = if bi.as_str() == "$chksub" {
                        String::from("ichk_sub")
                    } else if bi.as_str() == "$chkmul" {
                        String::from("ichk_mul")
                    } else if bi.as_str() == "$satadd" {
                        String::from("isat_add")
                    } else if bi.as_str() == "$satsub" {
                        String::from("isat_sub")
                    } else if bi.as_str() == "$satmul" {
                        String::from("isat_mul")
                    } else {
                        String::from("ichk_add")
                    };
                    let i1 = fresh(cx);
                    let i2 = fresh(cx);
                    let ir = fresh(cx);
                    let mut ca: Vec<H> = Vec::new();
                    ca.push(rv);
                    ca.push(av);
                    ca.push(H::Lit(lo, i1));
                    ca.push(H::Lit(hi, i2));
                    return H::CallH(fnm, ca, ir);
                }
                if bi.as_str() == "$wrap" {
                    let rv = lower(r, cx);
                    let av = lower(&args[0], cx);
                    let o = if m.as_str() == "wrapping_mul" {
                        String::from("*")
                    } else if m.as_str() == "wrapping_sub" {
                        String::from("-")
                    } else {
                        String::from("+")
                    };
                    let i = fresh(cx);
                    let bh = H::BinH(o, Box::new(rv), Box::new(av), i);
                    // narrow width: mask the wrapped result to the receiver's width (reuse the P-WIDTH cast)
                    let w = ety_of(cx, r);
                    if oast::is_int_width(&w) && oast::width_bits(&w) < 64 {
                        let i2 = fresh(cx);
                        return H::CastH(Box::new(bh), w, i2);
                    }
                    return bh;
                }
                // C-PUSHCOERCE: the element coercion lived on the CALL form of vec_push and not on the
                // METHOD form -- so `v.push(Box::new(B))` into a Vec of trait objects stored the bare value
                // instead of the {tag, payload} record, and every element then dispatched to the FIRST impl
                // of the trait. One push, one rule: the method form asks lower_coerce too.
                if bi.as_str() == "vec_push" && args.len() == 1 {
                    let rv = lower(r, cx);
                    let el = vec_elem_of(&rt);
                    let av = if el.len() > 0 {
                        lower_coerce(&args[0], cx, &el)
                    } else {
                        lower(&args[0], cx)
                    };
                    let mut pa: Vec<H> = Vec::new();
                    pa.push(rv);
                    pa.push(av);
                    let i = fresh(cx);
                    return H::CallH(String::from("vec_push"), pa, i);
                }
                if bi.as_str() == "$cellget" {
                    let lr = lower(r, cx);
                    let z = fresh(cx);
                    let mut ga: Vec<H> = Vec::new();
                    ga.push(lr);
                    ga.push(H::Lit(0, z));
                    // the vector helper by NAME: seq_get is resolved at the E level, not the H level
                    let i = fresh(cx);
                    return H::CallH(String::from("vec_get"), ga, i);
                }
                if bi.as_str() == "$cellset" && args.len() == 1 {
                    let lr = lower(r, cx);
                    let lv = lower(&args[0], cx);
                    let z = fresh(cx);
                    let mut sa: Vec<H> = Vec::new();
                    sa.push(lr);
                    sa.push(H::Lit(0, z));
                    sa.push(lv);
                    let i = fresh(cx);
                    return H::CallH(String::from("vec_set"), sa, i);
                }
                if bi.as_str() == "$strgrow" {
                    let rv = lower(r, cx);
                    let av = lower(&args[0], cx);
                    let arg = if m.as_str() == "push" {
                        let mut p1: Vec<H> = Vec::new();
                        p1.push(av);
                        let i0 = fresh(cx);
                        H::CallH(String::from("chr1"), p1, i0)
                    } else {
                        av
                    };
                    let i1 = fresh(cx);
                    // C-STRGROW: `x.push_str(y)` assigned back to x is the one place where the old handle is
                    // provably dead -- rustc's own borrow rules make the receiver uniquely borrowed -- so the
                    // append may happen IN PLACE. str_cat copies the whole prefix every time, which made
                    // building one string of n bytes cost n*n/2: our own IR emitter appends ~150k lines, and
                    // that quadratic is why the self-host needed 20 GB to compile the composed unit.
                    match r.as_ref() {
                        E::Var(n2, _a2, _b2) => {
                            let mut ca: Vec<H> = Vec::new();
                            ca.push(rv);
                            ca.push(arg);
                            let gr = H::CallH(String::from("str_grow"), ca, i1);
                            let sl = slot_of(cx, n2);
                            let i2 = fresh(cx);
                            if is_cellvar(cx, n2) {
                                let li = fresh(cx);
                                let z = fresh(cx);
                                let mut sa: Vec<H> = Vec::new();
                                sa.push(H::Local(sl, li));
                                sa.push(H::Lit(0, z));
                                sa.push(gr);
                                return H::CallH(String::from("vec_set"), sa, i2);
                            } /* CLOS-MUT: a captured String grows through its cell */
                            return H::AssignH(sl, Box::new(gr), i2);
                        }
                        _ => {
                            // STR-FIELD-GROW: a FIELD/INDEX receiver (`p.name.push(c)`) -- the grown string was computed and DISCARDED.
                            // Store it back: bind the argument to a temporary, then lower `place = str_cat(place, tmp)` through AssignTo.
                            let _ = rv;
                            let _ = i1;
                            match (clone_place(r), clone_place(r)) {
                                (Some(p1), Some(p2)) => {
                                    cx.names.push(String::from("$sgarg"));
                                    cx.stypes.push(String::from("str"));
                                    let ts = (cx.names.len() - 1) as u32;
                                    let mut cargs: Vec<E> = Vec::new();
                                    cargs.push(p2);
                                    cargs.push(E::Var(String::from("$sgarg"), 0, 0));
                                    let asg = E::AssignTo(
                                        Box::new(p1),
                                        Box::new(E::Call(String::from("str_cat"), cargs, 0, 0)),
                                        0,
                                        0,
                                    );
                                    let la = lower(&asg, cx);
                                    let _ = cx.names.pop();
                                    let _ = cx.stypes.pop();
                                    let li = fresh(cx);
                                    return H::LetH(ts, Box::new(arg), Box::new(la), li);
                                }
                                _ => {
                                    let a0: Vec<H> = Vec::new();
                                    let iz = fresh(cx);
                                    return H::CallH(
                                        String::from(
                                            "$push_onto_a_non_place_string_is_not_implemented",
                                        ),
                                        a0,
                                        iz,
                                    );
                                }
                            }
                        }
                    }
                }
                if bi.len() > 0 {
                    // LAZINESS IS OBSERVABLE: rustc runs an adaptor's closure during the consuming call, so a
                    // closure that PRINTS makes the output order part of the answer. Our adaptors are eager,
                    // so such a chain is refused rather than silently reordered.
                    {
                        let mut q = 0;
                        while q < args.len() {
                            match &args[q] {
                                E::ClosRef(cn, _c, _l2, _h2) => {
                                    let b = cn.as_bytes();
                                    if b.len() > 3
                                        && b[b.len() - 3] == 36
                                        && b[b.len() - 2] == 105
                                        && b[b.len() - 1] == 111
                                    {
                                        let mut pr: Vec<H> = Vec::new();
                                        let z = fresh(cx);
                                        pr.push(H::Lit(0, z));
                                        let i = fresh(cx);
                                        return H::CallH(String::from("$lazy_io"), pr, i);
                                    }
                                }
                                _ => {}
                            }
                            q = q + 1;
                        }
                    }
                    // `sum`/`map`/`filter`/`fold` are generic PRELUDE functions, so the call needs the same
                    // monomorphized name any other call to them would get
                    let nm2 = {
                        let mut atys: Vec<String> = Vec::new();
                        atys.push(String::from(rt.as_str()));
                        let mut q = 0;
                        while q < args.len() {
                            atys.push(ety_of(cx, &args[q]));
                            q = q + 1;
                        }
                        mono_name_tys(cx, &bi, &atys)
                    };
                    let mut la: Vec<H> = Vec::new();
                    la.push(lower(r, cx));
                    let mut k = 0;
                    while k < args.len() {
                        la.push(lower(&args[k], cx));
                        k = k + 1;
                    }
                    let i = fresh(cx);
                    return H::CallH(nm2, la, i);
                }
            }
            {
                let tr = dyn_trait_of(&rt);
                if tr.len() > 0 {
                    // bind the receiver (and each argument) to a synthetic local so the dispatch chain can read
                    // them once per branch without re-evaluating the expressions
                    let rv = lower(r, cx);
                    cx.names.push(String::from("$dynrecv"));
                    cx.stypes.push(String::from(rt.as_str()));
                    let rs = (cx.names.len() - 1) as u32;
                    let mut aslots: Vec<u32> = Vec::new();
                    let mut avals: Vec<H> = Vec::new();
                    let mut k = 0;
                    while k < args.len() {
                        let av = lower(&args[k], cx);
                        avals.push(av);
                        cx.names.push(String::from("$dynarg"));
                        cx.stypes.push(String::from(""));
                        aslots.push((cx.names.len() - 1) as u32);
                        k = k + 1;
                    }
                    let tys = dyn_types(cx, &tr);
                    let i0 = fresh(cx);
                    let mut chain = H::Lit(0, i0);
                    let mut q = tys.len();
                    while q > 0 {
                        q = q - 1;
                        let mut cn = String::from(tys[q].as_str());
                        cn.push_str("$");
                        cn.push_str(m);
                        let ri = fresh(cx);
                        let rl = H::Local(rs, ri);
                        let di = fresh(cx);
                        let mut cargs: Vec<H> = Vec::new();
                        cargs.push(H::FieldH(Box::new(rl), 1, di));
                        let mut j = 0;
                        while j < aslots.len() {
                            let aj = fresh(cx);
                            cargs.push(H::Local(aslots[j], aj));
                            j = j + 1;
                        }
                        let ci = fresh(cx);
                        let callh = H::CallH(cn, cargs, ci);
                        let ri2 = fresh(cx);
                        let rl2 = H::Local(rs, ri2);
                        let ti = fresh(cx);
                        let tagh = H::FieldH(Box::new(rl2), 0, ti);
                        let li = fresh(cx);
                        let lith = H::Lit(q as i64, li);
                        let bi = fresh(cx);
                        let test = H::BinH(String::from("=="), Box::new(tagh), Box::new(lith), bi);
                        let ii = fresh(cx);
                        chain = H::IfH(Box::new(test), Box::new(callh), Box::new(chain), ii);
                    }
                    // consume the lowered argument values from the END (H has no Clone), which is exactly the
                    // order that nests the first argument outermost
                    let mut body = chain;
                    let mut w = aslots.len();
                    while w > 0 {
                        w = w - 1;
                        match avals.pop() {
                            Some(vv) => {
                                let wi = fresh(cx);
                                body = H::LetH(aslots[w], Box::new(vv), Box::new(body), wi);
                            }
                            None => {}
                        }
                    }
                    let fi2 = fresh(cx);
                    return H::LetH(rs, Box::new(rv), Box::new(body), fi2);
                }
            }
            // C-UNNAMEDRECV: a receiver whose type lowering cannot NAME still answers the methods that
            // are representation-independent. Without this the mangled name came out as the empty type
            // name plus the method -- `$len`, then `$as_str` -- and nothing ever defined it, so the
            // driver's own IR check refused the program. A str receiver always names itself "str" and an
            // array "arr", so the only receivers that reach here are vector-shaped or genuinely unknown.
            // ORIGINAL NOTE: a receiver whose type lowering cannot NAME still has a length if it is a
            // sequence. Without this the mangled name came out as `$len` -- the empty type name, a `$`
            // and the method -- and nothing ever defined it, so the driver's own IR check refused the
            // program. A str receiver always names itself "str" and an array "arr", so the only receivers
            // that reach here are vector-shaped or genuinely unknown, which is exactly what seq_len
            // dispatches on.
            if rt.len() == 0 && args.len() == 0 {
                // the PASS-THROUGH methods are the identity on any receiver, named or not
                if ident_method(m) {
                    return lower(r, cx);
                }
                if m.as_str() == "parse" {
                    let mut ca: Vec<H> = Vec::new();
                    ca.push(lower(r, cx));
                    let i = fresh(cx);
                    return H::CallH(String::from("sparse"), ca, i);
                }
                if m.as_str() == "sum" || m.as_str() == "count" {
                    let mut ca: Vec<H> = Vec::new();
                    ca.push(lower(r, cx));
                    let i = fresh(cx);
                    return H::CallH(
                        String::from(if m.as_str() == "sum" {
                            "vsum_any"
                        } else {
                            "seq_len"
                        }),
                        ca,
                        i,
                    );
                } /* SEQ-UNNAMED: only a sequence has sum/count */
                /* PARSE-UNNAMED: only str has parse, so an unnamed receiver of it is a str */
                // the MUTATORS on an unnamed receiver. A str receiver always names itself "str", so an
                // unnamed `push`/`pop` is vector-shaped; `push_str` is the one that can only be a str, and
                // it takes the same re-assignment path the named case uses (C8: our String is a value).
                if m.as_str() == "push" && args.len() == 1 {
                    let lr = lower(r, cx);
                    let la = lower(&args[0], cx);
                    let mut a2: Vec<H> = Vec::new();
                    a2.push(lr);
                    a2.push(la);
                    let iq = fresh(cx);
                    return H::CallH(String::from("vec_push"), a2, iq);
                }
                if m.as_str() == "pop" && args.len() == 0 {
                    let lr = lower(r, cx);
                    let mut a2: Vec<H> = Vec::new();
                    a2.push(lr);
                    let iq = fresh(cx);
                    return H::CallH(String::from("vec_popraw"), a2, iq);
                }
                if m.as_str() == "len" {
                    let lr = lower(r, cx);
                    let mut a1: Vec<H> = Vec::new();
                    a1.push(lr);
                    let iq = fresh(cx);
                    return H::CallH(String::from("vec_len"), a1, iq);
                }
            }
            let mut nm = String::from(rt.as_str());
            nm.push_str("$");
            nm.push_str(m);
            // the same fallback `ety_of` already applies when deriving the RETURN type: a receiver whose
            // concrete type is not known (an `impl Trait` parameter, a lifted closure's parameter) still
            // dispatches when the method name is unambiguous. Without it the CALL named `$m` while the
            // return type resolved fine -- one rule, and only one of its two sites had it.
            if fidx_of(cx, &nm) < 0 {
                let u = unique_method(cx, m);
                if u.len() > 0 {
                    nm = u;
                }
            }
            // G1 G-P3 GENERIC-IMPLS: a method of a GENERIC impl (`impl<T: Clone> Tw for Vec<T>`: `Vec$T$tw`) is instanced per
            // receiver type -- the call named the generic method itself, which (its self type mentioning T) is never emitted
            {
                let fi = fidx_of(cx, &nm);
                if fi >= 0 && (fi as usize) < cx.fgps.len() && cx.fgps[fi as usize].len() > 0 {
                    let mut atys: Vec<String> = Vec::new();
                    atys.push(String::from(rt.as_str()));
                    let mut q = 0;
                    while q < args.len() {
                        atys.push(ety_of(cx, &args[q]));
                        q = q + 1;
                    }
                    let mn = mono_name_tys(cx, &nm, &atys);
                    if mn.len() > 0 && mn.as_str() != nm.as_str() {
                        nm = mn;
                    } else {
                        // G1 G-P3 part 2: a method of a generic USER struct (`impl<T: Debug> W<T>`) -- T is not in the
                        // parameter types (self is the base `W`), so it comes from the receiver's GEN-INST (`W:T=int`)
                        let gi = ginst_of(cx, r);
                        let gps = split_c(&cx.fgps[fi as usize]);
                        let mut ts: Vec<String> = Vec::new();
                        let mut g = 0;
                        while g < gps.len() {
                            let v = ginst_val(gi.as_str(), gps[g].as_str());
                            if v.len() == 0 {
                                ts = Vec::new();
                                break;
                            }
                            ts.push(v);
                            g = g + 1;
                        }
                        if ts.len() > 0 {
                            let ex = mono_explicit(cx, &nm, &ts);
                            if ex.len() > 0 {
                                nm = ex;
                            }
                        }
                    }
                }
            }
            let mut la: Vec<H> = Vec::new();
            la.push(lower(r, cx));
            let mut k = 0;
            while k < args.len() {
                la.push(lower(&args[k], cx));
                k = k + 1;
            }
            let i = fresh(cx);
            H::CallH(nm, la, i)
        }
        E::AssignTo(pl, v, _l, _h) => {
            match &**pl {
                // P-08-MUTRET C2-R3: `*p = v` where p is a reference local -- parse turned `*p` into
                // load_at(p); here the assignment stores through the same address.
                E::Call(ln, la2, _la, _lb) if ln.as_str() == "load_at" && la2.len() == 1 => {
                    let addr = lower(&la2[0], cx);
                    let lv = lower(v, cx);
                    let mut sa: Vec<H> = Vec::new();
                    sa.push(addr);
                    sa.push(lv);
                    let i = fresh(cx);
                    H::CallH(String::from("store_at"), sa, i)
                }
                // P-08-MUTRET C2-R2: `*id(&mut place)=v` where id is an identity passthrough (returns its
                // &mut param). *id(&mut place) IS place, so inline: store v straight into place. Sound -- id
                // is pure identity, skipping the call changes nothing; the &mut arg marks place semantics.
                E::Call(n, a2, _ca, _cb)
                    if is_pass_fn(cx, n) && a2.len() == 1 && is_ref_mut(&a2[0]) =>
                {
                    match &a2[0] {
                        E::Ref(inner, _m, _rl, _rh) => {
                            match &**inner {
                                E::Var(vn, _a, _b2) => {
                                    let lv = lower(v, cx);
                                    let sl = slot_of(cx, vn);
                                    let i = fresh(cx);
                                    if is_cellvar(cx, vn) {
                                        let li = fresh(cx);
                                        let z = fresh(cx);
                                        let mut sa: Vec<H> = Vec::new();
                                        sa.push(H::Local(sl, li));
                                        sa.push(H::Lit(0, z));
                                        sa.push(lv);
                                        return H::CallH(String::from("vec_set"), sa, i);
                                    }
                                    H::AssignH(sl, Box::new(lv), i)
                                } // CLOS-MUT
                                E::Index(b, ix, _a, _b2) if is_vec_ty(&ety_of(cx, b)) => {
                                    let lb = lower(b, cx);
                                    let li = lower(ix, cx);
                                    let lv = lower(v, cx);
                                    let mut ag: Vec<H> = Vec::new();
                                    ag.push(lb);
                                    ag.push(li);
                                    ag.push(lv);
                                    let i = fresh(cx);
                                    H::CallH(String::from("vec_set"), ag, i)
                                }
                                E::Index(b, ix, _a, _b2) => {
                                    let lb = lower(b, cx);
                                    let li = lower(ix, cx);
                                    let lv = lower(v, cx);
                                    let i = fresh(cx);
                                    H::StoreIdxH(Box::new(lb), Box::new(li), Box::new(lv), i)
                                }
                                E::Field(o, f, _a, _b2) => {
                                    let sn = ety_of(cx, o);
                                    let idx = oast::sfield_idx(&cx.sdefs, &sn, f);
                                    if idx < 0 {
                                        let a0: Vec<H> = Vec::new();
                                        let i = fresh(cx);
                                        H::CallH(String::from("$ref_unsupported"), a0, i)
                                    } else {
                                        let ix2 = idx as u32;
                                        let lo2 = lower(o, cx);
                                        let lv = lower(v, cx);
                                        let i = fresh(cx);
                                        H::StoreFieldH(Box::new(lo2), ix2, Box::new(lv), i)
                                    }
                                }
                                _ => {
                                    let a0: Vec<H> = Vec::new();
                                    let i = fresh(cx);
                                    H::CallH(String::from("$ref_unsupported"), a0, i)
                                }
                            }
                        }
                        _ => {
                            let a0: Vec<H> = Vec::new();
                            let i = fresh(cx);
                            H::CallH(String::from("$ref_unsupported"), a0, i)
                        }
                    }
                }
                // P-08-MUTRET R2 (compound, C2-R1): `*f(..) op= rhs` where f returns &mut v[i]/&mut p.x.
                // reparse_place refused the Call place ($noplace) but kept the call in the rhs Bin. Lower with
                // a single-eval place-temp: bind the ADDRESS once, then load+op+store through it. Sound -- one
                // evaluation of f, and no reference value escapes a let-binding (that stays the m2/m5 wall).
                E::Var(np, _na, _nb) if np.as_str() == "$noplace" => match &**v {
                    E::Bin(op, lhsx, rhsx, _lb, _hb) => match &**lhsx {
                        E::Call(cn, cargs, _lc, _hc) if is_addr_fn(cx, cn) => {
                            cx.names.push(String::from("$placetmp"));
                            cx.stypes.push(String::from("int"));
                            let slot = (cx.names.len() - 1) as u32;
                            let mut ar: Vec<H> = Vec::new();
                            let mut k = 0;
                            while k < cargs.len() {
                                ar.push(lower(&cargs[k], cx));
                                k = k + 1;
                            }
                            let ci = fresh(cx);
                            let addr = H::CallH(String::from(cn.as_str()), ar, ci);
                            let l1 = fresh(cx);
                            let mut la: Vec<H> = Vec::new();
                            la.push(H::Local(slot, l1));
                            let ldi = fresh(cx);
                            let loadh = H::CallH(String::from("load_at"), la, ldi);
                            let lr = lower(rhsx, cx);
                            let bi = fresh(cx);
                            let newv = H::BinH(
                                String::from(op.as_str()),
                                Box::new(loadh),
                                Box::new(lr),
                                bi,
                            );
                            let l2 = fresh(cx);
                            let mut sa: Vec<H> = Vec::new();
                            sa.push(H::Local(slot, l2));
                            sa.push(newv);
                            let si = fresh(cx);
                            let storeh = H::CallH(String::from("store_at"), sa, si);
                            let li = fresh(cx);
                            H::LetH(slot, Box::new(addr), Box::new(storeh), li)
                        }
                        E::MethodCall(cr, cm, ca, _lc, _hc)
                            if cm.as_str() == "borrow_mut"
                                && ca.len() == 0
                                && is_vec_ty(&ety_of(cx, cr)) =>
                        {
                            // R4 REFCELL: `*c.borrow_mut() op= rhs` arrives as $noplace = Bin(op, c.borrow_mut(), rhs): store
                            // op(get, rhs) back into the cell
                            let cur = lower(lhsx, cx);
                            let lr = lower(rhsx, cx);
                            let bi = fresh(cx);
                            let newv =
                                H::BinH(String::from(op.as_str()), Box::new(cur), Box::new(lr), bi);
                            let mut sa: Vec<H> = Vec::new();
                            sa.push(lower(cr, cx));
                            let z = fresh(cx);
                            sa.push(H::Lit(0, z));
                            sa.push(newv);
                            let si = fresh(cx);
                            H::CallH(String::from("vec_set"), sa, si) // the cell write (slot 0), as CLOS-MUT does
                        }
                        _ => {
                            let a2: Vec<H> = Vec::new();
                            let i = fresh(cx);
                            H::CallH(String::from("$ref_unsupported"), a2, i)
                        }
                    },
                    _ => {
                        let a2: Vec<H> = Vec::new();
                        let i = fresh(cx);
                        H::CallH(String::from("$ref_unsupported"), a2, i)
                    }
                },
                E::Call(n, a2, _a, _b2) if is_gname(n) && a2.len() == 0 => {
                    let lv = lower(v, cx);
                    let mut sa: Vec<H> = Vec::new();
                    sa.push(lv);
                    let mut sn = String::from("$gstore$");
                    sn.push_str(gname_of(n).as_str());
                    let i = fresh(cx);
                    H::CallH(sn, sa, i)
                }
                E::Call(n2, a3, _a4, _b4) if is_addr_fn(cx, n2) => {
                    let mut ar: Vec<H> = Vec::new();
                    let mut k = 0;
                    while k < a3.len() {
                        ar.push(lower(&a3[k], cx));
                        k = k + 1;
                    }
                    let ci = fresh(cx);
                    let callh = H::CallH(String::from(n2.as_str()), ar, ci);
                    let lv = lower(v, cx);
                    let mut sa: Vec<H> = Vec::new();
                    sa.push(callh);
                    sa.push(lv);
                    let i = fresh(cx);
                    H::CallH(String::from("store_at"), sa, i)
                }
                E::MethodCall(r, m, a3, _a4, _b4)
                    if m.as_str() == "borrow_mut" && a3.len() == 0 && is_vec_ty(&ety_of(cx, r)) =>
                {
                    // R4 REFCELL: `*c.borrow_mut() = v` / `op= v` stores into the cell (the read side already maps to $cellget)
                    let mut sa: Vec<H> = Vec::new();
                    sa.push(lower(r, cx));
                    let z = fresh(cx);
                    sa.push(H::Lit(0, z));
                    sa.push(lower(v, cx));
                    let i = fresh(cx);
                    H::CallH(String::from("vec_set"), sa, i)
                }
                E::MethodCall(r, m, _a3, _a4, _b4) => {
                    let rty = ety_of(cx, r);
                    let mut cand = String::from(rty.as_str());
                    cand.push_str("$");
                    cand.push_str(m.as_str());
                    if is_addr_fn(cx, &cand) {
                        let addr = lower(pl, cx);
                        let lv = lower(v, cx);
                        let mut sa: Vec<H> = Vec::new();
                        sa.push(addr);
                        sa.push(lv);
                        let i = fresh(cx);
                        H::CallH(String::from("store_at"), sa, i)
                    }
                    // P-08-MUTRET R3: *obj.m()=v where m returns &mut self.field -- the method dispatch yields the ADDRESS (single eval), store through it
                    else {
                        let a2: Vec<H> = Vec::new();
                        let i = fresh(cx);
                        H::CallH(String::from("$ref_unsupported"), a2, i)
                    }
                }
                E::Call(_n2, _a3, _a4, _b4) => {
                    let a2: Vec<H> = Vec::new();
                    let i = fresh(cx);
                    H::CallH(String::from("$ref_unsupported"), a2, i)
                } // P-08-MUTRET R1: assign through a non-address-returning call (m5 scalar param) unsupported
                E::Index(b, ix, _a, _b2) if is_vec_ty(&ety_of(cx, b)) => {
                    let lb = lower(b, cx);
                    let li = lower(ix, cx);
                    let lv = lower(v, cx);
                    let mut ag: Vec<H> = Vec::new();
                    ag.push(lb);
                    ag.push(li);
                    ag.push(lv);
                    let i = fresh(cx);
                    H::CallH(String::from("vec_set"), ag, i)
                }
                E::Index(b, ix, _a, _b2) => {
                    let lb = lower(b, cx);
                    let li = lower(ix, cx);
                    let lv = lower(v, cx);
                    let i = fresh(cx);
                    H::StoreIdxH(Box::new(lb), Box::new(li), Box::new(lv), i)
                }
                E::Field(o, f, _a, _b2) => {
                    let sn = ety_of(cx, o);
                    let idx = oast::sfield_idx(&cx.sdefs, &sn, f);
                    if idx < 0 {
                        let mut mk2 = String::from("$unknown_field$");
                        if sn.len() > 0 {
                            mk2.push_str(sn.as_str());
                        } else {
                            mk2.push_str("_");
                        }
                        mk2.push('$');
                        mk2.push_str(f.as_str());
                        let a2: Vec<H> = Vec::new();
                        let iu = fresh(cx);
                        return H::CallH(mk2, a2, iu);
                    }
                    let ix = idx as u32;
                    let lo2 = lower(o, cx);
                    let lv = lower(v, cx);
                    let i = fresh(cx);
                    H::StoreFieldH(Box::new(lo2), ix, Box::new(lv), i)
                }
                E::Var(n, _a, _b2) => {
                    let lv = lower(v, cx);
                    let s = slot_of(cx, n);
                    let i = fresh(cx);
                    if is_cellvar(cx, n) {
                        let li = fresh(cx);
                        let z = fresh(cx);
                        let mut sa: Vec<H> = Vec::new();
                        sa.push(H::Local(s, li));
                        sa.push(H::Lit(0, z));
                        sa.push(lv);
                        return H::CallH(String::from("vec_set"), sa, i);
                    }
                    H::AssignH(s, Box::new(lv), i)
                } // CLOS-MUT
                // TUP-SLOT-STORE (R6): `t.1 = v` REBUILDS the tuple -- `t = (t.0, v, t.2)` -- and rebinds the base place.
                // Tuples are shared, not copied (TUP-CLONE), so a store INTO one would also change every alias (`let b = t`);
                // a fresh tuple changes only `t`, which is Copy semantics exactly. Needs the arity (the tuple's ety) and a
                // fixed base place (recursing through the AssignTo arms: a struct-held tuple rebuilds then stores the field).
                E::TupIdx(b, ix, _a, _b2)
                    if oparse::tup_elems(&ety_of(cx, b)).len() > 0
                        && oast::tix_resolve(*ix, oparse::tup_elems(&ety_of(cx, b)).len()) >= 0
                        && oast::place_clone(b).is_some() =>
                {
                    let es = oparse::tup_elems(&ety_of(cx, b));
                    let slot = oast::tix_resolve(*ix, es.len()) as usize;
                    cx.stypes.push(ety_of(cx, v));
                    cx.names.push(String::from("$tsv"));
                    let ts = (cx.names.len() - 1) as u32;
                    let lv = lower(v, cx);
                    let mut parts: Vec<E> = Vec::new();
                    let mut k = 0;
                    while k < es.len() {
                        if k == slot {
                            parts.push(E::Var(String::from("$tsv"), 0, 0));
                        } else {
                            match oast::place_clone(b) {
                                Some(pb) => parts.push(E::TupIdx(Box::new(pb), k as u32, 0, 0)),
                                None => parts.push(E::Int(0, 0, 0)),
                            }
                        }
                        k = k + 1;
                    }
                    let whole = match oast::place_clone(b) {
                        Some(pb) => {
                            E::AssignTo(Box::new(pb), Box::new(E::TupLit(parts, 0, 0)), 0, 0)
                        }
                        None => E::Int(0, 0, 0),
                    };
                    let hs = lower(&whole, cx);
                    let _ = cx.names.pop();
                    let _ = cx.stypes.pop();
                    let i = fresh(cx);
                    H::LetH(ts, Box::new(lv), Box::new(hs), i)
                }
                // ASSIGN-UNSUP: any other place (`p.q.1 = v` of unknown arity, ...) has no store here -- it was lowered as just the rhs, a SILENT no-op (a wrong answer); refuse instead. (A tuple store is not simply StoreField: tuples are shared, not copied -- TUP-CLONE.)
                _ => {
                    let a0: Vec<H> = Vec::new();
                    let i = fresh(cx);
                    H::CallH(String::from("$assign_unsupported"), a0, i)
                }
            }
        }
        E::ArrLit(xs, _l, _h) => {
            let mut le: Vec<H> = Vec::new();
            let mut k = 0;
            while k < xs.len() {
                le.push(copy_val(cx, &xs[k]));
                k = k + 1;
            }
            let i = fresh(cx);
            H::ArrLitH(le, i)
        }
        E::Index(b, ix, _l, _h) if oast::str_starts(ety_of(cx, b).as_str(), "HashMap") => {
            // HASHMAP: m[k]
            let f = if ety_of(cx, b).as_str() == "HashMap$str" {
                String::from("hms_idx")
            } else {
                String::from("hmi_idx")
            };
            let mut ca: Vec<H> = Vec::new();
            ca.push(lower(b, cx));
            ca.push(lower(ix, cx));
            let i = fresh(cx);
            H::CallH(f, ca, i)
        }
        E::Index(b, ix, _l, _h) => {
            // a BYTE VIEW reads the string itself
            if ety_of(cx, b).as_str() == "bstr" {
                let lb = lower(b, cx);
                let li = lower(ix, cx);
                let mut ab: Vec<H> = Vec::new();
                ab.push(lb);
                ab.push(li);
                let i = fresh(cx);
                return H::CallH(String::from("str_at"), ab, i);
            }
            // a VECTOR index is vec_get; a fixed array keeps the direct slot read
            if is_vec_ty(&ety_of(cx, b)) {
                let lb = lower(b, cx);
                let li = lower(ix, cx);
                let mut ag: Vec<H> = Vec::new();
                ag.push(lb);
                ag.push(li);
                let i = fresh(cx);
                return H::CallH(String::from("vec_get"), ag, i);
            }
            let lb = lower(b, cx);
            let li = lower(ix, cx);
            let i = fresh(cx);
            H::IndexH(Box::new(lb), Box::new(li), i)
        }
        E::EnumLit(en, v, ps, _l, _h) => {
            let tag = oast::vidx(&cx.edefs, en, v);
            let t2 = if tag < 0 { 0 } else { tag as u32 };
            let mut lp: Vec<H> = Vec::new();
            let mut k = 0;
            while k < ps.len() {
                lp.push(lower(&ps[k], cx));
                k = k + 1;
            }
            {
                let mx = oast::max_payload(&cx.edefs, en);
                while (lp.len() as u32) < mx {
                    let z = fresh(cx);
                    lp.push(H::Lit(0, z));
                }
            }
            let i = fresh(cx);
            H::EnumLitH(t2, lp, i)
        }
        // `x?` desugars HERE, where the value's enum is known: bind it once, then match -- the success
        // variant yields its payload, every other variant RETURNS the value unchanged.
        E::Try(x, _l, _h) => {
            let en = oast::pay_base(&ety_of(cx, x));
            let lv = lower(x, cx);
            cx.names.push(String::from("$try"));
            cx.stypes.push(String::from(en.as_str()));
            let ts = (cx.names.len() - 1) as u32;
            let nv = oast::nvariants(&cx.edefs, &en);
            let mut ha: Vec<HArm> = Vec::new();
            let mut k = 0;
            while k < nv {
                let vn = oast::variant_name(&cx.edefs, &en, k as usize);
                let np = oast::variant_payload_n(&cx.edefs, &en, k as usize);
                let base = cx.names.len() as u32;
                let mut sl: Vec<u32> = Vec::new();
                let mut bi = 0;
                while bi < np {
                    cx.names.push(String::from("$tp"));
                    cx.stypes.push(String::from(""));
                    sl.push(base + bi);
                    bi = bi + 1;
                }
                let body = if oast::is_ok_variant(&vn) && np > 0 {
                    let i2 = fresh(cx);
                    H::Local(base, i2)
                } else {
                    let i3 = fresh(cx);
                    let lv2 = H::Local(ts, i3);
                    let i4 = fresh(cx);
                    H::RetH(Box::new(lv2), i4)
                };
                let mut bp = 0;
                while bp < np {
                    let _ = cx.names.pop();
                    let _ = cx.stypes.pop();
                    bp = bp + 1;
                }
                ha.push(HArm {
                    tag: k,
                    nbinds: np,
                    slots: sl,
                    body: body,
                });
                k = k + 1;
            }
            let mi = fresh(cx);
            let mh = H::MatchH(Box::new(H::Local(ts, fresh(cx))), ha, mi);
            let li = fresh(cx);
            let r = H::LetH(ts, Box::new(lv), Box::new(mh), li);
            let _ = cx.names.pop();
            let _ = cx.stypes.pop();
            r
        }
        E::Match(sc, arms, _l, _h)
            if {
                let en0 = oast::pay_base(&ety_of(cx, sc));
                oast::nvariants(&cx.edefs, &en0) < 0 || arms_have_lit_or_guard(arms)
            } =>
        {
            // IF-CHAIN match: integer literals, `_`, and GUARDS (MatchOp dispatches on a tag and has no
            // fallthrough, so a guarded arm cannot be expressed there). An ENUM scrutinee is still handled
            // exactly: the test is `tag == variant` and the payload bindings are read out of the value's
            // fields (layout [tag, payload..]), so `Some(v) if v > 3` both tests and binds.
            let en0 = oast::pay_base(&ety_of(cx, sc)); // RES-MATCH: a qualified `Result$i64$String` scrutinee is the prelude Result (its arms had tag -1 = wildcard, so the FIRST arm always ran)
                                                       // C-ENFROMVARIANT: a match's scrutinee may have no recorded type -- it is itself a payload
                                                       // binding, or a deref -- and then the matched ENUM was unknown, so every arm binding got no
                                                       // payload type and every field read on one failed. The variant NAMES identify the enum:
                                                       // oast::enum_of_variant has existed all along and neither match site called it.
            let en = if en0.len() > 0 {
                en0
            } else {
                let mut vns: Vec<String> = Vec::new();
                let mut q0 = 0;
                while q0 < arms.len() {
                    vns.push(String::from(arms[q0].variant.as_str()));
                    q0 = q0 + 1;
                }
                {
                    let e1 = oast::enum_of_variants(&cx.edefs, &vns);
                    // EN-ARITY (R2a P4): an untyped scrutinee (P-PAT's `$msN`) whose variant NAME is shared by two enums
                    // (`oast::E::If(c, t, e, l, h)` vs `olex::Tk::If`) had no enum -> no tag test -> the arm ran unconditionally
                    // (ourc compiled by ourc: SIGSEGV in check_e). The arm's payload count breaks the tie.
                    if e1.len() > 0 {
                        e1
                    } else {
                        enum_by_variant_arity(&cx.edefs, arms)
                    }
                }
            };
            // EN-AMBIG (R2a P4): still no enum, yet an arm names a variant some enum declares -- the tie (same name AND arity in two
            // enums) cannot be broken here (the pattern's written enum is not kept), and without an enum the arm would run
            // UNCONDITIONALLY (a WRONG answer). Refuse instead.
            if en.len() == 0 && arms_name_a_known_variant(&cx.edefs, arms) {
                let a0: Vec<H> = Vec::new();
                let i0 = fresh(cx);
                return H::CallH(String::from("$match_on_an_ambiguous_variant_name"), a0, i0);
            }
            // EN-CANON (R2a P4): the scrutinee's type names the enum as WRITTEN (`E`), but in module mode it is registered under its
            // qualified name (`oast$E`); nvariants / vidx match names exactly, so `is_en` was false, no tag test was emitted, and
            // `if let E::If(..) = el` ran its arm UNCONDITIONALLY (ourc compiled by ourc: garbage fields -> SIGSEGV in check_e).
            // eidx resolves the written name (a unique suffix) -- use the registered name.
            let en = if oast::nvariants(&cx.edefs, &en) < 0 && oast::eidx(&cx.edefs, &en) >= 0 {
                String::from(cx.edefs[oast::eidx(&cx.edefs, &en) as usize].name.as_str())
            } else {
                en
            };
            let is_en = oast::nvariants(&cx.edefs, &en) >= 0;
            let lv = lower(sc, cx);
            cx.names.push(String::from("$mscrut"));
            cx.stypes.push(String::from(en.as_str()));
            let ss2 = (cx.names.len() - 1) as u32;
            let z0 = fresh(cx);
            let mut chain = H::Lit(0, z0);
            let mut q = arms.len();
            while q > 0 {
                q = q - 1;
                let nb = arms[q].binds.len();
                // bind the payloads first so the guard and the body can both see them
                let base = cx.names.len() as u32;
                let mut bi = 0;
                while bi < nb {
                    cx.names.push(String::from(arms[q].binds[bi].as_str()));
                    cx.stypes.push({
                        let qt = if bi == 0 {
                            qual_pay(&qual_scrut(cx, sc), arms[q].variant.as_str())
                        } else {
                            String::from("")
                        };
                        if qt.len() > 0 {
                            qt
                        } else {
                            oast::vpayload_ty(&cx.edefs, &en, &arms[q].variant, bi)
                        }
                    }); /* RES-MATCH-PAY on the IF-CHAIN path (guarded arms) */
                    bi = bi + 1;
                }
                let body0 = lower(&arms[q].body, cx);
                let mut test = H::Lit(1, fresh(cx));
                if arms[q].has_slit {
                    let si = fresh(cx);
                    let sv = H::Local(ss2, si);
                    let li = fresh(cx);
                    let lit = H::StrH(String::from(arms[q].slit.as_str()), li);
                    let mut ag: Vec<H> = Vec::new();
                    ag.push(sv);
                    ag.push(lit);
                    let ci = fresh(cx);
                    test = H::CallH(String::from("streq"), ag, ci);
                } else if arms[q].has_lit {
                    let li = fresh(cx);
                    let lit = H::Lit(arms[q].lit, li);
                    let si = fresh(cx);
                    let sv = H::Local(ss2, si);
                    let b2 = fresh(cx);
                    test = H::BinH(String::from("=="), Box::new(sv), Box::new(lit), b2);
                } else if is_en && !arms[q].wild {
                    let tg = oast::vidx(&cx.edefs, &en, &arms[q].variant);
                    let si = fresh(cx);
                    let sv = H::Local(ss2, si);
                    let f0 = fresh(cx);
                    let tagh = H::FieldH(Box::new(sv), 0, f0);
                    let li = fresh(cx);
                    let lit = H::Lit(tg as i64, li);
                    let b2 = fresh(cx);
                    test = H::BinH(String::from("=="), Box::new(tagh), Box::new(lit), b2);
                }
                if arms[q].has_guard {
                    let g = lower(&arms[q].guard, cx);
                    let hasprev = arms[q].has_lit || arms[q].has_slit || (is_en && !arms[q].wild);
                    if hasprev {
                        let ii = fresh(cx);
                        let zz = H::Lit(0, fresh(cx));
                        test = H::IfH(Box::new(test), Box::new(g), Box::new(zz), ii);
                    } else {
                        test = g;
                    }
                }
                let mut bp = 0;
                while bp < nb {
                    let _ = cx.names.pop();
                    let _ = cx.stypes.pop();
                    bp = bp + 1;
                }
                // the payload reads wrap TEST AND BODY: a guard like `S(v) if v > 100` must see `v`
                let ii2 = fresh(cx);
                let mut arm = H::IfH(Box::new(test), Box::new(body0), Box::new(chain), ii2);
                let mut w = nb;
                while w > 0 {
                    w = w - 1;
                    let si = fresh(cx);
                    let sv = H::Local(ss2, si);
                    let f1 = fresh(cx);
                    let fld = H::FieldH(Box::new(sv), (w + 1) as u32, f1);
                    let wi = fresh(cx);
                    arm = H::LetH(base + (w as u32), Box::new(fld), Box::new(arm), wi);
                }
                chain = arm;
            }
            let li2 = fresh(cx);
            let r = H::LetH(ss2, Box::new(lv), Box::new(chain), li2);
            let _ = cx.names.pop();
            let _ = cx.stypes.pop();
            r
        }
        E::Match(sc, arms, _l, _h) if arms_str_vs_int(cx, sc, arms) => {
            // OPT-PAY-ARM (A2, E0308): one arm an int (`Some(v) => v` of an Option<int>), another a str (`None => ""`) --
            // typeck has no Option payload type, so it was accepted, lowered with one result type, and SEGFAULTED. Refused.
            let i0 = fresh(cx);
            H::CallH(
                String::from("$match_arms_have_incompatible_types"),
                Vec::new(),
                i0,
            )
        }
        E::Match(sc, arms, _l, _h) => {
            let ls = lower(sc, cx);
            let en0 = oast::pay_base(&ety_of(cx, sc)); // RES-MATCH: a qualified `Result$i64$String` scrutinee is the prelude Result (its arms had tag -1 = wildcard, so the FIRST arm always ran)
                                                       // C-ENFROMVARIANT: a match's scrutinee may have no recorded type -- it is itself a payload
                                                       // binding, or a deref -- and then the matched ENUM was unknown, so every arm binding got no
                                                       // payload type and every field read on one failed. The variant NAMES identify the enum:
                                                       // oast::enum_of_variant has existed all along and neither match site called it.
            let en = if en0.len() > 0 {
                en0
            } else {
                let mut vns: Vec<String> = Vec::new();
                let mut q0 = 0;
                while q0 < arms.len() {
                    vns.push(String::from(arms[q0].variant.as_str()));
                    q0 = q0 + 1;
                }
                {
                    let e1 = oast::enum_of_variants(&cx.edefs, &vns);
                    // EN-ARITY (R2a P4): an untyped scrutinee (P-PAT's `$msN`) whose variant NAME is shared by two enums
                    // (`oast::E::If(c, t, e, l, h)` vs `olex::Tk::If`) had no enum -> no tag test -> the arm ran unconditionally
                    // (ourc compiled by ourc: SIGSEGV in check_e). The arm's payload count breaks the tie.
                    if e1.len() > 0 {
                        e1
                    } else {
                        enum_by_variant_arity(&cx.edefs, arms)
                    }
                }
            };
            // EN-AMBIG (R2a P4): still no enum, yet an arm names a variant some enum declares -- the tie (same name AND arity in two
            // enums) cannot be broken here (the pattern's written enum is not kept), and without an enum the arm would run
            // UNCONDITIONALLY (a WRONG answer). Refuse instead.
            if en.len() == 0 && arms_name_a_known_variant(&cx.edefs, arms) {
                let a0: Vec<H> = Vec::new();
                let i0 = fresh(cx);
                return H::CallH(String::from("$match_on_an_ambiguous_variant_name"), a0, i0);
            }
            // EN-CANON (R2a P4): the scrutinee's type names the enum as WRITTEN (`E`), but in module mode it is registered under its
            // qualified name (`oast$E`); nvariants / vidx match names exactly, so `is_en` was false, no tag test was emitted, and
            // `if let E::If(..) = el` ran its arm UNCONDITIONALLY (ourc compiled by ourc: garbage fields -> SIGSEGV in check_e).
            // eidx resolves the written name (a unique suffix) -- use the registered name.
            let en = if oast::nvariants(&cx.edefs, &en) < 0 && oast::eidx(&cx.edefs, &en) >= 0 {
                String::from(cx.edefs[oast::eidx(&cx.edefs, &en) as usize].name.as_str())
            } else {
                en
            };
            let mut ha: Vec<HArm> = Vec::new();
            let mut k = 0;
            while k < arms.len() {
                let a = &arms[k];
                let tg = if a.wild {
                    -1
                } else {
                    oast::vidx(&cx.edefs, &en, &a.variant)
                };
                // bind payload names as fresh LOCAL slots, visible in the arm body
                let base = cx.names.len() as u32;
                let mut sl: Vec<u32> = Vec::new();
                let mut bi = 0;
                while bi < a.binds.len() {
                    cx.names.push(String::from(a.binds[bi].as_str()));
                    cx.stypes.push({
                        let qt = qual_pay(&qual_scrut(cx, sc), a.variant.as_str());
                        if qt.len() > 0 {
                            qt
                        } else {
                            oast::vpayload_ty(&cx.edefs, &en, &a.variant, bi)
                        }
                    });
                    sl.push(base + (bi as u32));
                    bi = bi + 1;
                }
                let lb = lower(&a.body, cx);
                let mut bp = 0;
                while bp < a.binds.len() {
                    let _ = cx.names.pop();
                    let _ = cx.stypes.pop();
                    bp = bp + 1;
                }
                ha.push(HArm {
                    tag: tg,
                    nbinds: a.binds.len() as u32,
                    slots: sl,
                    body: lb,
                });
                k = k + 1;
            }
            let i = fresh(cx);
            H::MatchH(Box::new(ls), ha, i)
        }
        E::Not(x, _l, _h) => {
            // C-BITNOT: on an INTEGER operand `!` is the bitwise not, which two's complement gives as
            // 0-x-1; on a bool it stays the logical not, x==0. Telling them apart needs the operand's
            // kind, which C-PSFILTER and C-ETYARITH made available.
            let tx = ety_of(cx, x);
            if tx.as_str() == "int" || oast::is_int_width(&tx) {
                let lx = lower(x, cx);
                let z = fresh(cx);
                let zero = H::Lit(0, z);
                let i1 = fresh(cx);
                let neg = H::BinH(String::from("-"), Box::new(zero), Box::new(lx), i1);
                let o = fresh(cx);
                let one = H::Lit(1, o);
                let i2 = fresh(cx);
                let r = H::BinH(String::from("-"), Box::new(neg), Box::new(one), i2);
                // NOT-WIDTH: `!x` on a narrow integer stays in its width (`!0u8` == 255, not -1): truncate back
                if oast::is_int_width(&tx) && tx.as_str() != "i64" {
                    let ic = fresh(cx);
                    return H::CastH(Box::new(r), String::from(tx.as_str()), ic);
                }
                return r;
            }
            let lx = lower(x, cx);
            let z = fresh(cx);
            let zero = H::Lit(0, z);
            let i = fresh(cx);
            H::BinH(String::from("=="), Box::new(lx), Box::new(zero), i)
        }
        E::StructLit(n, fs, _l, _h) => {
            let mut lf: Vec<H> = Vec::new();
            let mut k = 0;
            while k < fs.len() {
                lf.push(lower(&fs[k], cx));
                k = k + 1;
            }
            let i = fresh(cx);
            H::StructLitH(fs.len() as u32, lf, i)
        }
        E::Field(o, f, _l, _h) => {
            let sn = ety_of(cx, o);
            // VIS-FIELD-READ (R2a P6, rustc E0616): reading a PRIVATE field outside the struct's module (and its descendants)
            if field_read_private(cx, sn.as_str(), f.as_str()) {
                let a0: Vec<H> = Vec::new();
                let i0 = fresh(cx);
                return H::CallH(String::from("$private_field_E0616"), a0, i0);
            }
            if oast::str_starts(ety_of(cx, e).as_str(), "$proj$") {
                // PROJ-PICK: an associated-type field whose instance is unknown is REFUSED (it printed an address)
                let a0: Vec<H> = Vec::new();
                let i0 = fresh(cx);
                return H::CallH(
                    String::from("$this_associated_type_projection_is_not_resolved"),
                    a0,
                    i0,
                );
            }
            if const_drop_temp(cx, o) {
                let a0: Vec<H> = Vec::new();
                let i0 = fresh(cx);
                return H::CallH(
                    String::from("$a_field_of_a_drop_const_is_not_modelled"),
                    a0,
                    i0,
                );
            }
            {
                let mut ft = field_temp(o, f.as_str(), sn.as_str(), cx);
                if ft.len() > 0 {
                    return ft.pop().unwrap();
                }
            }
            let idx = oast::sfield_idx(&cx.sdefs, &sn, f);
            // C-FIELDUNKNOWN: a field whose index cannot be found used to read field 0 SILENTLY, in
            // both of the two places this rule lives. That is how a struct built in one module and read
            // in another printed its first field three times: the reader could not find the struct under
            // the name it had, so every field resolved to 0. Emit a marker the IR self-check refuses.
            if idx < 0 {
                let mut mk = String::from("$unknown_field$");
                if sn.len() > 0 {
                    mk.push_str(sn.as_str());
                } else {
                    mk.push_str("_");
                }
                mk.push('$');
                mk.push_str(f.as_str());
                let a: Vec<H> = Vec::new();
                let i = fresh(cx);
                return H::CallH(mk, a, i);
            }
            let ix = idx as u32;
            let lo2 = lower(o, cx);
            let i = fresh(cx);
            H::FieldH(Box::new(lo2), ix, i)
        }
        E::While(c, b, _l, _h) => {
            cx.recv_top = is_mcall(c); // DROP-RECV: a receiver temp in the condition drops at the condition's end
            let lc = lower(c, cx);
            cx.recv_top = false;
            cx.pdrops.push(String::from("|"));
            let lb = lower(b, cx);
            pd_unloop(cx);
            let i = fresh(cx);
            H::WhileH(Box::new(lc), Box::new(lb), i)
        }
        E::Seq(a, b, _l, _h) => {
            cx.recv_top = is_mcall(a); // DROP-RECV: a statement is a full expression (only when it IS the call)
            let la0 = lower(a, cx);
            cx.recv_top = false;
            let la = drop_now(a, la0, cx); // DROP-TEMP: a statement's temporary drops at the statement end
            let lb = lower(b, cx);
            let i = fresh(cx);
            H::SeqH(Box::new(la), Box::new(lb), i)
        }
        E::Assign(n, v, _l, _h) => {
            let lv = copy_val(cx, v);
            let s = slot_of(cx, n);
            let i = fresh(cx);
            if is_cellvar(cx, n) {
                let li = fresh(cx);
                let z = fresh(cx);
                let mut sa: Vec<H> = Vec::new();
                sa.push(H::Local(s, li));
                sa.push(H::Lit(0, z));
                sa.push(lv);
                return H::CallH(String::from("vec_set"), sa, i);
            } // CLOS-MUT: write through the cell
              // SELF-MOVE (self/self-re-assign): `x = x` moves the old value INTO the right-hand side, so nothing is dropped --
              // DROP-REASSIGN dropped it first, and an Rc's count reached 0 under the handle it then stored (a segfault)
            let mut selfmv = false;
            match &**v {
                E::Var(m, _a, _b) => {
                    if m.as_str() == n.as_str() {
                        selfmv = true;
                    }
                }
                _ => {}
            }
            // Only an Rc: copy_val of an Rc variable reads its CONTENTS (y = y became y = *y), so nothing is stored. Any other
            // `x = x` keeps its copy -- the parser writes `self = self` for a `mut self` param so a Copy receiver is COPIED
            // (the no-op for every type made `fn squared(mut self)` mutate the caller's value: two WRONGs, run-pass re-measure)
            if selfmv && is_rcvar(cx, n) {
                return H::Lit(0, i);
            }
            if dvar_live(cx, n) && !selfmv {
                // DROP-REASSIGN: `x = v` evaluates v, drops the OLD value, then stores (rustc: drop 9 before `e`, drop 10 at
                // scope end). Only for a binding never moved out in its scope (dvar_open), so the old value is always live.
                let vt = String::from(cx.stypes[s as usize].as_str());
                let nost: Vec<String> = Vec::new();
                let mut g = drop_glue(n, &nost, vt.as_str(), cx);
                if g.len() > 0 {
                    let ge = g.pop().unwrap();
                    cx.stypes.push(vt);
                    cx.names.push(String::from("$dropn"));
                    let t = (cx.names.len() - 1) as u32;
                    let dh = lower(&ge, cx); // AFTER reserving $dropn: the glue's own loop slot must not alias it (f7 crashed)
                    let _ = cx.names.pop();
                    let _ = cx.stypes.pop();
                    let ti = fresh(cx);
                    let st = H::AssignH(s, Box::new(H::Local(t, ti)), i);
                    let qi = fresh(cx);
                    let sq = H::SeqH(Box::new(dh), Box::new(st), qi);
                    let li = fresh(cx);
                    return H::LetH(t, Box::new(lv), Box::new(sq), li);
                }
            }
            H::AssignH(s, Box::new(lv), i)
        }
        // println!'s desugar emits print_any(x): the primitive is chosen HERE, where the type is known
        E::Call(n, args, _l, _h) if is_addr_fn(cx, n) => {
            let mut ar: Vec<H> = Vec::new();
            let mut k = 0;
            while k < args.len() {
                ar.push(lower(&args[k], cx));
                k = k + 1;
            }
            let i = fresh(cx);
            H::CallH(String::from(n.as_str()), ar, i)
        } // P-08-MUTRET C2-R3: a &mut-place-returning call in VALUE position yields the ADDRESS; a reference local bound to it reads through it via load_at (*p), stores via store_at (*p=v)
        E::Call(n, args, _l, _h)
            if is_pass_fn(cx, n) && args.len() == 1 && is_ref_mut(&args[0]) =>
        {
            let a2: Vec<H> = Vec::new();
            let i = fresh(cx);
            H::CallH(String::from("$ref_unsupported"), a2, i)
        } // P-08-MUTRET C2-R2: a passthrough(&mut place) used as a VALUE (bound then deref) is the m2 wall -- refuse
        E::Call(n, args, _l, _h) if n.as_str() == "seq_set" && args.len() == 3 => {
            let st = ety_of(cx, &args[0]);
            if is_arr_ty(&st) {
                let lb = lower(&args[0], cx);
                let li = lower(&args[1], cx);
                let lv = lower(&args[2], cx);
                let i = fresh(cx);
                return H::StoreIdxH(Box::new(lb), Box::new(li), Box::new(lv), i);
            }
            let mut la: Vec<H> = Vec::new();
            let mut k = 0;
            while k < args.len() {
                la.push(lower(&args[k], cx));
                k = k + 1;
            }
            let i = fresh(cx);
            H::CallH(String::from("vec_set"), la, i)
        }
        // an f64 IS our 64-bit word, so reading its bits is a no-op -- which makes a bit-exact
        // round-trip comparison an ordinary integer `==`
        E::Call(n, args, _l, _h) if n.as_str() == "f64_bits" && args.len() == 1 => {
            lower(&args[0], cx)
        }
        // P-PLACE: a global load/store rides as a NAMED call, the way every other primitive here does
        E::Call(n, args, _l, _h) if n.as_str() == "argc" && args.len() == 0 => {
            let a: Vec<H> = Vec::new();
            let i = fresh(cx);
            H::CallH(String::from("argc"), a, i)
        }
        E::Call(n, args, _l, _h) if n.as_str() == "argv_at" && args.len() == 1 => {
            let mut a: Vec<H> = Vec::new();
            a.push(lower(&args[0], cx));
            let i = fresh(cx);
            H::CallH(String::from("argv_at"), a, i)
        }
        E::Call(n, args, _l, _h) if n.as_str() == "stdio_swap" && args.len() == 1 => {
            let mut a: Vec<H> = Vec::new();
            a.push(lower(&args[0], cx));
            let i = fresh(cx);
            H::CallH(String::from("stdio_swap"), a, i)
        }
        E::Call(n, args, _l, _h) if n.as_str() == "fileok" && args.len() == 1 => {
            let mut a: Vec<H> = Vec::new();
            a.push(lower(&args[0], cx));
            let i = fresh(cx);
            H::CallH(String::from("fileok"), a, i)
        }
        // V1.2 DRIVE-LINK: the driver's `__ourc_system(cmd)` (a -1 stub, so SEM0 can build odrv) is libc `system` when ourc
        // compiles itself -- intercepted by name, as readfile is, so ourc' can run llc + cc for `-o`
        E::Call(n, args, _l, _h) if n.as_str() == "__ourc_system" && args.len() == 1 => {
            let mut a: Vec<H> = Vec::new();
            a.push(lower(&args[0], cx));
            let i = fresh(cx);
            H::CallH(String::from("syscmd"), a, i)
        }
        // V5.3: `__ourc_budget(secs, gb)` arms the compile time and memory limits
        E::Call(n, args, _l, _h) if n.as_str() == "__ourc_budget" && args.len() == 2 => {
            let mut a: Vec<H> = Vec::new();
            a.push(lower(&args[0], cx));
            a.push(lower(&args[1], cx));
            let i = fresh(cx);
            H::CallH(String::from("budget"), a, i)
        }
        // V1.2: `__ourc_writefile(path, text)` (a stub under SEM0) writes a file; answers 0 on success
        E::Call(n, args, _l, _h) if n.as_str() == "__ourc_writefile" && args.len() == 2 => {
            let mut a: Vec<H> = Vec::new();
            a.push(lower(&args[0], cx));
            a.push(lower(&args[1], cx));
            let i = fresh(cx);
            H::CallH(String::from("writefile"), a, i)
        }
        E::Call(n, args, _l, _h) if n.as_str() == "readfile" && args.len() == 1 => {
            let mut a: Vec<H> = Vec::new();
            a.push(lower(&args[0], cx));
            let i = fresh(cx);
            H::CallH(String::from("readfile"), a, i)
        }
        E::Call(n, args, _l, _h) if n.as_str() == "proc_exit" && args.len() == 1 => {
            let mut a: Vec<H> = Vec::new();
            a.push(lower(&args[0], cx));
            let i = fresh(cx);
            H::CallH(String::from("proc_exit"), a, i)
        }
        E::Call(n, args, _l, _h) if is_gname(n) => {
            let mut la: Vec<H> = Vec::new();
            let mut k = 0;
            while k < args.len() {
                la.push(lower(&args[k], cx));
                k = k + 1;
            }
            let i = fresh(cx);
            H::CallH(String::from(n.as_str()), la, i)
        }
        // S11 (probed from RC): `&s[a..b]` on a str is a SUBSTRING. It used to reach the vector slicer
        // with a string pointer, which segfaulted -- a divergence, not a gap.
        // `seq_len` on a STR is `len`: seq_len reads the length slot of a sequence handle, which for a
        // string is the first bytes of its text (the open-end slice `&s[a..]` emitted exactly this)
        // C-MEMTAKE: the DEFAULT of the argument's type -- the empty string, an empty vector, or zero.
        // C-CELL: Cell::new(v) is a one-slot vector: make it, push v, answer the handle
        // FMT-PAD: fmtpad_call(text, width, align, fill) -> fmt_pad; align 0 is the TYPE's default (text left, numbers right)
        E::Call(n, args, _l, _h) if n.as_str() == "fmtpad_call" && args.len() == 4 => {
            let mut al = match &args[2] {
                E::Int(v, _a, _b) => *v,
                _ => 0,
            };
            if al == 0 {
                al = match &args[0] {
                    E::Call(sn, sa, _a, _b) if sn.as_str() == "str_of" && sa.len() == 1 => {
                        let t = ety_of(cx, &sa[0]);
                        if t.as_str() == "str" || t.as_str() == "char" {
                            1
                        } else {
                            2
                        }
                    }
                    _ => 2,
                };
            }
            let mut ca: Vec<H> = Vec::new();
            ca.push(lower(&args[0], cx));
            ca.push(lower(&args[1], cx));
            let z = fresh(cx);
            ca.push(H::Lit(al, z));
            ca.push(lower(&args[3], cx));
            let i = fresh(cx);
            H::CallH(String::from("fmt_pad"), ca, i)
        }
        // RC-IDENTITY (R4c): Rc::new registers a FRESH referent (its pointer is the handle, count 1); anything that may share an
        // identity (a scalar, a literal, a moved binding) is refused until the boxed-Rc rung -- a side table keyed on it would merge counts
        // R4a LAYOUT: `size_of::<T>()` / `align_of::<T>()` of a non-scalar T -- rustc's layout from the turbofish's raw text
        E::Call(n, _args, _l, _h)
            if oast::str_starts(n.as_str(), "$sizeof$")
                || oast::str_starts(n.as_str(), "$alignof$") =>
        {
            let al = oast::str_starts(n.as_str(), "$alignof$");
            let tx = if al {
                String::from(&n[9..])
            } else {
                String::from(&n[8..])
            };
            let tx = gsub_text(cx, tx.as_str()); // G1 G-P1: `size_of::<T>()` in an instance reads ITS T
            let toks = lay_toks(tx.as_str());
            let mut pos: Vec<usize> = vec![0];
            let (sz, a) = layout_at(cx, &toks, &mut pos, 0);
            if sz < 0 || pos[0] != toks.len() {
                let a0: Vec<H> = Vec::new();
                let i0 = fresh(cx);
                return H::CallH(String::from("$size_of_this_type_is_not_modelled"), a0, i0);
            }
            let i = fresh(cx);
            H::Lit(if al { a } else { sz }, i)
        }
        // R4a LAYOUT: `size_of_val(&x)` -- by x's type: a scalar's width, an array's RUN-TIME length x element size, a
        // non-generic user struct's layout. `str` is refused (a `&str` -- its byte length -- and a String -- 24 -- share the
        // name here), as is anything else not modelled exactly.
        E::Call(n, args, _l, _h) if n.as_str() == "$sizeofval" && args.len() == 1 => {
            let x = unref(&args[0]);
            let t = ety_of(cx, x);
            let sw = lay_scalar(t.as_str());
            if sw > 0 {
                let la = lower(x, cx); // evaluated for its effects
                let i = fresh(cx);
                let z = fresh(cx);
                return H::SeqH(Box::new(la), Box::new(H::Lit(sw, z)), i);
            }
            if oast::str_starts(t.as_str(), "arr$") && lay_scalar(&t[4..]) > 0 {
                let ew = lay_scalar(&t[4..]);
                let mut la: Vec<H> = Vec::new();
                la.push(lower(x, cx));
                let i0 = fresh(cx);
                let len = H::CallH(String::from("arr_len"), la, i0);
                let iw = fresh(cx);
                let i = fresh(cx);
                return H::BinH(
                    String::from("*"),
                    Box::new(len),
                    Box::new(H::Lit(ew, iw)),
                    i,
                );
            }
            let (s, _a) = lay_struct(cx, t.as_str(), 0);
            if s >= 0 {
                let la = lower(x, cx);
                let i = fresh(cx);
                let z = fresh(cx);
                return H::SeqH(Box::new(la), Box::new(H::Lit(s, z)), i);
            }
            let a0: Vec<H> = Vec::new();
            let i0 = fresh(cx);
            H::CallH(
                String::from("$size_of_val_of_this_type_is_not_modelled"),
                a0,
                i0,
            )
        }
        // R4a XMUTE: `transmute` reinterprets bits. A scalar is one 64-bit word here (an f64 IS its IEEE bits), so word<->word is
        // the word itself (f64 <-> u64/i64) or a same-width int reinterpretation (`as`); an int <-> `[u8; N]` packs/unpacks
        // little-endian bytes. Any other pair (aggregates, f32, mismatched shapes) is REFUSED -- never a guess.
        E::Call(n, args, _l, _h) if oast::str_starts(n.as_str(), "$xmute$") && args.len() == 1 => {
            let dst = String::from(&n[7..]);
            let src = {
                let s0 = ety_of(cx, &args[0]);
                match unref(&args[0]) {
                    E::ArrLit(xs, _al, _ah) if s0.as_str() == "arr" && xs.len() > 0 => {
                        let mut s = String::from("arr$");
                        s.push_str(ety_of(cx, &xs[0]).as_str());
                        s
                    } // an array literal's element type is its first element's
                    _ => s0,
                }
            };
            let word = |t: &str| {
                oast::is_int_width(t) || t == "int" || t == "char" || t == "bool" || t == "f64"
            };
            let bytes = |t: &str| -> i64 {
                if t == "u8" || t == "i8" || t == "bool" {
                    1
                } else if t == "u16" || t == "i16" {
                    2
                } else if t == "u32" || t == "i32" || t == "char" {
                    4
                } else {
                    8
                }
            };
            let v = |s: &str| E::Var(String::from(s), 0, 0);
            let int = |x: i64| E::Int(x, 0, 0);
            let bin = |o: &str, a: E, b: E| E::Bin(String::from(o), Box::new(a), Box::new(b), 0, 0);
            if word(src.as_str()) && word(dst.as_str()) {
                if src.as_str() == "f64" || dst.as_str() == "f64" {
                    return lower(&args[0], cx); // the word IS the bits
                }
            }
            let xm = {
                let mut s = String::from("$xm");
                s.push_str(&fresh(cx).to_string());
                s
            };
            if word(src.as_str()) && word(dst.as_str()) {
                // same-width int reinterpretation (`u32 -> i32`): the `as` cast of the bound word
                let lv = lower(&args[0], cx);
                cx.stypes.push(String::from(src.as_str()));
                cx.names.push(String::from(xm.as_str()));
                let s = (cx.names.len() - 1) as u32;
                let lb = lower(&E::Cast(Box::new(v(xm.as_str())), dst, 0, 0), cx);
                let _ = cx.names.pop();
                let _ = cx.stypes.pop();
                let i = fresh(cx);
                return H::LetH(s, Box::new(lv), Box::new(lb), i);
            }
            if dst.as_str() == "arr$u8" && word(src.as_str()) && src.as_str() != "f64" {
                let nb = bytes(src.as_str());
                let mut el: Vec<E> = Vec::new();
                let mut k = 0;
                while k < nb {
                    let sh = bin(">>", v(xm.as_str()), int(8 * k));
                    el.push(E::Cast(
                        Box::new(bin("&", sh, int(255))),
                        String::from("u8"),
                        0,
                        0,
                    ));
                    k = k + 1;
                }
                let lv = lower(&args[0], cx);
                cx.stypes.push(src);
                cx.names.push(String::from(xm.as_str()));
                let s = (cx.names.len() - 1) as u32;
                let lb = lower(&E::ArrLit(el, 0, 0), cx);
                let _ = cx.names.pop();
                let _ = cx.stypes.pop();
                let i = fresh(cx);
                return H::LetH(s, Box::new(lv), Box::new(lb), i);
            }
            if src.as_str() == "arr$u8" && word(dst.as_str()) && dst.as_str() != "f64" {
                let nb = bytes(dst.as_str());
                let mut acc = int(0);
                let mut k = 0;
                while k < nb {
                    let b = E::Cast(
                        Box::new(E::Index(Box::new(v(xm.as_str())), Box::new(int(k)), 0, 0)),
                        String::from("u64"),
                        0,
                        0,
                    );
                    acc = bin("|", acc, bin("<<", b, int(8 * k)));
                    k = k + 1;
                }
                let lv = lower(&args[0], cx);
                cx.stypes.push(src);
                cx.names.push(String::from(xm.as_str()));
                let s = (cx.names.len() - 1) as u32;
                let lb = lower(&E::Cast(Box::new(acc), dst, 0, 0), cx);
                let _ = cx.names.pop();
                let _ = cx.stypes.pop();
                let i = fresh(cx);
                return H::LetH(s, Box::new(lv), Box::new(lb), i);
            }
            let a0: Vec<H> = Vec::new();
            let i0 = fresh(cx);
            H::CallH(
                String::from("$transmute_between_these_types_is_not_implemented"),
                a0,
                i0,
            )
        }
        E::Call(n, args, _l, _h) if n.as_str() == "$alias" && args.len() == 1 => {
            lower(unref(&args[0]), cx)
        } // R4f CHAN: the same handle
        // R4f CHAN: a sent value -- the queue is Vec<int>, so only an INTEGER may be sent (else REFUSED)
        // SLIT-BASE-DROP: a struct update from a VARIABLE base partially moves it -- refused when an overridden field needs drop
        E::Call(n, args, _l, _h) if n.as_str() == "$updfromdrop" => {
            // UPD-FROM-DROP (W7, E0509): a struct update moves the base's unwritten fields; from a type with `impl Drop` that is
            // rejected when a moved field is not Copy (it was accepted)
            let mut k = 0;
            while k < args.len() {
                if let E::Field(o, f, _fl, _fh) = &args[k] {
                    let st = ety_of(cx, o);
                    let mut dn = gbase(st.as_str());
                    dn.push_str("$drop");
                    let ft = oast::sfield_ty(&cx.sdefs, st.as_str(), f);
                    let plain = ft == "int"
                        || ft == "bool"
                        || ft == "char"
                        || ft == "f64"
                        || oast::is_int_width(ft.as_str())
                        || copy_struct_ty(cx, ft.as_str());
                    if fn_known(cx, &dn) && !plain {
                        let a0: Vec<H> = Vec::new();
                        let i0 = fresh(cx);
                        return H::CallH(
                            String::from("$cannot_move_out_of_a_type_that_implements_drop"),
                            a0,
                            i0,
                        );
                    }
                }
                k = k + 1;
            }
            let i = fresh(cx);
            H::Lit(0, i)
        }
        E::Call(n, args, _l, _h) if n.as_str() == "$partialmove" => {
            let mut k = 0;
            while k < args.len() {
                if let E::Field(o, f, _fl, _fh) = &args[k] {
                    let st = ety_of(cx, o);
                    let ft = oast::sfield_ty(&cx.sdefs, st.as_str(), f);
                    let nost: Vec<String> = Vec::new();
                    if drop_glue("$pm", &nost, ft.as_str(), cx).len() > 0 {
                        let a0: Vec<H> = Vec::new();
                        let i0 = fresh(cx);
                        return H::CallH(
                            String::from("$a_partial_move_out_of_a_drop_variable_is_not_modelled"),
                            a0,
                            i0,
                        );
                    }
                }
                k = k + 1;
            }
            let i = fresh(cx);
            H::Lit(0, i)
        }
        // SLIT-BASE-DROP: drop ONE field of a binding (the base fields a struct-update literal overrode), through the glue
        E::Call(n, args, _l, _h) if n.as_str() == "$dropfield" && args.len() == 1 => {
            if let E::Field(o, f, _fl, _fh) = &args[0] {
                if let E::Var(b, _vl, _vh) = o.as_ref() {
                    let st = ety_of(cx, o);
                    let ft = oast::sfield_ty(&cx.sdefs, st.as_str(), f);
                    let mut steps: Vec<String> = Vec::new();
                    steps.push(String::from(f.as_str()));
                    let mut g = drop_glue(b, &steps, ft.as_str(), cx);
                    if g.len() > 0 {
                        return lower(&g.pop().unwrap(), cx);
                    }
                }
            }
            let i = fresh(cx);
            H::Lit(0, i)
        }
        E::Call(n, args, _l, _h) if n.as_str() == "$ptrnull" && args.len() == 0 => {
            let i = fresh(cx);
            H::Lit(0, i) // R4b PTR: the null pointer word
        }
        E::Call(n, args, _l, _h) if n.as_str() == "$chanint" && args.len() == 1 => {
            let t = ety_of(cx, &args[0]);
            if oast::is_int_width(t.as_str()) || t.as_str() == "int" {
                return lower(&args[0], cx); // integers only: a bool / char received from a Vec<int> would DISPLAY as a number
            }
            let a0: Vec<H> = Vec::new();
            let i0 = fresh(cx);
            H::CallH(
                String::from("$a_channel_of_non_integer_values_is_not_modelled"),
                a0,
                i0,
            )
        }
        // RC-BOXED: the raw HANDLE of an Rc binding (drop glue decrements the handle, not the boxed value)
        E::Call(n, args, _l, _h) if n.as_str() == "$rcraw" && args.len() == 1 => {
            rc_raw(&args[0], cx)
        }
        // V1b BOX-IDENTITY: `Box::new(v)` where allocation identity is observed -- a one-slot cell (a real allocation; a
        // Box clone is a new one). Not counted like an Rc. A non-scalar payload stays the transparent value.
        E::Call(n, args, _l, _h) if n.as_str() == "$boxnew" && args.len() == 1 => {
            if !rc_scalar_ty(ety_of(cx, &args[0]).as_str()) {
                // a non-scalar payload is its own (heap) value handle; a Box clone of one is a real clone, not the same handle
                if let E::Var(cv, cl, ch) = &args[0] {
                    let ce = E::MethodCall(
                        Box::new(E::Var(String::from(cv.as_str()), *cl, *ch)),
                        String::from("clone"),
                        Vec::new(),
                        *cl,
                        *ch,
                    );
                    return lower(&ce, cx);
                }
                return lower(&args[0], cx);
            }
            let lv = lower(&args[0], cx);
            let an: Vec<H> = Vec::new();
            let i0 = fresh(cx);
            let mk = H::CallH(String::from("vec_new"), an, i0);
            let mut pa: Vec<H> = Vec::new();
            pa.push(mk);
            pa.push(lv);
            let i1 = fresh(cx);
            H::CallH(String::from("vec_push1"), pa, i1)
        }
        // V1b: the cell handle itself (the Box's address), not its value
        E::Call(n, args, _l, _h) if n.as_str() == "$cellhandle" && args.len() == 1 => {
            if let E::Var(cn, _cl, _ch) = &args[0] {
                if is_boxrc(cx, cn.as_str()) {
                    let s = slot_of(cx, cn);
                    let i = fresh(cx);
                    return H::Local(s, i);
                }
            }
            lower(&args[0], cx)
        }
        E::Call(n, args, _l, _h) if n.as_str() == "$rcnew" && args.len() == 1 => {
            if rc_scalar_ty(ety_of(cx, &args[0]).as_str()) {
                // RC-BOXED (R4c rung 2): box the scalar in a one-slot cell; the cell pointer is the counted handle
                let lv = lower(&args[0], cx);
                let an: Vec<H> = Vec::new();
                let i0 = fresh(cx);
                let mk = H::CallH(String::from("vec_new"), an, i0);
                let mut pa: Vec<H> = Vec::new();
                pa.push(mk);
                pa.push(lv);
                let i1 = fresh(cx);
                let cell = H::CallH(String::from("vec_push1"), pa, i1);
                let mut la: Vec<H> = Vec::new();
                la.push(cell);
                let i2 = fresh(cx);
                return H::CallH(String::from("rc_new"), la, i2);
            }
            if !rc_fresh(cx, &args[0]) {
                return lower(&args[0], cx); // not fresh: the UNTRACKED identity handle it always was (strong_count refused below)
            }
            let mut la: Vec<H> = Vec::new();
            la.push(lower(&args[0], cx));
            let i = fresh(cx);
            H::CallH(String::from("rc_new"), la, i)
        }
        E::Call(n, args, _l, _h)
            if (n.as_str() == "$rcclone" || n.as_str() == "$rccount") && args.len() == 1 =>
        {
            let tracked = match unref(&args[0]) {
                E::Var(x, _vl, _vh) => is_rcvar(cx, x) || is_boxrc(cx, x),
                _ => false,
            };
            if !tracked {
                if n.as_str() == "$rcclone" {
                    return lower(unref(&args[0]), cx); // an untracked (scalar) Rc: its clone is a copy
                }
                let a0: Vec<H> = Vec::new();
                let i0 = fresh(cx);
                return H::CallH(
                    String::from("$strong_count_of_an_untracked_Rc_is_not_implemented"),
                    a0,
                    i0,
                );
            }
            let mut la: Vec<H> = Vec::new();
            la.push(rc_raw(&args[0], cx)); // RC-BOXED: the HANDLE (a boxed binding's raw slot)
            let i = fresh(cx);
            let f = if n.as_str() == "$rcclone" {
                "rc_inc"
            } else {
                "rc_count"
            };
            H::CallH(String::from(f), la, i)
        }
        E::Call(n, args, _l, _h) if n.as_str() == "$cellnew" && args.len() == 1 => {
            let lv = lower(&args[0], cx);
            let an: Vec<H> = Vec::new();
            let i0 = fresh(cx);
            let mk = H::CallH(String::from("vec_new"), an, i0);
            let mut pa: Vec<H> = Vec::new();
            pa.push(mk);
            pa.push(lv);
            let i1 = fresh(cx);
            H::CallH(String::from("vec_push1"), pa, i1)
        }
        E::Call(n, args, _l, _h) if n.as_str() == "$zeroof" && args.len() == 1 => {
            let t = ety_of(cx, &args[0]);
            zero_of(cx, &t)
        }
        // C-DEFAULTTY: the same answer for a type given by NAME
        E::Call(n, _args, _l, _h) if zeroty_name(n).len() > 0 => {
            let t = zeroty_name(n);
            zero_of(cx, &t)
        }
        E::Call(n, args, _l, _h)
            if n.as_str() == "seq_len"
                && args.len() == 1
                && ety_of(cx, &args[0]).as_str() == "str" =>
        {
            let lx = lower(&args[0], cx);
            let mut a: Vec<H> = Vec::new();
            a.push(lx);
            let i = fresh(cx);
            H::CallH(String::from("len"), a, i)
        }
        E::Call(n, args, _l, _h)
            if n.as_str() == "seq_slice"
                && args.len() == 3
                && (ety_of(cx, &args[0]).as_str() == "str"
                    || ety_of(cx, &args[0]).as_str() == "bstr") =>
        {
            let ls = lower(&args[0], cx);
            let lo2 = lower(&args[1], cx);
            let hi2 = lower(&args[2], cx);
            let i1 = fresh(cx);
            let cnt = H::BinH(String::from("-"), Box::new(hi2), Box::new(lo2), i1);
            // the offset is needed twice, so it is bound once
            let lo3 = lower(&args[1], cx);
            let mut a: Vec<H> = Vec::new();
            a.push(ls);
            a.push(lo3);
            a.push(cnt);
            let i2 = fresh(cx);
            H::CallH(String::from("str_sub"), a, i2)
        }
        E::Call(n, args, _l, _h) if n.as_str() == "seq_slice" && args.len() == 3 => {
            // a slice of a vector or of an array; both answer with a fresh vector
            let st = ety_of(cx, &args[0]);
            let prim = if is_arr_ty(&st) {
                String::from("arr_slice")
            } else {
                String::from("vec_slice")
            };
            let mut la: Vec<H> = Vec::new();
            let mut k = 0;
            while k < args.len() {
                la.push(lower(&args[k], cx));
                k = k + 1;
            }
            let i = fresh(cx);
            H::CallH(prim, la, i)
        }
        E::Call(n, args, _l, _h)
            if (n.as_str() == "seq_len" && args.len() == 1)
                || (n.as_str() == "seq_get" && args.len() == 2) =>
        {
            // a `for` loop works over a vector, a fixed array, OR a byte view of a str; the primitive is
            // chosen here. The byte view was missing, so `for b in s.as_bytes()` fell through to the
            // VECTOR primitives and read a C string as a vector handle -- accepted, then a crash at run
            // time, which breaks the accepted==ran invariant. (C-SEQBSTR)
            let st = ety_of(cx, &args[0]);
            if st.as_str() == "bstr" {
                let lb = lower(&args[0], cx);
                if n.as_str() == "seq_len" {
                    let mut a: Vec<H> = Vec::new();
                    a.push(lb);
                    let i = fresh(cx);
                    return H::CallH(String::from("len"), a, i);
                }
                let li = lower(&args[1], cx);
                let mut a: Vec<H> = Vec::new();
                a.push(lb);
                a.push(li);
                let i = fresh(cx);
                return H::CallH(String::from("str_at"), a, i);
            }
            let isarr = is_arr_ty(&st);
            let prim = if n.as_str() == "seq_len" {
                if isarr {
                    String::from("arr_len")
                } else {
                    String::from("vec_len")
                }
            } else {
                if isarr {
                    String::from("$arrget")
                } else {
                    String::from("vec_get")
                }
            };
            if prim.as_str() == "$arrget" {
                let lb = lower(&args[0], cx);
                let li = lower(&args[1], cx);
                let i = fresh(cx);
                return H::IndexH(Box::new(lb), Box::new(li), i);
            }
            let mut la: Vec<H> = Vec::new();
            let mut k = 0;
            while k < args.len() {
                la.push(lower(&args[k], cx));
                k = k + 1;
            }
            let i = fresh(cx);
            H::CallH(prim, la, i)
        }
        E::Call(n, args, _l, _h) if n.as_str() == "str_of" && args.len() == 1 => {
            let at = ety_of(cx, &args[0]);
            if at.as_str() == "str" {
                return lower(&args[0], cx);
            }
            if at.as_str() == "char" {
                let mut ca: Vec<H> = Vec::new();
                ca.push(lower(&args[0], cx));
                let i = fresh(cx);
                return H::CallH(String::from("chr_str"), ca, i);
            } // CHR-STR: a char formats as its character, not its code
              // FMT-DISPLAY in format!/to-string position: the user's `fmt` returns the text
            if at.len() > 0 {
                let mut fm = String::from(at.as_str());
                fm.push_str("$fmt");
                let mut has = false;
                let mut q = 0;
                while q < cx.fnames.len() {
                    if cx.fnames[q].as_str() == fm.as_str() {
                        has = true;
                    }
                    q = q + 1;
                }
                if has {
                    let mut fa: Vec<H> = Vec::new();
                    fa.push(lower(&args[0], cx));
                    let z = fresh(cx);
                    fa.push(H::Lit(0, z));
                    let ci = fresh(cx);
                    return H::CallH(fm, fa, ci);
                }
            }
            // STR-OF-TYPED: a padded / format!'d f64 or bool is text of ITS type -- it fell to str_from_int, so `{:>5}` of 3.5
            // printed the raw f64 bits (4615063718147915776) and a bool printed 1/0. An aggregate (Vec, tuple, struct without
            // Display) would print a pointer -- refused.
            if at.as_str() == "f64" {
                let mut ca: Vec<H> = Vec::new();
                ca.push(lower(&args[0], cx));
                let z = fresh(cx);
                ca.push(H::Lit(0, z));
                let i = fresh(cx);
                return H::CallH(String::from("f64_str"), ca, i);
            } // f64_str(v, dbg): 0 = Display (7.0 -> "7")
            if at.as_str() == "bool" {
                let c = lower(&args[0], cx);
                let t1 = fresh(cx);
                let f1 = fresh(cx);
                let i = fresh(cx);
                return H::IfH(
                    Box::new(c),
                    Box::new(H::StrH(String::from("true"), t1)),
                    Box::new(H::StrH(String::from("false"), f1)),
                    i,
                );
            }
            if oast::str_starts(at.as_str(), "Vec")
                || oast::str_starts(at.as_str(), "tup")
                || oast::str_starts(at.as_str(), "arr")
                || oast::sfields(&cx.sdefs, &at) > 0
            {
                let a0: Vec<H> = Vec::new();
                let i = fresh(cx);
                return H::CallH(String::from("$display_of_aggregate_unsupported"), a0, i);
            }
            let mut la: Vec<H> = Vec::new();
            la.push(lower(&args[0], cx));
            let i = fresh(cx);
            H::CallH(String::from("str_from_int"), la, i)
        }
        // FMT-DBG-STR: `{:?}` inside format!/write!: the Debug TEXT of a value as a String. Scalars only (Debug prints straight
        // to stdout for aggregates); an aggregate is REFUSED -- format! used str_of on it and produced an address.
        // DROP-FN: `drop(x)` evaluates x and answers () -- this value model has no destructor to run (DROP-SCOPE handles user Drop impls at scope end)
        E::Call(n, args, _l, _h)
            if n.as_str() == "drop" && args.len() == 1 && !fn_known(cx, "drop") =>
        {
            if let E::Var(x, _vl, _vh) = &args[0] {
                // DROP-FN (W1/R4c): `drop(x)` of a BINDING runs x's glue right here -- may_move already treats the call as a
                // move, so x's scope-end drop is suppressed (exact when unconditional; inside a branch the other path misses
                // its drop -- safe). An Rc binding decrements (RC-IDENTITY).
                let st = {
                    let si = slot_of(cx, x) as usize;
                    if si < cx.stypes.len() && cx.names[si].as_str() == x.as_str() {
                        String::from(cx.stypes[si].as_str())
                    } else {
                        String::new()
                    }
                };
                if st.len() > 0 {
                    let g0 = ginst_vt(cx, x, st.as_str());
                    let gvt = if is_rcvar(cx, x) {
                        let mut r = String::from("Rc$");
                        r.push_str(g0.as_str());
                        r
                    } else {
                        g0
                    };
                    let nost: Vec<String> = Vec::new();
                    let mut g = drop_glue(x, &nost, gvt.as_str(), cx);
                    if g.len() > 0 {
                        let ge = g.pop().unwrap();
                        let dc = lower(&ge, cx);
                        let z = fresh(cx);
                        let i = fresh(cx);
                        return H::SeqH(Box::new(dc), Box::new(H::Lit(0, z)), i);
                    }
                }
            }
            {
                let mut dn = ety_of(cx, &args[0]);
                dn.push_str("$drop");
                if fn_known(cx, &dn) {
                    let a0: Vec<H> = Vec::new();
                    let i0 = fresh(cx);
                    return H::CallH(
                        String::from("$drop_of_a_value_with_a_user_Drop_impl_is_not_implemented"),
                        a0,
                        i0,
                    );
                }
            } /* drop flags (no double drop at scope end) are not modelled -- refuse rather than skip the user Drop */
            let la = lower(&args[0], cx);
            let z = fresh(cx);
            let zl = H::Lit(0, z);
            let i = fresh(cx);
            H::SeqH(Box::new(la), Box::new(zl), i)
        }
        // OVF-NEGABS: `$ovf1(k, x)` -- k=0 overflowing_neg (signed: x == MIN; unsigned: x != 0), k=1 overflowing_abs (signed:
        // x == MIN; unsigned: false). Width/signedness from x's EXPLICIT type (through a negation); an un-sized int is REFUSED.
        E::Call(n, args, _l, _h) if n.as_str() == "$ovf1" && args.len() == 2 => {
            let k = match &args[0] {
                E::Int(v, _a, _b) => *v,
                _ => 0,
            };
            let xt = {
                let t0 = ety_of(cx, &args[1]);
                if oast::is_int_width(&t0) {
                    t0
                } else {
                    match &args[1] {
                        E::Bin(_o, a, b, _l2, _h2) => {
                            let ta = ety_of(cx, a);
                            if oast::is_int_width(&ta) {
                                ta
                            } else {
                                ety_of(cx, b)
                            }
                        }
                        _ => t0,
                    }
                }
            };
            let w: i64 = if xt.as_str() == "u8" || xt.as_str() == "i8" {
                8
            } else if xt.as_str() == "u16" || xt.as_str() == "i16" {
                16
            } else if xt.as_str() == "u32" || xt.as_str() == "i32" {
                32
            } else if xt.as_str() == "u64"
                || xt.as_str() == "i64"
                || xt.as_str() == "usize"
                || xt.as_str() == "isize"
            {
                64
            } else {
                0
            };
            if w == 0 {
                let a0: Vec<H> = Vec::new();
                let i0 = fresh(cx);
                return H::CallH(
                    String::from("$overflowing_op_of_an_unsized_int_is_not_implemented"),
                    a0,
                    i0,
                );
            }
            let signed = oast::str_starts(xt.as_str(), "i");
            let lx = lower(&args[1], cx);
            if !signed {
                if k == 1 {
                    let z = fresh(cx);
                    let _ = lx;
                    return H::Lit(0, z);
                }
                let z = fresh(cx);
                let i = fresh(cx);
                return H::BinH(String::from("!="), Box::new(lx), Box::new(H::Lit(0, z)), i);
            }
            let mn: i64 = if w == 64 {
                0 - 9223372036854775807 - 1
            } else {
                0 - (1i64 << (w - 1))
            };
            let z = fresh(cx);
            let i = fresh(cx);
            H::BinH(String::from("=="), Box::new(lx), Box::new(H::Lit(mn, z)), i)
        }
        // OVF-SHIFT: `$shovf(x, n)` is `n >= BITS` of x's EXPLICIT width (an un-suffixed literal's width is not known here -- refused)
        E::Call(n, args, _l, _h) if n.as_str() == "$shovf" && args.len() == 2 => {
            let xt = {
                let t0 = ety_of(cx, &args[0]);
                if oast::is_int_width(&t0) {
                    t0
                } else {
                    match &args[0] {
                        E::Bin(_o, a, b, _l2, _h2) => {
                            let ta = ety_of(cx, a);
                            if oast::is_int_width(&ta) {
                                ta
                            } else {
                                ety_of(cx, b)
                            }
                        }
                        _ => t0,
                    }
                }
            }; /* `-64i8` is Bin(0 - 64i8): take the operand width */
            let w: i64 = if xt.as_str() == "u8" || xt.as_str() == "i8" {
                8
            } else if xt.as_str() == "u16" || xt.as_str() == "i16" {
                16
            } else if xt.as_str() == "u32" || xt.as_str() == "i32" {
                32
            } else if xt.as_str() == "u64"
                || xt.as_str() == "i64"
                || xt.as_str() == "usize"
                || xt.as_str() == "isize"
            {
                64
            } else {
                0
            };
            if w == 0 {
                let a0: Vec<H> = Vec::new();
                let i0 = fresh(cx);
                return H::CallH(
                    String::from("$overflowing_shift_of_an_unsized_int_is_not_implemented"),
                    a0,
                    i0,
                );
            }
            let ln = lower(&args[1], cx);
            let z = fresh(cx);
            let i = fresh(cx);
            H::BinH(String::from(">="), Box::new(ln), Box::new(H::Lit(w, z)), i)
        }
        E::Call(n, args, _l, _h) if n.as_str() == "str_dbg" && args.len() == 1 => {
            let at = ety_of(cx, &args[0]);
            let la = lower(&args[0], cx);
            let mut ca: Vec<H> = Vec::new();
            ca.push(la);
            if at.as_str() == "bool" {
                let i1 = fresh(cx);
                let t1 = H::StrH(String::from("true"), i1);
                let i2 = fresh(cx);
                let f1 = H::StrH(String::from("false"), i2);
                let i0 = fresh(cx);
                let lb = match ca.pop() {
                    Some(x) => x,
                    None => H::Lit(0, i0),
                };
                let i3 = fresh(cx);
                return H::IfH(Box::new(lb), Box::new(t1), Box::new(f1), i3);
            }
            let f = if at.as_str() == "str" {
                "dbgq_s"
            } else if at.as_str() == "char" {
                "dbgc_s"
            } else if at.as_str() == "int" || oast::is_int_width(&at) {
                "str_from_int"
            } else {
                ""
            };
            if at.as_str() == "f64" {
                let z = fresh(cx);
                ca.push(H::Lit(1, z));
                let i = fresh(cx);
                return H::CallH(String::from("f64_str"), ca, i);
            }
            if f.len() == 0 {
                // G1 FMT-AGG-DEBUG: an aggregate's Debug inside `format!` -- the PRINTING builder (dbg_build, as println!
                // uses) runs with stdout captured into a buffer, whose text is the value (cap_on / cap_off)
                let _ = ca.pop();
                // V4.3b FMT-DBG-SHAPE (refusal > WRONG): the same shape test println!'s `{:?}` makes -- a type whose Debug
                // shape cannot be emitted (a sequence of `()`) captured NOTHING and `format!` answered "" (8898 asserted
                // "[(), ()]"). Refused, as println! already refuses it. (A type-string test, not a second dbg_emit walk: the
                // arena never frees, and an extra walk at every `{:?}` site pushed ourc's self-compile past 26 GB.)
                // V4.3c FMT-DBG-MAP: a HashMap likewise captured nothing (format! gave "" where println! refuses;
                // hashmap-debug-format asserted "{\"one\": 1, \"two\": 2}"). The other std collections are refused upstream.
                if at.contains("unit") || oast::str_starts(at.as_str(), "HashMap") {
                    let mut pr: Vec<H> = Vec::new();
                    let z = fresh(cx);
                    pr.push(H::Lit(0, z));
                    let iu = fresh(cx);
                    return H::CallH(String::from("$dbg_unsupported"), pr, iu);
                }
                let i0 = fresh(cx);
                let on = H::CallH(String::from("cap_on"), Vec::new(), i0);
                let body = dbg_build(cx, &args[0], at.as_str());
                let i1 = fresh(cx);
                let off = H::CallH(String::from("cap_off"), Vec::new(), i1);
                let i2 = fresh(cx);
                let i3 = fresh(cx);
                return H::SeqH(
                    Box::new(on),
                    Box::new(H::SeqH(Box::new(body), Box::new(off), i2)),
                    i3,
                );
            }
            let i = fresh(cx);
            H::CallH(String::from(f), ca, i)
        }
        E::Call(n, args, _l, _h)
            if (n.as_str() == "print_any" || n.as_str() == "print_dbg") && args.len() == 1 =>
        {
            let at0 = ety_of(cx, &args[0]);
            // DBG-PAYLOAD-LIT: `{:?}` of an Option/Result VALUE expression (a `Some((4, 3))` literal, a call) qualifies its payload type here too
            let at = if n.as_str() == "print_dbg"
                && (at0.as_str() == "Option" || at0.as_str() == "Result")
            {
                let p = pay_of(cx, &args[0]);
                if p.len() > 0 {
                    let mut q = String::from(at0.as_str());
                    q.push('$');
                    q.push_str(&p);
                    q
                } else {
                    at0
                }
            } else {
                at0
            };
            // FMT-DISPLAY: `{}` of a type with a user `impl Display` calls its `fmt`. `write!(f, ..)` builds the string (the
            // Formatter sink is ignored), so `fmt` RETURNS the text; print that. The Formatter argument is a dummy 0.
            if n.as_str() == "print_any" && at.len() > 0 {
                let mut fm = String::from(at.as_str());
                fm.push_str("$fmt");
                let mut has = false;
                let mut q = 0;
                while q < cx.fnames.len() {
                    if cx.fnames[q].as_str() == fm.as_str() {
                        has = true;
                    }
                    q = q + 1;
                }
                if has {
                    let mut fa: Vec<H> = Vec::new();
                    fa.push(lower(&args[0], cx));
                    let z = fresh(cx);
                    fa.push(H::Lit(0, z));
                    let ci = fresh(cx);
                    let callf = H::CallH(fm, fa, ci);
                    let mut pa: Vec<H> = Vec::new();
                    pa.push(callf);
                    let pi = fresh(cx);
                    return H::CallH(String::from("print_raw"), pa, pi);
                }
            }
            if n.as_str() == "print_any" && at.as_str() == "char" {
                let mut ca: Vec<H> = Vec::new();
                ca.push(lower(&args[0], cx));
                let ci = fresh(cx);
                let cs = H::CallH(String::from("chr_str"), ca, ci);
                let mut pa: Vec<H> = Vec::new();
                pa.push(cs);
                let pi = fresh(cx);
                return H::CallH(String::from("print_raw"), pa, pi);
            } // UTF8-STR: a char Displays as its UTF-8 encoding
            let prim = if at.as_str() == "str" {
                String::from("print_raw")
            } else if at.as_str() == "char" {
                String::from("print_char")
            } else if at.as_str() == "f64" {
                if n.as_str() == "print_dbg" {
                    String::from("$f64dbg")
                } else {
                    String::from("$f64disp")
                }
            } else if at.as_str() == "u64" || at.as_str() == "usize" {
                String::from("print_uint_raw")
            } else {
                String::from("print_int_raw")
            };
            // the formatter is written in OUR language, in the prelude: it needs a round-trip search and
            // string trimming, neither of which belongs in hand-written IR
            // D1 (probed from RC): `{:?}` QUOTES -- a char as `'a'`, a str as `"hi"`. `{}` does not.
            // D1 (probed from RC): the quotes AND the escaping are the printer's job, and dbg_scalar is where
            // that rule lives -- it was kept here a second time, so a str's `{:?}` did not escape while the
            // same str inside a tuple did
            if n.as_str() == "print_dbg" {
                // R8 RANGE-TO: an int-valued `let x = ..b` binding: `..` then the bound
                if let E::Var(vn, vl, vh) = &args[0] {
                    let rp = rangeto_prefix(cx, vn.as_str());
                    if rp.len() > 0 {
                        let pre = dbg_print(cx, &rp);
                        let mut na: Vec<E> = Vec::new();
                        na.push(E::Cast(
                            Box::new(E::Var(String::from(vn.as_str()), *vl, *vh)),
                            String::from("i64"),
                            *vl,
                            *vh,
                        ));
                        let num = lower(&E::Call(String::from("print_dbg"), na, *vl, *vh), cx);
                        return dbg_seq(cx, pre, num);
                    }
                }
            }
            if n.as_str() == "print_dbg" && (at.as_str() == "char" || at.as_str() == "str") {
                let v0 = lower(&args[0], cx);
                return dbg_scalar(cx, v0, at.as_str());
            }
            if at.as_str() == "bool" {
                let c = lower(&args[0], cx);
                let i1 = fresh(cx);
                let t1 = H::StrH(String::from("true"), i1);
                let i2 = fresh(cx);
                let f1 = H::StrH(String::from("false"), i2);
                let i3 = fresh(cx);
                let pick = H::IfH(Box::new(c), Box::new(t1), Box::new(f1), i3);
                let mut pa: Vec<H> = Vec::new();
                pa.push(pick);
                let i4 = fresh(cx);
                return H::CallH(String::from("print_raw"), pa, i4);
            }
            if n.as_str() == "print_dbg" {
                let d = dbg_emit(cx, &args[0], &at);
                if d.len() > 0 {
                    return dbg_build(cx, &args[0], &at);
                }
                // a non-scalar whose Debug shape we cannot emit must be REFUSED, not printed as a word.
                // an UNKNOWN type (at=="") must ALSO refuse -- otherwise it fell through to print_int_raw and
                // printed a non-int handle (a Vec in a tuple field) as an int: a silent-wrong (tick 3).
                if at.len() == 0
                    || (at.as_str() != "int"
                        && at.as_str() != "bool"
                        && at.as_str() != "str"
                        && at.as_str() != "char"
                        && at.as_str() != "f64"
                        && !oast::is_int_width(&at))
                {
                    let mut pr: Vec<H> = Vec::new();
                    let z = fresh(cx);
                    pr.push(H::Lit(0, z));
                    let i = fresh(cx);
                    return H::CallH(String::from("$dbg_unsupported"), pr, i);
                }
            }
            if prim.as_str() == "$f64disp" || prim.as_str() == "$f64dbg" {
                let dbg = if prim.as_str() == "$f64dbg" { 1 } else { 0 };
                let mut fa: Vec<H> = Vec::new();
                fa.push(lower(&args[0], cx));
                let z = fresh(cx);
                fa.push(H::Lit(dbg, z));
                let i2 = fresh(cx);
                let st = H::CallH(String::from("f64_str"), fa, i2);
                let mut pa: Vec<H> = Vec::new();
                pa.push(st);
                let i3 = fresh(cx);
                return H::CallH(String::from("print_raw"), pa, i3);
            }
            let mut la: Vec<H> = Vec::new();
            la.push(lower(&args[0], cx));
            let i = fresh(cx);
            H::CallH(prim, la, i)
        }
        E::Call(n, args, _l, _h) if n.as_str() == "vec_push" && args.len() == 2 => {
            let vt = ety_of(cx, &args[0]);
            let el = vec_elem_of(&vt);
            let a0 = lower(&args[0], cx);
            let a1 = if el.len() > 0 {
                lower_coerce(&args[1], cx, &el)
            } else {
                lower(&args[1], cx)
            };
            let mut la: Vec<H> = Vec::new();
            la.push(a0);
            la.push(a1);
            let i = fresh(cx);
            H::CallH(String::from("vec_push"), la, i)
        }
        E::Call(n, args, _l, _h) if sty_of(cx, n).as_str() == "Fn" => {
            // the closure's identity is not known statically here, so dispatch on the tag the value carries
            let sl = slot_of(cx, n);
            let mut aslots: Vec<u32> = Vec::new();
            let mut avals: Vec<H> = Vec::new();
            let mut k = 0;
            while k < args.len() {
                let av = lower(&args[k], cx);
                avals.push(av);
                cx.names.push(String::from("$fnarg"));
                cx.stypes.push(String::from(""));
                aslots.push((cx.names.len() - 1) as u32);
                k = k + 1;
            }
            let mut ids: Vec<i32> = Vec::new();
            let mut q = 0;
            let mut idn: Vec<String> = Vec::new();
            while q < cx.fnames.len() {
                let ci = clos_name_ix(cx.fnames[q].as_str());
                if ci >= 0 {
                    ids.push(ci);
                    idn.push(String::from(cx.fnames[q].as_str()));
                }
                q = q + 1;
            } // CLOS-IO-DISPATCH: keep the REAL name (a `$closN$io` is not `$closN`)
            let z0 = fresh(cx);
            let mut chain = H::Lit(0, z0);
            let mut w = ids.len();
            while w > 0 {
                w = w - 1;
                let ci = ids[w];
                let cn = String::from(idn[w].as_str());
                let fi = fidx_of(cx, &cn);
                let nc = if fi >= 0 { cx.fncaps[fi as usize] } else { 0 };
                let np = if fi >= 0 { cx.fnpar[fi as usize] } else { 0 };
                if fi >= 0 && np - nc != (aslots.len() as i32) {
                    w = w;
                } else {
                    let mut ca: Vec<H> = Vec::new();
                    let mut j = 0;
                    while j < nc {
                        let li = fresh(cx);
                        let lv = H::Local(sl, li);
                        let f2 = fresh(cx);
                        ca.push(H::FieldH(Box::new(lv), (j + 1) as u32, f2));
                        j = j + 1;
                    }
                    let mut a2 = 0;
                    while a2 < aslots.len() {
                        let ai = fresh(cx);
                        ca.push(H::Local(aslots[a2], ai));
                        a2 = a2 + 1;
                    }
                    let callid = fresh(cx);
                    let callh = H::CallH(cn, ca, callid);
                    let li2 = fresh(cx);
                    let lv2 = H::Local(sl, li2);
                    let t0 = fresh(cx);
                    let tagh = H::FieldH(Box::new(lv2), 0, t0);
                    let l0 = fresh(cx);
                    let lith = H::Lit(ci as i64, l0);
                    let b0 = fresh(cx);
                    let test = H::BinH(String::from("=="), Box::new(tagh), Box::new(lith), b0);
                    let i0 = fresh(cx);
                    chain = H::IfH(Box::new(test), Box::new(callh), Box::new(chain), i0);
                }
            }
            let mut body = chain;
            let mut v2 = aslots.len();
            while v2 > 0 {
                v2 = v2 - 1;
                match avals.pop() {
                    Some(vv) => {
                        let wi = fresh(cx);
                        body = H::LetH(aslots[v2], Box::new(vv), Box::new(body), wi);
                    }
                    None => {}
                }
            }
            body
        }
        E::Call(n, args, _l, _h) if clos_of(&sty_of(cx, n)).len() > 0 => {
            let cn = clos_of(&sty_of(cx, n));
            let fi = fidx_of(cx, &cn);
            let nc = if fi >= 0 { cx.fncaps[fi as usize] } else { 0 };
            let sl = slot_of(cx, n);
            let mut la: Vec<H> = Vec::new();
            let mut k = 0;
            while k < nc {
                let li = fresh(cx);
                let lv = H::Local(sl, li);
                let fi2 = fresh(cx);
                la.push(H::FieldH(Box::new(lv), (k + 1) as u32, fi2));
                k = k + 1;
            }
            let mut j = 0;
            while j < args.len() {
                la.push(lower(&args[j], cx));
                j = j + 1;
            }
            let i = fresh(cx);
            H::CallH(cn, la, i)
        }
        E::Call(n0, args, _l, _h) => {
            let n = &self_fix(cx, n0);
            let nm = {
                // G1 G-P2: a turbofish call to a GENERIC fn names its instance from the explicit types (`sz::<u64>()`)
                let tf = tf_types(cx, *_l);
                let ex = if tf.len() > 0 {
                    mono_explicit(cx, n, &tf)
                } else {
                    String::new()
                };
                if ex.len() > 0 {
                    ex
                } else {
                    mono_name(cx, n, args)
                }
            };
            let fi = fidx_of(cx, &nm);
            let ptys = if fi >= 0 {
                split_c(cx.fptys[fi as usize].as_str())
            } else {
                Vec::new()
            };
            let mut la: Vec<H> = Vec::new();
            let mut k = 0;
            while k < args.len() {
                let want = if k < ptys.len() {
                    String::from(ptys[k].as_str())
                } else {
                    String::from("")
                };
                // C-ONECOERCE: the argument coercion rule lives in lower_coerce. This site used to keep
                // its own copy of the dyn half and nothing else, which is why the ARRAY-to-slice
                // conversion below had nowhere to be seen from.
                if rc_param(cx, nm.as_str(), k as u32) && rc_is_boxed_var(cx, &args[k]) {
                    la.push(rc_raw(&args[k], cx)); // RC-BOXED: an Rc param receives the HANDLE
                } else if want.len() > 0 {
                    la.push(lower_coerce(&args[k], cx, &want));
                } else {
                    la.push(lower(&args[k], cx));
                }
                k = k + 1;
            }
            let i = fresh(cx);
            H::CallH(nm, la, i)
        }
        E::Int(v, _l, _h) => {
            let i = fresh(cx);
            H::Lit(*v, i)
        }
        E::Bool(v, _l, _h) => {
            let i = fresh(cx);
            H::LitB(*v, i)
        }
        E::Var(n, _l, _h) => {
            let s = slot_of(cx, n);
            let i = fresh(cx);
            if is_cellvar(cx, n) || is_boxrc(cx, n) {
                let z = fresh(cx);
                let mut ga: Vec<H> = Vec::new();
                ga.push(H::Local(s, i));
                ga.push(H::Lit(0, z));
                let gi = fresh(cx);
                return H::CallH(String::from("vec_get"), ga, gi);
            } // CLOS-MUT: read through the cell
            H::Local(s, i)
        }
        E::Bin(o, a, b, _l, _h) => {
            // ORD-UNIT-ENUM: `<` `<=` `>` `>=` between two values of one unit-only enum compares the variant TAGS
            // (derive(PartialOrd) orders by declaration); field 0 of an enum value is its tag
            if o.as_str() == "<" || o.as_str() == "<=" || o.as_str() == ">" || o.as_str() == ">=" {
                let at = ety_of(cx, a);
                let ei = oast::eidx(&cx.edefs, &at);
                if ei >= 0 && ety_of(cx, b).as_str() == at.as_str() && {
                    let ed = &cx.edefs[ei as usize];
                    let mut unit = true;
                    let mut k = 0;
                    while k < ed.variants.len() {
                        if ed.variants[k].npayload > 0 {
                            unit = false;
                        }
                        k = k + 1;
                    }
                    unit
                } {
                    let mut mk = String::from(at.as_str());
                    mk.push_str("$ord_by_index_unsound");
                    if fn_known(cx, &mk) {
                        let a0: Vec<H> = Vec::new();
                        let i0 = fresh(cx);
                        return H::CallH(
                            String::from("$ordering_an_enum_whose_discriminants_are_out_of_declaration_order"),
                            a0,
                            i0,
                        );
                    } // ENUM-DISCR ORD guard (rustc orders by DISCRIMINANT, the tag is the INDEX)
                    let la = lower(a, cx);
                    let lb = lower(b, cx);
                    let fa = fresh(cx);
                    let ta = H::FieldH(Box::new(la), 0, fa);
                    let fb = fresh(cx);
                    let tb = H::FieldH(Box::new(lb), 0, fb);
                    let i = fresh(cx);
                    return H::BinH(String::from(o.as_str()), Box::new(ta), Box::new(tb), i);
                }
            }
            // OPERATOR OVERLOADING: a struct operand whose type has the operator's method (Add::add etc.)
            // routes to `Type$method`. Checked before the integer path, and only when that method exists, so
            // every primitive operation is untouched.
            {
                let at = ety_of(cx, a);
                let m = op_method(o);
                if at.len() > 0 && m.len() > 0 && !prim_ty(at.as_str()) {
                    let mut mn = String::from(at.as_str());
                    mn.push('$');
                    mn.push_str(&m);
                    if fidx_of(cx, &mn) >= 0 {
                        let la = lower(a, cx);
                        let lb = lower(b, cx);
                        let mut ag: Vec<H> = Vec::new();
                        ag.push(la);
                        ag.push(lb);
                        let i = fresh(cx);
                        return H::CallH(mn, ag, i);
                    }
                }
            }
            // AGG-EQ: `==`/`!=` on TUPLES and STRUCTS, at any nesting, when neither side is a plain variable or the shape nests: both
            // sides bound ONCE, then a recursive element/field-wise comparison over typed paths (scalars ==, str by content). Any
            // part it cannot compare (an untyped tuple, a Vec/enum/Option element) REFUSES the whole comparison -- the fallback
            // compared ADDRESSES (`mk(1) == mk(1)`, `((1, 2), 3) == ((1, 2), 3)` were false).
            if (o.as_str() == "==" || o.as_str() == "!=") && {
                let ta = ety_of(cx, a);
                oast::str_starts(ta.as_str(), "tup")
                    || (oast::sfields(&cx.sdefs, &ta) > 0
                        && !(is_simple_place(a) && is_simple_place(b)))
            } {
                let ta = ety_of(cx, a);
                let la = lower(a, cx);
                cx.names.push(String::from("$aea"));
                cx.stypes.push(String::from(ta.as_str()));
                let sa = (cx.names.len() - 1) as u32;
                let lb = lower(b, cx);
                cx.names.push(String::from("$aeb"));
                cx.stypes.push(String::from(""));
                let sb = (cx.names.len() - 1) as u32;
                let path: Vec<u32> = Vec::new();
                let r = agg_eq(cx, sa, sb, &path, ta.as_str());
                let _ = cx.names.pop();
                let _ = cx.stypes.pop();
                let _ = cx.names.pop();
                let _ = cx.stypes.pop();
                match r {
                    None => {
                        let a0: Vec<H> = Vec::new();
                        let i0 = fresh(cx);
                        return H::CallH(
                            String::from("$equality_of_this_aggregate_shape_is_not_implemented"),
                            a0,
                            i0,
                        );
                    }
                    Some(acc) => {
                        let res = if o.as_str() == "!=" {
                            let z = fresh(cx);
                            let i5 = fresh(cx);
                            H::BinH(
                                String::from("=="),
                                Box::new(acc),
                                Box::new(H::Lit(0, z)),
                                i5,
                            )
                        } else {
                            acc
                        };
                        let l2 = fresh(cx);
                        let inner = H::LetH(sb, Box::new(lb), Box::new(res), l2);
                        let l1 = fresh(cx);
                        return H::LetH(sa, Box::new(la), Box::new(inner), l1);
                    }
                }
            }
            // TUP-EQ: `==`/`!=` on two TYPED tuples compares element-wise (str by content) -- it compared ADDRESSES, so `(1, 2) == (1, 2)`
            // was false. Each side is bound once. An element type this cannot compare (nested tuple, struct, Vec, ...) is REFUSED.
            if (o.as_str() == "==" || o.as_str() == "!=")
                && oparse::tup_elems(&ety_of(cx, a)).len() > 0
            {
                let es = oparse::tup_elems(&ety_of(cx, a));
                let mut ok = true;
                let mut q = 0;
                while q < es.len() {
                    let t = es[q].as_str();
                    if !(t == "int"
                        || t == "bool"
                        || t == "char"
                        || t == "str"
                        || oast::is_int_width(t))
                    {
                        ok = false;
                    }
                    q = q + 1;
                }
                if !ok {
                    let a0: Vec<H> = Vec::new();
                    let i0 = fresh(cx);
                    return H::CallH(
                        String::from("$tuple_equality_of_these_element_types_is_not_implemented"),
                        a0,
                        i0,
                    );
                }
                let la = lower(a, cx);
                cx.names.push(String::from("$tea"));
                cx.stypes.push(ety_of(cx, a));
                let sa = (cx.names.len() - 1) as u32;
                let lb = lower(b, cx);
                cx.names.push(String::from("$teb"));
                cx.stypes.push(String::from(""));
                let sb = (cx.names.len() - 1) as u32;
                let it = fresh(cx);
                let mut acc = H::Lit(1, it);
                let mut k = es.len();
                while k > 0 {
                    k = k - 1;
                    let i1 = fresh(cx);
                    let i2 = fresh(cx);
                    let i3 = fresh(cx);
                    let i4 = fresh(cx);
                    let fa = H::FieldH(Box::new(H::Local(sa, i1)), k as u32, i2);
                    let fb = H::FieldH(Box::new(H::Local(sb, i3)), k as u32, i4);
                    let eq = if es[k].as_str() == "str" {
                        let mut ag: Vec<H> = Vec::new();
                        ag.push(fa);
                        ag.push(fb);
                        let ic = fresh(cx);
                        H::CallH(String::from("streq"), ag, ic)
                    } else {
                        let ib = fresh(cx);
                        H::BinH(String::from("=="), Box::new(fa), Box::new(fb), ib)
                    };
                    let ia = fresh(cx);
                    acc = if k + 1 == es.len() {
                        eq
                    } else {
                        H::BinH(String::from("&"), Box::new(eq), Box::new(acc), ia)
                    };
                }
                let res = if o.as_str() == "!=" {
                    let z = fresh(cx);
                    let i5 = fresh(cx);
                    H::BinH(
                        String::from("=="),
                        Box::new(acc),
                        Box::new(H::Lit(0, z)),
                        i5,
                    )
                } else {
                    acc
                };
                let _ = cx.names.pop();
                let _ = cx.stypes.pop();
                let _ = cx.names.pop();
                let _ = cx.stypes.pop();
                let l2 = fresh(cx);
                let inner = H::LetH(sb, Box::new(lb), Box::new(res), l2);
                let l1 = fresh(cx);
                return H::LetH(sa, Box::new(la), Box::new(inner), l1);
            }
            // STRUCT-EQ (derive(PartialEq)): `==`/`!=` on two struct PLACES is the conjunction of field equalities -- the
            // word compare was of two addresses, so a clone was never equal to its source. Fields recurse (str by content).
            if (o.as_str() == "==" || o.as_str() == "!=")
                && oast::sfields(&cx.sdefs, &ety_of(cx, a)) > 0
                && is_simple_place(a)
                && is_simple_place(b)
            {
                let st = oast::sresolve(&cx.sdefs, &ety_of(cx, a));
                let mut si = 0;
                while si < cx.sdefs.len() && cx.sdefs[si].name.as_str() != st.as_str() {
                    si = si + 1;
                }
                if si < cx.sdefs.len() {
                    let mut acc = E::Bool(true, 0, 0);
                    let nf = cx.sdefs[si].fields.len();
                    let mut q = nf;
                    while q > 0 {
                        q = q - 1;
                        let f = String::from(cx.sdefs[si].fields[q].as_str());
                        let fa = E::Field(
                            Box::new(oparse::clone_simple(a, 0, 0)),
                            String::from(f.as_str()),
                            0,
                            0,
                        );
                        let fb = E::Field(Box::new(oparse::clone_simple(b, 0, 0)), f, 0, 0);
                        let eq = E::Bin(String::from("=="), Box::new(fa), Box::new(fb), 0, 0);
                        acc = if q + 1 == nf {
                            eq
                        } else {
                            E::Bin(String::from("&&"), Box::new(eq), Box::new(acc), 0, 0)
                        };
                    }
                    let full = if o.as_str() == "!=" {
                        E::Not(Box::new(acc), 0, 0)
                    } else {
                        acc
                    };
                    return lower(&full, cx);
                }
            }
            // `==`/`!=` on str compares CONTENT, so it lowers to a streq call (strcmp in codegen), not an
            // integer compare of two addresses. Keyed on ety_of, which now reports "str" for a literal, a
            // str-typed param, and a let bound to either.
            // O12: an ENUM compares structurally. Every payload must be word-comparable; a payload we
            // cannot compare that way (a str, whose equality is by CONTENT) is refused, not guessed.
            {
                let ea = ety_of(cx, a);
                let eb = ety_of(cx, b);
                let en = if oast::eidx(&cx.edefs, &ea) >= 0 {
                    ea.clone()
                } else if oast::eidx(&cx.edefs, &eb) >= 0 {
                    eb.clone()
                } else {
                    String::from("")
                };
                if (o.as_str() == "==" || o.as_str() == "!=") && en.len() > 0 {
                    let ei = oast::eidx(&cx.edefs, &en) as usize;
                    let mut wordy = true;
                    {
                        let ed = &cx.edefs[ei];
                        let mut vi = 0;
                        while vi < ed.variants.len() {
                            let mut pi = 0;
                            while pi < ed.variants[vi].ptys.len() {
                                let t = &ed.variants[vi].ptys[pi];
                                if !(t.as_str() == "int"
                                    || t.as_str() == "bool"
                                    || t.as_str() == "char"
                                    || oast::is_int_width(t)
                                    || t.len() == 0)
                                {
                                    wordy = false;
                                }
                                pi = pi + 1;
                            }
                            vi = vi + 1;
                        }
                    }
                    if gbase(en.as_str()).as_str() == "Option"
                        || gbase(en.as_str()).as_str() == "Result"
                    {
                        // OPT-EQ-PAY (W7): word-compare only SCALAR payloads -- an enum / struct / str payload is a HANDLE, so
                        // `Some(T::Foo) == Some(T::Foo)` compared two allocations (false). The payload type is read off whichever side
                        // names it (`Option$T`); a non-scalar one is refused.
                        let pt = if oast::str_starts(ea.as_str(), "Option$")
                            || oast::str_starts(ea.as_str(), "Result$")
                        {
                            garg(ea.as_str())
                        } else if oast::str_starts(eb.as_str(), "Option$")
                            || oast::str_starts(eb.as_str(), "Result$")
                        {
                            garg(eb.as_str())
                        } else {
                            String::new()
                        };
                        let parts = split_dollar_l(pt.as_str());
                        let mut q = 0;
                        while q < parts.len() {
                            let t = parts[q].as_str();
                            if t.len() > 0
                                && !(t == "int"
                                    || t == "bool"
                                    || t == "char"
                                    || oast::is_int_width(t))
                            {
                                wordy = false;
                            }
                            q = q + 1;
                        }
                    }
                    if wordy {
                        // OPT-EQ-WIDTH (W7): the compared width is 1 + the widest payload. The prelude's `Some(v)` reports npayload 0
                        // (a binding-named payload), so Option compared its TAG only: `Some(4) == Some(3)` was true. Count the
                        // payload-type list too.
                        let n = {
                            let mut m = oast::max_payload(&cx.edefs, &en) as usize;
                            let ed = &cx.edefs[ei];
                            let mut vi = 0;
                            while vi < ed.variants.len() {
                                if ed.variants[vi].ptys.len() > m {
                                    m = ed.variants[vi].ptys.len();
                                }
                                vi = vi + 1;
                            }
                            1 + m as u32
                        };
                        let la = lower(a, cx);
                        let lb = lower(b, cx);
                        let z = fresh(cx);
                        let nn = H::Lit(n as i64, z);
                        let mut ag: Vec<H> = Vec::new();
                        ag.push(la);
                        ag.push(lb);
                        ag.push(nn);
                        let i = fresh(cx);
                        let c = H::CallH(String::from("enumeq"), ag, i);
                        if o.as_str() == "==" {
                            return c;
                        }
                        let z2 = fresh(cx);
                        let zero = H::Lit(0, z2);
                        let i2 = fresh(cx);
                        return H::BinH(String::from("=="), Box::new(c), Box::new(zero), i2);
                    } else {
                        let mut pr: Vec<H> = Vec::new();
                        let z3 = fresh(cx);
                        pr.push(H::Lit(0, z3));
                        let i3 = fresh(cx);
                        return H::CallH(String::from("$enumeq_unsupported"), pr, i3);
                    }
                }
            }
            // SEQ-EQ-MIXED: an ARRAY against a VECTOR (an ARR-COPY binding is a Vec) converts the array side, then compares by contents
            if (o.as_str() == "==" || o.as_str() == "!=")
                && ((is_arr_ty(&ety_of(cx, a)) && is_vec_ty(&ety_of(cx, b)))
                    || (is_vec_ty(&ety_of(cx, a)) && is_arr_ty(&ety_of(cx, b))))
            {
                let la = if is_arr_ty(&ety_of(cx, a)) {
                    arr_as_vec(a, cx)
                } else {
                    lower(a, cx)
                };
                let lb = if is_arr_ty(&ety_of(cx, b)) {
                    arr_as_vec(b, cx)
                } else {
                    lower(b, cx)
                };
                let mut ag: Vec<H> = Vec::new();
                ag.push(la);
                ag.push(lb);
                let i = fresh(cx);
                let c = H::CallH(String::from("veceq"), ag, i);
                if o.as_str() == "==" {
                    return c;
                }
                let z = fresh(cx);
                let zero = H::Lit(0, z);
                let i2 = fresh(cx);
                return H::BinH(String::from("=="), Box::new(c), Box::new(zero), i2);
            }
            if (o.as_str() == "==" || o.as_str() == "!=")
                && (is_arr_ty(&ety_of(cx, a)) || is_arr_ty(&ety_of(cx, b)))
            {
                // two ARRAYS compare by CONTENTS (they carry their length at slot 0)
                let la = lower(a, cx);
                let lb = lower(b, cx);
                let mut ag: Vec<H> = Vec::new();
                ag.push(la);
                ag.push(lb);
                let i = fresh(cx);
                let c = H::CallH(String::from("arreq"), ag, i);
                if o.as_str() == "==" {
                    return c;
                }
                let z = fresh(cx);
                let zero = H::Lit(0, z);
                let i2 = fresh(cx);
                return H::BinH(String::from("=="), Box::new(c), Box::new(zero), i2);
            }
            if (o.as_str() == "==" || o.as_str() == "!=")
                && (is_vec_ty(&ety_of(cx, a)) || is_vec_ty(&ety_of(cx, b)))
            {
                // two VECTORS compare by CONTENTS, not by handle
                {
                    let ve = vec_elem_of(&ety_of(cx, a));
                    if oast::str_starts(ve.as_str(), "tup")
                        || oast::sfields(&cx.sdefs, &ve) > 0
                        || vec_elem_of(&ve).len() > 0
                    {
                        let a0: Vec<H> = Vec::new();
                        let i0 = fresh(cx);
                        return H::CallH(
                            String::from("$equality_of_a_vec_of_aggregates_is_not_implemented"),
                            a0,
                            i0,
                        );
                    }
                } // AGG-EQ: veceq compares element WORDS -- right for scalars and strs, an address compare for aggregates
                let la = lower(a, cx);
                let lb = lower(b, cx);
                let mut ag: Vec<H> = Vec::new();
                ag.push(la);
                ag.push(lb);
                let i = fresh(cx);
                let c = H::CallH(String::from("veceq"), ag, i);
                if o.as_str() == "==" {
                    return c;
                }
                let z = fresh(cx);
                let zero = H::Lit(0, z);
                let i2 = fresh(cx);
                return H::BinH(String::from("=="), Box::new(c), Box::new(zero), i2);
            }
            // S14: strs ORDER lexicographically; we used to refuse the comparison outright
            if (o.as_str() == "<" || o.as_str() == "<=" || o.as_str() == ">" || o.as_str() == ">=")
                && (ety_of(cx, a).as_str() == "str" || ety_of(cx, b).as_str() == "str")
            {
                let la = lower(a, cx);
                let lb = lower(b, cx);
                let mut ag: Vec<H> = Vec::new();
                ag.push(la);
                ag.push(lb);
                let i = fresh(cx);
                let c = H::CallH(String::from("scmp"), ag, i);
                let z = fresh(cx);
                let zero = H::Lit(0, z);
                let i2 = fresh(cx);
                return H::BinH(String::from(o.as_str()), Box::new(c), Box::new(zero), i2);
            }
            // BSTR-SLICE (R2a P4): a byte string compares by CONTENT too -- a `bstr` operand (an `as_bytes()` binding or a slice of
            // one) fell to pointer equality, so `&fb[..2] == pb` was false for equal bytes
            if (o.as_str() == "==" || o.as_str() == "!=")
                && (ety_of(cx, a).as_str() == "str"
                    || ety_of(cx, b).as_str() == "str"
                    || ety_of(cx, a).as_str() == "bstr"
                    || ety_of(cx, b).as_str() == "bstr")
            {
                let la = lower(a, cx);
                let lb = lower(b, cx);
                let mut ag: Vec<H> = Vec::new();
                ag.push(la);
                ag.push(lb);
                let i = fresh(cx);
                let c = H::CallH(String::from("streq"), ag, i);
                if o.as_str() == "==" {
                    return c;
                }
                let z = fresh(cx);
                let zero = H::Lit(0, z);
                let i2 = fresh(cx);
                return H::BinH(String::from("=="), Box::new(c), Box::new(zero), i2);
            }
            // F-1: either operand being an f64 selects the FLOAT operation. The word is unchanged --
            // an f64 is exactly 64 bits -- so only the opcode differs, and the name that selects it is
            // the one P-NOM already carries.
            {
                let fa = ety_of(cx, a).as_str() == "f64";
                let fb = ety_of(cx, b).as_str() == "f64";
                // `-x` desugars to `0 - x`; on a float that must be a SIGN FLIP, because `0.0 - 0.0` is
                // +0.0 while Rust's `-0.0` is negative zero (and Display spells it `-0`)
                if fb && o.as_str() == "-" {
                    let isz = match a.as_ref() {
                        E::Int(0, _l2, _h2) => true,
                        _ => false,
                    };
                    if isz {
                        let lb = lower(b, cx);
                        let z = fresh(cx);
                        let sgn = H::Lit(0 - 9223372036854775807 - 1, z);
                        let i = fresh(cx);
                        return H::BinH(String::from("^"), Box::new(lb), Box::new(sgn), i);
                    }
                }
                if fa || fb {
                    let fo = if o.as_str() == "+" {
                        String::from("f+")
                    } else if o.as_str() == "-" {
                        String::from("f-")
                    } else if o.as_str() == "*" {
                        String::from("f*")
                    } else if o.as_str() == "/" {
                        String::from("f/")
                    } else if o.as_str() == "%" {
                        String::from("f%")
                    } else if o.as_str() == "==" {
                        String::from("f==")
                    } else if o.as_str() == "!=" {
                        String::from("f!=")
                    } else if o.as_str() == "<" {
                        String::from("f<")
                    } else if o.as_str() == "<=" {
                        String::from("f<=")
                    } else if o.as_str() == ">" {
                        String::from("f>")
                    } else if o.as_str() == ">=" {
                        String::from("f>=")
                    } else {
                        String::from("")
                    };
                    if fo.len() > 0 {
                        let la = lower(a, cx);
                        let lb = lower(b, cx);
                        let i = fresh(cx);
                        return H::BinH(fo, Box::new(la), Box::new(lb), i);
                    }
                }
            }
            if o.as_str() == "&&" {
                let la = lower(a, cx);
                let lb = lower(b, cx);
                let z = fresh(cx);
                let f = H::Lit(0, z);
                let i = fresh(cx);
                return H::IfH(Box::new(la), Box::new(lb), Box::new(f), i);
            }
            if o.as_str() == "||" {
                let la = lower(a, cx);
                let lb = lower(b, cx);
                let o1 = fresh(cx);
                let tt = H::Lit(1, o1);
                let i = fresh(cx);
                return H::IfH(Box::new(la), Box::new(tt), Box::new(lb), i);
            }
            // TY-UCMP: an ordering compare on u64/usize is UNSIGNED (`3u64.wrapping_sub(5) > 100` is true); narrower unsigned
            // widths are zero-extended in i64, so the signed compare is already right for them
            if (o.as_str() == "<"
                || o.as_str() == "<="
                || o.as_str() == ">"
                || o.as_str() == ">="
                || o.as_str() == ">>"
                || o.as_str() == "/"
                || o.as_str() == "%")
                && {
                    let ta = ety_of(cx, a);
                    let tb = ety_of(cx, b);
                    ta.as_str() == "u64"
                        || ta.as_str() == "usize"
                        || ((tb.as_str() == "u64" || tb.as_str() == "usize") && o.as_str() != ">>")
                }
            {
                // U-SHR: `>>` on u64/usize is logical (by the LEFT operand's type)
                let la = lower(a, cx);
                let lb = lower(b, cx);
                let i = fresh(cx);
                let mut uo = String::from("u");
                uo.push_str(o.as_str());
                return H::BinH(uo, Box::new(la), Box::new(lb), i);
            }
            let la = lower(a, cx);
            let lb = lower(b, cx);
            let i = fresh(cx);
            let bh = H::BinH(String::from(o.as_str()), Box::new(la), Box::new(lb), i);
            // P-WIDTH: `250u8 + 10` is 4, not 260. An `as` cast already truncates in the folder and in
            // the emitted IR; arithmetic truncated NOWHERE, so a width-typed operation wrapped only by
            // accident of the 64-bit word. The same trunc/ext pair is applied here, and only when a
            // width is actually KNOWN and narrower than the word -- an untyped or isize value is left
            // alone.
            {
                let wa = ety_of(cx, a);
                let wb = ety_of(cx, b);
                // a SHIFT's width is the LEFT operand's alone -- the amount is a count, not a value of the
                // same type -- and K4 (probed from RC) says the amount is taken modulo that width, so
                // `1u8 << 9` is 2 and not 0 and not 512
                let shift = o.as_str() == "<<" || o.as_str() == ">>";
                let w = if oast::is_int_width(&wa) && oast::width_bits(&wa) < 64 {
                    wa
                } else if !shift && oast::is_int_width(&wb) && oast::width_bits(&wb) < 64 {
                    wb
                } else {
                    String::from("")
                };
                if w.len() > 0 && shift {
                    let bits = oast::width_bits(&w);
                    let la2 = lower(a, cx);
                    let lb2 = lower(b, cx);
                    let z = fresh(cx);
                    let m = H::Lit((bits - 1) as i64, z);
                    let i3 = fresh(cx);
                    let amt = H::BinH(String::from("&"), Box::new(lb2), Box::new(m), i3);
                    let i4 = fresh(cx);
                    let sh = H::BinH(String::from(o.as_str()), Box::new(la2), Box::new(amt), i4);
                    let i5 = fresh(cx);
                    return H::CastH(Box::new(sh), w, i5);
                }
                if w.len() > 0 && arith_op(o) {
                    let i2 = fresh(cx);
                    return H::CastH(Box::new(bh), w, i2);
                }
            }
            bh
        }
        E::If(cd, t, f, _l, _h) => {
            cx.recv_top = is_mcall(cd);
            let lc = lower(cd, cx);
            cx.recv_top = false;
            let lt = lower(t, cx);
            let lf = lower(f, cx);
            let i = fresh(cx);
            H::IfH(Box::new(lc), Box::new(lt), Box::new(lf), i)
        }
        E::LetT(n, t, v, b, _l, _h) => {
            if let E::Call(rf, _ra, _rl, _rh) = v.as_ref() {
                if rf.as_str() == "range2" || rf.as_str() == "range" {
                    cx.rangevars.push(String::from(n.as_str())); // R8 RANGE-DBG
                }
                if rf.as_str() == "rangeto" || rf.as_str() == "rangetoinc" {
                    let mut rk = String::from("@"); // R8 RANGE-TO: `@rangeto:x`
                    rk.push_str(rf.as_str());
                    rk.push(':');
                    rk.push_str(n.as_str());
                    cx.rangevars.push(rk);
                }
            }
            // COLLECT-MAP: `let m: HashMap<..> = it.collect();` -- collect is the identity here, so the map slot held a
            // Vec of pairs and `m[&k]` read it as a hash table (SEGFAULT). Refused until a pairs->map build exists.
            if (oast::str_starts(t.as_str(), "HashMap")
                || oast::str_starts(t.as_str(), "BTreeMap")
                || oast::str_starts(t.as_str(), "HashSet"))
                && (match v.as_ref() {
                    E::MethodCall(_r, m, a, _l2, _h2) => m.as_str() == "collect" && a.len() == 0,
                    _ => false,
                })
            {
                let a0: Vec<H> = Vec::new();
                let i = fresh(cx);
                return H::CallH(String::from("$collect_into_map_unsupported"), a0, i);
            }
            let mut lv = lower_coerce(v, cx, t);
            if rc_is_boxed_var(cx, v) {
                lv = rc_raw(v, cx);
            } // RC-BOXED (R4c): `let b: Rc<_> = a` moves the HANDLE, not the boxed value
              /* COLLECT-STRING: `let s: String = it.collect();` -- collecting strings into a String concatenates them */
            if t.as_str() == "str" && {
                match v.as_ref() {
                    E::MethodCall(_r, m, a, _l2, _h2) => m.as_str() == "collect" && a.len() == 0,
                    _ => false,
                }
            } {
                let cf = match v.as_ref() {
                    E::MethodCall(r0, _m, _a, _l2, _h2) => {
                        let et = ety_of(cx, r0);
                        if et.as_str() == "Vec$char" {
                            "vchar_string"
                        } else if et.as_str() == "Vec$str" || et.as_str() == "arr$str" {
                            "vstr_concat"
                        } else {
                            "$collect_string_untyped"
                        }
                    }
                    _ => "$collect_string_untyped",
                }; /* COLLECT-STRING-REFUSE: an element type that is neither char nor str is refused -- defaulting to str-concat read chars as string pointers (SEGFAULT) */
                let mut ca: Vec<H> = Vec::new();
                ca.push(lv);
                let ci = fresh(cx);
                lv = H::CallH(String::from(cf), ca, ci);
            }
            if is_cellvar(cx, n) {
                let an: Vec<H> = Vec::new();
                let i0 = fresh(cx);
                let mk = H::CallH(String::from("vec_new"), an, i0);
                let mut pa: Vec<H> = Vec::new();
                pa.push(mk);
                pa.push(lv);
                let i1 = fresh(cx);
                lv = H::CallH(String::from("vec_push1"), pa, i1);
            } /* CLOS-MUT: an annotated let is a cell too */
            let st = {
                let it = ety_of(cx, v);
                if t.as_str() == "Fn" && clos_of(&it).len() > 0 {
                    it // FN-ITEM-RET: `let g: fn() -> bool = t;` -- keep WHICH closure the slot holds (the annotation's bare `Fn` hid it)
                } else if t.as_str() == "Vec" && is_vec_ty(&it) {
                    it
                } else if t.as_str() == "HashMap" {
                    let e1 = first_meth_ty(b, n, "insert", cx);
                    let et = if e1.len() > 0 {
                        e1
                    } else {
                        first_meth_ty(b, n, "entry", cx)
                    };
                    if et.as_str() == "str" {
                        String::from("HashMap$str")
                    } else {
                        String::from("HashMap$int")
                    }
                } else if t.contains('$') && {
                    // GEN-ANN: `let t: Tag<i32>` -- a generic struct's INSTANCE annotation erases to the base struct (GEN-INST
                    // below records the arguments from the value); PhantomData<T> is the unit its value is
                    let b0 = gbase(t.as_str());
                    b0.as_str() == "PhantomData" || {
                        let mut g = false;
                        let mut k = 0;
                        while k < cx.sdefs.len() {
                            if cx.sdefs[k].name.as_str() == b0.as_str() && cx.sdefs[k].sgp.len() > 0
                            {
                                g = true;
                            }
                            k = k + 1;
                        }
                        g
                    }
                } {
                    let b0 = gbase(t.as_str());
                    if b0.as_str() == "PhantomData" {
                        it
                    } else {
                        b0
                    }
                } else {
                    String::from(t.as_str())
                }
            }; /* VEC-INFER + HM-KEY on an annotated let */
            cx.stypes.push(String::from(st.as_str()));
            cx.names.push(String::from(n.as_str()));
            let s = (cx.names.len() - 1) as u32;
            {
                let gi = ginst_of(cx, v);
                if gi.len() > 0 {
                    let mut e = String::from(n.as_str());
                    e.push('=');
                    e.push_str(&gi);
                    cx.ginst.push(e);
                } else if let Some(e) = proj_ann_ginst(cx, n, t.as_str()) {
                    cx.ginst.push(e);
                }
            } // GEN-INST on an annotated let too (`let k: Op<D> = Op::Yes(D(2))` -- the annotation itself erases <D>)
            rc_open(n, v, st.as_str(), cx); // RC-IDENTITY
            let pd = pd_push(n, s, &st, b, false, cx);
            dvar_open(n, &st, b, cx); // DROP-REASSIGN: may `n = ..` drop the old value?
            let mut lb = lower(b, cx);
            pd_pop(pd, cx);
            let _ = cx.dvars.pop();
            lb = drop_scope(n, s, &st, b, lb, cx); // DROP-SCOPE on an annotated let (`let x: D = ..` was never dropped)
            rc_close(cx);
            let _ = cx.names.pop();
            let _ = cx.stypes.pop();
            let i = fresh(cx);
            H::LetH(s, Box::new(lv), Box::new(lb), i)
        }
        // POP the binding when its body ends. Lowering used to let scopes grow monotonically, so a shadowed
        // name stayed visible AFTER its block: `let x = 1; if c { let x = 7; 0 }; x` read 7. The slot index
        // baked into LetH stays valid (it is a frame slot) and a later sibling `let` reuses it -- correct,
        // because the two scopes are disjoint.
        E::Let(n, v, b, _l, _h) => {
            if let E::Call(rf, _ra, _rl, _rh) = v.as_ref() {
                if rf.as_str() == "range2" || rf.as_str() == "range" {
                    cx.rangevars.push(String::from(n.as_str())); // R8 RANGE-DBG
                }
                if rf.as_str() == "rangeto" || rf.as_str() == "rangetoinc" {
                    let mut rk = String::from("@"); // R8 RANGE-TO: `@rangeto:x`
                    rk.push_str(rf.as_str());
                    rk.push(':');
                    rk.push_str(n.as_str());
                    cx.rangevars.push(rk);
                }
            }
            cx.recv_top = is_mcall(v) && !oast::str_starts(n.as_str(), "$"); // DROP-RECV: a USER let initializer is a full expression (a synthetic `$` let -- println's argument temps -- is not)
            let mut lv = lower(v, cx);
            if rc_is_boxed_var(cx, v) {
                lv = rc_raw(v, cx);
            } // RC-BOXED (R4c): `let b = a` moves the HANDLE, not the boxed value
            cx.recv_top = false;
            if n.as_str() == "_" {
                lv = drop_now(v, lv, cx); // DROP-TEMP: `let _ = D(..)` binds nothing, so the value drops at once
            }
            let mut vt = ety_of(cx, v);
            let is_zd = match v.as_ref() {
                E::Call(zf, za, _zl, _zh) => zf.as_str() == "$zerodflt" && za.len() == 0,
                _ => false,
            };
            if is_zd {
                // R6 ZERO-DFLT-USE: an unannotated `Default::default()` takes its type from its first use as a fn argument
                let ut = dflt_use_ty(cx, b, n.as_str());
                if ut.len() > 0 {
                    let mut zn = String::from("$zeroty$");
                    zn.push_str(oparse::norm_tyname(ut.as_str()).as_str());
                    let ze = E::Call(zn, Vec::new(), 0, 0);
                    lv = lower(&ze, cx); // through the `$zeroty$T` lowering (the zero of a named type)
                    vt = ety_of(cx, &ze);
                }
            }
            {
                let gi = ginst_of(cx, v);
                if gi.len() > 0 {
                    let mut e = String::from(n.as_str());
                    e.push('=');
                    e.push_str(&gi);
                    cx.ginst.push(e);
                }
            } // GEN-INST
              /* ARR-COPY: arrays are Copy -- `let b = a;` is a fresh buffer (it aliased, so `b.sort()` reordered a). The copy is the Vec conversion
              slice params already use; b is then a Vec of the same elements, which prints and indexes identically. */
            if arr_elem_of(&vt).len() > 0 && {
                match v.as_ref() {
                    E::Var(_vn, _a, _b) => true,
                    _ => false,
                }
            } {
                let a0 = lower(v, cx);
                let a1 = lower(v, cx);
                let mut na: Vec<H> = Vec::new();
                na.push(a1);
                let il = fresh(cx);
                let ln = H::CallH(String::from("arr_len"), na, il);
                let z = fresh(cx);
                let mut sa: Vec<H> = Vec::new();
                sa.push(a0);
                sa.push(H::Lit(0, z));
                sa.push(ln);
                let ic = fresh(cx);
                lv = H::CallH(String::from("arr_slice"), sa, ic);
                let mut t2 = String::from("Vec$");
                t2.push_str(&arr_elem_of(&vt));
                vt = t2;
            }
            // STRUCT-COPY (W7): a `#[derive(Copy)]` struct is copied by `let y = x;` -- it aliased, so `f(&mut y)` also changed x
            if (match v.as_ref() {
                E::Var(_vn, _a, _b) => true,
                E::Field(_o, _f, _a, _b) => true,
                E::Index(_b, _i, _l2, _h2) => true,
                _ => false,
            }) && copy_struct_ty(cx, vt.as_str())
            {
                lv = copy_val(cx, v); // `let z = arr[0]` / `let z = s.p` copy too
            }
            /* DBG-PAYLOAD: a let of a call returning Option/Result records the payload types in its SLOT type (Option$T, Result$T$E) */
            if (vt.as_str() == "Option" || vt.as_str() == "Result") && {
                match v.as_ref() {
                    E::Call(_c, _a, _l2, _h2) => true,
                    E::MethodCall(_r, _m, _a, _l2, _h2) => true,
                    E::EnumLit(_e, _vn, _xs, _l2, _h2) => true,
                    _ => false,
                }
            } {
                let pq = pay_of(cx, v);
                if pq.len() > 0 {
                    let mut t2 = String::from(vt.as_str());
                    t2.push('$');
                    t2.push_str(pq.as_str());
                    vt = t2;
                }
            }
            if is_cellvar(cx, n) {
                let an: Vec<H> = Vec::new();
                let i0 = fresh(cx);
                let mk = H::CallH(String::from("vec_new"), an, i0);
                let mut pa: Vec<H> = Vec::new();
                pa.push(mk);
                pa.push(lv);
                let i1 = fresh(cx);
                lv = H::CallH(String::from("vec_push1"), pa, i1);
            } // CLOS-MUT: the binding IS a cell
              // INFER-VEC-ELEM: an un-annotated `Vec::new()` takes its element type from its first `push` (rustc infers from use);
              // without it `v[0]` of a pushed String printed the pointer
            if vt.as_str() == "HashMap" {
                let et = {
                    let e1 = first_meth_ty(b, n, "insert", cx);
                    if e1.len() > 0 {
                        e1
                    } else {
                        first_meth_ty(b, n, "entry", cx)
                    }
                };
                if et.as_str() == "str" {
                    vt = String::from("HashMap$str");
                } else {
                    vt = String::from("HashMap$int");
                }
            } // HASHMAP: key type from the first insert
            if vt.as_str() == "Vec" {
                let et = first_push_ty(b, n, cx);
                if et.len() > 0 && et.as_str() != "int" {
                    let mut t2 = String::from("Vec$");
                    t2.push_str(et.as_str());
                    vt = t2;
                }
            }
            cx.stypes.push(String::from(vt.as_str()));
            cx.names.push(String::from(n.as_str()));
            let s = (cx.names.len() - 1) as u32;
            rc_open(n, v, "", cx); // RC-IDENTITY
            if tup_owned_init(cx, v) {
                cx.tupowned.push(String::from(n.as_str())); // TUP-GLUE
            }
            let pd = pd_push(n, s, &vt, b, is_fcr_get(v), cx);
            dvar_open(n, &vt, b, cx); // DROP-REASSIGN: may `n = ..` drop the old value?
            let mut lb = lower(b, cx);
            pd_pop(pd, cx);
            let _ = cx.dvars.pop();
            cx.borrow_bind = is_fcr_get(v);
            lb = drop_scope(n, s, &vt, b, lb, cx);
            lb = cap_drops(n, v, b, lb, cx); // CAP-DROP: a `move` closure binding drops what it captured
            rc_close(cx);
            let _ = cx.names.pop();
            let _ = cx.stypes.pop();
            let i = fresh(cx);
            H::LetH(s, Box::new(lv), Box::new(lb), i)
        }
    }
}

// DROP-SCOPE (rustc_mir_build drop elaboration): a binding whose type has `impl Drop` is dropped when its scope ends,
// AFTER the scope's value is computed; nested lets drop in reverse declaration order. No drop flags here, so a binding
// that may have been MOVED out (by-value arg, re-bound, returned, stored) is not dropped -- a missed drop, never a double.
// One helper for every binding edge (plain and annotated let), so the schedule is a single meet.
pub fn drop_scope(n: &str, s: u32, vt: &str, b: &E, lb: H, cx: &mut LCx) -> H {
    let mut dropf = String::from(vt);
    dropf.push_str("$drop");
    let borrowed = cx.borrow_bind;
    cx.borrow_bind = false;
    // FOR-BORROW: a `$fct` container (a for-head temporary) is only read by seq_get/seq_len, so it drops after the loop
    let fct = oast::str_starts(n, "$fct");
    // DROP-ADAPT: the adapter loop's own `$aw..` bindings (source, predicate, output) never drop -- the output IS the value
    if n == "_"
        || (!fct && moves_out_cx(cx, b, n))
        || borrowed
        || oast::str_starts(n, "$aw")
        || oast::str_starts(n, "__sbase")
        || n == "$slit"
    {
        return lb;
    }
    let _ = s;
    let _ = dropf;
    let dc = {
        // every drop goes through the glue meet: own `drop` first, then what the value owns
        let nost: Vec<String> = Vec::new();
        let gvt = ginst_vt(cx, n, vt); // DROP-GLUE generics: `W<D>` is spelled `W`; its argument lives in GEN-INST
        let gvt = if is_rcvar(cx, n) {
            let mut r = String::from("Rc$");
            r.push_str(gvt.as_str());
            r
        } else {
            gvt
        }; // RC-IDENTITY: an Rc binding drops by decrementing
        cx.tupglue = oast::in_list(&cx.tupowned, n); // TUP-GLUE: only a binding known to own its tuple's elements
        let mut g = drop_glue(n, &nost, gvt.as_str(), cx);
        cx.tupglue = false;
        if g.len() == 0 {
            return lb;
        }
        let ge = g.pop().unwrap();
        lower(&ge, cx)
    };
    let bt = ety_of(cx, b);
    cx.stypes.push(bt);
    cx.names.push(String::from("$dropv"));
    let s2 = (cx.names.len() - 1) as u32;
    let ri = fresh(cx);
    let rv = H::Local(s2, ri);
    let si = fresh(cx);
    let sq = H::SeqH(Box::new(dc), Box::new(rv), si);
    let _ = cx.names.pop();
    let _ = cx.stypes.pop();
    let li2 = fresh(cx);
    H::LetH(s2, Box::new(lb), Box::new(sq), li2)
}

// DROP-FIELD-TEMP (heavy pass 2): `BAR.0` / `mk().x` reads a field of a TEMPORARY of a Drop type -- the temporary drops
// after the read (a field of a Drop type cannot be moved out, E0509, so the read is a copy). Answers [] when not applicable.
// a CONST of a Drop type read in place (`BAR.0`): each use is a fresh TEMPORARY that drops after the read -- not modelled
// (refused: the temporary was never dropped, dropck/issue-34053 counted 0 drops)
pub fn const_drop_temp(cx: &mut LCx, e: &E) -> bool {
    if let E::Var(n, _l, _h) = e {
        let si = slot_of(cx, n) as usize;
        let local = si < cx.names.len() && cx.names[si].as_str() == n.as_str();
        if !local && fn_known(cx, n) {
            let t = ety_of(cx, e);
            let nost: Vec<String> = Vec::new();
            return drop_glue("$cq", &nost, t.as_str(), cx).len() > 0;
        }
    }
    false
}
pub fn field_temp(o: &E, f: &str, sn: &str, cx: &mut LCx) -> Vec<H> {
    let mut res: Vec<H> = Vec::new();
    if let E::Call(cn, _ca, _cl, _ch) = o {
        // DROP-FIELD-TEMP (heavy pass 2): `BAR.0` / `mk().x` reads a field of a TEMPORARY of a Drop type -- the
        // temporary drops after the read (a field of a Drop type cannot be moved out, E0509, so the read is a copy).
        // It was never dropped (dropck/issue-34053 counted 0 drops)
        let mut tn = String::from("$ft");
        tn.push_str(&fresh(cx).to_string());
        let nost: Vec<String> = Vec::new();
        let gl = if cn.starts_with("$") {
            Vec::new()
        } else {
            drop_glue(tn.as_str(), &nost, sn, cx)
        };
        if gl.len() > 0 {
            let lo = lower(o, cx);
            cx.stypes.push(String::from(sn));
            cx.names.push(String::from(tn.as_str()));
            let s = (cx.names.len() - 1) as u32;
            let read = lower(
                &E::Field(
                    Box::new(E::Var(String::from(tn.as_str()), 0, 0)),
                    String::from(f),
                    0,
                    0,
                ),
                cx,
            );
            let ft = oast::sfield_ty(&cx.sdefs, sn, f);
            cx.stypes.push(ft);
            cx.names.push(String::from("$fv"));
            let s2 = (cx.names.len() - 1) as u32;
            let mut gl = drop_glue(tn.as_str(), &nost, sn, cx);
            let dc = lower(&gl.pop().unwrap(), cx);
            let vi = fresh(cx);
            let sq = H::SeqH(Box::new(dc), Box::new(H::Local(s2, vi)), fresh(cx));
            let _ = cx.names.pop();
            let _ = cx.stypes.pop();
            let inner = H::LetH(s2, Box::new(read), Box::new(sq), fresh(cx));
            let _ = cx.names.pop();
            let _ = cx.stypes.pop();
            res.push(H::LetH(s, Box::new(lo), Box::new(inner), fresh(cx)));
        }
    }
    res
}
// R4a LAYOUT: rustc's (size, align) for a type written as raw token text (`& u8`, `Option < & u8 >`, `( u8 , u64 )`,
// `[ u16 ; 5 ]`, a user struct). x86_64 rules: scalars by width (u128 aligns 16); `&T`/Box/Rc/Arc 8, fat (`&str`, `&[T]`,
// `&dyn T`) 16; String/Vec 24; Option<T> takes T's niche (refs, Box, Rc, Vec, String, bool, char) or adds one align
// unit; tuples and repr(Rust) structs sort fields by alignment (no interior padding for these kinds); repr(C) keeps
// declaration order with padding. Anything not modelled exactly answers -1 (the caller refuses).
pub fn lay_toks(s: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let b = s.as_bytes();
    let mut cur = String::new();
    let mut i = 0;
    while i <= b.len() {
        if i == b.len() || b[i] == 32 {
            if cur.as_str() == ">>" {
                out.push(String::from(">"));
                out.push(String::from(">"));
            } else if cur.len() > 0 {
                out.push(cur);
            }
            cur = String::new();
        } else {
            cur.push(b[i] as char);
        }
        i = i + 1;
    }
    out
}
fn lay_round(s: i64, a: i64) -> i64 {
    if a <= 1 {
        return s;
    }
    ((s + a - 1) / a) * a
}
fn lay_scalar(t: &str) -> i64 {
    if t == "u8" || t == "i8" || t == "bool" {
        1
    } else if t == "u16" || t == "i16" {
        2
    } else if t == "u32" || t == "i32" || t == "f32" || t == "char" {
        4
    } else if t == "u64" || t == "i64" || t == "f64" || t == "usize" || t == "isize" {
        8
    } else if t == "u128" || t == "i128" {
        16
    } else {
        0
    }
}
// the aggregate of field layouts: repr(Rust) sorts by alignment (so no interior padding), repr(C) pads in order
fn lay_fields(fs: &Vec<(i64, i64)>, reprc: bool) -> (i64, i64) {
    let mut al = 1;
    let mut k = 0;
    while k < fs.len() {
        if fs[k].1 > al {
            al = fs[k].1;
        }
        k = k + 1;
    }
    let mut off = 0;
    if reprc {
        let mut k = 0;
        while k < fs.len() {
            off = lay_round(off, fs[k].1) + fs[k].0;
            k = k + 1;
        }
    } else {
        let mut k = 0;
        while k < fs.len() {
            off = off + fs[k].0;
            k = k + 1;
        }
    }
    (lay_round(off, al), al)
}
// SELF-SUBSET (R2a P4): the layout cursor is `&mut Vec<usize>` (pos[0]), not `&mut usize` -- ourc models a scalar `&mut`
// param by handing it back as the return value, which it refuses with an early return; an aggregate &mut is a real handle
fn lay_expect(t: &Vec<String>, pos: &mut Vec<usize>, w: &str) -> bool {
    if pos[0] < t.len() && t[pos[0]].as_str() == w {
        pos[0] = pos[0] + 1;
        return true;
    }
    false
}
// a pointee after `&` / `Box<`: unsized (str, [T], dyn Trait) -> fat 16, else consume it and answer thin 8
fn lay_pointee(cx: &LCx, t: &Vec<String>, pos: &mut Vec<usize>, depth: u32) -> i64 {
    if pos[0] >= t.len() {
        return -1;
    }
    if t[pos[0]].as_str() == "str" {
        pos[0] = pos[0] + 1;
        return 16;
    }
    if t[pos[0]].as_str() == "dyn" {
        pos[0] = pos[0] + 1;
        while pos[0] < t.len()
            && t[pos[0]].as_str() != ">"
            && t[pos[0]].as_str() != ","
            && t[pos[0]].as_str() != ")"
            && t[pos[0]].as_str() != "]"
        {
            pos[0] = pos[0] + 1;
        }
        return 16;
    }
    if t[pos[0]].as_str() == "[" {
        // `[T]` (a slice, fat) or `[T; N]` (an array, thin)
        let save = pos[0];
        pos[0] = pos[0] + 1;
        let (es, _ea) = layout_at(cx, t, pos, depth + 1);
        if es >= 0 && lay_expect(t, pos, "]") {
            return 16;
        }
        pos[0] = save;
    }
    let (s, _a) = layout_at(cx, t, pos, depth + 1);
    if s < 0 {
        return -1;
    }
    8
}
pub fn layout_at(cx: &LCx, t: &Vec<String>, pos: &mut Vec<usize>, depth: u32) -> (i64, i64) {
    let bad = (0 - 1, 0);
    if depth > 16 || pos[0] >= t.len() {
        return bad;
    }
    let w = String::from(t[pos[0]].as_str());
    pos[0] = pos[0] + 1;
    let sw = lay_scalar(w.as_str());
    if sw > 0 {
        return (sw, sw);
    }
    if w.as_str() == "&" || w.as_str() == "&&" {
        let _ = lay_expect(t, pos, "mut");
        let p = lay_pointee(cx, t, pos, depth);
        if p < 0 {
            return bad;
        }
        if w.as_str() == "&&" {
            return (8, 8);
        }
        return (p, 8);
    }
    if w.as_str() == "Box" || w.as_str() == "Rc" || w.as_str() == "Arc" {
        if !lay_expect(t, pos, "<") {
            return bad;
        }
        let p = lay_pointee(cx, t, pos, depth);
        if p < 0 || !lay_expect(t, pos, ">") {
            return bad;
        }
        return (if w.as_str() == "Box" { p } else { 8 }, 8);
    }
    if w.as_str() == "String" {
        return (24, 8);
    }
    if w.as_str() == "Vec" {
        if !lay_expect(t, pos, "<") {
            return bad;
        }
        let (s, _a) = layout_at(cx, t, pos, depth + 1);
        if s < 0 || !lay_expect(t, pos, ">") {
            return bad;
        }
        return (24, 8);
    }
    if w.as_str() == "PhantomData" {
        if !lay_expect(t, pos, "<") {
            return bad;
        }
        let (s, _a) = layout_at(cx, t, pos, depth + 1);
        if s < 0 || !lay_expect(t, pos, ">") {
            return bad;
        }
        return (0, 1);
    }
    if w.as_str() == "Option" {
        if !lay_expect(t, pos, "<") || pos[0] >= t.len() {
            return bad;
        }
        let head = String::from(t[pos[0]].as_str());
        let (s, a) = layout_at(cx, t, pos, depth + 1);
        if s < 0 || !lay_expect(t, pos, ">") {
            return bad;
        }
        let niche = head.as_str() == "&"
            || head.as_str() == "&&"
            || head.as_str() == "Box"
            || head.as_str() == "Rc"
            || head.as_str() == "Arc"
            || head.as_str() == "Vec"
            || head.as_str() == "String"
            || head.as_str() == "bool"
            || head.as_str() == "char";
        if niche {
            return (s, a);
        }
        if s == 0 {
            return (1, 1);
        }
        return (lay_round(s + a, a), a);
    }
    if w.as_str() == "(" {
        let mut fs: Vec<(i64, i64)> = Vec::new();
        while pos[0] < t.len() && t[pos[0]].as_str() != ")" {
            let f = layout_at(cx, t, pos, depth + 1);
            if f.0 < 0 {
                return bad;
            }
            fs.push(f);
            let _ = lay_expect(t, pos, ",");
        }
        if !lay_expect(t, pos, ")") {
            return bad;
        }
        if fs.len() == 0 {
            return (0, 1);
        }
        return lay_fields(&fs, false);
    }
    if w.as_str() == "[" {
        let (s, a) = layout_at(cx, t, pos, depth + 1);
        if s < 0 || !lay_expect(t, pos, ";") || pos[0] >= t.len() {
            return bad;
        }
        let nb = t[pos[0]].as_bytes();
        let mut n: i64 = 0;
        let mut k = 0;
        while k < nb.len() && nb[k] >= 48 && nb[k] <= 57 {
            n = n * 10 + (nb[k] - 48) as i64;
            k = k + 1;
        }
        if k == 0 {
            return bad;
        }
        pos[0] = pos[0] + 1;
        if !lay_expect(t, pos, "]") {
            return bad;
        }
        return (n * s, a);
    }
    // a user struct (not generic): its recorded fields, with `&` fields from REF-FIELD and repr(C) from the lexer marker
    if pos[0] < t.len() && t[pos[0]].as_str() == "<" {
        return bad;
    }
    lay_struct(cx, w.as_str(), depth)
}
fn lay_struct(cx: &LCx, name: &str, depth: u32) -> (i64, i64) {
    let bad = (0 - 1, 0);
    let mut si = 0;
    while si < cx.sdefs.len() && cx.sdefs[si].name.as_str() != name {
        si = si + 1;
    }
    if si >= cx.sdefs.len() || cx.sdefs[si].sgp.len() > 0 {
        return bad;
    }
    let mut rc = String::from("@reprc:");
    rc.push_str(name);
    let reprc = oast::in_list(&cx.fpays, rc.as_str());
    let mut fs: Vec<(i64, i64)> = Vec::new();
    let mut k = 0;
    while k < cx.sdefs[si].fields.len() && k < cx.sdefs[si].ftys.len() {
        let ft = cx.sdefs[si].ftys[k].as_str();
        let mut rf = String::from("@rf:");
        rf.push_str(name);
        rf.push('.');
        rf.push_str(cx.sdefs[si].fields[k].as_str());
        let isref = oast::in_list(&cx.fpays, rf.as_str());
        let f = if isref {
            if ft == "str" || oast::str_starts(ft, "Vec") {
                (16, 8)
            } else {
                (8, 8)
            }
        } else if lay_scalar(ft) > 0 {
            (lay_scalar(ft), lay_scalar(ft))
        } else if ft == "str" || ft == "Vec" || oast::str_starts(ft, "Vec$") {
            (24, 8)
        } else if ft == "unit" {
            (0, 1)
        } else {
            lay_struct(cx, ft, depth + 1)
        };
        if f.0 < 0 {
            return bad;
        }
        fs.push(f);
        k = k + 1;
    }
    if fs.len() == 0 {
        return (0, 1);
    }
    lay_fields(&fs, reprc)
}

// RC-IDENTITY (R4c): `&x` and `x` name the same handle here
pub fn unref(e: &E) -> &E {
    match e {
        E::Ref(x, _m, _l, _h) => unref(x.as_ref()),
        _ => e,
    }
}
// RC-IDENTITY: is e a FRESH heap allocation (its pointer is a unique identity for the count side table)?
fn rc_fresh(cx: &LCx, e: &E) -> bool {
    match e {
        E::StructLit(_s, _f, _l, _h) => true,
        E::ArrLit(_x, _l, _h) => true,
        E::TupLit(x, _l, _h) => x.len() > 0,
        E::Call(n, _a, _l, _h) => {
            if n.as_str() == "$cellnew" || n.as_str() == "vec_new" || n.as_str() == "vec_push1" {
                return true;
            }
            // a tuple-struct constructor `D(1)` allocates
            let mut k = 0;
            while k < cx.sdefs.len() {
                if cx.sdefs[k].name.as_str() == n.as_str() {
                    return true;
                }
                k = k + 1;
            }
            false
        }
        _ => false,
    }
}
// RC-IDENTITY: every let pushes "n" (an Rc handle) or "!n" (not one -- it SHADOWS an outer Rc of the same name); the innermost entry decides
pub fn rc_open(n: &str, v: &E, ann: &str, cx: &mut LCx) {
    if rc_init_boxed(cx, v) {
        let mut s = String::from("^");
        s.push_str(n);
        cx.rcvars.push(s);
    } else if rc_init(cx, v) || oast::str_starts(ann, "Rc$") || oast::str_starts(ann, "Arc$") {
        cx.rcvars.push(String::from(n));
    } else {
        let mut s = String::from("!");
        s.push_str(n);
        cx.rcvars.push(s);
    }
}
// TUP-GLUE: does this initializer give a tuple of OWNED values -- a tuple literal with no `&` element, or a move of a binding
// already known to own one?
pub fn tup_owned_init(cx: &LCx, v: &E) -> bool {
    match v {
        E::TupLit(xs, _l, _h) => {
            let mut k = 0;
            while k < xs.len() {
                if let E::Ref(_x, _m, _rl, _rh) = &xs[k] {
                    return false;
                }
                k = k + 1;
            }
            xs.len() > 0
        }
        E::Var(x, _l, _h) => oast::in_list(&cx.tupowned, x.as_str()),
        _ => false,
    }
}
pub fn rc_close(cx: &mut LCx) {
    let _ = cx.rcvars.pop();
}
pub fn is_rcvar(cx: &LCx, n: &str) -> bool {
    let mut i = cx.rcvars.len();
    while i > 0 {
        i = i - 1;
        let e = cx.rcvars[i].as_str();
        if e == n || (oast::str_starts(e, "^") && &e[1..] == n) {
            return true;
        }
        if oast::str_starts(e, "!") && &e[1..] == n {
            return false;
        }
    }
    false
}
// RC-BOXED (R4c rung 2): an Rc over a SCALAR or string has no unique pointer identity (two `Rc::new(7)` would share one count),
// so its value is BOXED in a one-slot cell whose pointer is the counted handle. A boxed binding ("^n" in rcvars) reads
// THROUGH the cell (the CLOS-MUT cell read); the HANDLE itself is used only by clone / strong_count / drop / moves.
pub fn rc_scalar_ty(t: &str) -> bool {
    oast::is_int_width(t)
        || t == "int"
        || t == "bool"
        || t == "char"
        || t == "f64"
        || t == "f32"
        || t == "str"
}
pub fn is_boxrc(cx: &LCx, n: &str) -> bool {
    let mut i = cx.rcvars.len();
    while i > 0 {
        i = i - 1;
        let e = cx.rcvars[i].as_str();
        if oast::str_starts(e, "^") && &e[1..] == n {
            return true;
        }
        if e == n || (oast::str_starts(e, "!") && &e[1..] == n) {
            return false;
        }
    }
    false
}
pub fn rc_is_boxed_var(cx: &LCx, e: &E) -> bool {
    match unref(e) {
        E::Var(x, _l, _h) => is_boxrc(cx, x),
        _ => false,
    }
}
// RC-BOXED: the HANDLE of an Rc expression -- a boxed binding's raw slot (not its cell read); anything else lowers as usual
pub fn rc_raw(e: &E, cx: &mut LCx) -> H {
    if let E::Var(x, _l, _h) = unref(e) {
        if is_boxrc(cx, x) {
            let s = slot_of(cx, x);
            let i = fresh(cx);
            return H::Local(s, i);
        }
    }
    lower(unref(e), cx)
}
// RC-BOXED: does initializer v hand the new binding a BOXED Rc handle?
fn rc_init_boxed(cx: &LCx, v: &E) -> bool {
    match v {
        E::Call(n, a, _l, _h) => {
            (n.as_str() == "$rcnew" && a.len() == 1 && rc_scalar_ty(ety_of(cx, &a[0]).as_str()))
                || (n.as_str() == "$boxnew"
                    && a.len() == 1
                    && rc_scalar_ty(ety_of(cx, &a[0]).as_str()))
                || (n.as_str() == "$rcclone" && a.len() == 1 && rc_is_boxed_var(cx, &a[0]))
        }
        E::MethodCall(r, m, a, _l, _h) => {
            m.as_str() == "clone" && a.len() == 0 && rc_is_boxed_var(cx, r)
        }
        E::Var(_x, _l, _h) => rc_is_boxed_var(cx, v),
        _ => false,
    }
}
// RC-IDENTITY: does initializer v hand the new binding an Rc handle? (Rc::new / Rc::clone / `.clone()` of an Rc binding / a
// move of one)
fn rc_init(cx: &LCx, v: &E) -> bool {
    match v {
        E::Call(n, a, _l, _h) => {
            (n.as_str() == "$rcnew"
                && a.len() == 1
                && (rc_fresh(cx, &a[0]) || rc_scalar_ty(ety_of(cx, &a[0]).as_str())))
                || (n.as_str() == "$rcclone"
                    && a.len() == 1
                    && match unref(&a[0]) {
                        E::Var(x, _l2, _h2) => is_rcvar(cx, x),
                        _ => false,
                    })
        }
        E::MethodCall(r, m, a, _l, _h) => {
            m.as_str() == "clone"
                && a.len() == 0
                && match unref(r.as_ref()) {
                    E::Var(x, _l2, _h2) => is_rcvar(cx, x),
                    _ => false,
                }
        }
        E::Var(x, _l, _h) => is_rcvar(cx, x),
        _ => false,
    }
}

// CAP-DROP (W1): a `move` closure OWNS what it captures (may_move already suppresses the captured binding's own drop), so
// the captures drop when the CLOSURE binding's scope ends -- each whole captured variable through the glue meet, in capture
// order (a whole-struct drop is rustc 2021's preserved field order). Skipped when the closure itself may leave its scope, and
// for a capture the closure writes (a private MOVE-CELL, not the variable): missed, never doubled.
pub fn cap_drops(n: &str, v: &E, b: &E, lb: H, cx: &mut LCx) -> H {
    let (cn, caps) = match v {
        E::ClosRef(cn, caps, _l, _h) => (cn, caps),
        _ => return lb,
    };
    if n == "_" || !oast::in_list(&cx.moveclos, cn) || moves_out(b, n) {
        return lb;
    }
    let mut ds: Vec<H> = Vec::new();
    let mut k = 0;
    while k < caps.len() {
        let c = caps[k].as_str();
        let mut mk = String::from(cn.as_str());
        mk.push('|');
        mk.push_str(c);
        let si = slot_of(cx, c) as usize;
        if !oast::in_list(&cx.movecaps, &mk) && si < cx.names.len() && cx.names[si].as_str() == c {
            let vt = String::from(cx.stypes[si].as_str());
            let gvt = ginst_vt(cx, c, vt.as_str());
            if !cap_whole(cx, mk.as_str(), c, gvt.as_str()) {
                k = k + 1;
                continue;
            }
            let nost: Vec<String> = Vec::new();
            let mut g = drop_glue(c, &nost, gvt.as_str(), cx);
            if g.len() > 0 {
                let ge = g.pop().unwrap();
                ds.push(lower(&ge, cx));
            }
        }
        k = k + 1;
    }
    if ds.len() == 0 {
        return lb;
    }
    let bt = ety_of(cx, b);
    cx.stypes.push(bt);
    cx.names.push(String::from("$dropv"));
    let s2 = (cx.names.len() - 1) as u32;
    let ri = fresh(cx);
    let mut sq = H::Local(s2, ri);
    while ds.len() > 0 {
        let d = ds.pop().unwrap();
        let si = fresh(cx);
        sq = H::SeqH(Box::new(d), Box::new(sq), si);
    }
    let _ = cx.names.pop();
    let _ = cx.stypes.pop();
    let li2 = fresh(cx);
    H::LetH(s2, Box::new(lb), Box::new(sq), li2)
}

// CAP-DROP: does the closure own capture c WHOLE? rustc 2021 captures disjoint fields: `move || a.y` owns only a.y. The
// capture is whole when the body uses it bare ("=*"), or reads every field of its struct that needs drop (then the drop
// is the struct's own glue, field order preserved). No record (an unmodelled body) or a partial field set -> false.
fn cap_whole(cx: &mut LCx, key: &str, c: &str, vt: &str) -> bool {
    let mut pre = String::from(key);
    pre.push('=');
    let mut used = String::new();
    let mut found = false;
    let mut i = 0;
    while i < cx.capuse.len() {
        if oast::str_starts(cx.capuse[i].as_str(), pre.as_str()) {
            used = String::from(&cx.capuse[i][pre.len()..]);
            found = true;
            i = cx.capuse.len();
        }
        i = i + 1;
    }
    if !found {
        return false;
    }
    if used.as_str() == "*" {
        return true;
    }
    let st = oast::sresolve(&cx.sdefs, vt);
    let mut si = 0;
    while si < cx.sdefs.len() && cx.sdefs[si].name.as_str() != st.as_str() {
        si = si + 1;
    }
    if si >= cx.sdefs.len() {
        return false;
    }
    let mut uf: Vec<String> = Vec::new();
    let ub = used.as_bytes();
    let mut cur = String::new();
    let mut j = 0;
    while j <= ub.len() {
        if j == ub.len() || ub[j] == 44 {
            uf.push(cur);
            cur = String::new();
        } else {
            cur.push(ub[j] as char);
        }
        j = j + 1;
    }
    let mut f = 0;
    while f < cx.sdefs[si].fields.len() {
        let fname = String::from(cx.sdefs[si].fields[f].as_str());
        let fty = String::from(cx.sdefs[si].ftys[f].as_str());
        let mut st1: Vec<String> = Vec::new();
        st1.push(String::from(fname.as_str()));
        if !oast::in_list(&uf, fname.as_str()) && drop_glue(c, &st1, fty.as_str(), cx).len() > 0 {
            return false;
        }
        f = f + 1;
    }
    true
}

// DROP-ADAPT (W1): an OWNING iterator adapter (`v.into_iter().filter(p)`, take/skip/take_while/skip_while) on elements that
// need drop discarded them without dropping -- the prelude adapters copy words. Lowered inline instead, as the eager loop
// rustc's lazy one amounts to: each source element in order is either pushed or drop-glued on the spot, so drops interleave
// with predicate calls exactly as in rustc. Answers [the loop] or [] (not an owning chain / nothing to drop).
pub fn owned_chain(r: &E) -> bool {
    match r {
        E::MethodCall(i, m, a, _l, _h) => {
            (m.as_str() == "into_iter" && a.len() == 0)
                || (adapt_kind(m.as_str()) > 0 && a.len() == 1 && owned_chain(i))
        }
        _ => false,
    }
}
fn adapt_kind(m: &str) -> u32 {
    if m == "filter" {
        1
    } else if m == "take" {
        2
    } else if m == "skip" {
        3
    } else if m == "take_while" {
        4
    } else if m == "skip_while" {
        5
    } else {
        0
    }
}
pub fn adapt_owned(r: &E, m: &str, args: &Vec<E>, rt: &str, cx: &mut LCx) -> Vec<H> {
    let mut out: Vec<H> = Vec::new();
    let k = adapt_kind(m);
    if k == 0 || args.len() != 1 || !owned_chain(r) || !oast::str_starts(rt, "Vec$") {
        return out;
    }
    let et = String::from(&rt[4..]);
    let id = fresh(cx).to_string();
    let nm = |p: &str| {
        let mut s = String::from(p);
        s.push_str(id.as_str());
        s
    };
    let sn = nm("$aws");
    let fnm = nm("$awf");
    let on = nm("$awo");
    let iv = nm("$awi");
    let kv = nm("$awk");
    let mut ist = String::from("?i:");
    ist.push_str(iv.as_str());
    let mut st: Vec<String> = Vec::new();
    st.push(ist);
    if drop_glue(sn.as_str(), &st, et.as_str(), cx).len() == 0 {
        return out;
    }
    // the source and the predicate are evaluated ONCE, bound to slots by hand (no E is copied)
    let lr = lower(r, cx);
    let lf = lower(&args[0], cx);
    let ft = ety_of(cx, &args[0]);
    cx.stypes.push(String::from(rt));
    cx.names.push(String::from(sn.as_str()));
    let ss = (cx.names.len() - 1) as u32;
    cx.stypes.push(ft);
    cx.names.push(String::from(fnm.as_str()));
    let sf = (cx.names.len() - 1) as u32;
    let v = |s: &str| E::Var(String::from(s), 0, 0);
    let int = |x: i64| E::Int(x, 0, 0);
    let bin = |o: &str, a: E, b: E| E::Bin(String::from(o), Box::new(a), Box::new(b), 0, 0);
    let iff = |c: E, t: E, f: E| E::If(Box::new(c), Box::new(t), Box::new(f), 0, 0);
    let seq = |a: E, b: E| E::Seq(Box::new(a), Box::new(b), 0, 0);
    let elem = || {
        E::Index(
            Box::new(E::Var(String::from(sn.as_str()), 0, 0)),
            Box::new(E::Var(String::from(iv.as_str()), 0, 0)),
            0,
            0,
        )
    };
    let push = || {
        E::MethodCall(
            Box::new(E::Var(String::from(on.as_str()), 0, 0)),
            String::from("push"),
            vec![elem()],
            0,
            0,
        )
    };
    let pred = || {
        E::Call(
            String::from(fnm.as_str()),
            vec![E::Ref(Box::new(elem()), false, 0, 0)],
            0,
            0,
        )
    };
    let kon = || bin("==", v(kv.as_str()), int(1));
    let koff = || E::Assign(String::from(kv.as_str()), Box::new(int(0)), 0, 0);
    let mut d1 = drop_glue(sn.as_str(), &st, et.as_str(), cx);
    let mut d2 = drop_glue(sn.as_str(), &st, et.as_str(), cx);
    let drop1 = d1.pop().unwrap();
    let drop2 = d2.pop().unwrap();
    let body = if k == 1 {
        iff(pred(), push(), drop1)
    } else if k == 2 {
        iff(bin("<", v(iv.as_str()), v(fnm.as_str())), push(), drop1)
    } else if k == 3 {
        iff(bin(">=", v(iv.as_str()), v(fnm.as_str())), push(), drop1)
    } else if k == 4 {
        // take_while: once the predicate fails it is never called again; the rest drop
        iff(kon(), iff(pred(), push(), seq(koff(), drop1)), drop2)
    } else {
        // skip_while: drop while the predicate holds; from the first failure on, keep everything
        iff(kon(), iff(pred(), drop1, seq(koff(), push())), push())
    };
    let step = E::Assign(
        String::from(iv.as_str()),
        Box::new(bin("+", v(iv.as_str()), int(1))),
        0,
        0,
    );
    let len = E::MethodCall(
        Box::new(v(sn.as_str())),
        String::from("len"),
        Vec::new(),
        0,
        0,
    );
    let lp = E::While(
        Box::new(bin("<", v(iv.as_str()), len)),
        Box::new(seq(body, step)),
        0,
        0,
    );
    let res = seq(lp, v(on.as_str()));
    let e = E::Let(
        String::from(kv.as_str()),
        Box::new(int(1)),
        Box::new(res),
        0,
        0,
    );
    let e = E::Let(
        String::from(iv.as_str()),
        Box::new(int(0)),
        Box::new(e),
        0,
        0,
    );
    let mut ot = String::from("Vec$");
    ot.push_str(et.as_str());
    let e = E::LetT(
        String::from(on.as_str()),
        ot,
        Box::new(E::Call(String::from("vec_new"), Vec::new(), 0, 0)),
        Box::new(e),
        0,
        0,
    );
    let lb = lower(&e, cx);
    let _ = cx.names.pop();
    let _ = cx.stypes.pop();
    let _ = cx.names.pop();
    let _ = cx.stypes.pop();
    let i1 = fresh(cx);
    let inner = H::LetH(sf, Box::new(lf), Box::new(lb), i1);
    let i2 = fresh(cx);
    out.push(H::LetH(ss, Box::new(lr), Box::new(inner), i2));
    out
}

// DROP-JUMP: a scope's drops sit AFTER its value, so `continue`/`break` jumped past them -- `for x in v { continue; }`
// dropped nothing. Each binding drop_scope will drop is pending while its body lowers; a jump out of k+1 loop bodies
// first drops every pending binding opened inside them, innermost first. A name now shadowed is skipped (missed, safe).
pub fn pd_push(n: &str, s: u32, vt: &str, b: &E, borrowed: bool, cx: &mut LCx) -> bool {
    if n == "_"
        || borrowed
        || oast::str_starts(n, "$aw")
        || oast::str_starts(n, "__sbase")
        || n == "$slit"
        || (!oast::str_starts(n, "$fct") && moves_out_cx(cx, b, n))
    {
        return false;
    }
    let mut e = String::from(n);
    e.push('\t');
    e.push_str(vt);
    e.push('\t');
    e.push_str(s.to_string().as_str());
    cx.pdrops.push(e);
    true
}
pub fn pd_field(e: &str, k: u32) -> String {
    let b = e.as_bytes();
    let mut out = String::new();
    let mut f = 0;
    let mut i = 0;
    while i < b.len() {
        if b[i] == 9 {
            f = f + 1;
        } else if f == k {
            out.push(b[i] as char);
        }
        i = i + 1;
    }
    out
}
pub fn pd_pop(pd: bool, cx: &mut LCx) {
    if pd {
        let _ = cx.pdrops.pop();
    }
}
pub fn pd_unloop(cx: &mut LCx) {
    while cx.pdrops.len() > 0 {
        let t = cx.pdrops.pop().unwrap();
        if t.as_str() == "|" {
            return;
        }
    }
}
pub fn pd_jump(k: u32, jump: H, cx: &mut LCx) -> H {
    let mut out = jump;
    let mut marks = 0;
    let mut i = cx.pdrops.len();
    let mut ents: Vec<String> = Vec::new();
    while i > 0 {
        i = i - 1;
        if cx.pdrops[i].as_str() == "|" {
            marks = marks + 1;
            if marks > k {
                break;
            }
        } else {
            ents.push(String::from(cx.pdrops[i].as_str()));
        }
    }
    if marks <= k && k < 1000000 {
        return out; // not inside k+1 lowered loop bodies: leave the jump as it was (1000000 = a return: always drop)
    }
    // ents is innermost-first; build drops outermost-last: d_inner; d_outer; jump
    let mut j = ents.len();
    while j > 0 {
        j = j - 1;
        let f0 = pd_field(ents[j].as_str(), 0);
        let f1 = pd_field(ents[j].as_str(), 1);
        let f2 = pd_field(ents[j].as_str(), 2);
        if slot_of(cx, f0.as_str()).to_string() != f2 {
            continue;
        }
        let nost: Vec<String> = Vec::new();
        let gvt0 = ginst_vt(cx, f0.as_str(), f1.as_str());
        let gvt = if is_rcvar(cx, f0.as_str()) {
            let mut r = String::from("Rc$");
            r.push_str(gvt0.as_str());
            r
        } else {
            gvt0
        }; // RC-IDENTITY: a jump out of an Rc binding's scope decrements
        let mut g = drop_glue(f0.as_str(), &nost, gvt.as_str(), cx);
        if g.len() == 0 {
            continue;
        }
        let ge = g.pop().unwrap();
        let dc = lower(&ge, cx);
        let si = fresh(cx);
        out = H::SeqH(Box::new(dc), Box::new(out), si);
    }
    out
}

// DROP-GLUE: a type with no `impl Drop` of its own still drops what it OWNS -- each Drop-typed struct field in
// declaration order, an `Option<T>`'s payload when Some. The owned value is the path `base.steps..` (a step is a
// field name, or "?u" = unwrap); answers [the drop expression] or [] when the type owns nothing droppable.
pub fn glue_path(base: &str, steps: &Vec<String>) -> E {
    let mut e = E::Var(String::from(base), 0, 0);
    let mut k = 0;
    while k < steps.len() {
        if steps[k].as_str() == "?u" {
            e = E::MethodCall(Box::new(e), String::from("unwrap"), Vec::new(), 0, 0);
        } else if oast::str_starts(steps[k].as_str(), "?t:") {
            // TUP-GLUE: a tuple element step `?t:<index>`
            let sb = steps[k].as_bytes();
            let mut ix: u32 = 0;
            let mut j = 3;
            while j < sb.len() {
                ix = ix * 10 + (sb[j] - 48) as u32;
                j = j + 1;
            }
            e = E::TupIdx(Box::new(e), ix, 0, 0);
        } else if oast::str_starts(steps[k].as_str(), "?i:") {
            // DROP-GLUE: an element step `?i:<index var>` (Vec/array element glue recurses through it)
            let sb = steps[k].as_bytes();
            let mut iv = String::new();
            let mut j = 3;
            while j < sb.len() {
                iv.push(sb[j] as char);
                j = j + 1;
            }
            e = E::Index(Box::new(e), Box::new(E::Var(iv, 0, 0)), 0, 0);
        } else {
            e = E::Field(Box::new(e), String::from(steps[k].as_str()), 0, 0);
        }
        k = k + 1;
    }
    e
}
// DROP-GLUE guard: a RECURSIVE type (an enum whose payload holds itself through Box/Vec, an AST) would expand its glue
// forever -- ourc itself segfaulted on recursive_ast/ast_frontend. A type already being expanded answers nothing: its
// inner drops are missed (safe), never looped.
pub fn drop_glue(base: &str, steps: &Vec<String>, vt: &str, cx: &mut LCx) -> Vec<E> {
    let mut k = 0;
    while k < cx.gluestk.len() {
        if cx.gluestk[k].as_str() == vt {
            return Vec::new();
        }
        k = k + 1;
    }
    cx.gluestk.push(String::from(vt));
    let r = drop_glue_in(base, steps, vt, cx);
    let _ = cx.gluestk.pop();
    r
}
// DROP-GLUE: a type's own `Drop::drop` runs FIRST, then what it owns is dropped (rustc: `foobar` then its payload `foo`)
pub fn drop_glue_in(base: &str, steps: &Vec<String>, vt: &str, cx: &mut LCx) -> Vec<E> {
    let mut out: Vec<E> = Vec::new();
    if oast::str_starts(vt, "Rc$") {
        // RC-IDENTITY (R4c): dropping an Rc handle DECREMENTS; the referent drops (its own glue) only when the count reaches 0.
        // The decrement always runs (strong_count observes it) even when the referent owns nothing droppable.
        let mut inner = drop_glue(base, steps, &vt[3..], cx);
        let mut a: Vec<E> = Vec::new();
        a.push(E::Call(
            String::from("$rcraw"),
            vec![glue_path(base, steps)],
            0,
            0,
        ));
        let dec = E::Call(String::from("rc_dec"), a, 0, 0);
        let zero = E::Bin(
            String::from("=="),
            Box::new(dec),
            Box::new(E::Int(0, 0, 0)),
            0,
            0,
        );
        let then = if inner.len() > 0 {
            E::Seq(
                Box::new(inner.pop().unwrap()),
                Box::new(E::Int(0, 0, 0)),
                0,
                0,
            )
        } else {
            E::Int(0, 0, 0)
        };
        out.push(E::If(
            Box::new(zero),
            Box::new(then),
            Box::new(E::Int(0, 0, 0)),
            0,
            0,
        ));
        return out;
    }
    let mut own = gbase(vt); // an instance `W$D` has W's own drop
    own.push_str("$drop");
    {
        // TBASE (R2a P5): a module type `l$D` is l$D, not its module `l` -- the declared type's own drop when it exists
        let mut t = tbase(cx, vt);
        t.push_str("$drop");
        if fn_known(cx, &t) {
            own = t;
        }
    }
    let mut c = drop_contents(base, steps, vt, cx);
    if fn_known(cx, &own) {
        let mut a: Vec<E> = Vec::new();
        a.push(glue_path(base, steps));
        let oc = E::Call(own, a, 0, 0);
        if c.len() > 0 {
            let ce = c.pop().unwrap();
            out.push(E::Seq(Box::new(oc), Box::new(ce), 0, 0));
        } else {
            out.push(oc);
        }
        return out;
    }
    c
}
// DROP-GLUE generics: an instance spelled `Name$Arg` (`Op<D>` -> `Op$D`) -- its base name, and its single type argument
// ("" when there is none). Multi-argument instances are not split: their glue stays a miss (safe), never a guess.
// TBASE (R2a P5 sweep): the BASE TYPE of a type name. `$` both qualifies a module type (`l$G`) and spells a generic
// instance (`G$i64`, `l$G$i64`); gbase cut at the FIRST `$`, so every module type became its module (`l`) -- its Drop, its
// Drop-field glue, its generic instance were all missed (silent WRONGs). The base is the LONGEST `$`-delimited prefix that
// names a declared struct or enum; gbase only when none does. SCOPE: the drop glue only -- monomorphization registers
// instances (`Triple$str`) as struct definitions too, so the generic-instance sites keep gbase (heavy-gated: r3_gen_inst_*).
pub fn tbase(cx: &LCx, vt: &str) -> String {
    let b = vt.as_bytes();
    let mut best = String::new();
    let mut j = 0;
    while j <= b.len() {
        if j == b.len() || b[j] == 36 {
            let p = &vt[..j];
            let mut k = 0;
            while k < cx.sdefs.len() {
                if cx.sdefs[k].name.as_str() == p {
                    best = String::from(p);
                }
                k = k + 1;
            }
            let mut k = 0;
            while k < cx.edefs.len() {
                if cx.edefs[k].name.as_str() == p {
                    best = String::from(p);
                }
                k = k + 1;
            }
        }
        j = j + 1;
    }
    if best.len() > 0 {
        best
    } else {
        gbase(vt)
    }
}
pub fn gbase(vt: &str) -> String {
    let b = vt.as_bytes();
    let mut r = String::new();
    let mut j = 0;
    while j < b.len() && b[j] != 36 {
        r.push(b[j] as char);
        j = j + 1;
    }
    r
}
pub fn garg(vt: &str) -> String {
    let b = vt.as_bytes();
    let mut j = 0;
    while j < b.len() && b[j] != 36 {
        j = j + 1;
    }
    let mut r = String::new();
    j = j + 1;
    while j < b.len() {
        r.push(b[j] as char);
        j = j + 1;
    }
    r
}
// a field of generic struct si whose type is its (only) type parameter takes the instance's argument
pub fn struct_tsub(cx: &LCx, si: usize, vt: &str, ft: &str) -> String {
    let a = garg(vt);
    if oast::str_starts(ft, "$proj$") {
        return proj_pick(cx, si, a.as_str(), ft);
    }
    if a.len() > 0 && cx.sdefs[si].sgp.len() == 1 && cx.sdefs[si].sgp[0].as_str() == ft {
        return a;
    }
    String::from(ft)
}
// Edef records no type-parameter list: a payload type that names NO known type is taken as the parameter, and only when
// the enum has exactly one such name (so `Op<T> { No, Yes(T) }` substitutes, anything ambiguous does not)
pub fn enum_tsub(cx: &LCx, ei: usize, vt: &str, pt: &str) -> String {
    let a = garg(vt);
    if pt.len() == 0 {
        return a; // an erased type-parameter payload ("") takes the instance's argument
    }
    if a.len() == 0 || oast::is_known_ty2(&cx.sdefs, &cx.edefs, pt) || fn_known(cx, pt) {
        return String::from(pt);
    }
    let mut seen = String::new();
    let mut v = 0;
    while v < cx.edefs[ei].variants.len() {
        let mut q = 0;
        while q < cx.edefs[ei].variants[v].ptys.len() {
            let t = cx.edefs[ei].variants[v].ptys[q].as_str();
            if !oast::is_known_ty2(&cx.sdefs, &cx.edefs, t) {
                if seen.len() == 0 {
                    seen = String::from(t);
                } else if seen.as_str() != t {
                    return String::from(pt);
                }
            }
            q = q + 1;
        }
        v = v + 1;
    }
    if seen.as_str() == pt {
        return a;
    }
    String::from(pt)
}
// DROP-GLUE: the drops of what a value OWNS (its own Drop excluded): Option payload, sequence elements, enum payload,
// struct fields
pub fn drop_contents(base: &str, steps: &Vec<String>, vt: &str, cx: &mut LCx) -> Vec<E> {
    let mut out: Vec<E> = Vec::new();
    {
        // TUP-GLUE (W1): a TUPLE drops its elements in declaration order, each through the glue meet (a `let t = (Foo(0), 1)`
        // binding was never dropped)
        let es = oparse::tup_elems(vt);
        if es.len() > 0 && !cx.tupglue {
            return out; // element ownership unknown (`&` is erased from tuple element types): a missed drop, never a wrong one
        }
        if es.len() > 0 {
            let mut k = 0;
            while k < es.len() {
                let mut st2: Vec<String> = Vec::new();
                let mut q = 0;
                while q < steps.len() {
                    st2.push(String::from(steps[q].as_str()));
                    q = q + 1;
                }
                let mut ts = String::from("?t:");
                ts.push_str(&oast::u32s2(k as u32));
                st2.push(ts);
                let mut g = drop_glue(base, &st2, es[k].as_str(), cx);
                if g.len() > 0 {
                    let ge = g.pop().unwrap();
                    if out.len() == 0 {
                        out.push(ge);
                    } else {
                        let prev = out.pop().unwrap();
                        out.push(E::Seq(Box::new(prev), Box::new(ge), 0, 0));
                    }
                }
                k = k + 1;
            }
            return out;
        }
    }
    if oast::str_starts(vt, "Option$") {
        let vb = vt.as_bytes();
        let mut pt = String::new();
        let mut j = 7;
        while j < vb.len() {
            pt.push(vb[j] as char);
            j = j + 1;
        }
        let mut st2: Vec<String> = Vec::new();
        let mut k = 0;
        while k < steps.len() {
            st2.push(String::from(steps[k].as_str()));
            k = k + 1;
        }
        st2.push(String::from("?u"));
        let mut inner = drop_glue(base, &st2, pt.as_str(), cx);
        if inner.len() == 0 {
            return out;
        }
        let g = inner.pop().unwrap();
        let some = E::MethodCall(
            Box::new(glue_path(base, steps)),
            String::from("is_some"),
            Vec::new(),
            0,
            0,
        );
        out.push(E::If(
            Box::new(some),
            Box::new(E::Seq(Box::new(g), Box::new(E::Int(0, 0, 0)), 0, 0)),
            Box::new(E::Int(0, 0, 0)),
            0,
            0,
        ));
        return out;
    }
    if oast::str_starts(vt, "Vec$") || oast::str_starts(vt, "arr$") {
        // DROP-GLUE: a Vec (and an array / `vec![..]` literal, `arr$T`) drops its elements front to back: `let $giN = 0; while $giN < v.len() { T$drop(v[$giN]); $giN += 1 }`
        let vb = vt.as_bytes();
        let mut et = String::new();
        let mut j = 4;
        while j < vb.len() {
            et.push(vb[j] as char);
            j = j + 1;
        }
        let mut iv = String::from("$gi");
        let mut d = steps.len();
        while d > 0 {
            iv.push('_');
            d = d - 1;
        }
        // DROP-GLUE: each element drops through the glue meet (own Drop OR contents -- `Vec<Vec<D>>` drops the inner D's)
        let mut st2: Vec<String> = Vec::new();
        let mut q = 0;
        while q < steps.len() {
            st2.push(String::from(steps[q].as_str()));
            q = q + 1;
        }
        let mut ist = String::from("?i:");
        ist.push_str(iv.as_str());
        st2.push(ist);
        let mut eg = drop_glue(base, &st2, et.as_str(), cx);
        if eg.len() == 0 {
            return out;
        }
        let call = eg.pop().unwrap();
        let step = E::Assign(
            String::from(iv.as_str()),
            Box::new(E::Bin(
                String::from("+"),
                Box::new(E::Var(String::from(iv.as_str()), 0, 0)),
                Box::new(E::Int(1, 0, 0)),
                0,
                0,
            )),
            0,
            0,
        );
        let len = E::MethodCall(
            Box::new(glue_path(base, steps)),
            String::from("len"),
            Vec::new(),
            0,
            0,
        );
        let cond = E::Bin(
            String::from("<"),
            Box::new(E::Var(String::from(iv.as_str()), 0, 0)),
            Box::new(len),
            0,
            0,
        );
        let lp = E::While(
            Box::new(cond),
            Box::new(E::Seq(Box::new(call), Box::new(step), 0, 0)),
            0,
            0,
        );
        out.push(E::Let(iv, Box::new(E::Int(0, 0, 0)), Box::new(lp), 0, 0));
        return out;
    }
    {
        // DROP-GLUE: a user enum drops the payload of the variant it holds: `match v { V(g0, ..) => drops, _ => () }`
        let mut ei = cx.edefs.len();
        let mut k = 0;
        while k < cx.edefs.len() {
            if cx.edefs[k].name.as_str() == tbase(cx, vt).as_str() {
                ei = k;
            }
            k = k + 1;
        }
        if ei < cx.edefs.len() {
            let mut arms: Vec<Arm> = Vec::new();
            let nv = cx.edefs[ei].variants.len();
            let mut vi = 0;
            while vi < nv {
                let vname = String::from(cx.edefs[ei].variants[vi].name.as_str());
                let mut ptys: Vec<String> = Vec::new();
                let mut q = 0;
                while q < cx.edefs[ei].variants[vi].ptys.len() {
                    ptys.push(enum_tsub(
                        cx,
                        ei,
                        vt,
                        cx.edefs[ei].variants[vi].ptys[q].as_str(),
                    ));
                    q = q + 1;
                }
                let mut binds: Vec<String> = Vec::new();
                let mut body: Vec<E> = Vec::new();
                let mut q = 0;
                while q < ptys.len() {
                    let mut bn = String::from("$gp");
                    bn.push_str(&oast::u32s2(q as u32));
                    let mut d = steps.len();
                    while d > 0 {
                        bn.push('_');
                        d = d - 1;
                    }
                    let nost: Vec<String> = Vec::new();
                    let mut g = drop_glue(bn.as_str(), &nost, ptys[q].as_str(), cx);
                    if g.len() > 0 {
                        let ge = g.pop().unwrap();
                        if body.len() == 0 {
                            body.push(ge);
                        } else {
                            let p = body.pop().unwrap();
                            body.push(E::Seq(Box::new(p), Box::new(ge), 0, 0));
                        }
                    }
                    binds.push(bn);
                    q = q + 1;
                }
                if body.len() > 0 {
                    let b0 = body.pop().unwrap();
                    arms.push(Arm {
                        slit: String::from(""),
                        has_slit: false,
                        lit: 0,
                        has_lit: false,
                        has_guard: false,
                        guard: E::Bool(true, 0, 0),
                        variant: vname,
                        binds: binds,
                        body: E::Seq(Box::new(b0), Box::new(E::Int(0, 0, 0)), 0, 0),
                        wild: false,
                    });
                }
                vi = vi + 1;
            }
            if arms.len() == 0 {
                return out;
            }
            arms.push(Arm {
                slit: String::from(""),
                has_slit: false,
                lit: 0,
                has_lit: false,
                has_guard: false,
                guard: E::Bool(true, 0, 0),
                variant: String::from("_"),
                binds: Vec::new(),
                body: E::Int(0, 0, 0),
                wild: true,
            });
            out.push(E::Match(Box::new(glue_path(base, steps)), arms, 0, 0));
            return out;
        }
    }
    let mut fnames: Vec<String> = Vec::new();
    let mut ftys: Vec<String> = Vec::new();
    let mut i = 0;
    while i < cx.sdefs.len() {
        if cx.sdefs[i].name.as_str() == tbase(cx, vt).as_str() {
            let mut k = 0;
            while k < cx.sdefs[i].fields.len() && k < cx.sdefs[i].ftys.len() {
                // REF-FIELD (W1): a `&T` field is borrowed -- dropping through it dropped the referent (`R { c: &a }` dropped a
                // twice). The parser's "@rf:<Struct>.<field>" record names it; the type itself erased the `&`.
                let mut rf = String::from("@rf:");
                rf.push_str(cx.sdefs[i].name.as_str());
                rf.push('.');
                rf.push_str(cx.sdefs[i].fields[k].as_str());
                if !oast::in_list(&cx.fpays, rf.as_str()) {
                    fnames.push(String::from(cx.sdefs[i].fields[k].as_str()));
                    ftys.push(struct_tsub(cx, i, vt, cx.sdefs[i].ftys[k].as_str()));
                }
                k = k + 1;
            }
            break;
        }
        i = i + 1;
    }
    let mut k = 0;
    while k < fnames.len() {
        let mut st2: Vec<String> = Vec::new();
        let mut q = 0;
        while q < steps.len() {
            st2.push(String::from(steps[q].as_str()));
            q = q + 1;
        }
        st2.push(String::from(fnames[k].as_str()));
        let mut g = drop_glue(base, &st2, ftys[k].as_str(), cx);
        if g.len() > 0 {
            let ge = g.pop().unwrap();
            if out.len() == 0 {
                out.push(ge);
            } else {
                let p = out.pop().unwrap();
                out.push(E::Seq(Box::new(p), Box::new(ge), 0, 0));
            }
        }
        k = k + 1;
    }
    out
}

// DROP-REASSIGN: one entry per live binding, innermost last: `n` when assigning to it must drop the old value (its type
// owns a Drop and it is never moved out in its scope), `!n` otherwise -- so a shadowing let always answers for itself.
pub fn dvar_open(n: &str, vt: &str, b: &E, cx: &mut LCx) {
    let nost: Vec<String> = Vec::new();
    let ok = n != "_" && !moves_out(b, n) && drop_glue(n, &nost, vt, cx).len() > 0;
    let mut e = String::new();
    if !ok {
        e.push('!');
    }
    e.push_str(n);
    cx.dvars.push(e);
}
pub fn dvar_live(cx: &LCx, n: &str) -> bool {
    let mut k = cx.dvars.len();
    while k > 0 {
        k = k - 1;
        let d = cx.dvars[k].as_str();
        if d == n {
            return true;
        }
        let db = d.as_bytes();
        let nb = n.as_bytes();
        if db.len() == nb.len() + 1 && db[0] == 33 {
            let mut j = 0;
            while j < nb.len() && db[j + 1] == nb[j] {
                j = j + 1;
            }
            if j == nb.len() {
                return false;
            }
        }
    }
    false
}

// DROP-TEMP: an unbound VALUE of a Drop type (`let _ = D(4);`, the statement `D(5);`) is dropped at once, at the end of
// its statement. Only constructions count (struct/enum literal, tuple-struct ctor): a call or method may hand back a
// reference whose ety still reads `D`, and dropping that would be a double drop -- missed, never double.
pub fn drop_now(a: &E, la: H, cx: &mut LCx) -> H {
    let built = match a {
        E::StructLit(_n, _f, _l, _h) => true,
        E::EnumLit(_t, _v, _p, _l, _h) => true,
        E::Call(f, _a, _l, _h) => {
            ety_of(cx, a).as_str() == f.as_str() || is_fresh_fn(cx, f.as_str())
        }
        _ => false,
    };
    if !built {
        return la;
    }
    let vt = ety_of(cx, a);
    cx.stypes.push(String::from(vt.as_str()));
    cx.names.push(String::from("$dropt"));
    let s = (cx.names.len() - 1) as u32;
    let nost: Vec<String> = Vec::new();
    let mut g = drop_glue("$dropt", &nost, vt.as_str(), cx);
    if g.len() == 0 {
        let _ = cx.names.pop();
        let _ = cx.stypes.pop();
        return la;
    }
    let ge = g.pop().unwrap();
    let dc = lower(&ge, cx); // the glue meet: own drop, then contents
    let _ = cx.names.pop();
    let _ = cx.stypes.pop();
    let i = fresh(cx);
    H::LetH(s, Box::new(la), Box::new(dc), i)
}

// fill an LCx from the program's function table / trait impls (the lowering context's own setup)
pub fn fill_dyn(lc: &mut LCx, impls: &Vec<ImplRec>) {
    let mut i = 0;
    while i < impls.len() {
        let mut e = String::from(impls[i].tr.as_str());
        e.push('|');
        e.push_str(impls[i].ty.as_str());
        lc.dimpls.push(e);
        i = i + 1;
    }
}
pub fn tail_place_ref(e: &E) -> bool {
    match e {
        E::Ref(x, m, _l, _h) => {
            if *m {
                match &**x {
                    E::Index(_b, _i, _a, _b2) => true,
                    E::Field(_o, _f, _a, _b2) => true,
                    _ => false,
                }
            } else {
                false
            }
        }
        E::Seq(_a, b, _l, _h) => tail_place_ref(b),
        _ => false,
    }
}
// VEC-LIT-ELEM: the type of the first `vec_push(n, x)` in a statement chain ("" if none)
pub fn vpush_elem_ty(cx: &LCx, e: &E, n: &str) -> String {
    match e {
        E::Call(f, a, _l, _h) if f.as_str() == "vec_push" && a.len() == 2 && is_var(&a[0], n) => {
            ety_of(cx, &a[1])
        }
        E::Seq(a, b, _l, _h) => {
            let t = vpush_elem_ty(cx, a, n);
            if t.len() > 0 {
                return t;
            }
            vpush_elem_ty(cx, b, n)
        }
        _ => String::new(),
    }
}
// DROP-TEMP: does a body END in a construction (struct/enum literal, or a capitalised tuple-struct ctor call)?
pub fn tail_is_ctor(e: &E) -> bool {
    match e {
        E::StructLit(_n, _f, _l, _h) => true,
        E::EnumLit(_t, _v, _p, _l, _h) => true,
        E::Call(f, _a, _l, _h) => {
            let fb = f.as_bytes();
            fb.len() > 0 && fb[0] >= 65 && fb[0] <= 90 && !oast::str_contains2(f.as_str(), "$")
        }
        E::Seq(_a, b, _l, _h) => tail_is_ctor(b),
        _ => false,
    }
}
pub fn is_fresh_fn(cx: &LCx, n: &str) -> bool {
    let mut i = 0;
    while i < cx.fresh_fns.len() {
        if cx.fresh_fns[i].as_str() == n {
            return true;
        }
        i = i + 1;
    }
    false
}
pub fn is_addr_fn(cx: &LCx, n: &str) -> bool {
    let mut i = 0;
    while i < cx.addr_fns.len() {
        if cx.addr_fns[i].as_str() == n {
            return true;
        }
        i = i + 1;
    }
    false
}
pub fn is_pass_fn(cx: &LCx, n: &str) -> bool {
    let mut i = 0;
    while i < cx.pass_fns.len() {
        if cx.pass_fns[i].as_str() == n {
            return true;
        }
        i = i + 1;
    }
    false
}
pub fn is_ref_mut(e: &E) -> bool {
    match e {
        E::Ref(_x, m, _l, _h) => *m,
        _ => false,
    }
}
pub fn fill_fsigs(lc: &mut LCx, funs: &Vec<Fun>) {
    let mut i = 0;
    while i < funs.len() {
        lc.fnpar.push(funs[i].params.len() as i32);
        lc.fnames.push(String::from(funs[i].name.as_str()));
        if funs[i].params.len() > 0
            && funs[i].params[0].as_str() == "self"
            && funs[i].pref.len() > 0
            && funs[i].pref[0]
        {
            lc.refself.push(String::from(funs[i].name.as_str()));
        }
        lc.frets.push(String::from(funs[i].ret.as_str()));
        let mut j = String::from("");
        let mut k = 0;
        while k < funs[i].ptys.len() {
            if k > 0 {
                j.push(',');
            }
            j.push_str(&funs[i].ptys[k]);
            k = k + 1;
        }
        lc.fptys.push(j);
        lc.fncaps.push(funs[i].ncap);
        if tail_place_ref(&funs[i].body) {
            lc.addr_fns.push(String::from(funs[i].name.as_str()));
        } // P-08-MUTRET R1: fns returning &mut v[i]/&mut p.x return an ADDRESS
        if tail_is_ctor(&funs[i].body) {
            lc.fresh_fns.push(String::from(funs[i].name.as_str()));
        } // DROP-TEMP: its result is a fresh OWNED value (never a reference), so an unbound result may be dropped
        match &funs[i].body {
            E::Var(vn, _lv, _hv) => {
                let mut j = 0;
                while j < funs[i].params.len() {
                    if funs[i].params[j].as_str() == vn.as_str() {
                        lc.pass_fns.push(String::from(funs[i].name.as_str()));
                    }
                    j = j + 1;
                }
            }
            _ => {}
        } // P-08-MUTRET C2-R2: identity passthrough (body is exactly a param)
        {
            let mut g = String::from("");
            let mut k = 0;
            while k < funs[i].gparams.len() {
                if k > 0 {
                    g.push(',');
                }
                g.push_str(&funs[i].gparams[k]);
                k = k + 1;
            }
            lc.fgps.push(g);
        }
        {
            let mut b = String::from("");
            let mut k = 0;
            while k < funs[i].gbounds.len() {
                if k > 0 {
                    b.push(',');
                }
                b.push_str(&funs[i].gbounds[k]);
                k = k + 1;
            }
            lc.fgbs.push(b);
        }
        i = i + 1;
    }
}
// INFER-VEC-ELEM: the type of the first `n.push(x)` argument in e (the scope of `let n = Vec::new()`), "" if none
fn first_push_ty(e: &E, n: &str, cx: &mut LCx) -> String {
    first_meth_ty(e, n, "push", cx)
}
fn first_meth_ty(e: &E, n: &str, meth: &str, cx: &mut LCx) -> String {
    // HM-KEY-SCOPE: binds each let it walks, so a loop variable used as the key has its type
    match e {
        E::MethodCall(r, m, a, _l, _h) => {
            if m.as_str() == meth && a.len() >= 1 {
                match r.as_ref() {
                    E::Var(rn, _x, _y) => {
                        if rn.as_str() == n {
                            return ety_of(cx, &a[0]);
                        }
                    }
                    _ => {}
                }
            }
            let t = first_meth_ty(r, n, meth, cx);
            if t.len() > 0 {
                return t;
            }
            let mut k = 0;
            while k < a.len() {
                let t2 = first_meth_ty(&a[k], n, meth, cx);
                if t2.len() > 0 {
                    return t2;
                }
                k = k + 1;
            }
            String::from("")
        }
        E::Seq(a, b, _l, _h) => {
            let t = first_meth_ty(a, n, meth, cx);
            if t.len() > 0 {
                t
            } else {
                first_meth_ty(b, n, meth, cx)
            }
        }
        E::Let(m2, v, b, _l, _h) => {
            let t = first_meth_ty(v, n, meth, cx);
            if t.len() > 0 {
                return t;
            }
            if m2.as_str() == n {
                return String::from("");
            }
            let vt = ety_of(cx, v);
            cx.names.push(String::from(m2.as_str()));
            cx.stypes.push(vt);
            let r = first_meth_ty(b, n, meth, cx);
            let _ = cx.names.pop();
            let _ = cx.stypes.pop();
            r
        }
        E::LetT(m2, t0, v, b, _l, _h) => {
            let t = first_meth_ty(v, n, meth, cx);
            if t.len() > 0 {
                return t;
            }
            if m2.as_str() == n {
                return String::from("");
            }
            cx.names.push(String::from(m2.as_str()));
            cx.stypes.push(String::from(t0.as_str()));
            let r = first_meth_ty(b, n, meth, cx);
            let _ = cx.names.pop();
            let _ = cx.stypes.pop();
            r
        }
        E::If(c, t, f, _l, _h) => {
            let a = first_meth_ty(c, n, meth, cx);
            if a.len() > 0 {
                return a;
            }
            let b2 = first_meth_ty(t, n, meth, cx);
            if b2.len() > 0 {
                return b2;
            }
            first_meth_ty(f, n, meth, cx)
        }
        E::While(c, b, _l, _h) => {
            let a = first_meth_ty(c, n, meth, cx);
            if a.len() > 0 {
                a
            } else {
                first_meth_ty(b, n, meth, cx)
            }
        }
        _ => String::from(""),
    }
}
fn fn_known(cx: &LCx, n: &str) -> bool {
    let mut q = 0;
    while q < cx.fnames.len() {
        if cx.fnames[q].as_str() == n {
            return true;
        }
        q = q + 1;
    }
    false
}
// DROP-SCOPE: is `n` moved out of its scope -- returned as the scope's value (`let x = D(1); x`, and every `vec![..]`
// expansion's `$vecl`), or possibly moved within it? (may_move alone missed the tail: `$vecl` was dropped as it was returned)
pub fn moves_out(b: &E, n: &str) -> bool {
    seq_tail_is(b, n) || may_move(b, n)
}
// DROP-SCOPE: a method that only BORROWS its receiver; any other method on `n` (`into_iter`, `unwrap`, a user `self`
// method -- its self-ness is not recorded) may consume it
fn borrow_method(m: &str) -> bool {
    m == "len"
        || m == "is_empty"
        || m == "iter"
        || m == "iter_mut"
        || m == "get"
        || m == "get_mut"
        || m == "push"
        || m == "pop"
        || m == "insert"
        || m == "remove"
        || m == "contains"
        || m == "contains_key"
        || m == "first"
        || m == "last"
        || m == "clone"
        || m == "sort"
        || m == "reverse"
        || m == "truncate"
        || m == "clear"
        || m == "as_str"
        || m == "set"
        || m == "keys"
        || m == "to_string"
}
// DROP-SCOPE: could `n` be moved out within e? Conservative: any form not listed counts as a possible move.
fn may_move(e: &E, n: &str) -> bool {
    match e {
        E::Var(_x, _l, _h) => false,
        E::Int(_v, _l, _h) => false,
        E::StrLit(_s, _l, _h) => false,
        E::Bool(_b, _l, _h) => false,
        // DROP-RET: `return x` moves exactly what x moves (it fell to the catch-all: any return in a scope suppressed every drop)
        E::RetE(x, _l, _h) => is_var(x, n) || may_move(x, n),
        // DROP-JUMP: a bare `break`/`continue` moves nothing (it fell to the catch-all and suppressed every drop in the body)
        E::BreakE(_l, _h) => false,
        E::BreakN(_n, _l, _h) => false,
        E::ContE(_l, _h) => false,
        E::ContN(_n, _l, _h) => false,
        // LIT-MOVE (W1): an aggregate literal moves exactly the bindings it holds by value (it fell to the catch-all, so any later
        // `S { .. }` suppressed every drop in scope)
        E::StructLit(_s, xs, _l, _h) => any_moves(xs, n),
        E::EnumLit(_t, _v, xs, _l, _h) => any_moves(xs, n),
        E::TupLit(xs, _l, _h) => any_moves(xs, n),
        E::ArrLit(xs, _l, _h) => any_moves(xs, n),
        // CLOS-MOVE (W1): a closure value moves only what it CAPTURES (it fell to the catch-all, so `let f = |x| ..`
        // suppressed the drop of every binding in scope)
        E::ClosRef(_c, caps, _l, _h) => {
            let mut k = 0;
            while k < caps.len() {
                if caps[k].as_str() == n {
                    return true;
                }
                k = k + 1;
            }
            false
        }
        E::Ref(_x, _m, _l, _h) => false,
        E::Field(o, _f, _l, _h) => may_move(o, n),
        E::TupIdx(o, _i, _l, _h) => may_move(o, n),
        E::Index(o, i, _l, _h) => may_move(o, n) || may_move(i, n),
        E::Not(x, _l, _h) => may_move(x, n),
        E::Cast(x, _t, _l, _h) => may_move(x, n),
        E::Seq(a, b, _l, _h) => is_var(a, n) || may_move(a, n) || may_move(b, n),
        E::Let(_m, v, b, _l, _h) => is_var(v, n) || may_move(v, n) || may_move(b, n),
        E::LetT(_m, _t, v, b, _l, _h) => is_var(v, n) || may_move(v, n) || may_move(b, n),
        E::If(c, t, f, _l, _h) => {
            may_move(c, n) || is_var(t, n) || is_var(f, n) || may_move(t, n) || may_move(f, n)
        }
        E::While(c, b, _l, _h) => may_move(c, n) || may_move(b, n),
        // DROP-REASSIGN: writing INTO a binding is not a move of it; only the right side can move `n`
        E::Assign(_m, v, _l, _h) => is_var(v, n) || may_move(v, n),
        // DROP-PLACE-STORE (W1): `v[0].b = 5` / `x.b += 10` stores INTO a place of n -- not a move of n (it suppressed v's
        // scope-end drop); only the right side can move. The place's own sub-expressions (an index) are reads.
        E::AssignTo(p, v, _l, _h) => is_var(v, n) || may_move(v, n) || may_move(p, n),
        E::Bin(_o, a, b, _l, _h) => may_move(a, n) || may_move(b, n),
        E::MethodCall(r, m, a, _l, _h) => {
            if may_move(r, n) || (is_var(r, n) && !borrow_method(m.as_str())) {
                return true;
            } // DROP-SCOPE: `v.into_iter()` consumes v (it was dropped again at scope end: a DOUBLE drop)
            let mut k = 0;
            while k < a.len() {
                if is_var(&a[k], n) || may_move(&a[k], n) {
                    return true;
                }
                k = k + 1;
            }
            false
        }
        E::Call(_f, a, _l, _h) => {
            let mut k = 0;
            while k < a.len() {
                if is_var(&a[k], n) || may_move(&a[k], n) {
                    return true;
                }
                k = k + 1;
            }
            false
        }
        _ => true,
    }
}
// CLOS-BORROW (W1): may_move with the borrowed-capture list (a twin, so the 20 plain call sites keep their signature)
fn may_move_bc(e: &E, n: &str, bc: &Vec<String>) -> bool {
    match e {
        E::Var(_x, _l, _h) => false,
        E::Int(_v, _l, _h) => false,
        E::StrLit(_s, _l, _h) => false,
        E::Bool(_b, _l, _h) => false,
        // DROP-RET: `return x` moves exactly what x moves (it fell to the catch-all: any return in a scope suppressed every drop)
        E::RetE(x, _l, _h) => is_var(x, n) || may_move_bc(x, n, bc),
        // DROP-JUMP: a bare `break`/`continue` moves nothing (it fell to the catch-all and suppressed every drop in the body)
        E::BreakE(_l, _h) => false,
        E::BreakN(_n, _l, _h) => false,
        E::ContE(_l, _h) => false,
        E::ContN(_n, _l, _h) => false,
        // LIT-MOVE (W1): an aggregate literal moves exactly the bindings it holds by value (it fell to the catch-all, so any later
        // `S { .. }` suppressed every drop in scope)
        E::StructLit(_s, xs, _l, _h) => any_moves_bc(xs, n, bc),
        E::EnumLit(_t, _v, xs, _l, _h) => any_moves_bc(xs, n, bc),
        E::TupLit(xs, _l, _h) => any_moves_bc(xs, n, bc),
        E::ArrLit(xs, _l, _h) => any_moves_bc(xs, n, bc),
        // CLOS-MOVE (W1): a closure value moves only what it CAPTURES (it fell to the catch-all, so `let f = |x| ..`
        // suppressed the drop of every binding in scope)
        E::ClosRef(c, caps, _l, _h) => {
            let mut k = 0;
            while k < caps.len() {
                if caps[k].as_str() == n {
                    let mut key = String::from(c.as_str());
                    key.push('|');
                    key.push_str(n);
                    return !oast::in_list(bc, key.as_str()); // CLOS-BORROW: a field-only capture of a non-move closure borrows
                }
                k = k + 1;
            }
            false
        }
        E::Ref(_x, _m, _l, _h) => false,
        E::Field(o, _f, _l, _h) => may_move_bc(o, n, bc),
        E::TupIdx(o, _i, _l, _h) => may_move_bc(o, n, bc),
        E::Index(o, i, _l, _h) => may_move_bc(o, n, bc) || may_move_bc(i, n, bc),
        E::Not(x, _l, _h) => may_move_bc(x, n, bc),
        E::Cast(x, _t, _l, _h) => may_move_bc(x, n, bc),
        E::Seq(a, b, _l, _h) => is_var(a, n) || may_move_bc(a, n, bc) || may_move_bc(b, n, bc),
        E::Let(_m, v, b, _l, _h) => is_var(v, n) || may_move_bc(v, n, bc) || may_move_bc(b, n, bc),
        E::LetT(_m, _t, v, b, _l, _h) => {
            is_var(v, n) || may_move_bc(v, n, bc) || may_move_bc(b, n, bc)
        }
        E::If(c, t, f, _l, _h) => {
            may_move_bc(c, n, bc)
                || is_var(t, n)
                || is_var(f, n)
                || may_move_bc(t, n, bc)
                || may_move_bc(f, n, bc)
        }
        E::While(c, b, _l, _h) => may_move_bc(c, n, bc) || may_move_bc(b, n, bc),
        // DROP-REASSIGN: writing INTO a binding is not a move of it; only the right side can move `n`
        E::Assign(_m, v, _l, _h) => is_var(v, n) || may_move_bc(v, n, bc),
        // DROP-PLACE-STORE (W1): `v[0].b = 5` / `x.b += 10` stores INTO a place of n -- not a move of n (it suppressed v's
        // scope-end drop); only the right side can move. The place's own sub-expressions (an index) are reads.
        E::AssignTo(p, v, _l, _h) => is_var(v, n) || may_move_bc(v, n, bc) || may_move_bc(p, n, bc),
        E::Bin(_o, a, b, _l, _h) => may_move_bc(a, n, bc) || may_move_bc(b, n, bc),
        E::MethodCall(r, m, a, _l, _h) => {
            let mut bm = String::from("@bm|"); // REF-SELF: a `&self` method of n's own type (moves_out_cx)
            bm.push_str(m.as_str());
            if may_move_bc(r, n, bc)
                || (is_var(r, n) && !borrow_method(m.as_str()) && !oast::in_list(bc, bm.as_str()))
            {
                return true;
            } // DROP-SCOPE: `v.into_iter()` consumes v (it was dropped again at scope end: a DOUBLE drop)
            let mut k = 0;
            while k < a.len() {
                if is_var(&a[k], n) || may_move_bc(&a[k], n, bc) {
                    return true;
                }
                k = k + 1;
            }
            false
        }
        E::Call(_f, a, _l, _h) => {
            let mut k = 0;
            while k < a.len() {
                if is_var(&a[k], n) || may_move_bc(&a[k], n, bc) {
                    return true;
                }
                k = k + 1;
            }
            false
        }
        _ => true,
    }
}
fn any_moves_bc(xs: &Vec<E>, n: &str, bc: &Vec<String>) -> bool {
    let mut k = 0;
    while k < xs.len() {
        if is_var(&xs[k], n) || may_move_bc(&xs[k], n, bc) {
            return true;
        }
        k = k + 1;
    }
    false
}
// CLOS-BORROW (W1): moves_out that knows which closure captures only BORROW (cx.borrowcaps) -- the drop-schedule sites use it
pub fn moves_out_cx(cx: &LCx, b: &E, n: &str) -> bool {
    if seq_tail_is(b, n) {
        return true;
    }
    if upd_base_moved(cx, b, n) {
        return true;
    }
    // REF-SELF (W1): `n.m()` where m is a `&self` method of n's OWN type borrows n (every user method call read as a move,
    // so `a.hi()` left `a` undropped). Keyed by n's type, so a same-named consuming std method is not mistaken for it.
    let mut bc = cx.borrowcaps.clone();
    let mut pre = gbase(sty_of(cx, n).as_str());
    pre.push('$');
    let mut k = 0;
    while k < cx.refself.len() {
        if pre.len() > 1 && oast::str_starts(cx.refself[k].as_str(), pre.as_str()) {
            let mut e = String::from("@bm|");
            e.push_str(&cx.refself[k][pre.len()..]);
            bc.push(e);
        }
        k = k + 1;
    }
    may_move_bc(b, n, &bc)
}
fn any_moves(xs: &Vec<E>, n: &str) -> bool {
    let mut k = 0;
    while k < xs.len() {
        if is_var(&xs[k], n) || may_move(&xs[k], n) {
            return true;
        }
        k = k + 1;
    }
    false
}
fn is_var(e: &E, n: &str) -> bool {
    match e {
        E::Var(x, _l, _h) => x.as_str() == n,
        _ => false,
    }
}
// DROP-PARAM: a BY-VALUE parameter whose type has `impl Drop` is owned by the callee and dropped when its body ends (after the
// body's value), unless the body may move it on; reference parameters (`&T`) are borrowed and never dropped here
// RC-IDENTITY (R4c): is param p of fn fname declared `Rc<T>` / `Arc<T>`? (the parser's "@rcp:<fn>#<p>" record -- the type erases Rc)
pub fn rc_param(cx: &LCx, fname: &str, p: u32) -> bool {
    let mut k = String::from("@rcp:");
    k.push_str(fname);
    k.push('#');
    k.push_str(&oast::u32s2(p));
    oast::in_list(&cx.fpays, k.as_str())
}
// RC-IDENTITY: seed the Rc bindings of a fn body from its Rc-typed params (called before the body lowers)
pub fn rc_params(cx: &mut LCx, f: &Fun) {
    while cx.rcvars.len() > 0 {
        cx.rcvars.pop();
    }
    let mut p = 0;
    while p < f.params.len() {
        if rc_param(cx, f.name.as_str(), p as u32) {
            if p < f.ptys.len() && rc_scalar_ty(f.ptys[p].as_str()) {
                let mut s = String::from("^");
                s.push_str(f.params[p].as_str());
                cx.rcvars.push(s); // RC-BOXED: a scalar Rc param holds a cell handle
            } else {
                cx.rcvars.push(String::from(f.params[p].as_str()));
            }
        }
        p = p + 1;
    }
}
pub fn param_drops(h: H, f: &Fun, body: &E, cx: &mut LCx) -> H {
    let mut out = h;
    let mut p = f.params.len();
    while p > 0 {
        p = p - 1;
        let isref = p < f.pref.len() && f.pref[p];
        let mut pt = if p < f.ptys.len() {
            String::from(f.ptys[p].as_str())
        } else {
            String::from("")
        };
        {
            // PARAM-GINST (W1): a generic ENUM param (`_r: Op<P>`) is spelled `Op` -- its argument comes from the parser's
            // "@pg:<fn>#<p>=<arg>" record, giving the same `Op$P` instance a let's GEN-INST gives (it was never dropped)
            let mut pre = String::from("@pg:");
            pre.push_str(f.name.as_str());
            pre.push('#');
            pre.push_str(&oast::u32s2(p as u32));
            pre.push('=');
            let mut i = 0;
            while i < cx.fpays.len() {
                if oast::str_starts(cx.fpays[i].as_str(), pre.as_str()) {
                    let a = String::from(&cx.fpays[i][pre.len()..]);
                    pt.push('$');
                    pt.push_str(a.as_str());
                    i = cx.fpays.len();
                }
                i = i + 1;
            }
        }
        if rc_param(cx, f.name.as_str(), p as u32) {
            let mut r = String::from("Rc$");
            r.push_str(pt.as_str());
            pt = r;
        } // RC-IDENTITY (R4c): an `Rc<T>` param drops by decrementing (the referent drops only at count 0)
          // `self` is skipped: its ref-ness is not recorded, and dropping `&mut self` inside `drop` itself recursed forever
          // DROP-GLUE: the param goes through the same glue meet as a let (own Drop, struct fields, Option, Vec, enum payload)
          // SELF-BYVAL (W1): REF-SELF records `&self`, so a BY-VALUE `self: Self` (`fn take(self)`) is owned here and drops at the
          // body's end like any by-value param (it was never dropped). Never inside `drop` itself; only the plain `Self` type.
        let byval_self = f.params[p].as_str() == "self"
            && !isref
            && !f.name.ends_with("$drop")
            && f.name.contains('$')
            && {
                let nb = f.name.as_bytes();
                let mut c = nb.len();
                while c > 0 && nb[c - 1] != 36 {
                    c = c - 1;
                }
                let own = String::from(&f.name[..c - 1]);
                let ok = pt.as_str() == "Self" || pt.len() == 0 || pt.as_str() == own.as_str();
                if ok {
                    pt = own;
                }
                ok
            };
        let mut glue: Vec<E> = Vec::new();
        if !isref
            && (f.params[p].as_str() != "self" || byval_self)
            && pt.len() > 0
            && !moves_out_cx(cx, body, f.params[p].as_str())
        {
            let nost: Vec<String> = Vec::new();
            glue = drop_glue(f.params[p].as_str(), &nost, pt.as_str(), cx);
        }
        if glue.len() > 0 {
            let ge = glue.pop().unwrap();
            let dc = lower(&ge, cx);
            cx.stypes.push(String::from(""));
            cx.names.push(String::from("$dropr"));
            let s2 = (cx.names.len() - 1) as u32;
            let ri = fresh(cx);
            let rv = H::Local(s2, ri);
            let si = fresh(cx);
            let sq = H::SeqH(Box::new(dc), Box::new(rv), si);
            let _ = cx.names.pop();
            let _ = cx.stypes.pop();
            let li2 = fresh(cx);
            out = H::LetH(s2, Box::new(out), Box::new(sq), li2);
        }
    }
    out
}
// CLOS-CAPTY: the recorded type of capture k of closure cn ("" if none)
pub fn capty_of(cx: &LCx, cn: &str, k: u32) -> String {
    let mut pre = String::from(cn);
    pre.push('#');
    pre.push_str(&oast::u32s2(k));
    pre.push('=');
    let mut i = cx.captys.len();
    while i > 0 {
        i = i - 1;
        if oast::str_starts(cx.captys[i].as_str(), pre.as_str()) {
            return String::from(&cx.captys[i][pre.len()..]);
        }
    }
    String::from("")
}
// CLOS-MUT: the variables of fn `fname` (body `body`) that live in a shared CELL -- the captures a closure WRITES, both in the
// closure itself ("<closure>|v") and in the fn that creates it ("@<body lo>|v"). Called before the body is lowered.
pub fn set_cellvars(cx: &mut LCx, fname: &str, body: &E, mutcaps: &Vec<String>) {
    while cx.cellvars.len() > 0 {
        cx.cellvars.pop();
    }
    let mut k1 = String::from("@");
    k1.push_str(&oast::u32s2(oast::e_lo(body)));
    k1.push('|');
    let mut k2 = String::from(fname);
    k2.push('|');
    while cx.movecaps.len() > 0 {
        cx.movecaps.pop();
    }
    let mut q = 0;
    while q < mutcaps.len() {
        let e = mutcaps[q].as_str();
        if oast::str_starts(e, "!") {
            cx.movecaps.push(String::from(&e[1..])); // W3 MOVE-CELL: "<closure>|v" -- built as a fresh cell at the ClosRef
        } else if oast::str_starts(e, "%") {
            if !oast::in_list(&cx.borrowcaps, &String::from(&e[1..])) {
                cx.borrowcaps.push(String::from(&e[1..]));
            } // CLOS-BORROW
        } else if oast::str_starts(e, "~") {
            if !oast::in_list(&cx.capuse, &String::from(&e[1..])) {
                cx.capuse.push(String::from(&e[1..]));
            } // CAP-DROP
        } else if oast::str_starts(e, "^") {
            if !oast::in_list(&cx.moveclos, &String::from(&e[1..])) {
                cx.moveclos.push(String::from(&e[1..]));
            } // CAP-DROP
        } else if oast::str_starts(e, k1.as_str()) {
            cx.cellvars.push(String::from(&e[k1.len()..]));
        } else if oast::str_starts(e, k2.as_str()) {
            cx.cellvars.push(String::from(&e[k2.len()..]));
        }
        q = q + 1;
    }
}
fn is_cellvar(cx: &LCx, n: &str) -> bool {
    let mut i = 0;
    while i < cx.cellvars.len() {
        if cx.cellvars[i].as_str() == n {
            return true;
        }
        i = i + 1;
    }
    false
}
// DBG-PAYLOAD: "Result$int$str" -> ["Result","int","str"]
fn split_dollar_l(t: &str) -> Vec<String> {
    let mut o: Vec<String> = Vec::new();
    let b = t.as_bytes();
    let mut st = 0;
    let mut i = 0;
    while i <= b.len() {
        if i == b.len() || b[i] == 36 {
            o.push(String::from(&t[st..i]));
            st = i + 1;
        }
        i = i + 1;
    }
    o
}
// DBG-PAYLOAD: the payload types ("T" or "T$E") of an Option/Result-valued call: a fn's declared return (cx.fpays,
// recorded by the parser), or an Option-returning element method of a Vec$T receiver; "" when not known
// CLONE-DEEP: derive(Clone) of a struct copies EVERY level it owns -- a Vec field is a fresh buffer (vclone), a struct
// field is cloned recursively (bound once to a slot). The old field-wise copy shared a Vec field, so `b.v.push(2)` after
// `let b = a.clone()` also changed a (a wrong answer). Strings are immutable here; a Vec's ELEMENTS are still shared.
fn deep_clone_h(cx: &mut LCx, v: H, ty: &str) -> H {
    if ty == "int"
        || ty == "bool"
        || ty == "char"
        || ty == "f64"
        || ty == "str"
        || oast::is_int_width(ty)
    {
        return v; // CLONE-SCALAR (W7): a scalar is its own copy -- `i32` reached the struct path by suffix match and was DEREFERENCED
    }
    if ty == "arr" {
        // ARR-FIELD (W7): an array field of a Copy struct must be COPIED, but the array layout has no copy primitive yet
        // (arr_slice makes a Vec, whose layout an `arr` field read misreads: it returned the length) -- refused (it aliased:
        // `fn check(mut self)` wrote into the caller's array, mir/issue-76740 was WRONG). V1c gives arrays a layout copy.
        let _ = v;
        let i0 = fresh(cx);
        return H::CallH(
            String::from("$copy_of_an_array_field_is_not_modelled"),
            Vec::new(),
            i0,
        );
    }
    if is_vec_ty(ty) {
        let el = vec_elem_of(ty);
        // CLONE-DEEP-ELEM: a Vec of Vecs / structs clones EACH element (they are handles) -- vclone shared them, so
        // `b[0].push(9)` after `let b = a.clone()` also changed a[0]
        if is_vec_ty(&el) || oast::sfields(&cx.sdefs, &el) > 0 {
            cx.names.push(String::from("$dcs"));
            cx.stypes.push(String::from(ty));
            let ss = (cx.names.len() - 1) as u32;
            cx.names.push(String::from("$dcd"));
            cx.stypes.push(String::from(ty));
            let sd = (cx.names.len() - 1) as u32;
            cx.names.push(String::from("$dci"));
            cx.stypes.push(String::from("int"));
            let si = (cx.names.len() - 1) as u32;
            let mut la: Vec<H> = Vec::new();
            la.push(H::Local(ss, fresh(cx)));
            let ln = H::CallH(String::from("vec_len"), la, fresh(cx));
            let cond = H::BinH(
                String::from("<"),
                Box::new(H::Local(si, fresh(cx))),
                Box::new(ln),
                fresh(cx),
            );
            let mut ga: Vec<H> = Vec::new();
            ga.push(H::Local(ss, fresh(cx)));
            ga.push(H::Local(si, fresh(cx)));
            let gh = H::CallH(String::from("vec_get"), ga, fresh(cx));
            let ec = deep_clone_h(cx, gh, el.as_str());
            let mut pa: Vec<H> = Vec::new();
            pa.push(H::Local(sd, fresh(cx)));
            pa.push(ec);
            let push = H::CallH(String::from("vec_push"), pa, fresh(cx));
            let inc = H::AssignH(
                si,
                Box::new(H::BinH(
                    String::from("+"),
                    Box::new(H::Local(si, fresh(cx))),
                    Box::new(H::Lit(1, fresh(cx))),
                    fresh(cx),
                )),
                fresh(cx),
            );
            let body = H::SeqH(Box::new(push), Box::new(inc), fresh(cx));
            let lp = H::WhileH(Box::new(cond), Box::new(body), fresh(cx));
            let res = H::SeqH(Box::new(lp), Box::new(H::Local(sd, fresh(cx))), fresh(cx));
            let na: Vec<H> = Vec::new();
            let nv = H::CallH(String::from("vec_new"), na, fresh(cx));
            let l3 = H::LetH(si, Box::new(H::Lit(0, fresh(cx))), Box::new(res), fresh(cx));
            let l2 = H::LetH(sd, Box::new(nv), Box::new(l3), fresh(cx));
            let l1 = H::LetH(ss, Box::new(v), Box::new(l2), fresh(cx));
            let _ = cx.names.pop();
            let _ = cx.stypes.pop();
            let _ = cx.names.pop();
            let _ = cx.stypes.pop();
            let _ = cx.names.pop();
            let _ = cx.stypes.pop();
            return l1;
        }
        let mut a: Vec<H> = Vec::new();
        a.push(v);
        let i = fresh(cx);
        return H::CallH(String::from("vclone"), a, i);
    }
    let nf = oast::sfields(&cx.sdefs, ty);
    if nf <= 0 {
        return v;
    }
    cx.names.push(String::from("$dcl"));
    cx.stypes.push(String::from(ty));
    let sl = (cx.names.len() - 1) as u32;
    let mut lf: Vec<H> = Vec::new();
    let mut k = 0;
    while k < (nf as usize) {
        let fty = oast::sfield_ty_ix(&cx.sdefs, ty, k);
        let li = fresh(cx);
        let fi = fresh(cx);
        let fh = H::FieldH(Box::new(H::Local(sl, li)), k as u32, fi);
        // CLONE-TPARAM (W7): a field typed by a bare type PARAMETER (`T`) is copied as is -- recursing on `T` resolved it (by
        // the sfield suffix match) back to the struct itself: unbounded recursion, ourc SEGFAULTED on `#[derive(Copy)] W<T>`
        let c = if fty.len() == 0 || bare_param_name(cx, fty.as_str()) || fty.as_str() == ty {
            fh
        } else {
            deep_clone_h(cx, fh, fty.as_str())
        };
        lf.push(c);
        k = k + 1;
    }
    let i = fresh(cx);
    let lit = H::StructLitH(nf as u32, lf, i);
    let _ = cx.names.pop();
    let _ = cx.stypes.pop();
    let li2 = fresh(cx);
    H::LetH(sl, Box::new(v), Box::new(lit), li2)
}
// UNWRAP-PAY: the type an unwrap-family call answers -- the receiver's payload: a slot typed `Option$T` / `Result$T$E`, or a
// bare Option/Result whose payload pay_of knows. "" when unknown (the old path then applies). Without it `o.unwrap()` on a
// str payload was typed int and printed the string's POINTER (pre-existing).
fn unwrap_pay(cx: &LCx, r: &E) -> String {
    let t = ety_of(cx, r);
    let p = if oast::str_starts(t.as_str(), "Option$") {
        String::from(&t[7..])
    } else if oast::str_starts(t.as_str(), "Result$") {
        let s = split_dollar_l(t.as_str());
        if s.len() >= 2 {
            String::from(s[1].as_str())
        } else {
            String::from("")
        }
    } else if t.as_str() == "Option" {
        pay_of(cx, r)
    } else if t.as_str() == "Result" {
        let q = pay_of(cx, r);
        let s = split_dollar_l(q.as_str());
        if s.len() >= 1 {
            String::from(s[0].as_str())
        } else {
            String::from("")
        }
    } else {
        String::from("")
    };
    if p.len() > 0 && is_known_ty(cx, &p) {
        p
    } else {
        String::from("")
    }
}
fn pay_of(cx: &LCx, e: &E) -> String {
    match e {
        E::Call(n, _a, _l, _h) => {
            let mut pre = String::from(n.as_str());
            pre.push('=');
            let mut i = 0;
            while i < cx.fpays.len() {
                if oast::str_starts(cx.fpays[i].as_str(), pre.as_str()) {
                    let p = &cx.fpays[i][pre.len()..];
                    let mut o = String::from("");
                    for c in p.chars() {
                        if c == ',' {
                            o.push('$');
                        } else {
                            o.push(c);
                        }
                    }
                    return o;
                }
                i = i + 1;
            }
            String::from("")
        }
        E::EnumLit(_en, vn, xs, _l, _h) if vn.as_str() == "Some" && xs.len() == 1 => {
            ety_of(cx, &xs[0])
        } // DBG-PAYLOAD-LIT: `Some(x)` carries x's type
        E::MethodCall(r, m, _a, _l, _h) => {
            let mm = m.as_str();
            if mm == "max_by_key"
                || mm == "min_by_key"
                || mm == "max"
                || mm == "min"
                || mm == "first"
                || mm == "last"
                || mm == "find"
                || mm == "pop"
                || mm == "get"
                || mm == "next"
                || mm == "nth"
            {
                let el = vec_elem_of(&ety_of(cx, r));
                if el.len() > 0 {
                    return el;
                }
            }
            String::from("")
        }
        _ => String::from(""),
    }
}
pub fn is_simple_place(e: &E) -> bool {
    match e {
        E::Var(_n, _l, _h) => true,
        _ => false,
    }
} // STRUCT-EQ: a Var re-reads safely, so its fields can be named twice
  // RES-MATCH: the payload type of an arm of a QUALIFIED prelude Option/Result scrutinee (`Result$i64$str`): Some/Ok take the
  // first argument, Err the second. Only for the flat shapes (one `$` per argument), anything nested answers "" (unknown).
pub fn qual_pay(qt: &str, v: &str) -> String {
    let b = qt.as_bytes();
    let mut parts: Vec<String> = Vec::new();
    let mut st = 0;
    let mut i = 0;
    while i <= b.len() {
        if i == b.len() || b[i] == 36 {
            parts.push(String::from(&qt[st..i]));
            st = i + 1;
        }
        i = i + 1;
    }
    let fix = |s: &str| {
        if s == "String" {
            String::from("str")
        } else {
            String::from(s)
        }
    };
    if parts.len() >= 2 && parts[0].as_str() == "Option" && v == "Some" {
        return fix(&qt[7..]);
    } // the WHOLE rest: a tuple payload `tup2$str$i64` carries its own `$`s
    if parts.len() == 3 && parts[0].as_str() == "Result" {
        if v == "Ok" {
            return fix(parts[1].as_str());
        }
        if v == "Err" {
            return fix(parts[2].as_str());
        }
    }
    String::from("")
}
// SEQ-EQ-MIXED: an array PLACE as a fresh Vec of its elements (arr_slice over its whole length); a non-place is lowered as is
fn arr_as_vec(e: &E, cx: &mut LCx) -> H {
    if let E::ArrLit(ps, _l, _h) = e {
        let n = ps.len() as i64;
        let a0 = lower(e, cx);
        let z = fresh(cx);
        let zn = fresh(cx); // an array LITERAL: its length is static
        let mut sa: Vec<H> = Vec::new();
        sa.push(a0);
        sa.push(H::Lit(0, z));
        sa.push(H::Lit(n, zn));
        let ic = fresh(cx);
        return H::CallH(String::from("arr_slice"), sa, ic);
    }
    if oparse::place_name(e).len() == 0 {
        let mut pr: Vec<H> = Vec::new();
        let z = fresh(cx);
        pr.push(H::Lit(0, z));
        let i = fresh(cx);
        return H::CallH(String::from("$seqeq_unsupported"), pr, i);
    } /* a non-place, non-literal array has no Vec view here: refuse, not guess */
    let a0 = lower(e, cx);
    let a1 = lower(e, cx);
    let mut na: Vec<H> = Vec::new();
    na.push(a1);
    let il = fresh(cx);
    let ln = H::CallH(String::from("arr_len"), na, il);
    let z = fresh(cx);
    let mut sa: Vec<H> = Vec::new();
    sa.push(a0);
    sa.push(H::Lit(0, z));
    sa.push(ln);
    let ic = fresh(cx);
    H::CallH(String::from("arr_slice"), sa, ic)
}
// DYN-RET: the return type of trait method m called through `dyn Tr` -- an implementing type's `Type$m` (they agree), else the
// trait's default body `Tr$m`; "" when unknown. Without it `s.name()` / `s.area()` printed a String/f64 as an int.
fn dyn_ret(cx: &LCx, tr: &str, m: &str) -> String {
    let ts = dyn_types(cx, tr);
    let mut i = 0;
    while i < ts.len() {
        let mut f = String::from(ts[i].as_str());
        f.push('$');
        f.push_str(m);
        let r = fret_of(cx, &f);
        if r.len() > 0 {
            return r;
        }
        i = i + 1;
    }
    let mut d = String::from(tr);
    d.push('$');
    d.push_str(m);
    fret_of(cx, &d)
}
// GEN-INST: `let p = Pair { a: 2.5, b: 1.5 };` for a generic struct records "p=Pair:T=f64" (each type parameter taken from the
// first field DECLARED with it and the type of that field's value), so `p.max()` whose declared return is `T` answers f64.
// A side table, not a renamed slot type: field access and method dispatch key on the base struct name (GEN-IMPL).
fn ginst_of(cx: &LCx, v: &E) -> String {
    if let E::MethodCall(r, m, _a, _l, _h) = v {
        // R12e RET-GINST: `let q = p.swap()` with `fn swap(&self) -> Pair<B, A>` -- q's instance maps each return argument
        // (a parameter NAME) through p's own instance: p=Pair:A=int,B=char  ->  q=Pair:A=char,B=int
        if let E::Var(pn, _vl, _vh) = r.as_ref() {
            let base = gbase(sty_of(cx, pn).as_str());
            let mut key = String::from("@rg:");
            key.push_str(base.as_str());
            key.push('$');
            key.push_str(m.as_str());
            key.push('=');
            let mut q = 0;
            while q < cx.fpays.len() {
                if oast::str_starts(cx.fpays[q].as_str(), key.as_str()) {
                    let args = split_commas_l(&cx.fpays[q][key.len()..]);
                    let mut si = 0;
                    while si < cx.sdefs.len() {
                        if cx.sdefs[si].name.as_str() == base.as_str()
                            && cx.sdefs[si].sgp.len() == args.len()
                        {
                            let mut out = String::from(base.as_str());
                            out.push(':');
                            let mut k = 0;
                            while k < args.len() {
                                let val = ginst_param(cx, pn, base.as_str(), args[k].as_str());
                                if val.len() == 0 {
                                    return String::new();
                                }
                                if k > 0 {
                                    out.push(',');
                                }
                                out.push_str(cx.sdefs[si].sgp[k].as_str());
                                out.push('=');
                                out.push_str(val.as_str());
                                k = k + 1;
                            }
                            return out;
                        }
                        si = si + 1;
                    }
                    return String::new();
                }
                q = q + 1;
            }
        }
    }
    match v {
        // a statement-then-value (`$updfromdrop` before a struct-update literal) has the instance of its VALUE
        E::Seq(_a, b, _l, _h) => ginst_of(cx, b),
        E::Call(fnm0, cargs, _l, _h) => {
            // GEN-INST-PASS (R3): a generic fn whose return type is spelled exactly like one of its parameters' (`fn
            // box_it<T>(x: Triple<T>) -> Box<Triple<T>>`) returns that argument's instance; `x.y` typed as the bare `T`
            let si = slot_of(cx, fnm0) as usize;
            if si < cx.names.len() && cx.names[si].as_str() == fnm0.as_str() {
                return String::from("");
            }
            let fnm = &self_fix(cx, fnm0);
            let fi = fidx_of(cx, fnm);
            let rt = fret_of(cx, fnm);
            let mut pp = pty_pos(cx, fi, &rt);
            if pp < 0 && oast::str_starts(rt.as_str(), "Box$") {
                pp = pty_pos(cx, fi, &rt[4..]);
            }
            if pp >= 0 && (pp as usize) < cargs.len() {
                return ginst_of(cx, &cargs[pp as usize]);
            }
            // `fn reclift<T>(t: T) -> Recbox<T>`: a one-parameter generic struct whose argument is spelled exactly like a
            // parameter's type takes that argument's type
            let b = gbase(rt.as_str());
            let a = garg(rt.as_str());
            let mut k = 0;
            while k < cx.sdefs.len() {
                if cx.sdefs[k].name.as_str() == b.as_str()
                    && cx.sdefs[k].sgp.len() == 1
                    && a.len() > 0
                {
                    let q = pty_pos(cx, fi, &a);
                    if q >= 0 && (q as usize) < cargs.len() {
                        let at = ety_of(cx, &cargs[q as usize]);
                        if at.len() == 0 {
                            return String::from("");
                        }
                        let mut out = b.clone();
                        out.push(':');
                        out.push_str(cx.sdefs[k].sgp[0].as_str());
                        out.push('=');
                        out.push_str(&at);
                        return out;
                    }
                }
                k = k + 1;
            }
            String::from("")
        }
        E::StructLit(sn, fs, _l, _h) => {
            let st = oast::sresolve(&cx.sdefs, sn);
            let mut si = 0;
            while si < cx.sdefs.len() && cx.sdefs[si].name.as_str() != st.as_str() {
                si = si + 1;
            }
            if si >= cx.sdefs.len() || cx.sdefs[si].sgp.len() == 0 {
                return String::from("");
            }
            let mut out = String::from(st.as_str());
            out.push(':');
            let mut g = 0;
            while g < cx.sdefs[si].sgp.len() {
                let gp = String::from(cx.sdefs[si].sgp[g].as_str());
                let mut k = 0;
                let mut found = String::from("");
                while k < cx.sdefs[si].ftys.len() && k < fs.len() {
                    if cx.sdefs[si].ftys[k].as_str() == gp.as_str() {
                        found = ety_of(cx, &fs[k]);
                        k = fs.len();
                    }
                    k = k + 1;
                }
                if found.len() == 0 {
                    return String::from("");
                }
                if g > 0 {
                    out.push(',');
                }
                out.push_str(&gp);
                out.push('=');
                out.push_str(&found);
                g = g + 1;
            }
            out
        }
        E::EnumLit(t, vn, ps, _l, _h) => {
            // GEN-INST for an enum literal (DROP-GLUE generics): `Op::Yes(D(2))` records "Op:T=D" -- the payload declared with
            // a name that is no known type is the parameter, bound to the payload value's type
            let mut ei = 0;
            while ei < cx.edefs.len() && cx.edefs[ei].name.as_str() != t.as_str() {
                ei = ei + 1;
            }
            if ei >= cx.edefs.len() {
                return String::from("");
            }
            let mut vi = 0;
            while vi < cx.edefs[ei].variants.len()
                && cx.edefs[ei].variants[vi].name.as_str() != vn.as_str()
            {
                vi = vi + 1;
            }
            if vi >= cx.edefs[ei].variants.len() {
                return String::from("");
            }
            let mut q = 0;
            while q < cx.edefs[ei].variants[vi].ptys.len() && q < ps.len() {
                let pt = cx.edefs[ei].variants[vi].ptys[q].as_str();
                if is_type_param(cx, pt) {
                    let vt = ety_of(cx, &ps[q]);
                    if vt.len() == 0 {
                        return String::from("");
                    }
                    let mut out = String::from(t.as_str());
                    out.push(':');
                    out.push_str(pt);
                    out.push('=');
                    out.push_str(vt.as_str());
                    return out;
                }
                q = q + 1;
            }
            String::from("")
        }
        _ => String::from(""),
    }
}
// DROP-GLUE generics: the instance type of binding n from its GEN-INST record ("W:T=D" -> "W$D"); vt when there is no
// record or the record binds more than one parameter
pub fn ginst_vt(cx: &LCx, n: &str, vt: &str) -> String {
    let mut pre = String::from(n);
    pre.push('=');
    let mut i = cx.ginst.len();
    while i > 0 {
        i = i - 1;
        if oast::str_starts(cx.ginst[i].as_str(), pre.as_str()) {
            let rb = cx.ginst[i].as_bytes();
            let mut c = pre.len();
            let mut base = String::new();
            while c < rb.len() && rb[c] != 58 {
                base.push(rb[c] as char);
                c = c + 1;
            }
            let mut val = String::new();
            let mut seen_eq = false;
            c = c + 1;
            while c < rb.len() {
                if rb[c] == 44 {
                    return String::from(vt);
                }
                if seen_eq {
                    val.push(rb[c] as char);
                } else if rb[c] == 61 {
                    seen_eq = true;
                }
                c = c + 1;
            }
            if base.as_str() != vt || val.len() == 0 {
                return String::from(vt);
            }
            let mut r = base;
            r.push('$');
            r.push_str(val.as_str());
            return r;
        }
    }
    String::from(vt)
}
fn gen_ret(cx: &LCx, r: &E, m: &str) -> String {
    let vn = match r {
        E::Var(x, _a, _b) => String::from(x.as_str()),
        _ => {
            return String::from("");
        }
    };
    let mut pre = String::from(vn.as_str());
    pre.push('=');
    let mut rec = String::from("");
    let mut i = cx.ginst.len();
    while i > 0 {
        i = i - 1;
        if oast::str_starts(cx.ginst[i].as_str(), pre.as_str()) {
            rec = String::from(&cx.ginst[i][pre.len()..]);
            i = 0;
        }
    }
    if rec.len() == 0 {
        return String::from("");
    }
    let b = rec.as_bytes();
    let mut c = 0;
    while c < b.len() && b[c] != 58 {
        c = c + 1;
    }
    if c >= b.len() {
        return String::from("");
    }
    let st = String::from(&rec[..c]);
    let binds = String::from(&rec[c + 1..]);
    let mut fnm = String::from(st.as_str());
    fnm.push('$');
    fnm.push_str(m);
    let rt = fret_of(cx, &fnm);
    if rt.len() == 0 {
        return String::from("");
    }
    let parts = split_c(&binds);
    let mut q = 0;
    while q < parts.len() {
        let pb = parts[q].as_bytes();
        let mut e = 0;
        while e < pb.len() && pb[e] != 61 {
            e = e + 1;
        }
        if e < pb.len() && &parts[q][..e] == rt.as_str() {
            return String::from(&parts[q][e + 1..]);
        }
        q = q + 1;
    }
    String::from("")
}
// GEN-INST-GUARD: an `impl<T> S<T>` method body is lowered ONCE (GEN-IMPL shares the layout and the body), with integer
// operators. On a non-integer instance (T = f64 / str / ..) a body that COMPARES or does ARITHMETIC on T gives a wrong
// answer (-1.5 vs -2.5 compared as bit patterns, "pear" vs "apple" by address). Until impl methods are monomorphized per
// instance, such a call is REFUSED. gops = the generic-struct methods whose body has any such operator.
pub fn e_has_ordop(e: &E) -> bool {
    match e {
        E::Bin(o, a, b, _l, _h) => {
            let s = o.as_str();
            if s == "<"
                || s == ">"
                || s == "<="
                || s == ">="
                || s == "+"
                || s == "-"
                || s == "*"
                || s == "/"
                || s == "%"
            {
                return true;
            }
            e_has_ordop(a) || e_has_ordop(b)
        }
        E::Seq(a, b, _l, _h) => e_has_ordop(a) || e_has_ordop(b),
        E::If(c, t, f, _l, _h) => e_has_ordop(c) || e_has_ordop(t) || e_has_ordop(f),
        E::Let(_n, v, b, _l, _h) => e_has_ordop(v) || e_has_ordop(b),
        E::LetT(_n, _t, v, b, _l, _h) => e_has_ordop(v) || e_has_ordop(b),
        E::While(c, b, _l, _h) => e_has_ordop(c) || e_has_ordop(b),
        E::RetE(x, _l, _h) => e_has_ordop(x),
        E::Not(x, _l, _h) => e_has_ordop(x),
        E::Match(s, arms, _l, _h) => {
            if e_has_ordop(s) {
                return true;
            }
            let mut k = 0;
            while k < arms.len() {
                if e_has_ordop(&arms[k].body) {
                    return true;
                }
                k = k + 1;
            }
            false
        }
        E::Call(_f, a, _l, _h) => {
            let mut k = 0;
            while k < a.len() {
                if e_has_ordop(&a[k]) {
                    return true;
                }
                k = k + 1;
            }
            false
        }
        E::MethodCall(r, _m, a, _l, _h) => {
            if e_has_ordop(r) {
                return true;
            }
            let mut k = 0;
            while k < a.len() {
                if e_has_ordop(&a[k]) {
                    return true;
                }
                k = k + 1;
            }
            false
        }
        _ => false,
    }
}
pub fn fill_gops(lc: &mut LCx, funs: &Vec<oast::Fun>) {
    let mut i = 0;
    while i < funs.len() {
        let ow = funs[i].owner.as_str();
        if ow.len() > 0 {
            let mut si = 0;
            while si < lc.sdefs.len() {
                if lc.sdefs[si].name.as_str() == ow
                    && lc.sdefs[si].sgp.len() > 0
                    && e_has_ordop(&funs[i].body)
                {
                    lc.gops.push(String::from(funs[i].name.as_str()));
                }
                si = si + 1;
            }
        }
        i = i + 1;
    }
}
// true when receiver `r` is a recorded instance with a NON-integer type argument and `S$m` is in gops
pub fn gen_unsound(cx: &LCx, r: &E, m: &str) -> bool {
    let vn = match r {
        E::Var(x, _a, _b) => String::from(x.as_str()),
        _ => {
            return false;
        }
    };
    let mut pre = String::from(vn.as_str());
    pre.push('=');
    let mut rec = String::from("");
    let mut i = cx.ginst.len();
    while i > 0 {
        i = i - 1;
        if oast::str_starts(cx.ginst[i].as_str(), pre.as_str()) {
            rec = String::from(&cx.ginst[i][pre.len()..]);
            i = 0;
        }
    }
    if rec.len() == 0 {
        return false;
    }
    let b = rec.as_bytes();
    let mut c = 0;
    while c < b.len() && b[c] != 58 {
        c = c + 1;
    }
    if c >= b.len() {
        return false;
    }
    let mut fnm = String::from(&rec[..c]);
    fnm.push('$');
    fnm.push_str(m);
    let mut ing = false;
    let mut q = 0;
    while q < cx.gops.len() {
        if cx.gops[q].as_str() == fnm.as_str() {
            ing = true;
        }
        q = q + 1;
    }
    if !ing {
        return false;
    }
    let parts = split_c(&rec[c + 1..]);
    let mut p = 0;
    while p < parts.len() {
        let pb = parts[p].as_bytes();
        let mut e = 0;
        while e < pb.len() && pb[e] != 61 {
            e = e + 1;
        }
        if e < pb.len() {
            let t = &parts[p][e + 1..];
            if !(t == "int" || oast::is_int_width(t) || t == "bool" || t == "char") {
                return true;
            }
        }
        p = p + 1;
    }
    false
}
// RES-MATCH-PAY: the scrutinee type with its Option/Result payload qualified (`st.pop()` on Vec<(String, i64)> is
// `Option$tup2$str$i64`): ety_of answers the bare enum for a call/method, pay_of knows the element/declared payload.
fn qual_scrut(cx: &LCx, sc: &E) -> String {
    let e0 = ety_of(cx, sc);
    if (e0.as_str() == "Option" || e0.as_str() == "Result") {
        let p = pay_of(cx, sc);
        if p.len() > 0 {
            let mut q = String::from(e0.as_str());
            q.push('$');
            q.push_str(&p);
            return q;
        }
    }
    e0
}
// STR-FIELD-GROW: a structural copy of a PLACE expression (var, field, tuple index, index by a var/literal); None otherwise
fn clone_place(e: &E) -> Option<E> {
    match e {
        E::Var(n, l, h) => Some(E::Var(String::from(n.as_str()), *l, *h)),
        E::Field(b, f, l, h) => match clone_place(b) {
            Some(c) => Some(E::Field(Box::new(c), String::from(f.as_str()), *l, *h)),
            None => None,
        },
        E::TupIdx(b, i, l, h) => match clone_place(b) {
            Some(c) => Some(E::TupIdx(Box::new(c), *i, *l, *h)),
            None => None,
        },
        E::Index(b, ix, l, h) => {
            let ic = match ix.as_ref() {
                E::Int(v, a, c) => Some(E::Int(*v, *a, *c)),
                E::Var(n, a, c) => Some(E::Var(String::from(n.as_str()), *a, *c)),
                _ => None,
            };
            match (clone_place(b), ic) {
                (Some(c), Some(i2)) => Some(E::Index(Box::new(c), Box::new(i2), *l, *h)),
                _ => None,
            }
        }
        _ => None,
    }
}
// AGG-EQ: the value at `path` (a chain of field/element indices) below a bound slot -- pure re-readable reads
fn agg_get(cx: &mut LCx, s: u32, path: &Vec<u32>) -> H {
    let i0 = fresh(cx);
    let mut h = H::Local(s, i0);
    let mut k = 0;
    while k < path.len() {
        let i = fresh(cx);
        h = H::FieldH(Box::new(h), path[k], i);
        k = k + 1;
    }
    h
}
// AGG-EQ: equality of the values at `path` under slots sa/sb, of type ty; None = a part this cannot compare (refuse)
fn agg_eq(cx: &mut LCx, sa: u32, sb: u32, path: &Vec<u32>, ty: &str) -> Option<H> {
    if path.len() > 12 {
        return None; // AGG-EQ depth bound: a generic field type (`x: A`) resolved back to its own struct and recursed forever (ourc hung)
    }
    if ty == "int" || ty == "bool" || ty == "char" || oast::is_int_width(ty) {
        let x = agg_get(cx, sa, path);
        let y = agg_get(cx, sb, path);
        let i = fresh(cx);
        return Some(H::BinH(String::from("=="), Box::new(x), Box::new(y), i));
    }
    if ty == "str" {
        let x = agg_get(cx, sa, path);
        let y = agg_get(cx, sb, path);
        let mut ag: Vec<H> = Vec::new();
        ag.push(x);
        ag.push(y);
        let i = fresh(cx);
        return Some(H::CallH(String::from("streq"), ag, i));
    }
    let mut parts: Vec<String> = Vec::new();
    if oast::str_starts(ty, "tup") {
        parts = oparse::tup_elems(ty);
        if parts.len() == 0 {
            return None;
        }
    } else {
        let st = oast::sresolve(&cx.sdefs, ty);
        let mut si = 0;
        while si < cx.sdefs.len() && cx.sdefs[si].name.as_str() != st.as_str() {
            si = si + 1;
        }
        if si >= cx.sdefs.len() || cx.sdefs[si].fields.len() == 0 {
            return None;
        }
        let mut k = 0;
        while k < cx.sdefs[si].ftys.len() {
            parts.push(String::from(cx.sdefs[si].ftys[k].as_str()));
            k = k + 1;
        }
        if parts.len() != cx.sdefs[si].fields.len() {
            return None;
        }
    }
    let mut acc: Option<H> = None;
    let mut k = 0;
    while k < parts.len() {
        let mut p2: Vec<u32> = Vec::new();
        let mut q = 0;
        while q < path.len() {
            p2.push(path[q]);
            q = q + 1;
        }
        p2.push(k as u32);
        let e = match agg_eq(cx, sa, sb, &p2, parts[k].as_str()) {
            Some(x) => x,
            None => {
                return None;
            }
        };
        acc = match acc {
            None => Some(e),
            Some(prev) => {
                let i = fresh(cx);
                Some(H::BinH(String::from("&"), Box::new(prev), Box::new(e), i))
            }
        };
        k = k + 1;
    }
    match acc {
        Some(x) => Some(x),
        None => {
            let z = fresh(cx);
            Some(H::Lit(1, z))
        }
    }
}
// DROP-GLUE generics: a payload/field type that is a TYPE PARAMETER -- capitalised and naming no struct or enum
// (`i64`, `u8` are not parameters; they are only missing from is_known_ty2's spelling list)
pub fn is_type_param(cx: &LCx, t: &str) -> bool {
    let b = t.as_bytes();
    if b.len() == 0 {
        return true; // the parser stores a type-PARAMETER payload as "" (measured: `Op<T> { Yes(T) }` -> ptys [""])
    }
    b.len() > 0
        && b[0] >= 65
        && b[0] <= 90
        && !oast::str_contains2(t, "$")
        && !oast::is_struct_name(&cx.sdefs, t)
        && !oast::is_enum_name(&cx.edefs, t)
}
// R6 ZERO-DFLT-USE: `let i = Default::default(); .. f(i)` -- the binding's type is the declared type of the first user-fn
// parameter it is passed to ("" when no such use: the `$zerodflt` refusal stands)
pub fn dflt_use_ty(cx: &LCx, e: &E, n: &str) -> String {
    match e {
        E::Call(f, a, _l, _h) => {
            let mut k = 0;
            while k < a.len() {
                if is_var(&a[k], n) {
                    let fi = fidx_of(cx, f.as_str());
                    if fi >= 0 && (fi as usize) < cx.fptys.len() {
                        let ps = split_commas_l(cx.fptys[fi as usize].as_str());
                        if k < ps.len() && ps[k].len() > 0 {
                            return String::from(ps[k].as_str());
                        }
                    }
                }
                let t = dflt_use_ty(cx, &a[k], n);
                if t.len() > 0 {
                    return t;
                }
                k = k + 1;
            }
            String::new()
        }
        E::MethodCall(r, _m, a, _l, _h) => {
            let t = dflt_use_ty(cx, r, n);
            if t.len() > 0 {
                return t;
            }
            let mut k = 0;
            while k < a.len() {
                let t2 = dflt_use_ty(cx, &a[k], n);
                if t2.len() > 0 {
                    return t2;
                }
                k = k + 1;
            }
            String::new()
        }
        E::Seq(a, b, _l, _h) => dflt_two(cx, a, b, n),
        E::Bin(_o, a, b, _l, _h) => dflt_two(cx, a, b, n),
        E::While(a, b, _l, _h) => dflt_two(cx, a, b, n),
        E::Let(m, v, b, _l, _h) => {
            if m.as_str() == n {
                return dflt_use_ty(cx, v, n);
            }
            dflt_two(cx, v, b, n)
        }
        E::LetT(m, _t, v, b, _l, _h) => {
            if m.as_str() == n {
                return dflt_use_ty(cx, v, n);
            }
            dflt_two(cx, v, b, n)
        }
        E::If(c, t, f, _l, _h) => {
            let a = dflt_use_ty(cx, c, n);
            if a.len() > 0 {
                return a;
            }
            let b = dflt_use_ty(cx, t, n);
            if b.len() > 0 {
                return b;
            }
            dflt_use_ty(cx, f, n)
        }
        _ => String::new(),
    }
}
pub fn split_commas_l(s: &str) -> Vec<String> {
    let mut o: Vec<String> = Vec::new();
    let mut cur = String::new();
    let b = s.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == 44 {
            o.push(cur);
            cur = String::new();
        } else {
            cur.push(b[i] as char);
        }
        i = i + 1;
    }
    o.push(cur);
    o
}
fn dflt_two(cx: &LCx, a: &E, b: &E, n: &str) -> String {
    let t = dflt_use_ty(cx, a, n);
    if t.len() > 0 {
        return t;
    }
    dflt_use_ty(cx, b, n)
}
// DROP-RECV: could a value of type `t` OWN a value of type `want` (itself, a struct field, an enum payload, an
// Option/Vec/array element, recursively)? Conservative: an unknown or generic-erased type answers true.
pub fn ty_owns(cx: &LCx, t: &str, want: &str, depth: u32) -> bool {
    if t.len() == 0 {
        return false; // unit / no declared return
    }
    if depth > 6 || oast::str_contains2(t, want) {
        return true;
    }
    if t == "int" || t == "bool" || t == "char" || t == "f64" || t == "str" || oast::is_int_width(t)
    {
        return false;
    }
    if oast::str_starts(t, "Vec$") || oast::str_starts(t, "arr$") || oast::str_starts(t, "Option$")
    {
        let b = t.as_bytes();
        let mut j = 0;
        while j < b.len() && b[j] != 36 {
            j = j + 1;
        }
        let mut inner = String::new();
        j = j + 1;
        while j < b.len() {
            inner.push(b[j] as char);
            j = j + 1;
        }
        return ty_owns(cx, inner.as_str(), want, depth + 1);
    }
    let mut i = 0;
    while i < cx.sdefs.len() {
        if cx.sdefs[i].name.as_str() == t {
            let mut k = 0;
            while k < cx.sdefs[i].ftys.len() {
                if cx.sdefs[i].ftys[k].len() == 0
                    || ty_owns(cx, cx.sdefs[i].ftys[k].as_str(), want, depth + 1)
                {
                    return true;
                }
                k = k + 1;
            }
            return false;
        }
        i = i + 1;
    }
    let mut e = 0;
    while e < cx.edefs.len() {
        if cx.edefs[e].name.as_str() == t {
            let mut v = 0;
            while v < cx.edefs[e].variants.len() {
                let mut q = 0;
                while q < cx.edefs[e].variants[v].ptys.len() {
                    let pt = cx.edefs[e].variants[v].ptys[q].as_str();
                    if pt.len() == 0 || ty_owns(cx, pt, want, depth + 1) {
                        return true;
                    }
                    q = q + 1;
                }
                v = v + 1;
            }
            return false;
        }
        e = e + 1;
    }
    true
}
// DROP-RECV: is e itself a method call (so the end of e is the end of the full expression)?
pub fn is_mcall(e: &E) -> bool {
    match e {
        E::MethodCall(_r, _m, _a, _l, _h) => true,
        _ => false,
    }
}
// FOR-BORROW: is v `seq_get($fcr.., i)` -- an element read out of a BY-REFERENCE loop's container (a borrow)?
pub fn is_fcr_get(v: &E) -> bool {
    match v {
        E::Call(f, a, _l, _h) if f.as_str() == "seq_get" && a.len() == 2 => match &a[0] {
            E::Var(c, _l2, _h2) => {
                oast::str_starts(c.as_str(), "$fcr") || oast::str_starts(c.as_str(), "$fct")
            }
            _ => false,
        },
        _ => false,
    }
}
// GEN-INST-PASS (R3): a generic struct named WITH its parameter (`Triple$U`, from a generic fn's return type) reads as the bare
// struct, so the binding's GEN-INST applies; any other name is returned unchanged
fn gen_base_of(cx: &LCx, os: String) -> String {
    let b = gbase(os.as_str());
    if b.len() == os.len() || !bare_param_name(cx, garg(os.as_str()).as_str()) {
        return os;
    }
    let mut k = 0;
    while k < cx.sdefs.len() {
        if cx.sdefs[k].name.as_str() == b.as_str() && cx.sdefs[k].sgp.len() > 0 {
            return b;
        }
        k = k + 1;
    }
    os
}
// an unqualified capitalized name that is EXACTLY no struct or enum (is_type_param's suffix match took `T` for a `..$T` struct)
fn bare_param_name(cx: &LCx, t: &str) -> bool {
    let b = t.as_bytes();
    if b.len() == 0 || b[0] < 65 || b[0] > 90 || oast::str_contains2(t, "$") {
        return false;
    }
    let mut k = 0;
    while k < cx.sdefs.len() {
        if cx.sdefs[k].name.as_str() == t {
            return false;
        }
        k = k + 1;
    }
    k = 0;
    while k < cx.edefs.len() {
        if cx.edefs[k].name.as_str() == t {
            return false;
        }
        k = k + 1;
    }
    true
}
// PROJ-PICK (R3): a field `<R as Tr>::B` whose impls disagree is parsed as `$proj$R|X=u32;Y=str`; the instance's argument
// for R (one-parameter struct) picks its impl's type. No match keeps the marker, which lowering refuses.
fn proj_pick(cx: &LCx, si: usize, a: &str, ft: &str) -> String {
    let body = &ft[6..];
    let bb = body.as_bytes();
    let mut c = 0;
    while c < bb.len() && bb[c] != 124 {
        c = c + 1;
    }
    if c >= bb.len()
        || cx.sdefs[si].sgp.len() != 1
        || cx.sdefs[si].sgp[0].as_str() != &body[..c]
        || a.len() == 0
    {
        return String::from(ft);
    }
    let tab = split_semis(&body[c + 1..]);
    let mut k = 0;
    while k < tab.len() {
        let t = tab[k].as_bytes();
        let mut q = 0;
        while q < t.len() && t[q] != 61 {
            q = q + 1;
        }
        if q < t.len() && &tab[k][..q] == a {
            return String::from(&tab[k][q + 1..]);
        }
        k = k + 1;
    }
    String::from(ft)
}
fn split_semis(s: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut cur = String::new();
    let b = s.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == 59 {
            out.push(cur);
            cur = String::new();
        } else {
            cur.push(b[i] as char);
        }
        i = i + 1;
    }
    out.push(cur);
    out
}
// PROJ-PICK: `let b: H<Y> = H { v: .. }` -- a value of a struct with a `$proj$` field records its instance from the ANNOTATION
// (the literal cannot: the field's declared type is the projection, not the parameter)
fn proj_ann_ginst(cx: &LCx, n: &str, t: &str) -> Option<String> {
    let b = gbase(t);
    let a = garg(t);
    if a.len() == 0 {
        return None;
    }
    let mut k = 0;
    while k < cx.sdefs.len() {
        if cx.sdefs[k].name.as_str() == b.as_str() && cx.sdefs[k].sgp.len() == 1 {
            let mut q = 0;
            while q < cx.sdefs[k].ftys.len() {
                if oast::str_starts(cx.sdefs[k].ftys[q].as_str(), "$proj$") {
                    let mut e = String::from(n);
                    e.push('=');
                    e.push_str(b.as_str());
                    e.push(':');
                    e.push_str(cx.sdefs[k].sgp[0].as_str());
                    e.push('=');
                    e.push_str(a.as_str());
                    return Some(e);
                }
                q = q + 1;
            }
            return None;
        }
        k = k + 1;
    }
    None
}
// DBG-MODNAME (W7): `#[derive(Debug)]` prints the type's OWN name (`A`), not our module-qualified spelling (`m1$m2$A`). Only a
// recorded module prefix (the parser's "@modp:m1$m2$") is stripped, so a generic instance name (`Foo$int`) is untouched.
fn dbg_tyname(cx: &LCx, ty: &str) -> String {
    let mut best = 0;
    let mut k = 0;
    while k < cx.fpays.len() {
        if oast::str_starts(cx.fpays[k].as_str(), "@modp:") {
            let p = &cx.fpays[k][6..];
            if p.len() > best && oast::str_starts(ty, p) && ty.len() > p.len() {
                best = p.len();
            }
        }
        k = k + 1;
    }
    let r = String::from(&ty[best..]);
    // NESTED-ITEM-DBG (R13): an item declared INSIDE a block is renamed `Q$s0` (scope hygiene); Debug prints `Q` (it printed
    // `Q$s0 { x: 0 }`: a WRONG)
    let rb = r.as_bytes();
    let mut k = rb.len();
    while k > 0 && rb[k - 1] >= 48 && rb[k - 1] <= 57 {
        k = k - 1;
    }
    if k < rb.len() && k >= 3 && rb[k - 1] == 115 && rb[k - 2] == 36 {
        return String::from(&r[..k - 2]);
    }
    r
}
// STRUCT-COPY (W7): is `ty` a `#[derive(Copy)]` struct whose fields are all plain values (scalars, or such structs)? A real
// Copy struct can hold only Copy fields; the prelude's `#[derive(Copy)] struct HashMap { ks, vs }` is a HANDLE (Vec fields)
// and must stay shared -- copying it lost every insert (heavy-pass-3+ surface regression, 10 HashMap probes)
fn copy_struct_ty(cx: &LCx, ty: &str) -> bool {
    copy_struct_d(cx, ty, 0)
}
fn copy_struct_d(cx: &LCx, ty: &str, d: i32) -> bool {
    if d > 8 {
        return false;
    }
    let mut k = 0;
    while k < cx.sdefs.len() {
        if cx.sdefs[k].name.as_str() == ty && cx.sdefs[k].copy {
            if ty == "HashMap" {
                return false; // the prelude's Copy HANDLE (Vec fields shared by design) -- never copied
            }
            if cx.sdefs[k].sgp.len() > 0 {
                return false; // a GENERIC Copy struct: deep_clone_h of `Wrapper<T>` SEGFAULTED ourc -- left to V1c (it aliases, as before)
            }
            let mut q = 0;
            while q < cx.sdefs[k].ftys.len() {
                let ft = cx.sdefs[k].ftys[q].as_str();
                let plain = ft == "int"
                    || ft == "bool"
                    || ft == "char"
                    || ft == "f64"
                    || oast::is_int_width(ft)
                    // ARR-FIELD (W7): a Copy struct's `[T; N]` field is stored as `Vec$T` -- in a Copy struct it can only be an ARRAY
                    || ft == "arr"
                    || (oast::str_starts(ft, "Vec$") && {
                        let e = &ft[4..];
                        e == "int" || e == "bool" || e == "char" || e == "f64" || oast::is_int_width(e)
                    })
                    || copy_struct_d(cx, ft, d + 1);
                if !plain {
                    return false;
                }
                q = q + 1;
            }
            return true;
        }
        k = k + 1;
    }
    false
}
// STRUCT-COPY (W7): a PLACE (variable / field / index) of a Copy struct used as a VALUE is a fresh copy -- the value model
// shares aggregates by handle, so `y = x`, `[x, x]` and `let z = arr[0]` aliased (a later write showed through)
fn copy_val(cx: &mut LCx, e: &E) -> H {
    let place = match e {
        E::Var(_n, _a, _b) => true,
        E::Field(_o, _f, _a, _b) => true,
        E::Index(_b, _i, _l, _h) => true,
        _ => false,
    };
    let lv = lower(e, cx);
    if place {
        let ty = ety_of(cx, e);
        if copy_struct_ty(cx, ty.as_str()) {
            return deep_clone_h(cx, lv, ty.as_str());
        }
    }
    lv
}
// STRUCT-COPY (W7): a BY-VALUE parameter of a Copy struct is the callee's own copy -- `fn bump(mut p: A) { p.a += 1 }`
// wrote through to the caller's x (aggregates pass by handle). The entry re-binds `p = p`, which copies (copy_val).
pub fn copy_params(f: &Fun, cx: &mut LCx) -> Option<H> {
    let mut out: Option<H> = None;
    let mut p = 0;
    while p < f.params.len() {
        let isref = p < f.pref.len() && f.pref[p];
        let mut pt = if p < f.ptys.len() {
            String::from(f.ptys[p].as_str())
        } else {
            String::from("")
        };
        if f.params[p].as_str() == "self"
            && (pt.as_str() == "Self" || pt.len() == 0)
            && f.name.contains('$')
        {
            // a by-value `mut self` of a Copy struct is a copy too (`x.squared()` changed x) -- its type is the owner
            let nb = f.name.as_bytes();
            let mut c = nb.len();
            while c > 0 && nb[c - 1] != 36 {
                c = c - 1;
            }
            pt = String::from(&f.name[..c - 1]);
        }
        if !isref && f.params[p].as_str() != "_" && copy_struct_ty(cx, pt.as_str()) {
            // (`_` binds nothing: there is no slot to copy into -- lowering `_ = _` SEGFAULTED ourc)
            let n = String::from(f.params[p].as_str());
            let a = E::Assign(String::from(n.as_str()), Box::new(E::Var(n, 0, 0)), 0, 0);
            let h = lower(&a, cx);
            out = match out {
                None => Some(h),
                Some(prev) => {
                    let i = fresh(cx);
                    Some(H::SeqH(Box::new(prev), Box::new(h), i))
                }
            };
        }
        p = p + 1;
    }
    out
}
// GEN-MULTI (R12): the value of type parameter `p` in binding `n`'s GEN-INST record "n=Base:P1=v1,P2=v2" ("" if `p` is
// not a parameter of `base`, or the record has no value for it)
fn ginst_param(cx: &LCx, n: &str, base: &str, p: &str) -> String {
    if !multi_param_of(cx, base, p) {
        return String::new(); // (an early-return helper: an outer flag set in a loop miscompiles under SEM0)
    }
    let mut pre = String::from(n);
    pre.push('=');
    pre.push_str(base);
    pre.push(':');
    let mut i = cx.ginst.len();
    while i > 0 {
        i = i - 1;
        if oast::str_starts(cx.ginst[i].as_str(), pre.as_str()) {
            let rest = String::from(&cx.ginst[i][pre.len()..]);
            let parts = split_commas_l(rest.as_str());
            let mut k = 0;
            while k < parts.len() {
                let pb = parts[k].as_bytes();
                let mut q = 0;
                while q < pb.len() && pb[q] != 61 {
                    q = q + 1;
                }
                if q < pb.len() && &parts[k][..q] == p {
                    return String::from(&parts[k][q + 1..]);
                }
                k = k + 1;
            }
            return String::new();
        }
    }
    String::new()
}
fn multi_param_of(cx: &LCx, base: &str, p: &str) -> bool {
    let mut si = 0;
    while si < cx.sdefs.len() {
        if cx.sdefs[si].name.as_str() == base
            && cx.sdefs[si].sgp.len() > 1
            && oast::in_list(&cx.sdefs[si].sgp, p)
        {
            return true;
        }
        si = si + 1;
    }
    false
}
// LETELSE-PAYLOAD-TY (W7): the payload type of the binding an arm's body returns (`V2(b) => b` -> V2's payload 0), from the
// scrutinee's enum. "" when the body is not a binding, the enum or variant is unknown, or the payload is a type parameter.
fn arm_bind_payload_ty(cx: &LCx, sc: &E, arm: &oast::Arm) -> String {
    let bv = match &arm.body {
        E::Var(n, _l, _h) => String::from(n.as_str()),
        _ => return String::new(),
    };
    let mut bi = arm.binds.len();
    let mut q = 0;
    while q < arm.binds.len() {
        if arm.binds[q].as_str() == bv.as_str() {
            bi = q;
        }
        q = q + 1;
    }
    if bi >= arm.binds.len() {
        return String::new();
    }
    let en = gbase(ety_of(cx, sc).as_str());
    let ei = oast::eidx(&cx.edefs, en.as_str());
    // the scrutinee's enum, or else the ONE enum that declares this variant name (a nested-tuple position `__pmN` is untyped;
    // a name two enums share answers nothing)
    let mut ei = ei; // (self-host: assigned, not an if-value)
    if ei < 0 {
        ei = unique_variant_owner(cx, arm.variant.as_str());
    }
    if ei < 0 {
        return String::new();
    }
    let ed = &cx.edefs[ei as usize];
    let mut vi = 0;
    while vi < ed.variants.len() {
        if ed.variants[vi].name.as_str() == arm.variant.as_str() {
            if bi < ed.variants[vi].ptys.len() {
                let pt = ed.variants[vi].ptys[bi].as_str();
                if pt.len() > 0
                    && (pt == "int"
                        || pt == "bool"
                        || pt == "char"
                        || pt == "str"
                        || pt == "f64"
                        || oast::is_int_width(pt)
                        || oast::sfields(&cx.sdefs, pt) > 0)
                {
                    return String::from(pt);
                }
            }
            return String::new();
        }
        vi = vi + 1;
    }
    String::new()
}
fn unique_variant_owner(cx: &LCx, vn: &str) -> i32 {
    let mut found = 0 - 1;
    let mut n = 0;
    let mut i = 0;
    while i < cx.edefs.len() {
        let mut v = 0;
        while v < cx.edefs[i].variants.len() {
            if cx.edefs[i].variants[v].name.as_str() == vn {
                found = i as i32;
                n = n + 1;
            }
            v = v + 1;
        }
        i = i + 1;
    }
    if n == 1 {
        found
    } else {
        0 - 1
    }
}
// OPT-PAY-ARM: an arm's value type as lowering sees it -- its own type, or, for `Some(v) => v`, the scrutinee's payload
fn arm_ety(cx: &LCx, sc: &E, a: &Arm) -> String {
    // ARM-STMT-BODY (R2a P4): a statement-shaped body (a block / let / if / loop / nested match) has no reliable value type
    // here -- an `if let P = x { ..stmts.. }` without else is a match whose `_` arm is () (int), and ety_of read the block's
    // tail as str, so ourc's own olower::ety_of was refused as "arms have incompatible types". Only VALUE arms count.
    match &a.body {
        E::Seq(_a, _b, _l, _h) => return String::new(),
        E::Let(_n, _v, _b, _l, _h) => return String::new(),
        E::LetT(_n, _t, _v, _b, _l, _h) => return String::new(),
        E::If(_c, _t, _e, _l, _h) => return String::new(),
        E::While(_c, _b, _l, _h) => return String::new(),
        E::Match(_s, _a, _l, _h) => return String::new(),
        // a DIVERGING arm (`=> return x`, break, continue) has no value type either (it was read as x's type)
        E::RetE(_v, _l, _h) => return String::new(),
        E::BreakE(_l, _h) => return String::new(),
        E::ContE(_l, _h) => return String::new(),
        _ => {}
    }
    let t = ety_of(cx, &a.body);
    if t.len() > 0 {
        return t;
    }
    if a.binds.len() == 1 && a.variant.as_str() == "Some" {
        if let E::Var(bv, _bl, _bh) = &a.body {
            if bv.as_str() == a.binds[0].as_str() {
                let st = ety_of(cx, sc);
                if oast::str_starts(st.as_str(), "Option$") {
                    return String::from(&st[7..]);
                }
            }
        }
    }
    String::new()
}
// do the arms mix a str value with an integer value? (both typed: an untyped arm never counts)
fn arms_str_vs_int(cx: &LCx, sc: &E, arms: &Vec<Arm>) -> bool {
    let mut s = false;
    let mut n = false;
    let mut k = 0;
    // NARROWED (self-compile): lowering mistypes some arms of valid matches (ourc's own ety_of was refused), so only the
    // crash shape is judged -- a `Some(v) => v` arm over an INTEGER payload against an arm that is a string LITERAL
    while k < arms.len() {
        let a = &arms[k];
        let mut lit = false;
        if let E::StrLit(_x, _l, _h) = &a.body {
            lit = true;
        }
        if lit {
            s = true;
        } else if a.binds.len() == 1 && a.variant.as_str() == "Some" {
            let t = arm_ety(cx, sc, a);
            if t.as_str() == "int" || oast::is_int_width(t.as_str()) {
                n = true;
            }
        }
        k = k + 1;
    }
    s && n
}
// BIN-W64 width, walked ONCE: through nested arithmetic it recurses on the operators itself and asks `ety_of` only at a
// non-operator leaf. (The arm used to call `ety_of` on both operands in its guard, again in its body, and the later arms
// again when the guard failed -- every operator level re-typed its whole left subtree several times, so a 12-operator
// chain like `2*2*3*3*..*17` blew up exponentially and was OOM-killed: run e's two CRASHes.)
fn bin_w64_ty(cx: &LCx, o: &String, a: &E, b: &E) -> String {
    let ta = w64_of(cx, a);
    if is_w64(ta.as_str()) {
        return ta;
    }
    if o.as_str() == "<<" || o.as_str() == ">>" {
        return String::new();
    }
    w64_of(cx, b)
}
fn w64_arith(o: &str) -> bool {
    arith_op(o) || o == "&" || o == "|" || o == "^" || o == "<<" || o == ">>"
}
fn w64_of(cx: &LCx, e: &E) -> String {
    if let E::Bin(o, a, b, _l, _h) = e {
        if w64_arith(o.as_str()) {
            return bin_w64_ty(cx, o, a, b);
        }
    }
    let t = ety_of(cx, e);
    if is_w64(t.as_str()) {
        t
    } else {
        String::new()
    }
}
// ETY-EXP: a binary operator's result type -- the former guarded arms of ety_of, IN THEIR ORDER, over operand types computed once
fn ety_bin(cx: &LCx, o: &String, a: &E, b2: &E) -> String {
    let ta = ety_of(cx, a);
    let tb = ety_of(cx, b2);
    let bitop = o.as_str() == "&" || o.as_str() == "|" || o.as_str() == "^";
    let shift = o.as_str() == "<<" || o.as_str() == ">>";
    // an overloaded operator's result type is its method's return type, so `(p + q).x` resolves -- f64 arithmetic is f64
    if (ta.as_str() == "f64" || tb.as_str() == "f64") && arith_op(o) {
        return String::from("f64");
    }
    if o.as_str() == "^" && tb.as_str() == "f64" {
        return String::from("f64");
    }
    if cmp_op(o) {
        return String::from("bool");
    }
    // BIN-W64 (R2a P4): integer arithmetic with an operand of a declared 64-bit width has that width; a shift takes the LEFT's
    if arith_op(o) || bitop || shift {
        if is_w64(ta.as_str()) {
            return ta;
        }
        if !shift && is_w64(tb.as_str()) {
            return tb;
        }
    }
    // BOOL-BITOPS
    if bitop && ta.as_str() == "bool" {
        return String::from("bool");
    }
    // BITOP-WIDTH: `&` `|` `^` keep their typed operand's width (`0b1010_u8 | 0x0F` is u8); mixed widths stay "int"
    if bitop {
        let keep = (oast::is_int_width(&ta)
            && (tb.as_str() == "int" || tb.as_str() == ta.as_str()))
            || (oast::is_int_width(&tb) && ta.as_str() == "int");
        if keep {
            if oast::is_int_width(&ta) {
                return ta;
            }
            return tb;
        }
    }
    // C-UPRINT: arithmetic over a 64-bit UNSIGNED operand is u64/usize (prints unsigned); otherwise "int"
    if arith_op(o) {
        let intish = ta.as_str() == "int"
            || oast::is_int_width(&ta)
            || tb.as_str() == "int"
            || oast::is_int_width(&tb);
        if intish {
            if ta.as_str() == "u64" || tb.as_str() == "u64" {
                return String::from("u64");
            }
            if ta.as_str() == "usize" || tb.as_str() == "usize" {
                return String::from("usize");
            }
            return String::from("int");
        }
    }
    // an overloaded operator (`impl Add for P`): its method's return type
    let m = op_method(o);
    if ta.len() > 0 && m.len() > 0 && !prim_ty(ta.as_str()) {
        let mut mn = String::from(ta.as_str());
        mn.push('$');
        mn.push_str(&m);
        if fidx_of(cx, &mn) >= 0 {
            let r = oast::sresolve(&cx.sdefs, &fret_of(cx, &mn));
            if r.len() > 0 && is_known_ty(cx, &r) {
                return r;
            }
        }
    }
    String::new()
}
// V1-STR-REF: is this initializer `&mut s` with s a String-typed variable?
fn let_mut_ref_str(cx: &LCx, v: &E) -> bool {
    if let E::Ref(x, mt, _l, _h) = v {
        if *mt {
            if let E::Var(_n, _a, _b) = x.as_ref() {
                let t = ety_of(cx, x);
                return t.as_str() == "str" || t.as_str() == "String";
            }
        }
    }
    false
}
// R8 RANGE-TO: the Debug prefix (`..` / `..=`) of a `let x = ..b` binding, or "" for any other name
fn rangeto_prefix(cx: &LCx, n: &str) -> String {
    let mut k1 = String::from("@rangeto:");
    k1.push_str(n);
    let mut k2 = String::from("@rangetoinc:");
    k2.push_str(n);
    if oast::in_list(&cx.rangevars, k1.as_str()) {
        return String::from("..");
    }
    if oast::in_list(&cx.rangevars, k2.as_str()) {
        return String::from("..=");
    }
    String::new()
}
// R8 RANGE-TO: is this index a `let x = ..b` binding (`&v[x]`)?
fn rangeto_index(cx: &LCx, ix: &E) -> bool {
    match ix {
        E::Var(n, _l, _h) => rangeto_prefix(cx, n.as_str()).len() > 0,
        _ => false,
    }
}
// UPD-BASE-MOVE (R2a P5): did a struct-update literal `S { .., ..n }` MOVE n? The desugar's `$partialmove(n.written.., "n")`
// marker names n last; n is moved when one of S's UNWRITTEN fields is not Copy (it moved into the new value -- dropping n
// too was a double drop). All-Copy unwritten fields are copied, and n still drops (rustc: `D { a: 3, ..d0 }` keeps d0).
fn upd_base_moved(cx: &LCx, e: &E, n: &str) -> bool {
    match e {
        E::Call(f, a, _l, _h) => {
            if f.as_str() == "$partialmove" && a.len() > 0 {
                if let E::StrLit(bn, _bl, _bh) = &a[a.len() - 1] {
                    if bn.as_str() == n {
                        let st = tbase(cx, sty_of(cx, n).as_str());
                        let mut i = 0;
                        while i < cx.sdefs.len() {
                            if cx.sdefs[i].name.as_str() == st.as_str() {
                                let mut k = 0;
                                while k < cx.sdefs[i].fields.len() {
                                    let fname = cx.sdefs[i].fields[k].as_str();
                                    let mut written = false;
                                    let mut q = 0;
                                    while q + 1 < a.len() {
                                        if let E::Field(_o, wf, _fl, _fh) = &a[q] {
                                            if wf.as_str() == fname {
                                                written = true;
                                            }
                                        }
                                        q = q + 1;
                                    }
                                    let ft = if k < cx.sdefs[i].ftys.len() {
                                        cx.sdefs[i].ftys[k].as_str()
                                    } else {
                                        ""
                                    };
                                    if !written && !upd_copy_ty(ft) {
                                        return true;
                                    }
                                    k = k + 1;
                                }
                                return false;
                            }
                            i = i + 1;
                        }
                        return true; // unknown struct: the conservative answer is a move (a missed drop, never a double)
                    }
                }
            }
            upd_any(cx, a, n)
        }
        E::Seq(x, y, _l, _h) => upd_base_moved(cx, x, n) || upd_base_moved(cx, y, n),
        E::Let(_v, x, y, _l, _h) => upd_base_moved(cx, x, n) || upd_base_moved(cx, y, n),
        E::LetT(_v, _t, x, y, _l, _h) => upd_base_moved(cx, x, n) || upd_base_moved(cx, y, n),
        E::If(c, t, el, _l, _h) => {
            upd_base_moved(cx, c, n) || upd_base_moved(cx, t, n) || upd_base_moved(cx, el, n)
        }
        E::While(c, b, _l, _h) => upd_base_moved(cx, c, n) || upd_base_moved(cx, b, n),
        E::Match(sc, arms, _l, _h) => {
            if upd_base_moved(cx, sc, n) {
                return true;
            }
            let mut k = 0;
            while k < arms.len() {
                if upd_base_moved(cx, &arms[k].body, n) {
                    return true;
                }
                k = k + 1;
            }
            false
        }
        E::MethodCall(r, _m, a, _l, _h) => upd_base_moved(cx, r, n) || upd_any(cx, a, n),
        E::StructLit(_s, a, _l, _h) => upd_any(cx, a, n),
        E::TupLit(a, _l, _h) => upd_any(cx, a, n),
        E::ArrLit(a, _l, _h) => upd_any(cx, a, n),
        E::EnumLit(_t, _v, a, _l, _h) => upd_any(cx, a, n),
        E::Bin(_o, x, y, _l, _h) => upd_base_moved(cx, x, n) || upd_base_moved(cx, y, n),
        E::Assign(_v, x, _l, _h) => upd_base_moved(cx, x, n),
        E::RetE(x, _l, _h) => upd_base_moved(cx, x, n),
        _ => false,
    }
}
fn upd_any(cx: &LCx, a: &Vec<E>, n: &str) -> bool {
    let mut k = 0;
    while k < a.len() {
        if upd_base_moved(cx, &a[k], n) {
            return true;
        }
        k = k + 1;
    }
    false
}
// a field type copied (not moved) by a struct update: the primitive scalars and shared references
fn upd_copy_ty(t: &str) -> bool {
    t == "int"
        || t == "bool"
        || t == "char"
        || t == "f64"
        || t == "f32"
        || oast::is_int_width(t)
        || oast::str_starts(t, "&")
}
// G1 G-P1 INSTANCE-SUBST: a type text written inside a generic body, with the instance's parameters replaced by its concrete
// types (whole identifiers only: `T` in `(T, u8)`, never inside `Tag`). Outside an instance (gsub empty) it is unchanged.
pub fn gsub_text(cx: &LCx, tx: &str) -> String {
    if cx.gsub.len() == 0 {
        return String::from(tx);
    }
    let b = tx.as_bytes();
    let mut out = String::new();
    let mut i = 0;
    while i < b.len() {
        let c = b[i];
        let idc = (c >= 65 && c <= 90) || (c >= 97 && c <= 122) || c == 95 || (c >= 48 && c <= 57);
        if idc {
            let st = i;
            while i < b.len() {
                let d = b[i];
                if (d >= 65 && d <= 90) || (d >= 97 && d <= 122) || d == 95 || (d >= 48 && d <= 57)
                {
                    i = i + 1;
                } else {
                    break;
                }
            }
            let w = &tx[st..i];
            let mut rep = String::from(w);
            // (G1 G-P2: a lossless `R_..` key decodes back to its type text below)
            let mut k = 0;
            while k < cx.gsub.len() {
                let e = cx.gsub[k].as_str();
                let eb = e.as_bytes();
                let mut q = 0;
                while q < eb.len() && eb[q] != 61 {
                    q = q + 1;
                }
                if q < eb.len() && &e[..q] == w {
                    rep = raw_unkey(&e[q + 1..]);
                }
                k = k + 1;
            }
            out.push_str(rep.as_str());
        } else {
            out.push(c as char);
            i = i + 1;
        }
    }
    out
}
// G1 G-P2: the explicit turbofish types of the user call at position `lo` (an `@tf:lo|T1|T2` fact), or none
pub fn tf_types(cx: &LCx, lo: u32) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut key = String::from("@tf:");
    key.push_str(&oast::u32s2(lo));
    key.push('|');
    let mut k = 0;
    while k < cx.fpays.len() {
        if oast::str_starts(cx.fpays[k].as_str(), key.as_str()) {
            let rest = &cx.fpays[k][key.len()..];
            let parts = split_bar(rest);
            // [0] the first type (ourc spelling), [1] the second, [2] the first's LOSSLESS text: a composite (tuple / array /
            // reference) is keyed by its text -- ourc's spelling is lossy (`(u8, u32)` = `tup2$int$int`, `[u16; 3]` = `arr`)
            let raw = if parts.len() > 2 {
                String::from(parts[2].as_str())
            } else {
                String::new()
            };
            let comp =
                raw.contains("(") || raw.contains("[") || raw.contains("&") || raw.contains(";");
            if parts.len() > 0 && parts[0].len() > 0 {
                if comp {
                    out.push(raw_key(raw.as_str()));
                } else {
                    out.push(String::from(parts[0].as_str()));
                }
            }
            if parts.len() > 1 && parts[1].len() > 0 {
                out.push(String::from(parts[1].as_str()));
            }
            return out;
        }
        k = k + 1;
    }
    out
}
// G1 G-P2: the instance of generic fn `n` for EXPLICIT type arguments (a turbofish), demanded like an inferred one; "" when
// `n` is not generic or the turbofish does not name every parameter
pub fn mono_explicit(cx: &mut LCx, n: &str, ts: &Vec<String>) -> String {
    let fi = fidx_of(cx, n);
    if fi < 0 || (fi as usize) >= cx.fgps.len() {
        return String::new();
    }
    let gps = split_c(&cx.fgps[fi as usize]);
    if gps.len() == 0 || ts.len() < gps.len() {
        return String::new();
    }
    let mut r = String::from(n);
    let mut d = String::from(n);
    d.push('|');
    let mut g = 0;
    while g < gps.len() {
        r.push('$');
        r.push_str(ts[g].as_str());
        if g > 0 {
            d.push(',');
        }
        d.push_str(ts[g].as_str());
        g = g + 1;
    }
    cx.demands.push(d);
    r
}
// G1 G-P2: a composite turbofish type as a LOSSLESS, symbol-safe instance key: `(u8, u32)` -> `R_Lu8_Cu32_R` (reversible)
pub fn raw_key(t: &str) -> String {
    let mut o = String::from("R_");
    let b = t.as_bytes();
    let mut i = 0;
    while i < b.len() {
        let c = b[i] as char;
        if c == '(' {
            o.push_str("_L");
        } else if c == ')' {
            o.push_str("_R");
        } else if c == '[' {
            o.push_str("_A");
        } else if c == ']' {
            o.push_str("_B");
        } else if c == ';' {
            o.push_str("_S");
        } else if c == ',' {
            o.push_str("_C");
        } else if c == '&' {
            o.push_str("_r");
        } else if c == '<' {
            o.push_str("_G");
        } else if c == '>' {
            o.push_str("_H");
        } else if c != ' ' {
            o.push(c);
        }
        i = i + 1;
    }
    o
}
pub fn raw_unkey(k: &str) -> String {
    if !oast::str_starts(k, "R_") {
        return String::from(k);
    }
    let b = k.as_bytes();
    let mut o = String::new();
    let mut i = 2;
    while i < b.len() {
        if b[i] == 95 && i + 1 < b.len() {
            let d = b[i + 1] as char;
            // space-separated tokens, the way the parser joins a turbofish's raw text (lay_toks splits on spaces)
            let r = if d == 'L' {
                " ( "
            } else if d == 'R' {
                " ) "
            } else if d == 'A' {
                " [ "
            } else if d == 'B' {
                " ] "
            } else if d == 'S' {
                " ; "
            } else if d == 'C' {
                " , "
            } else if d == 'r' {
                " & "
            } else if d == 'G' {
                " < "
            } else if d == 'H' {
                " > "
            } else {
                ""
            };
            if r.len() > 0 {
                o.push_str(r);
                i = i + 2;
                continue;
            }
        }
        o.push(b[i] as char);
        i = i + 1;
    }
    let mut c = String::new();
    let ob = o.as_bytes();
    let mut j = 0;
    while j < ob.len() {
        if ob[j] == 32 && (c.len() == 0 || c.as_bytes()[c.len() - 1] == 32) {
            j = j + 1;
            continue;
        }
        c.push(ob[j] as char);
        j = j + 1;
    }
    String::from(c.trim())
}
// G1 G-P3: the return type of generic method `nm` called on a receiver of type `rt`: each type parameter is unified from the
// method's first parameter type (its self type) against `rt` and substituted into `ret`; unchanged when `nm` is not generic
pub fn gen_meth_ret(cx: &LCx, nm: &str, rt: &str, ret: String) -> String {
    let fi = fidx_of(cx, nm);
    if fi < 0 || (fi as usize) >= cx.fgps.len() || cx.fgps[fi as usize].len() == 0 || rt.len() == 0
    {
        return ret;
    }
    let gps = split_c(&cx.fgps[fi as usize]);
    let ptys = split_c(&cx.fptys[fi as usize]);
    if ptys.len() == 0 {
        return ret;
    }
    let mut r = ret;
    let mut g = 0;
    while g < gps.len() {
        let c = oast::unify_seg(ptys[0].as_str(), rt, gps[g].as_str());
        if c.len() > 0 {
            r = oast::subst_seg(r.as_str(), gps[g].as_str(), c.as_str());
        }
        g = g + 1;
    }
    r
}
// G1 G-P3: the value of type parameter `g` in a GEN-INST record (`W:T=int` / `Pair:A=int,B=char`), or ""
pub fn ginst_val(gi: &str, g: &str) -> String {
    let b = gi.as_bytes();
    let mut c = 0;
    while c < b.len() && b[c] != 58 {
        c = c + 1;
    }
    if c >= b.len() {
        return String::new();
    }
    let parts = split_commas_l(&gi[c + 1..]);
    let mut k = 0;
    while k < parts.len() {
        let e = parts[k].as_str();
        let eb = e.as_bytes();
        let mut q = 0;
        while q < eb.len() && eb[q] != 61 {
            q = q + 1;
        }
        if q < eb.len() && &e[..q] == g {
            return String::from(&e[q + 1..]);
        }
        k = k + 1;
    }
    String::new()
}
