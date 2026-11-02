#[derive(Clone)] struct P { x: i64 } fn f(p: P) -> i64 { p.x } fn main() { let p = P { x: 21 }; let q = p.clone(); println!("{}", f(p) + f(q)); }
