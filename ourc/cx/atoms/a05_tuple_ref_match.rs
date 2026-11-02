#[derive(Debug, Clone, PartialEq)] enum Ty { Int, Var(u32), Fn(Vec<Ty>, Box<Ty>) }
fn k(a: &Ty, b: &Ty) -> &'static str { match (a, b) { (Ty::Var(_), Ty::Var(_)) => "vv", (Ty::Var(_), _) | (_, Ty::Var(_)) => "v", (Ty::Fn(x, _), Ty::Fn(y, _)) if x.len() == y.len() => "ff", _ if a == b => "eq", _ => "no" } }
fn main() { let f = Ty::Fn(vec![Ty::Int], Box::new(Ty::Int)); println!("{} {} {} {} {}", k(&Ty::Var(1), &Ty::Var(2)), k(&Ty::Int, &Ty::Var(0)), k(&f, &f.clone()), k(&Ty::Int, &Ty::Int), k(&Ty::Int, &f)); }
