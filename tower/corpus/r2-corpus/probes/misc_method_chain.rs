fn main() { let v: i64 = (1..=5).filter(|x| x % 2 == 1).map(|x| x * x).sum(); println!("{}", v); }
