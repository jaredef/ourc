#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct P { x: i32, y: i32 }
fn main() { let mut v = vec![P { x: 2, y: 1 }, P { x: 0, y: 5 }, P { x: 2, y: 0 }]; v.sort(); println!("{:?}", v); }
