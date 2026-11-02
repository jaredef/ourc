struct Node { val: i64, next: Option<Box<Node>> }
fn sum(n: &Node) -> i64 { match &n.next { Some(x) => n.val + sum(x), None => n.val } }
fn main() {
    let a = Node { val: 5, next: Some(Box::new(Node { val: 7, next: None })) };
    println!("{}", sum(&a));
}
