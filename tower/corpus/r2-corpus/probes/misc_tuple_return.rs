fn mm(a: i64, b: i64) -> (i64, i64) { if a < b { (a, b) } else { (b, a) } }
fn main() { let (lo, hi) = mm(7, 3); println!("{}", hi * 10 + lo); }
