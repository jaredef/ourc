type P<T> = Box<T>;
enum EK { Lit(i64), Call(Vec<P<Expr>>) }
struct Expr { kind: EK }
fn tot(e: &Expr) -> i64 {
    match &e.kind {
        EK::Lit(n) => *n,
        EK::Call(args) => { let mut s = 0; for a in args.iter() { s = s + tot(a); } s }
    }
}
fn main() {
    let mut v: Vec<P<Expr>> = Vec::new();
    v.push(Box::new(Expr { kind: EK::Lit(1) }));
    v.push(Box::new(Expr { kind: EK::Lit(2) }));
    let e = Expr { kind: EK::Call(v) };
    println!("{}", tot(&e));
}
