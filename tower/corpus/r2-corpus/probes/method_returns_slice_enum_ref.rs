enum Tok { Num(i64), Plus }
struct P<'a> { toks: &'a [Tok] }
impl<'a> P<'a> { fn peek(&self) -> &Tok { &self.toks[0] } }
fn main() { let v = vec![Tok::Num(5), Tok::Plus]; let p = P { toks: &v }; let a = match p.peek() { Tok::Num(n) => *n, _ => 0 }; println!("{}", a); }
