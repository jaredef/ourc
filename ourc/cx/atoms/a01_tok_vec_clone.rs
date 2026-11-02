#[derive(Debug, Clone, PartialEq)] enum Tok { Ident(String), Int(i64), Eof }
fn main() { let toks = vec![Tok::Ident("x".into()), Tok::Int(3), Tok::Eof]; let t = toks[1].clone(); let e = toks[2] == Tok::Eof; println!("{:?} {} {}", t, e, toks.len()); }
