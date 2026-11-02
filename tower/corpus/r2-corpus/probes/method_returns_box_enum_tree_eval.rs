enum Expr { Num(i64), Bin(Box<Expr>, u8, Box<Expr>) }
struct P { v: i64 }
impl P {
    fn mk(&self) -> Box<Expr> { Box::new(Expr::Bin(Box::new(Expr::Num(3)), 43, Box::new(Expr::Num(8)))) }
}
fn eval(e: &Expr) -> i64 { match e { Expr::Num(n) => *n, Expr::Bin(a, op, b) => { let x = eval(a); let y = eval(b); if *op == 43 { x + y } else { x * y } } } }
fn main() { let p = P { v: 0 }; let ast = p.mk(); println!("{}", eval(&ast)); }
