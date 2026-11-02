#[derive(Debug, Clone, PartialEq)] enum Ty { Int, Var(u32) }
#[derive(Debug)] struct P { x: i64 }
fn main() { let t = Ty::Var(4); let s = format!("<{:?}>", t); println!("{}", s); let p = P { x: 2 }; println!("{}", format!("{:?}", p)); let r = &t; println!("{}", format!("{:?}", r)); println!("{}", format!("{:?}", vec![1, 2])); }
