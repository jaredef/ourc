#[derive(Debug)]
enum Node { Num(i64), Var(String), Add(Box<Node>, Box<Node>), Neg(Box<Node>) }
fn eval(n: &Node) -> i64 {
    match n {
        Node::Num(v) => *v,
        Node::Var(_s) => 10,
        Node::Add(a, b) => eval(a) + eval(b),
        Node::Neg(a) => -eval(a),
    }
}
fn main() {
    let e = Node::Add(Box::new(Node::Num(3)), Box::new(Node::Neg(Box::new(Node::Var(String::from("x"))))));
    println!("{}", eval(&e));
    println!("{:?}", e);
}
