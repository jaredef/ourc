enum Expr { Lit(i64), Add(Box<Expr>, Box<Expr>) }
fn mk(v: i64) -> Expr { Expr::Lit(v) }
fn eval(e: &Expr) -> i64 { match e { Expr::Lit(v) => *v, Expr::Add(a, b) => eval(a) + eval(b) } }
fn main() {
    let t = Expr::Add(Box::new(mk(3)), Box::new(mk(4)));
    println!("{}", eval(&t));
}
