type P<T> = Box<T>;
type ThinVec<T> = Vec<T>;
enum ExprKind { Lit(i64), Bin(u8, P<Expr>, P<Expr>) }
struct Expr { kind: ExprKind }
enum StmtKind { Let(u8, P<Expr>), Expr(P<Expr>) }
struct Stmt { kind: StmtKind }
struct Block { stmts: ThinVec<P<Stmt>> }
fn dump_expr(e: &Expr) -> String {
    match &e.kind {
        ExprKind::Lit(n) => format!("{}", n),
        ExprKind::Bin(op, a, b) => format!("({} {} {})", *op as char, dump_expr(a), dump_expr(b)),
    }
}
fn dump_stmt(s: &Stmt) -> String {
    match &s.kind {
        StmtKind::Let(name, e) => format!("(let {} {})", *name as char, dump_expr(e)),
        StmtKind::Expr(e) => format!("(expr {})", dump_expr(e)),
    }
}
fn dump_block(b: &Block) -> String {
    let mut s = String::new(); s.push_str("{");
    for st in b.stmts.iter() { s.push_str(" "); s.push_str(&dump_stmt(st)); }
    s.push_str(" }"); s
}
fn lit(n: i64) -> P<Expr> { Box::new(Expr { kind: ExprKind::Lit(n) }) }
fn main() {
    let mut stmts: ThinVec<P<Stmt>> = Vec::new();
    stmts.push(Box::new(Stmt { kind: StmtKind::Let(120, Box::new(Expr { kind: ExprKind::Bin(43, lit(1), lit(2)) })) }));
    stmts.push(Box::new(Stmt { kind: StmtKind::Expr(lit(9)) }));
    let b = Block { stmts: stmts };
    println!("{}", dump_block(&b));
}
