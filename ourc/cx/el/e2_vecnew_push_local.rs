#[derive(Debug, Clone, PartialEq)] enum Tok { Int(i64), Eof }
fn main() { let mut v = Vec::new(); let t = Tok::Int(4); v.push(t); v.push(Tok::Eof); println!("{:?}", v); }
