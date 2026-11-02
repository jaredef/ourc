#[derive(Clone)]
enum Tok { Num, Open, Close, End }
struct Cur { toks: Vec<Tok>, pos: usize }
impl Cur {
    fn peek(&self) -> Tok { self.toks[self.pos].clone() }
    fn bump(&mut self) -> Tok { let t = self.toks[self.pos].clone(); self.pos += 1; t }
    fn go(&mut self) -> Vec<i64> {
        let mut out: Vec<i64> = Vec::new();
        loop {
            let t = self.peek();
            match t {
                Tok::End => break,
                Tok::Close => { self.bump(); break; }
                Tok::Open => { self.bump(); let inner = self.go(); out.push(100 + inner.len() as i64); }
                Tok::Num => { self.bump(); out.push(1); }
            }
        }
        out
    }
}
fn main() {
    let toks = vec![Tok::Num, Tok::Open, Tok::Num, Tok::Num, Tok::Close, Tok::Num, Tok::End];
    let mut c = Cur { toks: toks, pos: 0 };
    let r = c.go();
    let mut s = 0; for x in r.iter() { s = s + x; }
    println!("{}", s);
}
