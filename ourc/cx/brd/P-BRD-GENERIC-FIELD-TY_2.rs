struct S { v: Vec<char> }
struct G<T> { v: Vec<T> }
fn main() { let s = S { v: vec!['x'] }; let g: G<char> = G { v: vec!['y'] }; println!("{:?} {:?}", s.v, g.v); }
