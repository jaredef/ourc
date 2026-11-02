// cx07 trait solving + borrowck: impl selection with type-param matching and nested obligations
// (rustc_trait_selection-shaped), and a MIR-shaped borrow dataflow over a CFG with bitsets to fixpoint.
#[derive(Debug, Clone, PartialEq)] enum T { Named(String, Vec<T>), Param(usize) }
struct Impl { trait_: String, self_ty: T, nparams: usize, where_: Vec<(String, usize)> }
fn matches(pat: &T, ty: &T, subst: &mut Vec<Option<T>>) -> bool {
    match (pat, ty) {
        (T::Param(i), t) => match &subst[*i] { Some(prev) => prev == t, None => { subst[*i] = Some(t.clone()); true } },
        (T::Named(a, xs), T::Named(b, ys)) => a == b && xs.len() == ys.len() && xs.iter().zip(ys).all(|(x, y)| matches(x, y, subst)),
        _ => false,
    }
}
fn solve(impls: &[Impl], tr: &str, ty: &T, depth: usize, trace: &mut Vec<String>) -> bool {
    if depth > 6 { trace.push("overflow".into()); return false; }
    for (k, im) in impls.iter().enumerate() {
        if im.trait_ != tr { continue; }
        let mut s = vec![None; im.nparams];
        if !matches(&im.self_ty, ty, &mut s) { continue; }
        let ok = im.where_.iter().all(|(wt, p)| match &s[*p] { Some(t) => solve(impls, wt, &t.clone(), depth + 1, trace), None => false });
        trace.push(format!("{}impl#{} {} for {:?} => {}", "  ".repeat(depth), k, tr, ty, ok));
        if ok { return true; }
    }
    false
}
fn n(s: &str) -> T { T::Named(s.into(), vec![]) }
// Borrowck: statements per block; loans are bits; a write to a place conflicts with a live loan of it.
#[derive(Clone, Copy)] enum St { Borrow(usize, usize, bool), Use(usize), Write(usize), Kill(usize) }
fn borrowck(blocks: &[(Vec<St>, Vec<usize>)], loan_place: &[usize]) -> Vec<String> {
    let nb = blocks.len(); let mut ins = vec![0u64; nb]; let mut errs = Vec::new(); let mut changed = true; let mut iters = 0;
    while changed { changed = false; iters += 1;
        for b in 0..nb { let mut live = ins[b];
            for st in &blocks[b].0 { match *st { St::Borrow(l, _, _) => live |= 1 << l, St::Kill(l) => live &= !(1 << l), _ => {} } }
            for &s in &blocks[b].1 { let nw = ins[s] | live; if nw != ins[s] { ins[s] = nw; changed = true; } } } }
    for b in 0..nb { let mut live = ins[b];
        for st in &blocks[b].0 { match *st {
            St::Borrow(l, p, mutable) => { for o in 0..64 { if live & (1 << o) != 0 && loan_place[o] == p && (mutable || loan_place.len() > 64) { errs.push(format!("bb{}: cannot borrow p{} as mutable, loan L{} live", b, p, o)); } } live |= 1 << l; }
            St::Write(p) => { for o in 0..loan_place.len() { if live & (1 << o) != 0 && loan_place[o] == p { errs.push(format!("bb{}: cannot assign p{} while borrowed (L{})", b, p, o)); } } }
            St::Kill(l) => live &= !(1 << l), St::Use(_) => {} } } }
    errs.push(format!("fixpoint after {} iters, ins={:?}", iters, ins)); errs
}
fn main() {
    let impls = vec![
        Impl { trait_: "Clone".into(), self_ty: n("i32"), nparams: 0, where_: vec![] },
        Impl { trait_: "Clone".into(), self_ty: T::Named("Vec".into(), vec![T::Param(0)]), nparams: 1, where_: vec![("Clone".into(), 0)] },
        Impl { trait_: "Clone".into(), self_ty: T::Named("Pair".into(), vec![T::Param(0), T::Param(0)]), nparams: 1, where_: vec![("Clone".into(), 0)] },
        Impl { trait_: "Debug".into(), self_ty: T::Named("Box".into(), vec![T::Param(0)]), nparams: 1, where_: vec![("Debug".into(), 0)] },
        Impl { trait_: "Debug".into(), self_ty: n("str"), nparams: 0, where_: vec![] },
    ];
    let goals = vec![("Clone", T::Named("Vec".into(), vec![T::Named("Vec".into(), vec![n("i32")])])), ("Clone", T::Named("Vec".into(), vec![n("str")])),
        ("Clone", T::Named("Pair".into(), vec![n("i32"), n("i32")])), ("Clone", T::Named("Pair".into(), vec![n("i32"), n("str")])), ("Debug", T::Named("Box".into(), vec![T::Named("Box".into(), vec![n("str")])]))];
    for (tr, ty) in &goals { let mut tr_ = vec![]; let ok = solve(&impls, tr, ty, 0, &mut tr_); println!("{} {:?}: {}", tr, ty, ok); for l in tr_ { println!("  {}", l); } }
    // bb0: L0=&mut p0; goto bb1 | bb1: use p0; write p0 (ERR: L0 live); goto bb2,bb1 | bb2: kill L0; write p0 (ok); L1=&p1; L2=&mut p1 (ERR)
    let blocks = vec![(vec![St::Borrow(0, 0, true)], vec![1]), (vec![St::Use(0), St::Write(0)], vec![2, 1]), (vec![St::Kill(0), St::Write(0), St::Borrow(1, 1, false), St::Borrow(2, 1, true)], vec![])];
    for e in borrowck(&blocks, &[0, 1, 1]) { println!("{}", e); }
}
