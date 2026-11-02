enum Expr { Lit(i64), Add(Box<Expr>, Box<Expr>) }
fn simplify(e: &Expr) -> i64 {
    match e {
        Expr::Add(a, b) => match a.as_ref() {
            Expr::Lit(0) => match b.as_ref() { Expr::Lit(n) => *n, _ => 999 },
            _ => -1,
        },
        Expr::Lit(n) => *n,
    }
}
fn main() { println!("{}", simplify(&Expr::Add(Box::new(Expr::Lit(0)), Box::new(Expr::Lit(42))))); }
