enum Tok { Num(i64), Plus }
struct P { pos: i64 }
impl P { fn mk(&self) -> Tok { Tok::Num(self.pos) } }
fn main() {
    let p = P { pos: 7 };
    let a = match p.mk() { Tok::Num(n) => n, Tok::Plus => 0 };
    println!("{}", a);
}
