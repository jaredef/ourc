enum Expr { Num(i64), Bin(Box<Expr>, u8, Box<Expr>) }
fn eval(e: &Expr) -> i64 { match e { Expr::Num(n) => *n, Expr::Bin(a, op, b) => { let _ = op; eval(a) + eval(b) } } }
fn mk() -> Box<Expr> { Box::new(Expr::Bin(Box::new(Expr::Num(3)), 43, Box::new(Expr::Num(8)))) }
fn main() { let ast = mk(); let r = eval(&ast); println!("{}", r); }
