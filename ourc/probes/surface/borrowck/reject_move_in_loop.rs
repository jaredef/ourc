struct P { x: i64 } fn f(p: P) -> i64 { p.x } fn main() { let p = P { x: 1 }; for _ in 0..2 { f(p); } }
