/// An AST node kind.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
#[repr(u8)]
enum NodeKind {
    /// a literal
    Lit,
    /// a binary op
    Bin,
    Unary,
}
#[inline]
#[must_use]
fn classify(k: &NodeKind) -> i32 { match k { NodeKind::Lit => 0, NodeKind::Bin => 1, NodeKind::Unary => 2 } }
fn main() {
    let k = NodeKind::Bin;
    println!("{}", classify(&k));
    println!("{:?}", k);
    println!("{}", (k == NodeKind::Bin) as i32);
}
