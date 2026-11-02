fn mkv() -> Vec<i64> { vec![1,2,3] }
fn suml(v: &[i64]) -> i64 { let mut s=0; for &x in v { s+=x; } s }
fn main() { println!("{}", suml(&mkv())); }
