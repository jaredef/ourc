#[derive(Clone)]
enum Ty { Bool, I32, Str, Ref(Box<Ty>), Infer(u32), Error }
fn tag(t: &Ty) -> u32 { match t { Ty::Bool => 0, Ty::I32 => 1, Ty::Str => 2, Ty::Ref(_) => 3, Ty::Infer(_) => 4, Ty::Error => 5 } }
fn f(a: &Vec<Ty>) -> usize { a.len() }
fn main() { println!("{}", f(&vec![Ty::I32, Ty::Bool])); }
