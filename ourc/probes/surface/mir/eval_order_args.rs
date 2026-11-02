fn p(x: i64) -> i64 { println!("{}", x); x } fn add(a: i64, b: i64) -> i64 { a + b } fn main() { println!("{}", add(p(1), p(2))); }
