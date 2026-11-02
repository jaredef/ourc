//! A rustc_hir-style HIR (High-level Intermediate Representation) model.
//!
//! This fixture mirrors the shape of `rustc_hir`: newtype id wrappers
//! (`HirId`, `ItemId`), a `Node`/`Expr`/`ExprKind` node hierarchy carrying
//! an arena lifetime `'hir`, a `Body` for executable code, and a `Map`
//! offering id-keyed lookups. References are `&'hir` throughout to model
//! arena allocation, and `impl<'hir> Map<'hir>` carries the arena lifetime
//! into every method that returns a borrowed node.

use std::collections::HashMap;

/// Stable per-node identifier, newtype over a raw index.
#[derive(Clone, Copy, PartialEq)]
pub struct HirId(pub u32);

/// Identifies an owner item within the crate.
#[derive(Clone, Copy)]
pub struct ItemId {
    pub owner: u32,
}

/// A body identifier referencing an executable HIR `Body`.
pub struct BodyId {
    pub hir_id: HirId,
}

/// The kind of an expression node, borrowed from the arena.
///
/// Each variant that recurses does so through an `&'hir Expr<'hir>`,
/// modelling how HIR children live in the same arena as their parent.
/// The lifetime parameter threads that arena through the whole tree.
pub enum ExprKind<'hir> {
    Lit(i64),
    Path(&'hir str),
    Call(&'hir Expr<'hir>, &'hir [Expr<'hir>]),
    Binary(&'hir Expr<'hir>, &'hir Expr<'hir>),
    Block(&'hir Block<'hir>),
}

/// An expression: a HirId plus its arena-borrowed kind.
pub struct Expr<'hir> {
    pub hir_id: HirId,
    pub kind: ExprKind<'hir>,
}

/// A lexical block: statements plus an optional trailing expression.
pub struct Block<'hir> {
    pub stmts: &'hir [Expr<'hir>],
    pub expr: Option<&'hir Expr<'hir>>,
}

/// An executable body: parameters plus a value expression.
pub struct Body<'hir> {
    pub params: &'hir [HirId],
    pub value: &'hir Expr<'hir>,
}

/// A HIR node, the union reachable through the map.
pub enum Node<'hir> {
    Expr(&'hir Expr<'hir>),
    Block(&'hir Block<'hir>),
    Body(&'hir Body<'hir>),
}

/// The HIR map: owns arena-borrowed lookups keyed by HirId.
///
/// Analogous to `rustc_middle`'s `hir::Map`, this indirection lets a
/// caller resolve a raw id back to the borrowed node or body it names
/// without walking the tree. The stored `&'hir` references outlive every
/// individual query because they point into the shared arena.
pub struct Map<'hir> {
    nodes: HashMap<u32, Node<'hir>>,
    bodies: HashMap<u32, &'hir Body<'hir>>,
}

impl<'hir> Map<'hir> {
    /// Construct an empty map.
    pub fn new() -> Map<'hir> {
        Map {
            nodes: HashMap::new(),
            bodies: HashMap::new(),
        }
    }

    /// Look up a node by its HirId, returning an arena reference.
    pub fn find(&self, id: HirId) -> Option<&Node<'hir>> {
        self.nodes.get(&id.0)
    }

    /// Resolve a body to its value expression.
    pub fn body_value(&self, id: BodyId) -> Option<&'hir Expr<'hir>> {
        match self.bodies.get(&id.hir_id.0) {
            Some(body) => Some(body.value),
            None => None,
        }
    }

    /// Classify an expression by its HIR node kind.
    ///
    /// The `<'a>` reborrow of `self` is independent of the arena lifetime
    /// `'hir`, matching how visitor methods borrow the map briefly while
    /// the nodes they inspect live for the whole arena.
    pub fn classify<'a>(&'a self, expr: &'hir Expr<'hir>) -> u32 {
        match &expr.kind {
            ExprKind::Lit(_) => 1,
            ExprKind::Path(_) => 2,
            ExprKind::Call(callee, _) => self.classify(callee),
            ExprKind::Binary(lhs, rhs) => self.classify(lhs) + self.classify(rhs),
            ExprKind::Block(_) => 5,
        }
    }
}
