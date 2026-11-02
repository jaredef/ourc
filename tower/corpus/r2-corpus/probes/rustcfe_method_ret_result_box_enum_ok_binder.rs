enum Expr { Num(i32), Add(Box<Expr>, Box<Expr>) }
impl Expr { fn eval(&self) -> i32 { match self { Expr::Num(n)=>*n, Expr::Add(a,b)=>a.eval()+b.eval() } } }
struct P { toks: Vec<i32>, pos: usize }
impl P {
    fn parse(&mut self) -> Result<Box<Expr>, String> {
        if self.pos >= self.toks.len() { return Err("eof".to_string()); }
        let t = self.toks[self.pos]; self.pos = self.pos + 1;
        Ok(Box::new(Expr::Num(t)))
    }
}
fn main() {
    let mut p = P { toks: vec![7], pos: 0 };
    match p.parse() { Ok(e) => println!("{}", e.eval()), Err(m) => println!("err {}", m) }
    match p.parse() { Ok(e) => println!("{}", e.eval()), Err(m) => println!("err {}", m) }
}
