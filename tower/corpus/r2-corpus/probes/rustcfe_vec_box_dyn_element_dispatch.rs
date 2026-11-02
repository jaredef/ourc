trait Named { fn tag(&self) -> i32; }
struct A; struct B;
impl Named for A { fn tag(&self) -> i32 { 1 } }
impl Named for B { fn tag(&self) -> i32 { 2 } }
fn main() { let v: Vec<Box<dyn Named>> = vec![Box::new(A), Box::new(B), Box::new(A)];
    let mut s = 0; let mut i = 0; while i < v.len() { s = s + v[i].tag(); i = i + 1; } println!("{}", s); }
