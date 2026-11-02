#[derive(Clone)]
enum Expr { Lit(i64), Add(Box<Expr>, Box<Expr>) }
fn eval(e: &Expr) -> i64 { match e { Expr::Lit(n) => *n, Expr::Add(a, b) => eval(a) + eval(b) } }
fn main() {
    let e = Expr::Add(Box::new(Expr::Lit(3)), Box::new(Expr::Lit(4)));
    println!("{}", eval(&e));
    let c = e.clone();
    println!("{}", eval(&c));
}
