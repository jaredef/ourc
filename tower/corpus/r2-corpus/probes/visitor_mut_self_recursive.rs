enum Expr { Lit(i64), Add(Box<Expr>, Box<Expr>) }
struct Counter { n: i64, sum: i64 }
impl Counter {
    fn visit(&mut self, e: &Expr) {
        self.n += 1;
        match e {
            Expr::Lit(v) => { self.sum += *v; }
            Expr::Add(a, b) => { self.visit(a); self.visit(b); }
        }
    }
}
fn main() {
    let t = Expr::Add(Box::new(Expr::Lit(3)), Box::new(Expr::Add(Box::new(Expr::Lit(4)), Box::new(Expr::Lit(5)))));
    let mut c = Counter { n: 0, sum: 0 };
    c.visit(&t);
    println!("{} {}", c.n, c.sum);
}
