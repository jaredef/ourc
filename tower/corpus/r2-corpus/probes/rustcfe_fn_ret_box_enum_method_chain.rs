enum Expr { Num(i32), Add(Box<Expr>, Box<Expr>) }
impl Expr { fn eval(&self) -> i32 { match self { Expr::Num(n)=>*n, Expr::Add(a,b)=>a.eval()+b.eval() } } }
fn mk(v: i32) -> Box<Expr> { Box::new(Expr::Add(Box::new(Expr::Num(v)), Box::new(Expr::Num(1)))) }
fn main() { println!("{}", mk(5).eval()); }
