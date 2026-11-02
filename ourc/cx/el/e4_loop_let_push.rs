#[derive(Debug, Clone, PartialEq)] enum Tok { Int(i64), Eof }
fn mk(i: i64) -> Tok { if i < 2 { Tok::Int(i) } else { Tok::Eof } }
fn main() { let mut v = Vec::new(); let mut i = 0; loop { let t = mk(i); let end = t == Tok::Eof; v.push(t); if end { break; } i += 1; } println!("{:?}", v); }
