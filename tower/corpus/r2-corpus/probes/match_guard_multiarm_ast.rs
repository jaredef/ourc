enum Expr { Lit(i64), Add(Box<Expr>, Box<Expr>) }
fn eval(e: &Expr) -> i64 { match e { Expr::Lit(v) => *v, Expr::Add(a,b) => eval(a)+eval(b) } }
fn classify(e: &Expr) -> i64 {
    match e {
        Expr::Add(a, b) if eval(a) == eval(b) => 1,
        Expr::Add(_, _) => 2,
        Expr::Lit(n) if *n < 0 => 3,
        Expr::Lit(_) => 4,
    }
}
fn main() {
    println!("{}", classify(&Expr::Add(Box::new(Expr::Lit(2)), Box::new(Expr::Lit(2)))));
    println!("{}", classify(&Expr::Add(Box::new(Expr::Lit(2)), Box::new(Expr::Lit(3)))));
    println!("{}", classify(&Expr::Lit(-5)));
    println!("{}", classify(&Expr::Lit(5)));
}
