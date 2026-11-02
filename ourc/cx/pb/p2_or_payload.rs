enum Tok { Punct(char), Eof }
fn f(t: &Tok) -> i32 { match t { Tok::Punct('+' | '-') => 1, Tok::Punct('*' | '/') => 2, _ => 0 } }
fn main() { println!("{} {} {} {}", f(&Tok::Punct('-')), f(&Tok::Punct('/')), f(&Tok::Punct('x')), f(&Tok::Eof)); }
