pub struct Rib {
    pub names: Vec<String>,
    pub marks: Vec<usize>,
}
pub fn rib_new() -> Rib {
    Rib {
        names: Vec::new(),
        marks: Vec::new(),
    }
}
pub fn push_scope(r: &mut Rib) {
    r.marks.push(r.names.len());
}
pub fn pop_scope(r: &mut Rib) {
    if r.marks.len() > 0 {
        let m = r.marks[r.marks.len() - 1];
        r.marks.pop();
        while r.names.len() > m {
            r.names.pop();
        }
    }
}
pub fn nnames(r: &Rib) -> usize {
    r.names.len()
}
pub fn define(r: &mut Rib, n: &str) {
    r.names.push(String::from(n));
}
pub fn resolves(r: &Rib, n: &str) -> bool {
    let mut i = r.names.len();
    while i > 0 {
        i = i - 1;
        if r.names[i].as_str() == n {
            return true;
        }
    }
    false
}
// the resolution pass: walk the AST, report every unresolved Var as a spanned diagnostic
pub fn resolve_expr(e: &E, r: &mut Rib, ds: &mut Vec<Diag>) {
    match e {
        E::Not(x, _l, _h) => {
            resolve_expr(x, r, ds);
        }
        E::StrLit(_v, _l, _h) => {}
        E::RetE(x, _l, _h) => {
            resolve_expr(x, r, ds);
        }
        E::ClosRef(_n, _c, _l, _h) => {}
        E::Try(x, _l, _h) => {
            resolve_expr(x, r, ds);
        }
        E::BreakE(_l, _h) => {}
        E::BreakN(_n, _l, _h) => {}
        E::Ref(x, _m, _l, _h) => {
            resolve_expr(x, r, ds);
        }
        E::Cast(x, _t, _l, _h) => {
            resolve_expr(x, r, ds);
        }
        E::ContE(_l, _h) => {}
        E::ContN(_n, _l, _h) => {}
        E::LetT(n, _t, v, b, _l, _h) => {
            resolve_expr(v, r, ds);
            push_scope(r);
            define(r, n);
            resolve_expr(b, r, ds);
            pop_scope(r);
        }
        E::TupLit(parts, _l, _h) => {
            let mut k = 0;
            while k < parts.len() {
                resolve_expr(&parts[k], r, ds);
                k = k + 1;
            }
        }
        E::TupIdx(b, _i, _l, _h) => {
            resolve_expr(b, r, ds);
        }
        E::MethodCall(rv, _m, args, _l, _h) => {
            resolve_expr(rv, r, ds);
            let mut k = 0;
            while k < args.len() {
                resolve_expr(&args[k], r, ds);
                k = k + 1;
            }
        }
        E::AssignTo(pl, v, _l, _h) => {
            resolve_expr(pl, r, ds);
            resolve_expr(v, r, ds);
        }
        E::ArrLit(xs, _l, _h) => {
            let mut k = 0;
            while k < xs.len() {
                resolve_expr(&xs[k], r, ds);
                k = k + 1;
            }
        }
        E::Index(b, ix, _l, _h) => {
            resolve_expr(b, r, ds);
            resolve_expr(ix, r, ds);
        }
        E::EnumLit(_e2, _v, ps, _l, _h) => {
            let mut k = 0;
            while k < ps.len() {
                resolve_expr(&ps[k], r, ds);
                k = k + 1;
            }
        }
        E::Match(sc, arms, _l, _h) => {
            resolve_expr(sc, r, ds);
            let mut k = 0;
            while k < arms.len() {
                push_scope(r);
                let mut bi = 0;
                while bi < arms[k].binds.len() {
                    define(r, &arms[k].binds[bi]);
                    bi = bi + 1;
                }
                resolve_expr(&arms[k].body, r, ds);
                pop_scope(r);
                k = k + 1;
            }
        }
        E::StructLit(_n, fs, _l, _h) => {
            let mut k = 0;
            while k < fs.len() {
                resolve_expr(&fs[k], r, ds);
                k = k + 1;
            }
        }
        E::Field(o, _f, _l, _h) => {
            resolve_expr(o, r, ds);
        }
        E::While(c, b, _l, _h) => {
            resolve_expr(c, r, ds);
            push_scope(r);
            resolve_expr(b, r, ds);
            pop_scope(r);
        }
        E::Seq(a, b, _l, _h) => {
            resolve_expr(a, r, ds);
            resolve_expr(b, r, ds);
        }
        E::Assign(n, v, l, h) => {
            resolve_expr(v, r, ds);
            if !resolves(r, n) {
                let mut m = String::from("cannot find value `");
                m.push_str(n);
                m.push_str("` in this scope");
                ds.push(oerrors::err(m, *l, *h));
            }
        }
        E::Call(_n, args, _l, _h) => {
            let mut k = 0;
            while k < args.len() {
                resolve_expr(&args[k], r, ds);
                k = k + 1;
            }
        }
        E::Int(_v, _l, _h) => {}
        E::Bool(_v, _l, _h) => {}
        E::Var(n, l, h) => {
            if n.as_str() != "_" && n.as_str() != "$noplace" && !resolves(r, n) {
                let mut m = String::from("cannot find value `");
                m.push_str(n);
                m.push_str("` in this scope");
                ds.push(oerrors::err(m, *l, *h));
            }
        }
        E::Bin(_o, a, b, _l, _h) => {
            resolve_expr(a, r, ds);
            resolve_expr(b, r, ds);
        }
        E::If(c, t, f, _l, _h) => {
            resolve_expr(c, r, ds);
            push_scope(r);
            resolve_expr(t, r, ds);
            pop_scope(r);
            push_scope(r);
            resolve_expr(f, r, ds);
            pop_scope(r);
        }
        E::Let(n, v, b, _l, _h) => {
            resolve_expr(v, r, ds);
            push_scope(r);
            define(r, n);
            resolve_expr(b, r, ds);
            pop_scope(r);
        }
    }
}
