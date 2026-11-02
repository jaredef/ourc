fn count(n: i64) -> i64 { if n == 0 { 0 } else { 1 + count(n - 1) } }
fn main() { println!("{}", count(5000)); }
