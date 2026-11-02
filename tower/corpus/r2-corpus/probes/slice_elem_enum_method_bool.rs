enum Tok { Num(i64), Plus, End }
impl Tok { fn is_op(&self) -> bool { match self { Tok::Plus => true, _ => false } } }
struct P<'a> { toks: &'a [Tok], pos: usize }
fn main() {
    let v = vec![Tok::Num(3), Tok::Plus, Tok::End];
    let p = P { toks: &v, pos: 1 };
    println!("{} {}", v[1].is_op(), p.toks[0].is_op());
}
