#[derive(Clone)]
enum Tok { Num(i64), Plus }
struct P<'a> { toks: &'a [Tok] }
fn f(p: &P) -> i64 { let t = p.toks[0].clone(); match t { Tok::Num(n) => n, _ => 0 } }
fn main() { let v = vec![Tok::Num(7), Tok::Plus]; let p = P { toks: &v }; println!("{}", f(&p)); }
