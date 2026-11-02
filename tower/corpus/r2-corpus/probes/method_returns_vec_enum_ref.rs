enum Tok { Num(i64), Plus }
struct P { toks: Vec<Tok> }
impl P { fn peek(&self) -> &Tok { &self.toks[0] } }
fn main() { let p = P { toks: vec![Tok::Num(5), Tok::Plus] }; let a = match p.peek() { Tok::Num(n) => *n, _ => 0 }; println!("{}", a); }
