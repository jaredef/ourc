// oborrowck -- MOVE CHECKING: Rust's rule that passing a non-Copy value BY VALUE moves it, and using it
// afterwards is an error. This is the phase ourc had no equivalent of at all.
//
// Scope of the rule implemented here: a local whose type is a STRUCT or an ENUM is move-tracked (ints and
// bools are Copy; `str`/`Vec` are deliberately left out of the first cut). Passing such a local by value to a
// function moves it; passing `&x` / `&mut x` does NOT (which is why references are explicit in the AST).
// A move inside a LOOP body is also a second use of the same value, so it is reported.
pub struct Mv {
    pub names: Vec<String>,
    pub moved: Vec<bool>,
    pub aggr: Vec<bool>,
    pub ldep: Vec<i32>,
    pub imm: Vec<bool>,
    pub strm: Vec<bool>,
}
pub fn mv_new() -> Mv {
    Mv {
        names: Vec::new(),
        moved: Vec::new(),
        aggr: Vec::new(),
        ldep: Vec::new(),
        imm: Vec::new(),
        strm: Vec::new(),
    }
}
pub fn mv_ix(m: &Mv, n: &str) -> i32 {
    let mut i = m.names.len();
    while i > 0 {
        i = i - 1;
        if m.names[i].as_str() == n {
            return i as i32;
        }
    }
    0 - 1
}
pub fn mv_bind_d(m: &mut Mv, n: &str, aggregate: bool, depth: i32) {
    m.names.push(String::from(n));
    m.moved.push(false);
    m.aggr.push(aggregate);
    m.ldep.push(depth);
    m.imm.push(false);
    m.strm.push(false);
}
pub fn mv_bind(m: &mut Mv, n: &str, aggregate: bool) {
    mv_bind_d(m, n, aggregate, 0)
}
pub fn mv_pop(m: &mut Mv) {
    let _ = m.names.pop();
    let _ = m.moved.pop();
    let _ = m.aggr.pop();
    let _ = m.ldep.pop();
    let _ = m.imm.pop();
    let _ = m.strm.pop();
}

// is this expression's type a struct or enum (hence move-tracked)?
pub fn aggr_ty(ss: &Vec<Sdef>, ez: &Vec<Edef>, t: &str) -> bool {
    if t.len() == 0 {
        return false;
    }
    // `#[derive(Copy)]` types are NOT moved by a by-value pass -- without this the checker reports
    // use-after-move on perfectly valid Rust
    if oast::is_copy_struct(ss, t) {
        return false;
    }
    oast::is_struct_name(ss, t) || (oast::is_enum_name(ez, t) && t != "Option" && t != "Result")
}
pub fn init_is_aggr(prog: &Prog, e: &E) -> bool {
    match e {
        // a struct LITERAL is move-tracked unless its type is Copy
        E::StructLit(n, _f, _l, _h) => !oast::is_copy_struct(&prog.sdefs, n),
        // Option/Result are not move-tracked: Option<i64> is Copy in rustc, and the payload type is not known here --
        // under-reporting a move beats a false "use of moved value" (matches!(o, ..) twice was rejected)
        E::EnumLit(n, _v, _p, _l, _h) => n.as_str() != "Option" && n.as_str() != "Result",
        E::Call(n, _a, _l, _h) => {
            let mut i = 0;
            while i < prog.funs.len() {
                if prog.funs[i].name.as_str() == n.as_str() {
                    return aggr_ty(&prog.sdefs, &prog.edefs, prog.funs[i].ret.as_str());
                }
                i = i + 1;
            }
            false
        }
        _ => false,
    }
}

pub fn check_prog(prog: &Prog, ds: &mut Vec<Diag>) {
    let_flow(prog, ds);
    {
        let mut m = mv_new();
        walk(&prog.main, prog, &mut m, ds, 0);
    }
    if prog.realrs {
        let mut wn: Vec<String> = Vec::new();
        let mut wt: Vec<String> = Vec::new();
        wfn_sigs(prog, &mut wn, &mut wt);
        wwalk(&prog.main, &mut wn, &mut wt, ds);
    }
    if prog.realrs {
        shared_mut_walk(&prog.main, ds);
        let mut i3 = 0;
        while i3 < prog.funs.len() {
            if prog.funs[i3].dflt < 0 {
                shared_mut_walk(&prog.funs[i3].body, ds);
            }
            i3 = i3 + 1;
        }
    }
    let mut i = 0;
    while i < prog.funs.len() {
        if prog.funs[i].dflt < 0 {
            let mut m = mv_new();
            let mut p = 0;
            while p < prog.funs[i].params.len() {
                let pt = if p < prog.funs[i].ptys.len() {
                    String::from(prog.funs[i].ptys[p].as_str())
                } else {
                    String::from("")
                };
                // a param declared `&T`/`&mut T` is a REFERENCE binding: passing it onward is a REBORROW,
                // never a move, however many times (rustc's table, C2 axis (c) -- unbounded). The parser is the
                // only tier that still knows, because parse_tyname strips the `&`: `&T` IS `T` in the value
                // model and that stays true; this is the one bit the move checker needs.
                let isref = p < prog.funs[i].pref.len() && prog.funs[i].pref[p];
                mv_bind(
                    &mut m,
                    prog.funs[i].params[p].as_str(),
                    aggr_ty(&prog.sdefs, &prog.edefs, &pt) && !isref,
                );
                p = p + 1;
            }
            walk(&prog.funs[i].body, prog, &mut m, ds, 0);
            if prog.realrs {
                let mut wn: Vec<String> = Vec::new();
                let mut wt: Vec<String> = Vec::new();
                let mut p2 = 0;
                while p2 < prog.funs[i].params.len() {
                    wn.push(String::from(prog.funs[i].params[p2].as_str()));
                    wt.push(if p2 < prog.funs[i].ptys.len() {
                        String::from(prog.funs[i].ptys[p2].as_str())
                    } else {
                        String::from("")
                    });
                    p2 = p2 + 1;
                }
                // A2 ARG-WIDTH: every fn's parameter types, as `$fn:<name>` = "t0,t1,.." (read at each call)
                wfn_sigs(prog, &mut wn, &mut wt);
                // A2 RET-WIDTH: the declared return width, read by `return x` and by the body's tail
                wn.push(String::from("$ret"));
                wt.push(String::from(prog.funs[i].ret.as_str()));
                wwalk(&prog.funs[i].body, &mut wn, &mut wt, ds);
                wtail(&prog.funs[i].body, &mut wn, &mut wt, ds);
            }
        }
        i = i + 1;
    }
}

fn use_of(m: &mut Mv, n: &str, lo: u32, hi: u32, ds: &mut Vec<Diag>) {
    let ix = mv_ix(m, n);
    if ix >= 0 {
        let k = ix as usize;
        if (m.aggr[k] || m.strm[k]) && m.moved[k] {
            let mut msg = String::from("use of moved value `");
            msg.push_str(n);
            msg.push_str("`");
            ds.push(oerrors::err(msg, lo, hi));
        }
    }
}
fn do_move(m: &mut Mv, n: &str, lo: u32, hi: u32, ds: &mut Vec<Diag>, depth: i32) {
    let ix = mv_ix(m, n);
    if ix >= 0 {
        let k = ix as usize;
        if m.aggr[k] {
            if m.moved[k] {
                let mut msg = String::from("value `");
                msg.push_str(n);
                msg.push_str("` moved twice");
                ds.push(oerrors::err(msg, lo, hi));
            } else if depth > 0 && k < m.ldep.len() && m.ldep[k] < depth {
                let mut msg = String::from("value `");
                msg.push_str(n);
                msg.push_str("` moved in a loop body (moved on every iteration)");
                ds.push(oerrors::err(msg, lo, hi));
            }
            m.moved[k] = true;
        }
    }
}

// the Vec/String methods that take `&mut self`
fn mut_method(mm: &str) -> bool {
    mm == "push"
        || mm == "pop"
        || mm == "insert"
        || mm == "remove"
        || mm == "clear"
        || mm == "push_str"
        || mm == "truncate"
        || mm == "sort"
        || mm == "reverse"
        || mm == "extend"
        || mm == "swap"
        || mm == "retain"
        || mm == "dedup"
        || mm == "drain"
        || mm == "append"
}
// a let initializer: a bare variable is MOVED into the new binding (BCK-LETMOVE); anything else is walked
fn let_init(v: &E, prog: &Prog, m: &mut Mv, ds: &mut Vec<Diag>, depth: i32) {
    match v {
        E::Var(n, l, h) => {
            use_of(m, n, *l, *h, ds);
            do_move(m, n, *l, *h, ds, depth);
            // BCK-STRMOVE: a String binding moves ONLY here (`let t = s;`) -- calls are not tracked for strings, since println!
            // lowers to calls that take the string as an argument
            let ix = mv_ix(m, n);
            if ix >= 0 {
                let k = ix as usize;
                if m.strm[k] {
                    m.moved[k] = true;
                }
            }
        }
        _ => {
            walk(v, prog, m, ds, depth);
        }
    }
}
// is this initializer a String VALUE (String::from / String::new / .to_string() / .to_owned())?
fn init_is_string(e: &E, prog: &Prog) -> bool {
    if oast::in_u32(&prog.strvals, oast::e_lo(e)) {
        return true;
    }
    match e {
        E::Call(n, _a, _l, _h) => n.as_str() == "$id1" || n.as_str() == "$emptystr",
        E::MethodCall(_r, mm, a, _l, _h) => {
            (mm.as_str() == "to_string" || mm.as_str() == "to_owned") && a.len() == 0
        }
        _ => false,
    }
}
fn walk(e: &E, prog: &Prog, m: &mut Mv, ds: &mut Vec<Diag>, depth: i32) {
    match e {
        E::Var(n, l, h) => {
            use_of(m, n, *l, *h, ds);
        }
        E::Ref(x, mt, rl0, rh0) => {
            // A3 BCK-MUT-REF (E0596): `&mut x` of a binding declared without `mut` (the method form was already refused)
            if *mt {
                if let E::Var(rn, _vl, _vh) = x.as_ref() {
                    let ix = mv_ix(m, rn);
                    if ix >= 0 && m.imm[ix as usize] {
                        let mut msg = String::from("cannot borrow `");
                        msg.push_str(rn);
                        msg.push_str("` as mutable, as it is not declared as mutable");
                        ds.push(oerrors::err(msg, *rl0, *rh0));
                    }
                }
            }
            // a BORROW is not a move: only the inner place is "used"
            match x.as_ref() {
                E::Var(n, l2, h2) => {
                    use_of(m, n, *l2, *h2, ds);
                }
                _ => {
                    walk(x, prog, m, ds, depth);
                }
            }
        }
        E::Call(cn, _a, _l, _h) if cn.as_str() == "$zeroof" => {} // `$zeroof(x)` (mem::take's desugar) reads only x's TYPE, never its value
        // UPD-BASE-MOVE (R2a P5): `$partialmove(..b)` is the struct-update desugar's marker that b gave its fields to the new
        // value -- lowering's drop analysis reads it as b's move; it is not a use for the move checker (it precedes the literal)
        E::Call(cn, _a, _l, _h) if cn.as_str() == "$partialmove" => {}
        // R15.4 FMT-BORROWS: the formatting builtins `format!` / `println!` / `{:?}` expand to take their arguments BY
        // REFERENCE (`format!("{:?}", foo_a)` then `if let Foo::A(v) = foo_a` is valid): a use, never a move
        // R15.4 INDEX-BORROWS: `x[i]` on a user `impl Index` / `IndexMut` desugars to `T$index(x, i)`; the operator borrows
        // x (`&self` / `&mut self`), so the receiver is a use, not a move. The index argument is walked as usual.
        E::Call(cn, args, _l, _h) if index_op(cn) && args.len() > 0 => {
            walk_recv_use(&args[0], prog, m, ds, depth);
            let mut k = 1;
            while k < args.len() {
                walk(&args[k], prog, m, ds, depth);
                k = k + 1;
            }
        }
        E::Call(cn, args, _l, _h) if fmt_builtin(cn) => {
            let mut k = 0;
            while k < args.len() {
                walk(&args[k], prog, m, ds, depth);
                k = k + 1;
            }
        }
        E::Call(_n, args, _l, _h) => {
            let mut k = 0;
            while k < args.len() {
                match &args[k] {
                    E::Var(n2, l2, h2) => {
                        use_of(m, n2, *l2, *h2, ds);
                        do_move(m, n2, *l2, *h2, ds, depth);
                    }
                    _ => {
                        walk(&args[k], prog, m, ds, depth);
                    }
                }
                k = k + 1;
            }
        }
        E::MethodCall(r, mm, args, _l, _h) => {
            walk(r, prog, m, ds, depth);
            // BCK-MUT (E0596): a mutating method on a binding declared without `mut`
            match r.as_ref() {
                E::Var(rn, rl, rh) => {
                    if mut_method(mm) && !user_method_named(prog, mm) {
                        let ix = mv_ix(m, rn);
                        if ix >= 0 && m.imm[ix as usize] {
                            let mut msg = String::from("cannot borrow `");
                            msg.push_str(rn);
                            msg.push_str("` as mutable, as it is not declared as mutable");
                            ds.push(oerrors::err(msg, *rl, *rh));
                        }
                    }
                }
                _ => {}
            }
            let mut k = 0;
            while k < args.len() {
                walk(&args[k], prog, m, ds, depth);
                k = k + 1;
            }
        }
        // MATCH-NO-MOVE: the match desugar's scrutinee temp `let $msN = e` READS the place -- `match e { E::Foo{..} => .. }`
        // does not move e (rustc moves only when an arm binds a non-Copy payload by value; that partial move is not modelled)
        E::Let(n, v, b, _l, _h)
            if oast::str_starts(n.as_str(), "$ms")
                && (match v.as_ref() {
                    E::Var(_a, _b, _c) => true,
                    _ => false,
                }) =>
        {
            walk(v, prog, m, ds, depth);
            mv_bind_d(m, n, false, depth);
            walk(b, prog, m, ds, depth);
            mv_pop(m);
        }
        E::Let(n, v, b, l, _h) => {
            // R15.4 LET-WILD: `let _ = x;` binds nothing, so it moves nothing (rustc: the `_` pattern never moves its place)
            if n.as_str() == "_" {
                walk(v, prog, m, ds, depth);
            } else {
                let_init(v, prog, m, ds, depth);
            }
            // BCK-LETMOVE: `let t = s;` moves s, exactly like passing it by value
            let ag = init_is_aggr(prog, v) || {
                match v.as_ref() {
                    E::Var(vn, _a, _b) => {
                        let ix = mv_ix(m, vn);
                        ix >= 0 && m.aggr[ix as usize]
                    }
                    _ => false,
                }
            };
            // a binding that HOLDS a `&mut` reference mutates through it without being `mut` itself
            let mref = match v.as_ref() {
                E::Ref(_x, mt, _a, _b) => *mt,
                _ => false,
            };
            let sv = prog.realrs
                && (init_is_string(v, prog) || init_is_vec(v) || {
                    match v.as_ref() {
                        E::Var(vn, _a, _b) => {
                            let ix = mv_ix(m, vn);
                            ix >= 0 && m.strm[ix as usize]
                        }
                        _ => false,
                    }
                });
            mv_bind_d(m, n, ag, depth);
            {
                let k = m.imm.len() - 1;
                m.imm[k] = oast::in_u32(&prog.immlets, *l) && !mref;
                m.strm[k] = sv;
            }
            walk(b, prog, m, ds, depth);
            mv_pop(m);
        }
        E::LetT(n, t, v, b, l, _h) => {
            let_init(v, prog, m, ds, depth);
            mv_bind_d(m, n, aggr_ty(&prog.sdefs, &prog.edefs, t), depth);
            {
                // a `&mut` initializer (also `&mut x as *mut T`) makes the binding a mutable REFERENCE: mutating through
                // it needs no `mut` on the binding (`let w: &mut Vec<i32> = &mut v; w.push(2)` was a false E0596)
                let mref = match v.as_ref() {
                    E::Ref(_x, mt, _a, _b) => *mt,
                    E::Cast(x, _t, _a, _b) => match x.as_ref() {
                        E::Ref(_y, mt, _c, _d) => *mt,
                        _ => false,
                    },
                    _ => false,
                };
                let k = m.imm.len() - 1;
                m.imm[k] = oast::in_u32(&prog.immlets, *l) && !mref;
            }
            walk(b, prog, m, ds, depth);
            mv_pop(m);
        }
        E::Seq(a, b, _l, _h) => {
            walk(a, prog, m, ds, depth);
            walk(b, prog, m, ds, depth);
        }
        E::Bin(_o, a, b, _l, _h) => {
            walk(a, prog, m, ds, depth);
            walk(b, prog, m, ds, depth);
        }
        E::If(c, t, f, _l, _h) => {
            // IF-PATHS (R2a P4): the branches are ALTERNATIVE paths -- each starts from the state after the condition, and a
            // binding is moved after the `if` when a NON-diverging branch moved it. Walked in sequence (one shared state), a
            // move in `then` made a use in `else` "use of moved value", and `if c { return C(Box::new(b)) } b` was rejected
            // (rustc accepts both: the paths never meet / the branch never reaches the later use)
            walk(c, prog, m, ds, depth);
            let snap = bools_copy(&m.moved);
            walk(t, prog, m, ds, depth);
            let after_t = bools_copy(&m.moved);
            bools_restore(&mut m.moved, &snap);
            walk(f, prog, m, ds, depth);
            let dt = diverges(t);
            let df = diverges(f);
            let mut k = 0;
            while k < m.moved.len() && k < after_t.len() {
                let mt = after_t[k] && !dt;
                let mf = m.moved[k] && !df;
                m.moved[k] = if dt && df {
                    after_t[k] || m.moved[k]
                } else {
                    mt || mf
                };
                k = k + 1;
            }
        }
        E::While(c, b, _l, _h) => {
            walk(c, prog, m, ds, depth);
            // LOOP-REINIT (R2a P4): a binding the body UNCONDITIONALLY reassigns (a top-level statement, not inside an if/match)
            // is fresh on every iteration -- `let a = C(Box::new(chain)); ..; chain = a;` is not "moved on every iteration"
            // (rustc accepts). It is treated as local to the body for the walk, exactly as REINIT-LOOP does within one statement.
            let mut ra: Vec<String> = Vec::new();
            top_assigns(b, &mut ra);
            let nv = fl_nevers(prog);
            cont_reinits(b, &nv, &mut ra);
            let mut saved: Vec<i32> = Vec::new();
            let mut q = 0;
            while q < ra.len() {
                let ix = mv_ix(m, ra[q].as_str());
                if ix >= 0 && (ix as usize) < m.ldep.len() {
                    saved.push(m.ldep[ix as usize]);
                    m.ldep[ix as usize] = depth + 1;
                } else {
                    saved.push(0 - 99);
                }
                q = q + 1;
            }
            walk(b, prog, m, ds, depth + 1);
            q = 0;
            while q < ra.len() {
                let ix = mv_ix(m, ra[q].as_str());
                if ix >= 0 && (ix as usize) < m.ldep.len() && saved[q] != 0 - 99 {
                    m.ldep[ix as usize] = saved[q];
                }
                q = q + 1;
            }
        }
        E::Match(sc, arms, _l, _h) => {
            walk(sc, prog, m, ds, depth);
            // ARM-PATHS (R2a P4, IF-PATHS for match): arms are ALTERNATIVE paths -- each starts from the post-scrutinee state;
            // a binding is moved after the match when a NON-diverging arm moved it (in sequence, a move in `Some(..) =>` made
            // the same binding "moved twice" in `None =>`)
            let snap = bools_copy(&m.moved);
            let mut acc: Vec<bool> = Vec::new();
            let mut alldiv = true;
            let mut anyacc: Vec<bool> = Vec::new();
            let mut z = 0;
            while z < snap.len() {
                acc.push(false);
                anyacc.push(false);
                z = z + 1;
            }
            let mut a = 0;
            while a < arms.len() {
                bools_restore(&mut m.moved, &snap);
                let mut bi = 0;
                while bi < arms[a].binds.len() {
                    mv_bind_d(m, arms[a].binds[bi].as_str(), false, depth);
                    bi = bi + 1;
                }
                walk(&arms[a].body, prog, m, ds, depth);
                let mut bp = 0;
                while bp < arms[a].binds.len() {
                    mv_pop(m);
                    bp = bp + 1;
                }
                let dv = diverges(&arms[a].body);
                if !dv {
                    alldiv = false;
                }
                let mut k = 0;
                while k < acc.len() && k < m.moved.len() {
                    if m.moved[k] {
                        anyacc[k] = true;
                        if !dv {
                            acc[k] = true;
                        }
                    }
                    k = k + 1;
                }
                a = a + 1;
            }
            if arms.len() > 0 {
                let mut k = 0;
                while k < acc.len() && k < m.moved.len() {
                    m.moved[k] = if alldiv { anyacc[k] } else { acc[k] };
                    k = k + 1;
                }
            }
        }
        E::Field(o, _f, _l, _h) => {
            walk(o, prog, m, ds, depth);
        }
        E::TupIdx(o, _i, _l, _h) => {
            walk(o, prog, m, ds, depth);
        }
        E::Index(b, i, _l, _h) => {
            walk(b, prog, m, ds, depth);
            walk(i, prog, m, ds, depth);
        }
        E::Assign(n, v, l, h) => {
            // REINIT-LOOP (heavy pass 3): `e = Ex::Mul(e, r)` in a loop moves e and re-initializes it in the same statement, so
            // it is not "moved on every iteration" (LIT-MOVE made it an error: cli ast_frontend). The RHS sees n as local to this
            // depth; a second move inside the RHS is still `moved twice`.
            let ix0 = mv_ix(m, n);
            let mut saved = 0 - 1;
            if ix0 >= 0 && (ix0 as usize) < m.ldep.len() {
                saved = m.ldep[ix0 as usize];
                m.ldep[ix0 as usize] = depth;
            }
            walk(v, prog, m, ds, depth);
            if ix0 >= 0 && (ix0 as usize) < m.ldep.len() {
                m.ldep[ix0 as usize] = saved;
            }
            let ix = mv_ix(m, n);
            if ix >= 0 {
                let k = ix as usize;
                // BCK-MUT (E0384): assigning to a binding declared without `mut`
                if m.imm[k] {
                    let mut msg = String::from("cannot assign twice to immutable variable `");
                    msg.push_str(n);
                    msg.push_str("`");
                    ds.push(oerrors::err(msg, *l, *h));
                }
                // BCK-REINIT: assignment re-initializes a moved binding (`p = P{..}` after `f(p)` makes p usable again)
                m.moved[k] = false;
            }
        }
        E::AssignTo(p, v, _l, _h) => {
            walk(p, prog, m, ds, depth);
            walk(v, prog, m, ds, depth);
        }
        E::Not(x, _l, _h) => {
            walk(x, prog, m, ds, depth);
        }
        E::RetE(x, _l, _h) => {
            walk(x, prog, m, ds, depth);
        }
        E::Try(x, _l, _h) => {
            walk(x, prog, m, ds, depth);
        }
        E::TupLit(ps, _l, _h) => {
            let mut k = 0;
            while k < ps.len() {
                lit_elem(&ps[k], prog, m, ds, depth); // LIT-MOVE (BCK): a binding placed in a literal is MOVED into it
                k = k + 1;
            }
        }
        E::ArrLit(ps, _l, _h) => {
            let mut k = 0;
            while k < ps.len() {
                lit_elem(&ps[k], prog, m, ds, depth); // LIT-MOVE (BCK): a binding placed in a literal is MOVED into it
                k = k + 1;
            }
        }
        E::StructLit(_n, ps, _l, _h) => {
            let mut k = 0;
            while k < ps.len() {
                lit_elem(&ps[k], prog, m, ds, depth);
                k = k + 1;
            }
        }
        E::EnumLit(_n, _v, ps, _l, _h) => {
            let mut k = 0;
            while k < ps.len() {
                lit_elem(&ps[k], prog, m, ds, depth);
                k = k + 1;
            }
        }
        _ => {}
    }
}
// LIT-MOVE (BCK): a binding written as a literal element (`T { x: s }`, `(s, 1)`, `[s]`, `Some(s)`) is MOVED into the value --
// `let t = T { x: s }; println!("{}", s)` was ACCEPTED (rustc E0382). Same rule as a by-value call argument.
fn lit_elem(e: &E, prog: &Prog, m: &mut Mv, ds: &mut Vec<Diag>, depth: i32) {
    match e {
        E::Var(n2, l2, h2) => {
            use_of(m, n2, *l2, *h2, ds);
            do_move(m, n2, *l2, *h2, ds, depth);
            let ix = mv_ix(m, n2);
            if ix >= 0 && m.strm[ix as usize] {
                m.moved[ix as usize] = true; // a String / Vec placed in a literal is moved (do_move skips strings for println!)
            }
        }
        _ => walk(e, prog, m, ds, depth),
    }
}
// BCK-VECMOVE: a Vec VALUE (`vec![..]` -- a LetT around vec_new -- or `Vec::new()`) is move-tracked exactly like a String:
// `let w = v;` moves v, and a later use of v is E0382
fn init_is_vec(e: &E) -> bool {
    match e {
        E::Call(n, _a, _l, _h) => n.as_str() == "vec_new",
        E::LetT(_n, _t, v, _b, _l, _h) => match v.as_ref() {
            E::Call(n, _a, _l2, _h2) => n.as_str() == "vec_new",
            _ => false,
        },
        _ => false,
    }
}
// WIDTH-MISMATCH (E0308): Rust never converts between integer widths implicitly. `let y: i32 = x;` where x was DECLARED
// with another width (an annotated let or a typed parameter) is a type error. Only declared widths are tracked, so an
// unannotated binding (whose width rustc infers) never fires.
fn wfind(wn: &Vec<String>, wt: &Vec<String>, n: &str) -> String {
    let mut i = wn.len();
    while i > 0 {
        i = i - 1;
        if wn[i].as_str() == n {
            return String::from(wt[i].as_str());
        }
    }
    String::from("")
}
fn wwalk(e: &E, wn: &mut Vec<String>, wt: &mut Vec<String>, ds: &mut Vec<Diag>) {
    match e {
        E::LetT(n, t, v, b, l, h) => {
            wwalk(v, wn, wt, ds);
            if oast::is_int_width(t) {
                match v.as_ref() {
                    E::Var(x, _a, _b) => {
                        let xw = wfind(wn, wt, x);
                        if xw.len() > 0 && oast::is_int_width(&xw) && xw.as_str() != t.as_str() {
                            let mut msg = String::from("mismatched types: expected `");
                            msg.push_str(t);
                            msg.push_str("`, found `");
                            msg.push_str(&xw);
                            msg.push_str("`");
                            ds.push(oerrors::err(msg, *l, *h));
                        }
                    }
                    _ => {}
                }
            }
            wn.push(String::from(n.as_str()));
            wt.push(String::from(t.as_str()));
            wwalk(b, wn, wt, ds);
            let _ = wn.pop();
            let _ = wt.pop();
        }
        E::Let(n, v, b, _l, _h) => {
            wwalk(v, wn, wt, ds);
            wn.push(String::from(n.as_str()));
            wt.push(String::from(""));
            wwalk(b, wn, wt, ds);
            let _ = wn.pop();
            let _ = wt.pop();
        }
        E::Seq(a, b, _l, _h) => {
            wwalk(a, wn, wt, ds);
            wwalk(b, wn, wt, ds);
        }
        E::RetE(x, l, h) => {
            wwalk(x, wn, wt, ds);
            wret(x, *l, *h, wn, wt, ds);
        }
        E::Call(f, args, l, h) => {
            let mut k = 0;
            while k < args.len() {
                wwalk(&args[k], wn, wt, ds);
                k = k + 1;
            }
            warg(f, args, *l, *h, wn, wt, ds);
        }
        E::If(c, t, f, _l, _h) => {
            wwalk(c, wn, wt, ds);
            wwalk(t, wn, wt, ds);
            wwalk(f, wn, wt, ds);
        }
        E::While(c, b, _l, _h) => {
            wwalk(c, wn, wt, ds);
            wwalk(b, wn, wt, ds);
        }
        E::Match(s, arms, _l, _h) => {
            wwalk(s, wn, wt, ds);
            let mut k = 0;
            while k < arms.len() {
                let mut q = 0;
                while q < arms[k].binds.len() {
                    wn.push(String::from(arms[k].binds[q].as_str()));
                    wt.push(String::from(""));
                    q = q + 1;
                }
                wwalk(&arms[k].body, wn, wt, ds);
                q = 0;
                while q < arms[k].binds.len() {
                    let _ = wn.pop();
                    let _ = wt.pop();
                    q = q + 1;
                }
                k = k + 1;
            }
        }
        _ => {}
    }
}
// BCK-SHARED-MUT (E0502): `let r = &v[..]` / `let r = &v;` then a MUTATING method on v (push, insert, clear, ...) while r is
// still used afterwards. The let body is read as its statement sequence (the Seq chain): once a statement mutates v, any
// LATER statement that mentions r is the conflict. (Written without or-patterns or borrowed-element Vecs: SEM0 miscompiled
// that shape into a compiler that crashed.)
fn mentions_v(a: &Vec<E>, n: &str) -> bool {
    let mut k = 0;
    while k < a.len() {
        if mentions(&a[k], n) {
            return true;
        }
        k = k + 1;
    }
    false
}
fn mentions(e: &E, n: &str) -> bool {
    match e {
        E::Var(x, _l, _h) => x.as_str() == n,
        E::Seq(a, b, _l, _h) => mentions(a, n) || mentions(b, n),
        E::Bin(_o, a, b, _l, _h) => mentions(a, n) || mentions(b, n),
        E::While(a, b, _l, _h) => mentions(a, n) || mentions(b, n),
        E::Index(a, b, _l, _h) => mentions(a, n) || mentions(b, n),
        E::AssignTo(a, b, _l, _h) => mentions(a, n) || mentions(b, n),
        E::If(c, t, f, _l, _h) => mentions(c, n) || mentions(t, n) || mentions(f, n),
        E::Let(x, v, b, _l, _h) => mentions(v, n) || (x.as_str() != n && mentions(b, n)),
        E::LetT(x, _t, v, b, _l, _h) => mentions(v, n) || (x.as_str() != n && mentions(b, n)),
        E::Call(_f, a, _l, _h) => mentions_v(a, n),
        E::TupLit(a, _l, _h) => mentions_v(a, n),
        E::ArrLit(a, _l, _h) => mentions_v(a, n),
        E::StructLit(_s, a, _l, _h) => mentions_v(a, n),
        E::EnumLit(_s, _v, a, _l, _h) => mentions_v(a, n),
        E::MethodCall(r, _m, a, _l, _h) => mentions(r, n) || mentions_v(a, n),
        E::Field(x, _f, _l, _h) => mentions(x, n),
        E::Not(x, _l, _h) => mentions(x, n),
        E::Ref(x, _m, _l, _h) => mentions(x, n),
        E::Cast(x, _t, _l, _h) => mentions(x, n),
        E::TupIdx(x, _i, _l, _h) => mentions(x, n),
        E::RetE(x, _l, _h) => mentions(x, n),
        E::Try(x, _l, _h) => mentions(x, n),
        E::Assign(_v, x, _l, _h) => mentions(x, n),
        E::Match(s, arms, _l, _h) => {
            if mentions(s, n) {
                return true;
            }
            let mut k = 0;
            while k < arms.len() {
                if mentions(&arms[k].body, n) {
                    return true;
                }
                k = k + 1;
            }
            false
        }
        _ => false,
    }
}
fn mutates(e: &E, v: &str) -> bool {
    match e {
        E::MethodCall(r, m, _a, _l, _h) => match r.as_ref() {
            E::Var(x, _a2, _b2) => x.as_str() == v && mut_method(m.as_str()),
            _ => false,
        },
        E::Seq(a, b, _l, _h) => mutates(a, v) || mutates(b, v),
        E::If(c, t, f, _l, _h) => mutates(c, v) || mutates(t, v) || mutates(f, v),
        _ => false,
    }
}
// walk the Seq chain: true when a statement after the first mutation of `src` mentions `r`
fn seq_conflict(e: &E, src: &str, r: &str, mutated: bool) -> u32 {
    match e {
        E::Seq(a, b, _l, _h) => {
            if mutated && mentions(a, r) {
                return oast::e_lo(a) + 1;
            }
            let m2 = mutated || mutates(a, src);
            seq_conflict(b, src, r, m2)
        }
        _ => {
            if mutated && mentions(e, r) {
                return oast::e_lo(e) + 1;
            }
            0
        }
    }
}
fn shared_src(v: &E) -> String {
    match v {
        E::Ref(x, mt, _l, _h) => {
            if *mt {
                return String::from("");
            }
            match x.as_ref() {
                E::Var(n, _a, _b) => String::from(n.as_str()),
                E::Index(b, _i, _a, _b2) => match b.as_ref() {
                    E::Var(n, _a3, _b3) => String::from(n.as_str()),
                    _ => String::from(""),
                },
                _ => String::from(""),
            }
        }
        _ => String::from(""),
    }
}
fn shared_hit(r: &str, v: &E, b: &E, ds: &mut Vec<Diag>) {
    let src = shared_src(v);
    if src.len() > 0 {
        let at = seq_conflict(b, src.as_str(), r, false);
        if at > 0 {
            let mut msg = String::from("cannot borrow `");
            msg.push_str(src.as_str());
            msg.push_str("` as mutable because it is also borrowed as immutable");
            ds.push(oerrors::err(msg, at - 1, at));
        }
    }
}
pub fn shared_mut_walk(e: &E, ds: &mut Vec<Diag>) {
    match e {
        E::Let(r, v, b, _l, _h) => {
            shared_hit(r.as_str(), v, b, ds);
            shared_mut_walk(v, ds);
            shared_mut_walk(b, ds);
        }
        E::LetT(r, _t, v, b, _l, _h) => {
            shared_hit(r.as_str(), v, b, ds);
            shared_mut_walk(v, ds);
            shared_mut_walk(b, ds);
        }
        E::Seq(a, b, _l, _h) => {
            shared_mut_walk(a, ds);
            shared_mut_walk(b, ds);
        }
        E::While(a, b, _l, _h) => {
            shared_mut_walk(a, ds);
            shared_mut_walk(b, ds);
        }
        E::If(c, t, f, _l, _h) => {
            shared_mut_walk(c, ds);
            shared_mut_walk(t, ds);
            shared_mut_walk(f, ds);
        }
        E::Match(s, arms, _l, _h) => {
            shared_mut_walk(s, ds);
            let mut k = 0;
            while k < arms.len() {
                shared_mut_walk(&arms[k].body, ds);
                k = k + 1;
            }
        }
        _ => {}
    }
}
// USER-METHOD-SHADOW (R12): a USER method named like a std mutator (`fn swap(&self)` on Pair) is not `Vec::swap` -- the
// BCK-MUT rule for std mutators refused `p.swap()` on a non-`mut` binding ("cannot borrow `p` as mutable")
fn user_method_named(prog: &Prog, mm: &str) -> bool {
    let mut sfx = String::from("$");
    sfx.push_str(mm);
    let mut i = 0;
    while i < prog.funs.len() {
        let n = prog.funs[i].name.as_str();
        if n.len() > sfx.len() && n.ends_with(sfx.as_str()) && !oast::str_starts(n, "$") {
            return true;
        }
        i = i + 1;
    }
    false
}
// IF-PATHS (R2a P4): a branch that never falls through (return / break / continue / panic, or a block ending in one)
fn diverges(e: &E) -> bool {
    match e {
        E::RetE(_x, _l, _h) => true,
        E::BreakE(_l, _h) => true,
        E::BreakN(_n, _l, _h) => true,
        E::ContE(_l, _h) => true,
        E::ContN(_n, _l, _h) => true,
        E::Call(n, _a, _l, _h) => n.as_str() == "panic_exit" || n.as_str() == "proc_exit", // R15.4: process::exit never returns
        E::Seq(_a, b, _l, _h) => diverges(b),
        E::Let(_n, _v, b, _l, _h) => diverges(b),
        E::LetT(_n, _t, _v, b, _l, _h) => diverges(b),
        E::If(_c, t, f, _l, _h) => diverges(t) && diverges(f),
        _ => false,
    }
}
fn bools_copy(v: &Vec<bool>) -> Vec<bool> {
    let mut o: Vec<bool> = Vec::new();
    let mut i = 0;
    while i < v.len() {
        o.push(v[i]);
        i = i + 1;
    }
    o
}
fn bools_restore(v: &mut Vec<bool>, s: &Vec<bool>) {
    let mut i = 0;
    while i < v.len() && i < s.len() {
        v[i] = s[i];
        i = i + 1;
    }
}
// LET-FLOW (REGRESS-D): definite assignment for a DEFERRED `let x;`. LET-DEFER made such a binding assignable once, which
// silenced E0381 (use of a possibly-uninitialized binding) and E0384 (an immutable one assigned twice). States: 0 unassigned,
// 1 assigned, 2 maybe, 3 unreachable (after return / break / continue / panic). One diagnostic per binding.
fn fl_join(a: u32, b: u32) -> u32 {
    // (self-host: early returns, not an if-value -- ourc typed a u32 parameter as unknown against the literal)
    if a == 3 {
        return b;
    }
    if b == 3 || a == b {
        return a;
    }
    2
}
struct Fl {
    x: String,
    imm: bool,
    brk: u32,
    bad: bool,
    asg: bool,           // A2 DEFER-NOANN: some path assigns the binding
    nevers: Vec<String>, // REGRESS-E: user fns declared `-> !` (a call to one diverges)
}
fn fl_err(f: &mut Fl, msg: &str, lo: u32, hi: u32, ds: &mut Vec<Diag>) {
    if !f.bad {
        f.bad = true;
        let mut m = String::from(msg);
        m.push_str(" `");
        m.push_str(f.x.as_str());
        m.push('`');
        ds.push(oerrors::err(m, lo, hi));
    }
}
fn fl_v(a: &Vec<E>, f: &mut Fl, st: u32, ds: &mut Vec<Diag>) -> u32 {
    let mut s = st;
    let mut k = 0;
    while k < a.len() {
        s = fl(&a[k], f, s, ds);
        k = k + 1;
    }
    s
}
fn fl_loop(c: &E, body: &E, f: &mut Fl, st: u32, ds: &mut Vec<Diag>) -> u32 {
    let forever = match c {
        E::Bool(true, _l, _h) => true,
        _ => false,
    };
    let ob = f.brk;
    f.brk = 3;
    let s1 = fl(c, f, st, ds);
    let sb = fl(body, f, s1, ds);
    if sb != 3 && !f.bad {
        // the second iteration: an immutable binding assigned in the body is assigned twice
        let s2 = fl(c, f, fl_join(s1, sb), ds);
        let _ = fl(body, f, s2, ds);
    }
    // (self-host: assigned, not an if-value -- ourc typed the field read `f.brk` as unknown against the call)
    let mut out = f.brk;
    if !forever {
        out = fl_join(fl_join(s1, sb), f.brk);
    }
    f.brk = ob;
    out
}
fn fl(e: &E, f: &mut Fl, st: u32, ds: &mut Vec<Diag>) -> u32 {
    if st == 3 || f.bad {
        return st;
    }
    match e {
        E::Var(n, lo, hi) => {
            if n.as_str() == f.x.as_str() && st != 1 {
                fl_err(
                    f,
                    "used binding isn't initialized (possibly-uninitialized)",
                    *lo,
                    *hi,
                    ds,
                );
            }
            st
        }
        E::ClosRef(_n, caps, lo, hi) => {
            let mut cap = false;
            let mut k = 0;
            while k < caps.len() {
                if caps[k].as_str() == f.x.as_str() {
                    cap = true;
                }
                k = k + 1;
            }
            if st != 1 && cap {
                fl_err(
                    f,
                    "used binding isn't initialized (captured, possibly-uninitialized)",
                    *lo,
                    *hi,
                    ds,
                );
            }
            st
        }
        E::Assign(n, v, lo, hi) => {
            let s = fl(v, f, st, ds);
            if s == 3 {
                return 3;
            }
            if n.as_str() == f.x.as_str() {
                if f.imm && s != 0 {
                    fl_err(f, "cannot assign twice to immutable variable", *lo, *hi, ds);
                }
                f.asg = true;
                return 1;
            }
            s
        }
        E::Seq(a, b, _l, _h) => {
            let s = fl(a, f, st, ds);
            fl(b, f, s, ds)
        }
        E::Let(n, v, b, _l, _h) => {
            let s = fl(v, f, st, ds);
            if n.as_str() == f.x.as_str() {
                return s; // shadowed: the rest is another binding
            }
            fl(b, f, s, ds)
        }
        E::LetT(n, _t, v, b, _l, _h) => {
            let s = fl(v, f, st, ds);
            if n.as_str() == f.x.as_str() {
                return s;
            }
            fl(b, f, s, ds)
        }
        E::If(c, t, el, _l, _h) => {
            let s = fl_true(c, f, st, ds); // REGRESS-E: the then-branch sees the condition's TRUE state
            let a = fl(t, f, s, ds);
            let b = fl(el, f, s, ds);
            fl_join(a, b)
        }
        E::Match(sc, arms, _l, _h) => {
            let s = fl(sc, f, st, ds);
            let mut out = 3;
            let mut k = 0;
            while k < arms.len() {
                let mut shadow = false;
                let mut j = 0;
                while j < arms[k].binds.len() {
                    if arms[k].binds[j].as_str() == f.x.as_str() {
                        shadow = true;
                    }
                    j = j + 1;
                }
                let mut g = s;
                if arms[k].has_guard {
                    g = fl(&arms[k].guard, f, s, ds);
                }
                let mut a = g;
                if !shadow {
                    a = fl(&arms[k].body, f, g, ds);
                }
                out = fl_join(out, a);
                k = k + 1;
            }
            if arms.len() == 0 {
                return s;
            }
            out
        }
        E::While(c, b, _l, _h) => fl_loop(c, b, f, st, ds),
        E::BreakE(_l, _h) => {
            f.brk = fl_join(f.brk, st);
            3
        }
        E::BreakN(_n, _l, _h) => {
            f.brk = fl_join(f.brk, st);
            3
        }
        E::ContE(_l, _h) => 3,
        E::ContN(_n, _l, _h) => 3,
        E::RetE(v, _l, _h) => {
            let _ = fl(v, f, st, ds);
            3
        }
        E::Bin(op, a, b, _l, _h) => {
            if op.as_str() == "&&" {
                // REGRESS-E: the right side of `&&` runs only when the left was TRUE (`true && { z = 3; true } && z == 3`)
                let s = fl_true(a, f, st, ds);
                let r = fl(b, f, s, ds);
                return fl_join(s, r);
            }
            if op.as_str() == "||" {
                // REGRESS-E: the right side of `||` runs only when the left was FALSE
                let s = fl_false(a, f, st, ds);
                let r = fl(b, f, s, ds);
                return fl_join(s, r);
            }
            let s = fl(a, f, st, ds);
            fl(b, f, s, ds)
        }
        E::Call(n, a, _l, _h) => {
            let s = fl_v(a, f, st, ds);
            if n.as_str() == "panic_exit" || oast::in_list(&f.nevers, n.as_str()) {
                return 3;
            }
            s
        }
        E::MethodCall(r, _m, a, _l, _h) => {
            let s = fl(r, f, st, ds);
            fl_v(a, f, s, ds)
        }
        E::AssignTo(a, b, lo, hi) => {
            let s = fl(b, f, st, ds);
            let s = fl(a, f, s, ds);
            // `x op= v` / `x = v` through a place: a plain-local target is a (re)assignment
            let tgt = match a.as_ref() {
                E::Var(n, _l, _h) => n.as_str() == f.x.as_str(),
                _ => false,
            };
            if tgt && s != 3 {
                if f.imm && s != 0 {
                    fl_err(f, "cannot assign twice to immutable variable", *lo, *hi, ds);
                }
                f.asg = true;
                return 1;
            }
            s
        }
        E::Index(a, b, _l, _h) => {
            let s = fl(a, f, st, ds);
            fl(b, f, s, ds)
        }
        E::Cast(a, _t, _l, _h) => fl(a, f, st, ds),
        E::Ref(a, _m, _l, _h) => fl(a, f, st, ds),
        E::Try(a, _l, _h) => fl(a, f, st, ds),
        E::TupIdx(a, _i, _l, _h) => fl(a, f, st, ds),
        E::Field(a, _n, _l, _h) => fl(a, f, st, ds),
        E::Not(a, _l, _h) => fl(a, f, st, ds),
        E::TupLit(a, _l, _h) => fl_v(a, f, st, ds),
        E::ArrLit(a, _l, _h) => fl_v(a, f, st, ds),
        E::EnumLit(_t, _v, a, _l, _h) => fl_v(a, f, st, ds),
        E::StructLit(_t, a, _l, _h) => fl_v(a, f, st, ds),
        _ => st,
    }
}
// find each deferred `let` (keyed by its start, `@defer:` / `@deferm:` in fpays) and check its scope
fn fl_find(e: &E, prog: &Prog, ds: &mut Vec<Diag>) {
    match e {
        E::Let(n, v, b, lo, _h) => {
            fl_at(n, b, *lo, prog, ds);
            fl_find(v, prog, ds);
            fl_find(b, prog, ds);
        }
        E::LetT(n, _t, v, b, lo, _h) => {
            fl_at(n, b, *lo, prog, ds);
            fl_find(v, prog, ds);
            fl_find(b, prog, ds);
        }
        E::Seq(a, b, _l, _h) => {
            fl_find(a, prog, ds);
            fl_find(b, prog, ds);
        }
        E::If(c, t, el, _l, _h) => {
            fl_find(c, prog, ds);
            fl_find(t, prog, ds);
            fl_find(el, prog, ds);
        }
        E::While(c, b, _l, _h) => {
            fl_find(c, prog, ds);
            fl_find(b, prog, ds);
        }
        E::Match(s, arms, _l, _h) => {
            fl_find(s, prog, ds);
            let mut k = 0;
            while k < arms.len() {
                fl_find(&arms[k].body, prog, ds);
                k = k + 1;
            }
        }
        _ => {}
    }
}
fn fl_at(n: &String, b: &E, lo: u32, prog: &Prog, ds: &mut Vec<Diag>) {
    let mut imm = String::from("@defer:");
    imm.push_str(lo.to_string().as_str());
    let mut mu = String::from("@deferm:");
    mu.push_str(lo.to_string().as_str());
    let mut k = 0;
    while k < prog.fpays.len() {
        let p = prog.fpays[k].as_str();
        if p == imm.as_str() || p == mu.as_str() {
            let mut f = Fl {
                x: String::from(n.as_str()),
                imm: p == imm.as_str(),
                brk: 3,
                bad: false,
                asg: false,
                nevers: fl_nevers(prog),
            };
            let _ = fl(b, &mut f, 0, ds);
            if !f.bad && !f.asg {
                let mut un = String::from("@deferu:");
                un.push_str(lo.to_string().as_str());
                if oast::in_list(&prog.fpays, un.as_str()) {
                    let mut m = String::from("type annotations needed for `");
                    m.push_str(n.as_str());
                    m.push_str("` (never assigned)");
                    ds.push(oerrors::err(m, lo, lo + 1));
                }
            }
            return;
        }
        k = k + 1;
    }
}
pub fn let_flow(prog: &Prog, ds: &mut Vec<Diag>) {
    fl_find(&prog.main, prog, ds);
    let mut i = 0;
    while i < prog.funs.len() {
        fl_find(&prog.funs[i].body, prog, ds);
        i = i + 1;
    }
}
// A2 RET-WIDTH (E0308): a returned variable DECLARED with one int width (annotated let / typed param) from a fn declared
// to return another (`fn f() -> usize { let i: isize = 3; return i; }`). Only declared widths, as WIDTH-MISMATCH.
fn wret(x: &E, l: u32, h: u32, wn: &Vec<String>, wt: &Vec<String>, ds: &mut Vec<Diag>) {
    match x {
        E::Var(v, _a, _b) => {
            let xw = wfind(wn, wt, v);
            let rw = wfind(wn, wt, "$ret");
            if oast::is_int_width(&xw) && oast::is_int_width(&rw) && xw.as_str() != rw.as_str() {
                let mut msg = String::from("mismatched types: expected `");
                msg.push_str(&rw);
                msg.push_str("`, found `");
                msg.push_str(&xw);
                msg.push_str("` (returned)");
                ds.push(oerrors::err(msg, l, h));
            }
        }
        _ => {}
    }
}
// the body's TAIL value, through its let / seq chain (bindings in scope as the walk goes)
fn wtail(e: &E, wn: &mut Vec<String>, wt: &mut Vec<String>, ds: &mut Vec<Diag>) {
    match e {
        E::LetT(n, t, _v, b, _l, _h) => {
            wn.push(String::from(n.as_str()));
            wt.push(String::from(t.as_str()));
            wtail(b, wn, wt, ds);
            let _ = wn.pop();
            let _ = wt.pop();
        }
        E::Let(n, _v, b, _l, _h) => {
            wn.push(String::from(n.as_str()));
            wt.push(String::from(""));
            wtail(b, wn, wt, ds);
            let _ = wn.pop();
            let _ = wt.pop();
        }
        E::Seq(_a, b, _l, _h) => wtail(b, wn, wt, ds),
        E::Var(_v, l, h) => wret(e, *l, *h, wn, wt, ds),
        _ => {}
    }
}
// A2 ARG-WIDTH: `$fn:<name>` = the fn's parameter types joined by `,` (a generic / unknown type stays as written and
// never fires: only two declared int widths that differ are a mismatch)
fn wfn_sigs(prog: &Prog, wn: &mut Vec<String>, wt: &mut Vec<String>) {
    let mut i = 0;
    while i < prog.funs.len() {
        let f = &prog.funs[i];
        let mut k = String::from("$fn:");
        k.push_str(f.name.as_str());
        let mut v = String::new();
        let mut p = 0;
        while p < f.params.len() {
            if p > 0 {
                v.push(',');
            }
            if p < f.ptys.len() && !oast::in_list(&f.gparams, f.ptys[p].as_str()) {
                v.push_str(f.ptys[p].as_str());
            }
            p = p + 1;
        }
        wn.push(k);
        wt.push(v);
        i = i + 1;
    }
}
// A2 ARG-WIDTH (E0308): a variable DECLARED with one int width passed where the callee's parameter is declared another
fn warg(
    f: &str,
    args: &Vec<E>,
    l: u32,
    h: u32,
    wn: &Vec<String>,
    wt: &Vec<String>,
    ds: &mut Vec<Diag>,
) {
    let mut key = String::from("$fn:");
    key.push_str(f);
    let sig = wfind(wn, wt, key.as_str());
    if sig.len() == 0 {
        return;
    }
    let pts: Vec<&str> = sig.split(',').collect();
    if pts.len() != args.len() {
        return;
    }
    let mut k = 0;
    while k < args.len() {
        if let E::Var(v, _a, _b) = &args[k] {
            let xw = wfind(wn, wt, v);
            let pw = pts[k];
            if oast::is_int_width(&xw) && oast::is_int_width(pw) && xw.as_str() != pw {
                let mut msg = String::from("mismatched types: expected `");
                msg.push_str(pw);
                msg.push_str("`, found `");
                msg.push_str(&xw);
                msg.push_str("` (argument)");
                ds.push(oerrors::err(msg, l, h));
                return;
            }
        }
        k = k + 1;
    }
}
// LOOP-REINIT (R2a P4): names assigned by a TOP-LEVEL statement of a block (the Seq/Let spine; an if/match/loop body is
// conditional and does not count)
fn top_assigns(e: &E, out: &mut Vec<String>) {
    match e {
        E::Assign(n, _v, _l, _h) => out.push(String::from(n.as_str())),
        E::Seq(a, b, _l, _h) => {
            top_assigns(a, out);
            top_assigns(b, out);
        }
        E::Let(_n, _v, b, _l, _h) => top_assigns(b, out),
        E::LetT(_n, _t, _v, b, _l, _h) => top_assigns(b, out),
        _ => {}
    }
}
// REGRESS-E: the state when a condition evaluates TRUE -- through an `&&` chain both sides ran
fn fl_true(e: &E, f: &mut Fl, st: u32, ds: &mut Vec<Diag>) -> u32 {
    if let E::Bin(op, a, b, _l, _h) = e {
        if op.as_str() == "&&" {
            let s = fl_true(a, f, st, ds);
            return fl_true(b, f, s, ds);
        }
    }
    fl(e, f, st, ds)
}
// REGRESS-E: the user fns that never return (`fn baz() -> ! { .. }`): a call to one diverges like `panic!`
fn fl_nevers(prog: &Prog) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    let mut i = 0;
    while i < prog.funs.len() {
        if prog.funs[i].ret.as_str() == "!"
            || diverges(&prog.funs[i].body)
            || loop_forever(&prog.funs[i].body)
        {
            v.push(String::from(prog.funs[i].name.as_str()));
        }
        i = i + 1;
    }
    v
}
// REGRESS-E: the state when a condition evaluates FALSE -- through an `||` chain both sides ran
fn fl_false(e: &E, f: &mut Fl, st: u32, ds: &mut Vec<Diag>) -> u32 {
    if let E::Bin(op, a, b, _l, _h) = e {
        if op.as_str() == "||" {
            let s = fl_false(a, f, st, ds);
            return fl_false(b, f, s, ds);
        }
    }
    fl(e, f, st, ds)
}
// R15.4 LOOP-REINIT-CONT: in a loop body, `if c { ..; x = v; continue; } <rest>` with <rest> diverging (return / panic / a
// call to a fn returning `!`) reaches the back-edge only through the branch that reassigned x -- so x is fresh on every
// iteration, as an unconditional reassignment is (rustc accepts `let _g = g; if f { g = S; continue; } my_panic();`)
fn cont_reinits(e: &E, nv: &Vec<String>, out: &mut Vec<String>) {
    match e {
        E::Seq(a, b, _l, _h) => {
            if let E::If(_c, t, _f, _l2, _h2) = a.as_ref() {
                if ends_continue(t) && diverges_nv(b, nv) {
                    top_assigns(t, out);
                }
            }
            cont_reinits(b, nv, out);
        }
        E::Let(_n, _v, b, _l, _h) => cont_reinits(b, nv, out),
        E::LetT(_n, _t, _v, b, _l, _h) => cont_reinits(b, nv, out),
        // the parser folds `if c { .. } rest` into `If(c, .., rest)`: the same shape with the rest as the else branch
        E::If(_c, t, f, _l, _h) => {
            if ends_continue(t) && diverges_nv(f, nv) {
                top_assigns(t, out);
            }
            cont_reinits(t, nv, out);
            cont_reinits(f, nv, out);
        }
        _ => {}
    }
}
fn ends_continue(e: &E) -> bool {
    match e {
        E::ContE(_l, _h) => true,
        E::ContN(_n, _l, _h) => true,
        E::Seq(a, b, _l, _h) => ends_continue(a) || ends_continue(b), // what follows a `continue` is unreachable
        E::Let(_n, _v, b, _l, _h) => ends_continue(b),
        E::LetT(_n, _t, _v, b, _l, _h) => ends_continue(b),
        _ => false,
    }
}
// diverges, counting a call to a user fn that never returns
fn diverges_nv(e: &E, nv: &Vec<String>) -> bool {
    if diverges(e) {
        return true;
    }
    match e {
        E::Call(n, _a, _l, _h) => oast::in_list(nv, n.as_str()),
        E::Seq(a, b, _l, _h) => diverges_nv(a, nv) || diverges_nv(b, nv),
        E::Let(_n, _v, b, _l, _h) => diverges_nv(b, nv),
        E::LetT(_n, _t, _v, b, _l, _h) => diverges_nv(b, nv),
        _ => false,
    }
}
// R15.4: a body that is (or ends in) `loop { .. }` with no `break` anywhere inside never returns (`fn my_panic() -> ! { loop {} }`).
// Conservative: any construct the scan does not know counts as possibly breaking.
fn loop_forever(e: &E) -> bool {
    match e {
        E::While(c, b, _l, _h) => match c.as_ref() {
            E::Bool(v, _a, _b) => *v && no_break(b),
            _ => false,
        },
        E::Seq(_a, b, _l, _h) => loop_forever(b),
        E::Let(_n, _v, b, _l, _h) => loop_forever(b),
        _ => false,
    }
}
fn no_break(e: &E) -> bool {
    match e {
        E::BreakE(_l, _h) => false,
        E::BreakN(_n, _l, _h) => false,
        E::Int(_v, _l, _h) => true,
        E::Bool(_v, _l, _h) => true,
        E::Var(_n, _l, _h) => true,
        E::ContE(_l, _h) => true,
        E::Call(_n, a, _l, _h) => {
            let mut k = 0;
            while k < a.len() {
                if !no_break(&a[k]) {
                    return false;
                }
                k = k + 1;
            }
            true
        }
        E::Seq(a, b, _l, _h) => no_break(a) && no_break(b),
        E::Bin(_o, a, b, _l, _h) => no_break(a) && no_break(b),
        E::If(c, t, f, _l, _h) => no_break(c) && no_break(t) && no_break(f),
        E::Let(_n, v, b, _l, _h) => no_break(v) && no_break(b),
        E::Assign(_n, v, _l, _h) => no_break(v),
        _ => false,
    }
}
// the READ-ONLY builtins: formatting (R15.4 FMT-BORROWS) and the slice reads `v[a..b]` desugars to (INDEX-BORROWS:
// `seq_slice(v, a, seq_len(v))` copies out of v and moves nothing)
fn fmt_builtin(n: &str) -> bool {
    n == "print_any"
        || n == "print_dbg"
        || n == "print_raw"
        || n == "str_of"
        || n == "str_dbg"
        || n == "seq_len"
        || n == "seq_slice"
}
fn index_op(n: &str) -> bool {
    suffix_is(n, "$index") || suffix_is(n, "$index_mut")
}
fn suffix_is(n: &str, s: &str) -> bool {
    let b = n.as_bytes();
    let sb = s.as_bytes();
    b.len() >= sb.len() && &b[b.len() - sb.len()..] == sb
}
// the receiver of a borrowing operator: a bare variable is only USED; anything else is walked
fn walk_recv_use(e: &E, prog: &Prog, m: &mut Mv, ds: &mut Vec<Diag>, depth: i32) {
    match e {
        E::Var(n, l, h) => use_of(m, n, *l, *h, ds),
        _ => walk(e, prog, m, ds, depth),
    }
}
