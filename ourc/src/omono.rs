// omono -- MONOMORPHIZATION and signature inference: generic struct instances, unannotated return
// types, and the demanded (fn, concrete-type) instantiation set discovered by a throwaway lowering pass
pub fn sdef_ix(ss: &Vec<Sdef>, n: &str) -> i32 {
    let mut i = 0;
    while i < ss.len() {
        if ss[i].name.as_str() == n {
            return i as i32;
        }
        i = i + 1;
    }
    0 - 1
}
pub fn want_sinst(prog: &Prog, out: &mut Vec<String>) {
    let mut add = |t: &str, o: &mut Vec<String>| {
        if t.len() > 0 && sdef_ix(&prog.sdefs, t) < 0 && !oast::in_list(o, t) {
            o.push(String::from(t));
        }
    };
    let mut i = 0;
    while i < prog.funs.len() {
        let mut k = 0;
        while k < prog.funs[i].ptys.len() {
            add(prog.funs[i].ptys[k].as_str(), out);
            k = k + 1;
        }
        add(prog.funs[i].ret.as_str(), out);
        i = i + 1;
    }
    let mut j = 0;
    while j < prog.sdefs.len() {
        let mut k = 0;
        while k < prog.sdefs[j].ftys.len() {
            add(prog.sdefs[j].ftys[k].as_str(), out);
            k = k + 1;
        }
        j = j + 1;
    }
}
pub fn mono_structs(prog: &mut Prog) {
    let mut rounds = 0;
    while rounds < 4 {
        let mut want: Vec<String> = Vec::new();
        want_sinst(prog, &mut want);
        let mut made = 0;
        let mut q = 0;
        while q < want.len() {
            let t = String::from(want[q].as_str());
            // split at the LAST `$`: `Wrap$int` -> base `Wrap`, arg `int`
            let b = t.as_bytes();
            let mut cut = 0 - 1;
            let mut k = 0;
            while k < b.len() {
                if b[k] == 36 {
                    cut = k as i32;
                }
                k = k + 1;
            }
            if cut > 0 {
                let base = String::from(&t[..(cut as usize)]);
                let arg = String::from(&t[((cut as usize) + 1)..]);
                let bi = sdef_ix(&prog.sdefs, &base);
                if bi >= 0 && prog.sdefs[bi as usize].sgp.len() > 0 {
                    let mut fl: Vec<String> = Vec::new();
                    let mut fy: Vec<String> = Vec::new();
                    {
                        let d = &prog.sdefs[bi as usize];
                        let mut f = 0;
                        while f < d.fields.len() {
                            fl.push(String::from(d.fields[f].as_str()));
                            let mut ty = String::from(d.ftys[f].as_str());
                            let mut g = 0;
                            while g < d.sgp.len() {
                                ty = oast::subst_seg(ty.as_str(), d.sgp[g].as_str(), arg.as_str());
                                g = g + 1;
                            }
                            fy.push(ty);
                            f = f + 1;
                        }
                    }
                    {
                        let bc = prog.sdefs[bi as usize].copy;
                        prog.sdefs.push(Sdef {
                            copy: bc,
                            sgp: Vec::new(),
                            name: t,
                            fields: fl,
                            ftys: fy,
                        });
                    }
                    made = made + 1;
                }
            }
            q = q + 1;
        }
        if made == 0 {
            rounds = 4;
        } else {
            rounds = rounds + 1;
        }
    }
}
pub fn infer_rets(prog: &mut Prog) {
    let mut got: Vec<String> = Vec::new();
    {
        let mut lc = olower::lcx_new();
        lc.sdefs = oast::copy_sdefs(&prog.sdefs);
        olower::fill_fsigs(&mut lc, &prog.funs);
        olower::fill_dyn(&mut lc, &prog.impls);
        lc.edefs = oast::copy_edefs(&prog.edefs);
        let mut i = 0;
        while i < prog.funs.len() {
            let mut r = String::from("");
            if prog.funs[i].ret.len() == 0
                && !oast::has_bound(&prog.funs[i])
                && !oast::has_composite_gp(&prog.funs[i])
            {
                let bx = oast::fbody_ix(&prog.funs, i);
                olower::lcx_reset(&mut lc);
                let mut pi = 0;
                while pi < prog.funs[i].params.len() {
                    lc.names
                        .push(String::from(prog.funs[i].params[pi].as_str()));
                    let pt = if pi < prog.funs[i].ptys.len() {
                        String::from(prog.funs[i].ptys[pi].as_str())
                    } else {
                        String::from("")
                    };
                    let ps = olower::local_sty(&prog.sdefs, &prog.edefs, &pt);
                    lc.stypes.push(ps);
                    pi = pi + 1;
                }
                let et = olower::body_ty(&lc, &prog.funs[bx].body);
                if et.len() > 0
                    && (oast::is_struct_name(&prog.sdefs, &et)
                        || oast::is_enum_name(&prog.edefs, &et))
                {
                    r = et;
                }
            }
            got.push(r);
            i = i + 1;
        }
    }
    let mut j = 0;
    while j < got.len() {
        if got[j].len() > 0 {
            prog.funs[j].ret = String::from(got[j].as_str());
        }
        j = j + 1;
    }
}
// KEYSTONE (element-inference Root A): infer an UNANNOTATED closure param as the receiver's element type,
// for the closure-consumers, so e.g. v.flat_map(|&x| vec![x,x*10]) types x as the element instead of <generic>.
fn kv_vce(t: &str) -> String {
    let b = t.as_bytes();
    if b.len() > 4 && b[0] == 86 && b[1] == 101 && b[2] == 99 && b[3] == 36 {
        String::from(&t[4..])
    } else {
        String::from("")
    }
}
fn kv_is_consumer(m: &str) -> bool {
    m == "map"
        || m == "filter"
        || m == "flat_map"
        || m == "for_each"
        || m == "any"
        || m == "all"
        || m == "position"
        || m == "take_while"
        || m == "skip_while"
        || m == "find"
        || m == "retain"
        || m == "max_by_key"
        || m == "min_by_key"
        || m == "sort_by_key"
}
fn kv_relem(e: &E, ln: &Vec<String>, lt: &Vec<String>) -> String {
    match e {
        E::MethodCall(_x, m, a, _l, _h) if a.len() == 0 && m.as_str() == "char_indices" => {
            String::from("tup2$int$char")
        } // CHAR-IDX: a closure over char_indices takes (usize, char) -- untyped, map fell back to the tuple type and {:?} SEGFAULTED
        E::MethodCall(_x, m, a, _l, _h) if a.len() == 0 && m.as_str() == "chars" => {
            String::from("char")
        } // STR-CHARS-UTF8: a closure over chars() takes a char
        E::MethodCall(_x, m, a, _l, _h)
            if (a.len() == 1 && m.as_str() == "split")
                || (a.len() == 0
                    && (m.as_str() == "split_whitespace" || m.as_str() == "lines")) =>
        {
            String::from("str")
        } // SPLIT-ELEM: a closure over split pieces takes a &str -- untyped, `x.is_empty()` called an undefined `$is_empty`
        E::MethodCall(x, m, a, _l, _h)
            if (a.len() == 0 && m.as_str() == "rev")
                || (a.len() == 1
                    && (m.as_str() == "filter"
                        || m.as_str() == "skip"
                        || m.as_str() == "take")) =>
        {
            kv_relem(x, ln, lt)
        } // SPLIT-ELEM: these adapters keep the element type
        E::MethodCall(x, m, a, _l, _h)
            if a.len() == 1 && (m.as_str() == "windows" || m.as_str() == "chunks") =>
        {
            let ie = kv_relem(x, ln, lt);
            if ie.len() == 0 {
                return String::from("");
            }
            let mut t = String::from("Vec$");
            t.push_str(&ie);
            t
        } // SLICE-WIN: each window/chunk is a Vec of the elements
        E::MethodCall(x, m, a, _l, _h) => {
            if a.len() == 0
                && (m.as_str() == "iter"
                    || m.as_str() == "into_iter"
                    || m.as_str() == "iter_mut"
                    || m.as_str() == "copied"
                    || m.as_str() == "cloned"
                    || m.as_str() == "to_vec"
                    || m.as_str() == "clone")
            {
                kv_relem(x, ln, lt)
            } else {
                String::from("")
            }
        }
        E::Var(n, _l, _h) => {
            let mut i = 0;
            while i < ln.len() {
                if ln[i].as_str() == n.as_str() {
                    return String::from(lt[i].as_str());
                }
                i = i + 1;
            }
            String::from("")
        }
        E::LetT(_n, t, _v, _b, _l, _h) => kv_vce(t.as_str()),
        E::Call(n, _a, _l, _h) => {
            if n.as_str() == "range2" || n.as_str() == "range" {
                String::from("int")
            } else {
                String::from("")
            }
        }
        _ => String::from(""),
    }
}
fn kv_walk(
    e: &E,
    ln: &mut Vec<String>,
    lt: &mut Vec<String>,
    ocn: &mut Vec<String>,
    oel: &mut Vec<String>,
) {
    match e {
        E::LetT(nm, t, v, b, _l, _h) => {
            kv_walk(v, ln, lt, ocn, oel);
            let el = kv_vce(t.as_str());
            ln.push(String::from(nm.as_str()));
            lt.push(el);
            kv_walk(b, ln, lt, ocn, oel);
            let _ = ln.pop();
            let _ = lt.pop();
        }
        E::Let(nm, v, b, _l, _h) => {
            kv_walk(v, ln, lt, ocn, oel);
            let el = kv_relem(v, ln, lt);
            ln.push(String::from(nm.as_str()));
            lt.push(el);
            kv_walk(b, ln, lt, ocn, oel);
            let _ = ln.pop();
            let _ = lt.pop();
        }
        E::MethodCall(r, m, args, _l, _h) => {
            kv_walk(r, ln, lt, ocn, oel);
            if m.as_str() == "fold" && args.len() == 2 {
                match &args[1] {
                    E::ClosRef(cn, _c, _cl, _ch) => {
                        let at = kv_init_ty(&args[0]);
                        if at.len() > 0 {
                            ocn.push(String::from(cn.as_str()));
                            oel.push(at);
                        }
                        let el = kv_relem(r, ln, lt);
                        if el.len() > 0 {
                            let mut c1 = String::from(cn.as_str());
                            c1.push_str("#1");
                            ocn.push(c1);
                            oel.push(el);
                        }
                    }
                    _ => {}
                }
            } // FOLD-TY: |acc, x| -- acc is the init's type, x the element
            if m.as_str() == "sort_by" || m.as_str() == "sort_unstable_by" {
                // R13 SORTBY-PTY: `v.sort_by(|a, b| ..)` -- BOTH params are elements (they were untyped: `a.x` was an unknown field)
                let el = kv_relem(r, ln, lt);
                if el.len() > 0 && args.len() == 1 {
                    if let E::ClosRef(cn, _c, _cl, _ch) = &args[0] {
                        ocn.push(String::from(cn.as_str()));
                        oel.push(String::from(el.as_str()));
                        let mut c1 = String::from(cn.as_str());
                        c1.push_str("#1");
                        ocn.push(c1);
                        oel.push(String::from(el.as_str()));
                        let mut cr = String::from(cn.as_str());
                        cr.push_str("#R");
                        ocn.push(cr);
                        oel.push(String::from("Ordering")); // the comparator RETURNS Ordering
                    }
                }
            }
            if kv_is_consumer(m.as_str()) {
                let el = kv_relem(r, ln, lt);
                if el.len() > 0 {
                    let mut k = 0;
                    while k < args.len() {
                        match &args[k] {
                            E::ClosRef(cn, _c, _cl, _ch) => {
                                ocn.push(String::from(cn.as_str()));
                                oel.push(String::from(el.as_str()));
                            }
                            _ => {}
                        }
                        k = k + 1;
                    }
                }
            }
            let mut k = 0;
            while k < args.len() {
                kv_walk(&args[k], ln, lt, ocn, oel);
                k = k + 1;
            }
        }
        E::Seq(a, b, _l, _h) => {
            kv_walk(a, ln, lt, ocn, oel);
            kv_walk(b, ln, lt, ocn, oel);
        }
        E::If(c, t, f, _l, _h) => {
            kv_walk(c, ln, lt, ocn, oel);
            kv_walk(t, ln, lt, ocn, oel);
            kv_walk(f, ln, lt, ocn, oel);
        }
        E::While(c, b, _l, _h) => {
            kv_walk(c, ln, lt, ocn, oel);
            kv_walk(b, ln, lt, ocn, oel);
        }
        E::Bin(_o, a, b, _l, _h) => {
            kv_walk(a, ln, lt, ocn, oel);
            kv_walk(b, ln, lt, ocn, oel);
        }
        E::Call(fnm, args, _l, _h) => {
            {
                // FN-BOUND-ARGS (W7): a closure passed to a fn PARAMETER bounded `Fn*(A, B)` -- a marker resolved where the
                // program's "@fnargs" records are visible (below)
                let mut k = 0;
                while k < args.len() {
                    if let E::ClosRef(cn, _c, _cl, _ch) = &args[k] {
                        let mut mk = String::from("@C:");
                        mk.push_str(fnm.as_str());
                        mk.push('#');
                        mk.push_str(&oast::u32s2(k as u32));
                        mk.push('|');
                        mk.push_str(cn.as_str());
                        ocn.push(mk);
                        oel.push(String::new());
                    }
                    k = k + 1;
                }
            }
            let mut k = 0;
            while k < args.len() {
                kv_walk(&args[k], ln, lt, ocn, oel);
                k = k + 1;
            }
        }
        E::Ref(x, _m, _l, _h) => {
            kv_walk(x, ln, lt, ocn, oel);
        }
        E::Match(sc, arms, _l, _h) => {
            kv_walk(sc, ln, lt, ocn, oel);
            let mut k = 0;
            while k < arms.len() {
                kv_walk(&arms[k].body, ln, lt, ocn, oel);
                k = k + 1;
            }
        }
        _ => {}
    }
}
pub fn infer_clos_params(prog: &mut Prog) {
    let mut ocn: Vec<String> = Vec::new();
    let mut oel: Vec<String> = Vec::new();
    {
        let mut ln: Vec<String> = Vec::new();
        let mut lt: Vec<String> = Vec::new();
        kv_walk(&prog.main, &mut ln, &mut lt, &mut ocn, &mut oel);
    }
    {
        let mut fi = 0;
        while fi < prog.funs.len() {
            let mut ln: Vec<String> = Vec::new();
            let mut lt: Vec<String> = Vec::new();
            let mut pi = 0;
            while pi < prog.funs[fi].params.len() {
                ln.push(String::from(prog.funs[fi].params[pi].as_str()));
                let pt = if pi < prog.funs[fi].ptys.len() {
                    String::from(prog.funs[fi].ptys[pi].as_str())
                } else {
                    String::from("")
                };
                lt.push(kv_vce(pt.as_str()));
                pi = pi + 1;
            }
            kv_walk(&prog.funs[fi].body, &mut ln, &mut lt, &mut ocn, &mut oel);
            fi = fi + 1;
        }
    }
    let mut q = 0;
    while q < ocn.len() {
        if oast::str_starts(ocn[q].as_str(), "@C:") {
            // FN-BOUND-ARGS (W7): "@C:<fn>#<k>|<closure>" -- type the closure's params from fn's param k bound args
            let body = String::from(&ocn[q][3..]);
            let bb = body.as_bytes();
            let mut h = 0;
            while h < bb.len() && bb[h] != 35 {
                h = h + 1;
            }
            let mut bar = h;
            while bar < bb.len() && bb[bar] != 124 {
                bar = bar + 1;
            }
            if bar < bb.len() {
                let callee = String::from(&body[..h]);
                let pk = oast::parse_i32(&body[h + 1..bar]) as usize;
                let cn = String::from(&body[bar + 1..]);
                let cx = oparse::find_fun_ix(&prog.funs, callee.as_str());
                let kx = oparse::find_fun_ix(&prog.funs, cn.as_str());
                if cx >= 0 && kx >= 0 && pk < prog.funs[cx as usize].params.len() {
                    let mut key = String::from("@fnargs:");
                    key.push_str(callee.as_str());
                    key.push('#');
                    key.push_str(prog.funs[cx as usize].params[pk].as_str());
                    key.push('=');
                    let mut f = 0;
                    while f < prog.fpays.len() {
                        if oast::str_starts(prog.fpays[f].as_str(), key.as_str()) {
                            let tys = olower::split_commas_l(&prog.fpays[f][key.len()..]);
                            let nc = prog.funs[kx as usize].ncap as usize;
                            let mut a = 0;
                            while a < tys.len() {
                                let ix = nc + a;
                                if ix < prog.funs[kx as usize].ptys.len()
                                    && prog.funs[kx as usize].ptys[ix].len() == 0
                                    && tys[a].len() > 0
                                {
                                    prog.funs[kx as usize].ptys[ix] = String::from(tys[a].as_str());
                                }
                                a = a + 1;
                            }
                        }
                        f = f + 1;
                    }
                }
            }
            q = q + 1;
            continue;
        }
        if ocn[q].ends_with("#R") {
            // R13 SORTBY-PTY: a sort comparator's RETURN type (it was inferred as int; the body is an Ordering)
            let cnm = String::from(&ocn[q][..ocn[q].len() - 2]);
            let fx = oparse::find_fun_ix(&prog.funs, cnm.as_str());
            if fx >= 0 && prog.funs[fx as usize].ret.len() == 0 {
                prog.funs[fx as usize].ret = String::from(oel[q].as_str());
            }
            q = q + 1;
            continue;
        }
        let (cnm, off) = {
            let b = ocn[q].as_bytes();
            let mut h = 0;
            while h < b.len() && b[h] != 35 {
                h = h + 1;
            }
            if h < b.len() {
                (String::from(&ocn[q][..h]), 1)
            } else {
                (String::from(ocn[q].as_str()), 0)
            }
        }; // FOLD-TY: "#1" = the next param
        let fx = oparse::find_fun_ix(&prog.funs, cnm.as_str());
        if fx >= 0 {
            let nc = prog.funs[fx as usize].ncap;
            let idx = (nc as usize) + off;
            if idx < prog.funs[fx as usize].ptys.len()
                && prog.funs[fx as usize].ptys[idx].len() == 0
            {
                prog.funs[fx as usize].ptys[idx] = String::from(oel[q].as_str());
            }
        }
        q = q + 1;
    }
    // KEYSTONE ret-inference: with closure params now typed, infer each lifted closure's RETURN type so map's
    // result element can be the closure's return (D5 map-elemtype). Modeled on infer_rets, but any type.
    {
        let mut lc = olower::lcx_new();
        lc.sdefs = oast::copy_sdefs(&prog.sdefs);
        olower::fill_fsigs(&mut lc, &prog.funs);
        olower::fill_dyn(&mut lc, &prog.impls);
        lc.edefs = oast::copy_edefs(&prog.edefs);
        let mut got: Vec<String> = Vec::new();
        let mut i = 0;
        while i < prog.funs.len() {
            let mut r = String::from("");
            let nm = String::from(prog.funs[i].name.as_str());
            let isclos = nm.len() > 5 && {
                let b = nm.as_bytes();
                b[0] == 36 && b[1] == 99 && b[2] == 108 && b[3] == 111 && b[4] == 115
            };
            if isclos && prog.funs[i].ret.len() == 0 {
                let bx = oast::fbody_ix(&prog.funs, i);
                olower::lcx_reset(&mut lc);
                let mut pi = 0;
                while pi < prog.funs[i].params.len() {
                    lc.names
                        .push(String::from(prog.funs[i].params[pi].as_str()));
                    let pt = if pi < prog.funs[i].ptys.len() {
                        String::from(prog.funs[i].ptys[pi].as_str())
                    } else {
                        String::from("")
                    };
                    {
                        let ls = olower::local_sty(&prog.sdefs, &prog.edefs, &pt);
                        lc.stypes
                            .push(if ls.len() == 0 && oast::str_starts(pt.as_str(), "tup") {
                                pt
                            } else {
                                ls
                            });
                    }
                    pi = pi + 1;
                } // CLOS-DESTR-RET: keep a tuple param type
                  // CLOS-DESTR-RET: a destructuring closure body begins `let a = $cp.0; let b = $cp.1; ..` -- bind those names (their element
                  // types come from the tuple-typed parameter) before inferring the return type; body_ty alone saw a, b as unknown
                let et = {
                    let mut bodyr = &prog.funs[bx].body;
                    loop {
                        match bodyr {
                            E::Let(n, v, b, _l, _h) => match v.as_ref() {
                                E::TupIdx(base, k, _l2, _h2) => match base.as_ref() {
                                    E::Var(pn, _a, _b) => {
                                        let mut st = String::from("");
                                        let mut q = 0;
                                        while q < lc.names.len() {
                                            if lc.names[q].as_str() == pn.as_str() {
                                                st = String::from(lc.stypes[q].as_str());
                                            }
                                            q = q + 1;
                                        }
                                        let es = oparse::tup_elems(&st);
                                        let kr = oast::tix_resolve(*k, es.len());
                                        if kr >= 0 {
                                            lc.names.push(String::from(n.as_str()));
                                            lc.stypes.push(String::from(es[kr as usize].as_str()));
                                            bodyr = b;
                                            continue;
                                        }
                                        break;
                                    }
                                    _ => {
                                        break;
                                    }
                                },
                                _ => {
                                    break;
                                }
                            },
                            _ => {
                                break;
                            }
                        }
                    }
                    olower::body_ty(&lc, bodyr)
                };
                if et.len() > 0 && oast::is_known_ty2(&prog.sdefs, &prog.edefs, &et) {
                    r = et;
                }
            }
            got.push(r);
            i = i + 1;
        }
        let mut j = 0;
        while j < got.len() {
            if got[j].len() > 0 {
                prog.funs[j].ret = String::from(got[j].as_str());
            }
            j = j + 1;
        }
    }
}
pub fn monomorphize(prog: &mut Prog) {
    let mut dem: Vec<String> = Vec::new();
    {
        let mut lc = olower::lcx_new();
        lc.sdefs = oast::copy_sdefs(&prog.sdefs);
        olower::fill_fsigs(&mut lc, &prog.funs);
        olower::fill_dyn(&mut lc, &prog.impls);
        lc.edefs = oast::copy_edefs(&prog.edefs);
        tf_facts(prog, &mut lc); // G1 G-P2: turbofish-only calls demand their instances
        let _ = olower::lower(&prog.main, &mut lc);
        let mut l2 = olower::lcx_new();
        l2.sdefs = oast::copy_sdefs(&prog.sdefs);
        olower::fill_fsigs(&mut l2, &prog.funs);
        olower::fill_dyn(&mut l2, &prog.impls);
        l2.edefs = oast::copy_edefs(&prog.edefs);
        tf_facts(prog, &mut l2);
        let mut i = 0;
        while i < prog.funs.len() {
            if !oast::has_bound(&prog.funs[i]) && !oast::has_composite_gp(&prog.funs[i]) {
                let bx = oast::fbody_ix(&prog.funs, i);
                olower::lcx_reset(&mut l2);
                let mut pi = 0;
                while pi < prog.funs[i].params.len() {
                    l2.names
                        .push(String::from(prog.funs[i].params[pi].as_str()));
                    let pt = if pi < prog.funs[i].ptys.len() {
                        String::from(prog.funs[i].ptys[pi].as_str())
                    } else {
                        String::from("")
                    };
                    let ps = olower::local_sty(&prog.sdefs, &prog.edefs, &pt);
                    l2.stypes.push(ps);
                    pi = pi + 1;
                }
                let _ = olower::lower(&prog.funs[bx].body, &mut l2);
                let mut k = 0;
                while k < l2.demands.len() {
                    dem.push(String::from(l2.demands[k].as_str()));
                    k = k + 1;
                }
            }
            i = i + 1;
        }
        let mut k = 0;
        while k < lc.demands.len() {
            dem.push(String::from(lc.demands[k].as_str()));
            k = k + 1;
        }
    }
    let mut q = 0;
    while q < dem.len() {
        let parts = olower::split_bar(dem[q].as_str());
        if parts.len() == 2 {
            let fx = oparse::find_fun_ix(&prog.funs, parts[0].as_str());
            if fx >= 0 {
                let cs = olower::split_c(parts[1].as_str());
                let mut mn = String::from(parts[0].as_str());
                {
                    let mut ci = 0;
                    while ci < cs.len() {
                        mn.push_str("$");
                        mn.push_str(cs[ci].as_str());
                        ci = ci + 1;
                    }
                }
                if oparse::find_fun_ix(&prog.funs, &mn) < 0 {
                    let mut ps: Vec<String> = Vec::new();
                    let mut pt: Vec<String> = Vec::new();
                    let mut rt = String::from("");
                    let mut gp: Vec<String> = Vec::new();
                    let mut gb: Vec<String> = Vec::new();
                    {
                        let f = &prog.funs[fx as usize];
                        let mut k2 = 0;
                        while k2 < f.params.len() {
                            ps.push(String::from(f.params[k2].as_str()));
                            k2 = k2 + 1;
                        }
                        let mut k3 = 0;
                        while k3 < f.gparams.len() {
                            gp.push(String::from(f.gparams[k3].as_str()));
                            k3 = k3 + 1;
                        }
                        let mut k4 = 0;
                        while k4 < f.gbounds.len() {
                            gb.push(String::from(f.gbounds[k4].as_str()));
                            k4 = k4 + 1;
                        }
                        let mut k5 = 0;
                        while k5 < f.ptys.len() {
                            let mut t = String::from(f.ptys[k5].as_str());
                            let mut gi2 = 0;
                            while gi2 < gp.len() {
                                if gi2 < cs.len() {
                                    t = oast::subst_seg(
                                        t.as_str(),
                                        gp[gi2].as_str(),
                                        cs[gi2].as_str(),
                                    );
                                }
                                gi2 = gi2 + 1;
                            }
                            pt.push(t);
                            k5 = k5 + 1;
                        }
                        {
                            let mut t = String::from(f.ret.as_str());
                            let mut gi3 = 0;
                            while gi3 < gp.len() {
                                if gi3 < cs.len() {
                                    t = oast::subst_seg(
                                        t.as_str(),
                                        gp[gi3].as_str(),
                                        cs[gi3].as_str(),
                                    );
                                }
                                gi3 = gi3 + 1;
                            }
                            rt = t;
                        }
                        // G1 G-P3: a method returning `Self` -- the instance's self type (its receiver's substituted type)
                        if rt.as_str() == "Self" && pt.len() > 0 {
                            rt = String::from(pt[0].as_str());
                        }
                    }
                    // G1 G-P1 INSTANCE-SUBST: the instance's own parameter -> type map, for the types written inside the shared body
                    let mut gs: Vec<String> = Vec::new();
                    {
                        let mut g = 0;
                        while g < gp.len() && g < cs.len() {
                            let mut e = String::from(gp[g].as_str());
                            e.push('=');
                            e.push_str(cs[g].as_str());
                            gs.push(e);
                            g = g + 1;
                        }
                    }
                    prog.funs.push(Fun {
                        gsub: gs,
                        pref: Vec::new(),
                        ncap: 0,
                        gbounds: Vec::new(),
                        dflt: fx,
                        name: mn,
                        params: ps,
                        ptys: pt,
                        ret: rt,
                        gparams: Vec::new(),
                        owner: String::from(""),
                        body: E::Int(0, 0, 0),
                    });
                    // TRANSITIVE demands: a bounded generic's body is skipped by the pre-pass (its parameter
                    // types are not concrete yet), so the instances IT calls were never demanded and the call
                    // emitted an undefined name. Lower the instance we just made, with its concrete parameter
                    // types, and feed its demands back into this same worklist.
                    {
                        let ni = prog.funs.len() - 1;
                        let mut l3 = olower::lcx_new();
                        l3.gsub = oast::copy_strs(&prog.funs[ni].gsub); // G1 G-P1
                        tf_facts(prog, &mut l3);
                        l3.sdefs = oast::copy_sdefs(&prog.sdefs);
                        olower::fill_fsigs(&mut l3, &prog.funs);
                        olower::fill_dyn(&mut l3, &prog.impls);
                        l3.edefs = oast::copy_edefs(&prog.edefs);
                        let mut pi = 0;
                        while pi < prog.funs[ni].params.len() {
                            l3.names
                                .push(String::from(prog.funs[ni].params[pi].as_str()));
                            let pt2 = if pi < prog.funs[ni].ptys.len() {
                                String::from(prog.funs[ni].ptys[pi].as_str())
                            } else {
                                String::from("")
                            };
                            let ps2 = olower::local_sty(&prog.sdefs, &prog.edefs, &pt2);
                            l3.stypes.push(ps2);
                            pi = pi + 1;
                        }
                        let bx2 = oast::fbody_ix(&prog.funs, ni);
                        let _ = olower::lower(&prog.funs[bx2].body, &mut l3);
                        let mut k6 = 0;
                        while k6 < l3.demands.len() {
                            dem.push(String::from(l3.demands[k6].as_str()));
                            k6 = k6 + 1;
                        }
                    }
                }
            }
        }
        q = q + 1;
    }
}
// FOLD-TY: the type of a fold's initial accumulator, from its literal shape ("" when not evident)
fn kv_init_ty(e: &E) -> String {
    match e {
        E::StrLit(_s, _l, _h) => String::from("str"),
        E::Call(n, _a, _l, _h) => {
            if n.as_str() == "$emptystr" || n.as_str() == "$id1" || n.as_str() == "str_copy" {
                String::from("str")
            } else if n.as_str() == "vec_new" {
                String::from("Vec")
            } else {
                String::from("")
            }
        }
        E::Int(_v, _l, _h) => String::from("int"),
        _ => String::from(""),
    }
}
// G1 G-P2: hand the parser's `@tf:` facts (turbofish user calls) to a lowering context that collects mono demands
fn tf_facts(prog: &Prog, cx: &mut olower::LCx) {
    let mut q = 0;
    while q < prog.fpays.len() {
        if oast::str_starts(prog.fpays[q].as_str(), "@tf:") {
            cx.fpays.push(String::from(prog.fpays[q].as_str()));
        }
        q = q + 1;
    }
}
