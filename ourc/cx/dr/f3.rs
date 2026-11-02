fn a(x: i64) -> i64 { x + 1 }
fn main() { let f: fn(i64) -> i64 = a; println!("{}", f(5)); }
