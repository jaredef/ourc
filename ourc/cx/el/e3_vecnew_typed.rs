#[derive(Debug, Clone, PartialEq)] enum Tok { Int(i64), Eof }
fn mk(i: i64) -> Tok { if i < 2 { Tok::Int(i) } else { Tok::Eof } }
fn main() { let mut v: Vec<Tok> = Vec::new(); for i in 0..3 { v.push(mk(i)); } println!("{:?}", v); }
