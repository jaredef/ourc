#[derive(Debug, Clone, PartialEq)] struct P { x: i64, y: i64 } fn main() { let a = P { x: 1, y: 2 }; let b = a.clone(); println!("{:?} {}", b, a == b); }
