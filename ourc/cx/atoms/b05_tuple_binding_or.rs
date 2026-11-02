#[derive(Debug, Clone, PartialEq)] enum Ty { Int, Var(u32), IntVar(u32) }
fn f(a: &Ty, b: &Ty) -> String { match (a, b) { (Ty::Var(x), Ty::Var(y)) | (Ty::IntVar(x), Ty::IntVar(y)) => format!("vv{}{}", x, y), (Ty::Var(x), t) | (t, Ty::Var(x)) => format!("v{}{:?}", x, t), _ => "o".into() } }
fn main() { println!("{} {} {} {}", f(&Ty::Var(1), &Ty::Var(2)), f(&Ty::IntVar(3), &Ty::IntVar(4)), f(&Ty::Int, &Ty::Var(5)), f(&Ty::Var(6), &Ty::Int)); }
