#[derive(Debug, Clone, PartialEq)]
enum Expr { Lit(i32), Add(Box<Expr>, Box<Expr>) }
fn main() {
    let a = Expr::Add(Box::new(Expr::Lit(1)), Box::new(Expr::Lit(2)));
    let b = a.clone();
    println!("{}", (a == b) as i32);
    println!("{:?}", a);
}
