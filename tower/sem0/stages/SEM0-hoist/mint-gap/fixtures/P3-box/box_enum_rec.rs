enum E{ Leaf(i32), Node(Box<E>,Box<E>) }
fn sum(e:&E)->i32{ match e { E::Leaf(v)=>*v, E::Node(a,b)=>sum(a)+sum(b) } }
fn main(){ let t=E::Node(Box::new(E::Leaf(3)),Box::new(E::Leaf(4))); println!("{}",sum(&t)); }
