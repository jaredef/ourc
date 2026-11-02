enum Tok { Num(i64), Plus }
impl Tok { fn val(&self) -> i64 { match self { Tok::Num(n) => *n, _ => 0 } } }
fn main() { let v = vec![Tok::Num(9), Tok::Plus]; println!("{}", v[0].val()); }
