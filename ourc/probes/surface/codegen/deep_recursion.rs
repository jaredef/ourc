fn d(n: i64) -> i64 { if n == 0 { 0 } else { 1 + d(n - 1) } } fn main() { println!("{}", d(100000)); }
