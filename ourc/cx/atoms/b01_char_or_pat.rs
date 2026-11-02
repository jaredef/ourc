#[derive(Debug, Clone, PartialEq)] enum Tok { Punct(char), Eof }
fn prec(t: &Tok) -> Option<(u8, char)> { match t { Tok::Punct(c @ ('+' | '-')) => Some((1, *c)), Tok::Punct(c @ ('*' | '/')) => Some((2, *c)), _ => None } }
fn main() { println!("{:?} {:?} {:?}", prec(&Tok::Punct('+')), prec(&Tok::Punct('/')), prec(&Tok::Eof)); }
