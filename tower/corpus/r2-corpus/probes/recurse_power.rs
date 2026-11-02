fn pow(b: i64, e: i64) -> i64 { if e == 0 { 1 } else { b * pow(b, e - 1) } }
fn main() { println!("{}", pow(2, 10)); }
