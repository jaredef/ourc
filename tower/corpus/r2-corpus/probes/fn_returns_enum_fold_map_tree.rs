enum Expr { Lit(i64), Add(Box<Expr>, Box<Expr>) }
fn double(e: &Expr) -> Expr {
    match e {
        Expr::Lit(v) => Expr::Lit(*v * 2),
        Expr::Add(a, b) => Expr::Add(Box::new(double(a)), Box::new(double(b))),
    }
}
fn eval(e: &Expr) -> i64 { match e { Expr::Lit(v) => *v, Expr::Add(a, b) => eval(a) + eval(b) } }
fn main() {
    let t = Expr::Add(Box::new(Expr::Lit(3)), Box::new(Expr::Lit(4)));
    let d = double(&t);
    println!("{} {}", eval(&t), eval(&d));
}
