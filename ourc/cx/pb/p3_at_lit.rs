enum Tok { Punct(char), Eof }
fn f(t: &Tok) -> String { match t { Tok::Punct(c @ '+') => format!("p{}", c), Tok::Punct(c @ '/') => format!("d{}", c), _ => "o".to_string() } }
fn main() { println!("{} {} {} {}", f(&Tok::Punct('+')), f(&Tok::Punct('/')), f(&Tok::Punct('x')), f(&Tok::Eof)); }
