type P<T> = Box<T>;
type ThinVec<T> = Vec<T>;
enum EK { Lit(i64), Call(ThinVec<P<Expr>>) }
struct Expr { kind: EK }
fn tot(e: &Expr) -> i64 {
    match &e.kind { EK::Lit(n) => *n, EK::Call(args) => { let mut s = 0; for a in args.iter() { s = s + tot(a); } s } }
}
fn main() {
    let mut v: ThinVec<P<Expr>> = Vec::new();
    v.push(Box::new(Expr { kind: EK::Lit(10) })); v.push(Box::new(Expr { kind: EK::Lit(20) }));
    println!("{}", tot(&Expr { kind: EK::Call(v) }));
}
