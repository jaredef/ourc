type P<T> = Box<T>;
struct Expr { v: i64 }
struct Q;
impl Q { fn atom(&self) -> P<Expr> { Box::new(Expr { v: 9 }) } }
fn main() { let q = Q; let a = q.atom(); println!("{}", a.v); }
