struct P { x: i64, y: i64 } fn main() { let a = P { x: 1, y: 2 }; let b = P { x: 5, ..a }; println!("{} {}", b.x, b.y); }
