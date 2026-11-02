#[derive(Clone)]
enum Tok { Num(i64), Plus, End }
struct P<'a> { toks: &'a [Tok], pos: usize }
impl<'a> P<'a> {
    fn peek(&self) -> &Tok { &self.toks[self.pos] }
    fn bump(&mut self) -> Tok { let t = self.toks[self.pos].clone(); self.pos += 1; t }
}
fn main() {
    let v = vec![Tok::Num(3), Tok::Plus, Tok::Num(4), Tok::End];
    let mut p = P { toks: &v, pos: 0 };
    let mut sum = 0;
    loop {
        let stop = match p.peek() { Tok::End => true, _ => false };
        if stop { break; }
        match p.bump() { Tok::Num(n) => sum += n, _ => {} }
    }
    println!("{}", sum);
}
