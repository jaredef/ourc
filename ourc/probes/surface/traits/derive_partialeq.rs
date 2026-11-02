#[derive(PartialEq, Clone, Copy, Debug)] struct P { x: i64 } fn main() { let a = P { x: 1 }; let b = a; println!("{} {:?}", a == b, a); }
