// A reduced AST module modeled on rustc_ast node definitions.

pub type NodeId = u32;
pub type P<T> = Box<T>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mutability {
    Not = 0,
    Mut = 1,
}

#[derive(Clone, Debug)]
pub struct Expr {
    pub id: NodeId,
    pub kind: ExprKind,
}

#[derive(Clone, Debug)]
pub enum ExprKind {
    Lit(u64),
    Path(String),
    Binary(P<Expr>, P<Expr>),
    Call(P<Expr>, Vec<P<Expr>>),
    If(P<Expr>, P<Block>, Option<P<Expr>>),
    Match(P<Expr>, Vec<Arm>),
}

#[derive(Clone, Debug)]
pub struct Arm {
    pub pat: P<Pat>,
    pub body: P<Expr>,
}

#[derive(Clone, Debug)]
pub struct Block {
    pub id: NodeId,
    pub stmts: Vec<Stmt>,
}

#[derive(Clone, Debug)]
pub enum Stmt {
    Local(P<Pat>, Option<P<Expr>>),
    Expr(P<Expr>),
    Item(P<Item>),
}

#[derive(Clone, Debug)]
pub enum Pat {
    Wild,
    Ident(String),
    Box(P<Pat>),
    Tuple(Vec<P<Pat>>),
    TupleStruct(String, Vec<P<Pat>>),
    Struct(String, Vec<FieldPat>),
}

#[derive(Clone, Debug)]
pub struct FieldPat {
    pub name: String,
    pub pat: P<Pat>,
}

#[derive(Clone, Debug)]
pub enum TyKind {
    Path(String),
    Ref(P<TyKind>, Mutability),
    Slice(P<TyKind>),
}

#[derive(Clone, Debug)]
pub struct Item {
    pub id: NodeId,
    pub kind: ItemKind,
}

#[derive(Clone, Debug)]
pub enum ItemKind {
    Fn(Vec<Pat>, P<Block>),
    Struct(Vec<FieldDef>),
    Enum(Vec<Variant>),
}

#[derive(Clone, Debug)]
pub struct FieldDef {
    pub name: String,
    pub ty: P<TyKind>,
}

#[derive(Clone, Debug)]
pub struct Variant {
    pub name: String,
    pub disr: Option<u64>,
}

pub trait Visitor {
    fn visit_expr(&mut self, e: &Expr) {
        walk_expr(self, e);
    }
    fn visit_stmt(&mut self, s: &Stmt) {}
    fn visit_pat(&mut self, p: &Pat) {}
}

pub fn walk_expr<V: Visitor>(v: &mut V, e: &Expr) {
    match &e.kind {
        ExprKind::Lit(_) => {}
        ExprKind::Path(_) => {}
        ExprKind::Binary(lhs, rhs) => {
            v.visit_expr(lhs);
            v.visit_expr(rhs);
        }
        ExprKind::Call(callee, args) => {
            v.visit_expr(callee);
            for a in args {
                v.visit_expr(a);
            }
        }
        ExprKind::If(cond, _then, els) => {
            v.visit_expr(cond);
            if let Some(e) = els {
                v.visit_expr(e);
            }
        }
        ExprKind::Match(scrut, arms) => {
            v.visit_expr(scrut);
            for arm in arms {
                v.visit_pat(&arm.pat);
                v.visit_expr(&arm.body);
            }
        }
    }
}

