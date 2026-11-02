fn main() { let mut count = 0; let mut inc = |k: i64| { count += k; count }; inc(2); inc(3); println!("{}", inc(0)); }
