enum Tree { Node(i64, Box<Tree>, Box<Tree>), Leaf }
fn sum(t: &Tree) -> i64 { match t { Tree::Node(v, l, r) => v + sum(l) + sum(r), Tree::Leaf => 0 } }
fn main() {
    let t = Tree::Node(1, Box::new(Tree::Node(2, Box::new(Tree::Leaf), Box::new(Tree::Leaf))), Box::new(Tree::Node(3, Box::new(Tree::Leaf), Box::new(Tree::Leaf))));
    println!("{}", sum(&t));
}
