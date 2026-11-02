pub enum ExprKind {
    Lit(u128),
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Unary(UnOp, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
    Path(Vec<String>),
    Tup(Vec<Expr>),
}

pub struct Expr {
    pub kind: ExprKind,
    pub span: (u32, u32),
}

pub enum BinOp { Add, Sub, Mul, Div }
pub enum UnOp { Neg, Not }

pub trait Visitor: Sized {
    fn visit_expr(&mut self, e: &Expr) {
        walk_expr(self, e);
    }
}

pub fn walk_expr<V: Visitor>(v: &mut V, e: &Expr) {
    match &e.kind {
        ExprKind::Binary(_, l, r) => {
            v.visit_expr(l);
            v.visit_expr(r);
        }
        ExprKind::Unary(_, x) => v.visit_expr(x),
        ExprKind::Call(f, args) => {
            v.visit_expr(f);
            for a in args {
                v.visit_expr(a);
            }
        }
        ExprKind::Tup(xs) => {
            for x in xs.iter() {
                v.visit_expr(x);
            }
        }
        _ => {}
    }
}
