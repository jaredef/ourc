enum Expr { Num(i32), Add(Box<Expr>, Box<Expr>) }
impl Expr { fn eval(&self) -> i32 { match self { Expr::Num(n)=>*n, Expr::Add(a,b)=>a.eval()+b.eval() } } }
fn parse(t: i32) -> Result<Box<Expr>, String> { if t < 0 { return Err("neg".to_string()); } Ok(Box::new(Expr::Num(t))) }
fn main() { match parse(7) { Ok(e) => println!("{}", e.eval()), Err(m) => println!("e {}", m) } }
