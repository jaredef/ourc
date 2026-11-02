#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)] struct P { x: i32, y: i32 }
fn main() { let a = P { x: 1, y: 2 }; let c = P { x: 0, y: 0 }; println!("{:?}", a.cmp(&c)); }
