#[derive(Clone)]
enum Tok { Ident(String), Num(i64) }
enum Expr { Num(i64), Assign(String, Box<Expr>) }
fn f(t: Tok) -> Box<Expr> {
    if let Tok::Ident(name) = t { Box::new(Expr::Assign(name, Box::new(Expr::Num(5)))) } else { Box::new(Expr::Num(0)) }
}
fn show(e: &Expr) -> i64 { match e { Expr::Num(n) => *n, Expr::Assign(_n, rhs) => match rhs.as_ref() { Expr::Num(n) => *n, _ => -1 } } }
fn main() { println!("{}", show(&f(Tok::Ident(String::from("x"))))); }
