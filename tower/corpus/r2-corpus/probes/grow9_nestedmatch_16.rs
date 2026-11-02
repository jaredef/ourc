enum Tree { Leaf(i64), Node(Box<Tree>, Box<Tree>) }
fn depth(t: &Tree) -> i64 {
    match t {
        Tree::Leaf(_) => 1,
        Tree::Node(l, r) => {
            let dl = depth(l);
            let dr = depth(r);
            1 + if dl > dr { dl } else { dr }
        }
    }
}
fn main() {
    let t = Tree::Node(
        Box::new(Tree::Node(Box::new(Tree::Leaf(1)), Box::new(Tree::Leaf(2)))),
        Box::new(Tree::Leaf(3)));
    println!("{}", depth(&t));
}
