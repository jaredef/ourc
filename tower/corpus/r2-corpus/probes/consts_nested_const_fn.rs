const fn add(a: i64, b: i64) -> i64 { a + b }
const fn tri(n: i64) -> i64 { add(add(n, n), n) }
const V: i64 = tri(4);
fn main() { println!("{}", V); }
