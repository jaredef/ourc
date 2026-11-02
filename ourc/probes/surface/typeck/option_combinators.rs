fn main() { let o: Option<i64> = Some(20); println!("{}", o.map(|x| x * 2).unwrap_or(0) + 2); }
