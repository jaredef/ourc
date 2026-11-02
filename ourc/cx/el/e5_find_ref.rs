#[derive(Debug, Clone, Copy, PartialEq)] enum Res { Local(u32), Err }
fn main() { let v = vec![Res::Local(2), Res::Err]; let a = v.iter().find(|x| **x != Res::Err); println!("{:?}", a); let b = v.iter().rev().find(|x| **x != Res::Err); println!("{:?}", b); }
