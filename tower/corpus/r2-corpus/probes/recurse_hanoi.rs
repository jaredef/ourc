fn moves(n: i64) -> i64 { if n == 0 { 0 } else { 2 * moves(n - 1) + 1 } }
fn main() { println!("{}", moves(10)); }
