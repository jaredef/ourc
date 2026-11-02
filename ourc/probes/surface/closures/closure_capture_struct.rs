struct P { x: i64 } fn main() { let p = P { x: 41 }; let g = || p.x + 1; println!("{}", g()); }
