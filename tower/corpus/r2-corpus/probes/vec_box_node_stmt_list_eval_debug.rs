#[derive(Debug)]
enum Node { Num(i64), Add(Box<Node>, Box<Node>) }
fn eval(n: &Node) -> i64 { match n { Node::Num(v) => *v, Node::Add(a, b) => eval(a) + eval(b) } }
fn main() {
    let mut stmts: Vec<Box<Node>> = Vec::new();
    stmts.push(Box::new(Node::Num(3)));
    stmts.push(Box::new(Node::Add(Box::new(Node::Num(4)), Box::new(Node::Num(5)))));
    let mut total = 0;
    for s in &stmts { total += eval(s); }
    println!("{} {}", total, stmts.len());
    println!("{:?}", stmts[1]);
}
