type P<T> = Box<T>;
struct Node { v: i32 }
fn f(p: P<Node>) -> P<Node> { p }
fn main() {}
