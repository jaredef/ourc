#[derive(Debug)]
enum Expr { Lit(i64), Add(Box<Expr>, Box<Expr>) }
fn main() {
    let e = Expr::Add(Box::new(Expr::Lit(1)), Box::new(Expr::Lit(2)));
    println!("{:?}", e);
    println!("{:?}", Expr::Lit(7));
}
