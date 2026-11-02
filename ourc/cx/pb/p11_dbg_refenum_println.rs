#[derive(Debug, Clone, PartialEq)] enum Ty { Int, Var(u32) }
fn h(a: &Ty) { println!("{:?}", a); }
fn k(a: &Ty) -> String { let s = format!("<{:?}>", a); s }
fn main() { h(&Ty::Var(3)); println!("{}", k(&Ty::Int)); let t = Ty::Var(9); let r = &t; println!("{:?}", r); }
