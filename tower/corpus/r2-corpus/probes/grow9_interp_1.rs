enum Expr { Num(i64), Add(Box<Expr>, Box<Expr>), Mul(Box<Expr>, Box<Expr>), Neg(Box<Expr>) }
fn eval(e: &Expr) -> i64 {
    match e {
        Expr::Num(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
        Expr::Neg(a) => -eval(a),
    }
}
fn main() {
    let e = Expr::Add(Box::new(Expr::Mul(Box::new(Expr::Num(3)), Box::new(Expr::Num(4)))),
                      Box::new(Expr::Neg(Box::new(Expr::Num(5)))));
    println!("{}", eval(&e));
}
