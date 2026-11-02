#[derive(Debug, Clone, PartialEq)] enum Ty { Int, Var(u32) }
fn f(a: &Ty, b: &Ty) -> String { match (a, b) { (Ty::Var(x), t) | (t, Ty::Var(x)) => format!("v{} {} {:?}", x, matches!(t, Ty::Int), t), _ => "o".into() } }
fn g(a: &Ty, b: &Ty) -> String { match (a, b) { (Ty::Var(x), t) => format!("v{} {:?}", x, t), _ => "o".into() } }
fn main() { println!("{} | {} | {}", f(&Ty::Int, &Ty::Var(5)), f(&Ty::Var(6), &Ty::Int), g(&Ty::Var(7), &Ty::Int)); }
