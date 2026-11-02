fn sumto(n: i64) -> i64 { if n == 0 { 0 } else { n + sumto(n - 1) } }
fn main() { println!("{}", sumto(100)); }
