enum Tree { Node(Box<Tree>, Box<Tree>), Leaf }
fn depth(t: &Tree) -> i64 { match t { Tree::Node(l, r) => { let a = depth(l); let b = depth(r); 1 + if a > b { a } else { b } }, Tree::Leaf => 0 } }
fn main() {
    let t = Tree::Node(Box::new(Tree::Node(Box::new(Tree::Leaf), Box::new(Tree::Leaf))), Box::new(Tree::Leaf));
    println!("{}", depth(&t));
}
