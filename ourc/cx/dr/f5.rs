fn a(x: i64) -> i64 { x + 1 } fn b(x: i64) -> i64 { x * 2 }
fn main() { let fs: Vec<fn(i64) -> i64> = vec![a, b]; for f in &fs { println!("{}", f(5)); } }
