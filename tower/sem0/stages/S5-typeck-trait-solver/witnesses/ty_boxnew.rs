struct Node { v: i32 }
fn f() -> Box<Node> { let b = Box::new(Node { v: 1 }); b }
fn g() -> Box<i32> { Box::new(5) }
fn main() {}
