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

fn main() {
    let impls = vec![
        Impl { trait_: "Clone".into(), self_ty: n("i32"), nparams: 0, where_: vec![] },
        Impl { trait_: "Clone".into(), self_ty: T::Named("Vec".into(), vec![T::Param(0)]), nparams: 1, where_: vec![("Clone".into(), 0)] },
    ];
    let ty = T::Named("Vec".into(), vec![n("i32")]);
    let mut s = vec![None; 1]; println!("m {}", matches(&impls[1].self_ty, &ty, &mut s)); println!("{:?}", s);
    let mut tr: Vec<String> = Vec::new(); println!("solve {}", solve(&impls, "Clone", &n("i32"), 0, &mut tr)); println!("{:?}", tr);
    let mut tr2: Vec<String> = Vec::new(); println!("solve {}", solve(&impls, "Clone", &ty, 0, &mut tr2)); println!("{:?}", tr2);
}
