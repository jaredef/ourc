type P<T> = Box<T>;
enum EK { Lit(i64), Bin(u8, P<Expr>, P<Expr>) }
struct Expr { kind: EK }
struct Q;
impl Q {
    fn atom(&self) -> P<Expr> { Box::new(Expr { kind: EK::Lit(2) }) }
    fn build(&self) -> P<Expr> {
        let mut lhs = self.atom();
        let rhs = self.atom();
        lhs = Box::new(Expr { kind: EK::Bin(43, lhs, rhs) });
        lhs
    }
}
fn eval(e: &Expr) -> i64 { match &e.kind { EK::Lit(n) => *n, EK::Bin(_o, a, b) => eval(a) + eval(b) } }
fn main() { let q = Q; let e = q.build(); println!("{}", eval(&e)); }
