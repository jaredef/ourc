#[derive(Debug, Clone, PartialEq)] enum Ty { Int, Var(u32) }
fn main() { let val: Vec<Option<Ty>> = vec![None, Some(Ty::Var(3))]; let a = val[1].clone(); println!("{:?}", a); println!("{:?}", val[1].clone()); println!("{:?}", val); }
