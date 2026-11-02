#[derive(Clone, Copy)] struct P { x: i64 } fn f(p: P) -> i64 { p.x } fn main() { let p = P { x: 21 }; println!("{}", f(p) + f(p)); }
