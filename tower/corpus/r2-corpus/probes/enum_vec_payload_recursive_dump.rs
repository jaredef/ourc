type P<T> = Box<T>;
enum EK { Lit(i64), Call(Vec<P<Expr>>) }
struct Expr { kind: EK }
fn dump(e: &Expr) -> String {
    match &e.kind {
        EK::Lit(n) => format!("{}", n),
        EK::Call(args) => {
            let mut s = String::new(); s.push_str("(call");
            for a in args.iter() { s.push_str(" "); s.push_str(&dump(a)); }
            s.push_str(")"); s
        }
    }
}
fn main() {
    let mut v: Vec<P<Expr>> = Vec::new();
    v.push(Box::new(Expr { kind: EK::Lit(1) }));
    v.push(Box::new(Expr { kind: EK::Lit(2) }));
    let e = Expr { kind: EK::Call(v) };
    println!("{}", dump(&e));
}
