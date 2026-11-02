enum Expr { Lit(i64), Add(Box<Expr>, Box<Expr>) }
trait Visit { fn visit(&mut self, e: &Expr); }
struct Depth { max: i64, cur: i64 }
impl Visit for Depth {
    fn visit(&mut self, e: &Expr) {
        self.cur += 1;
        if self.cur > self.max { self.max = self.cur; }
        match e {
            Expr::Lit(_) => {}
            Expr::Add(a, b) => { self.visit(a); self.visit(b); }
        }
        self.cur -= 1;
    }
}
fn main() {
    let t = Expr::Add(Box::new(Expr::Add(Box::new(Expr::Lit(1)), Box::new(Expr::Lit(2)))), Box::new(Expr::Lit(3)));
    let mut d = Depth { max: 0, cur: 0 };
    d.visit(&t);
    println!("{}", d.max);
}
