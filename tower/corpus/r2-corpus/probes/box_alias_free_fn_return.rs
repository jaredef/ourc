type P<T> = Box<T>;
struct Expr { v: i64 }
fn atom() -> P<Expr> { Box::new(Expr { v: 9 }) }
fn main() { let a = atom(); println!("{}", a.v); }
