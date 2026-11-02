enum Tree{ Leaf(i64), Node(Box<Tree>,Box<Tree>) }
fn sum(t:&Tree)->i64{ match t { Tree::Leaf(v)=>*v, Tree::Node(l,r)=>sum(l)+sum(r) } }
fn main(){ let t=Tree::Node(Box::new(Tree::Leaf(3)),Box::new(Tree::Node(Box::new(Tree::Leaf(4)),Box::new(Tree::Leaf(5))))); println!("{}", sum(&t)); }
