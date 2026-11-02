#[derive(Debug, Clone, PartialEq)] enum Ty { Int, Var(u32) }
fn h(a: &Ty) -> String { format!("{:?}", a) }
fn k(a: &Ty) -> String { let t = a; format!("{:?}", t) }
fn main() { println!("{} {}", h(&Ty::Var(3)), k(&Ty::Int)); }
