struct P { x: i64 } fn f(p: P) -> i64 { p.x } fn main() { let p = P { x: 1 }; let a = f(p); let b = f(p); }
