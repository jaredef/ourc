type P<T> = Box<T>;
type ThinVec<T> = Vec<T>;
enum ExprKind {
    Lit(i64),
    Str(String),
    Bin(u8, P<Expr>, P<Expr>),
    MethodCall(P<Expr>, String, ThinVec<P<Expr>>),
}
struct Expr { kind: ExprKind }
fn dump(e: &Expr) -> String {
    match &e.kind {
        ExprKind::Lit(n) => format!("{}", n),
        ExprKind::Str(s) => format!("\"{}\"", s),
        ExprKind::Bin(op, a, b) => format!("({} {} {})", *op as char, dump(a), dump(b)),
        ExprKind::MethodCall(recv, name, args) => {
            let mut out = String::new();
            out.push_str("(.");
            out.push_str(name);
            out.push_str(" ");
            out.push_str(&dump(recv));
            for a in args.iter() { out.push_str(" "); out.push_str(&dump(a)); }
            out.push_str(")");
            out
        }
    }
}
fn lit(n: i64) -> P<Expr> { Box::new(Expr { kind: ExprKind::Lit(n) }) }
fn main() {
    let mut args: ThinVec<P<Expr>> = Vec::new();
    args.push(lit(2)); args.push(lit(3));
    let mc = Expr { kind: ExprKind::MethodCall(lit(1), String::from("add"), args) };
    println!("{}", dump(&mc));
    let sx = Expr { kind: ExprKind::Str(String::from("hi")) };
    println!("{}", dump(&sx));
}
