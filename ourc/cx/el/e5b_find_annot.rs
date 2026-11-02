#[derive(Debug, Clone, Copy, PartialEq)] enum Res { Local(u32), Err }
fn main() { let v = vec![Res::Local(2), Res::Err]; let a: Option<&Res> = v.iter().find(|x| **x != Res::Err); println!("{:?}", a); let f = v.first(); println!("{:?}", f); let g: Option<&Res> = v.get(1); println!("{:?}", g);
  if let Some(r) = v.iter().find(|x| **x == Res::Err) { println!("{:?}", r); } }
