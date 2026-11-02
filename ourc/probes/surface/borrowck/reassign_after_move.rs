struct P { x: i64 } fn f(p: P) -> i64 { p.x } fn main() { let mut p = P { x: 1 }; let a = f(p); p = P { x: 41 }; println!("{}", a + f(p)); }
