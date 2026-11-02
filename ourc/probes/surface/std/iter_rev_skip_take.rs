fn main() { let v: Vec<i64> = (1..=10).rev().skip(2).take(3).collect(); println!("{:?}", v); }
