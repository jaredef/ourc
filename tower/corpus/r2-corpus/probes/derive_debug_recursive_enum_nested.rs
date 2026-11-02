#[derive(Debug)]
enum Expr { Lit(i64), Neg(Box<Expr>), Add(Box<Expr>, Box<Expr>) }
#[derive(Debug)]
enum Color { Red, Green }
fn main() {
    let e = Expr::Add(Box::new(Expr::Neg(Box::new(Expr::Lit(3)))), Box::new(Expr::Lit(4)));
    println!("{:?}", e);
    println!("{:?}", Color::Green);
}
