enum Expr { Lit(i64), Call { name: String, args: Vec<Box<Expr>> } }
fn eval(e: &Expr) -> i64 { match e { Expr::Lit(v) => *v, Expr::Call { name: _n, args } => { let mut t = 0; for a in args { t += eval(a); } t } } }
fn main() {
    let c = Expr::Call { name: String::from("sum"), args: vec![Box::new(Expr::Lit(1)), Box::new(Expr::Lit(2))] };
    println!("{}", eval(&c));
}
