struct P { x: i64 } impl P { fn new(x: i64) -> P { P { x } } } fn main() { println!("{}", P::new(42).x); }
