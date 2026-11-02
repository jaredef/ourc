struct Node<'hir> { kind: i32, name: &'hir str }
impl<'hir> Node<'hir> {
    fn new(kind: i32, name: &'hir str) -> Node<'hir> { Node { kind: kind, name: name } }
    fn tag(&self) -> &'hir str { self.name }
    fn is_expr(&self) -> bool { self.kind == 1 }
}
fn first<'a>(xs: &'a [i32]) -> &'a i32 { &xs[0] }
fn main() {
    let n = Node::new(1, "expr");
    println!("{} {}", n.tag(), n.is_expr() as i32);
    let v = vec![10, 20]; println!("{}", *first(&v));
}
