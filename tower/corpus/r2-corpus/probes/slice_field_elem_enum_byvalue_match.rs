#[derive(Clone, Copy)]
enum Tok { Num(i64), Plus }
struct P<'a> { toks: &'a [Tok] }
fn main() {
    let v = vec![Tok::Num(7), Tok::Plus];
    let p = P { toks: &v };
    let a = match p.toks[0] { Tok::Num(n) => n, _ => 0 };
    println!("{}", a);
}
