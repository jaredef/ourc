fn go(args: &Vec<i64>) -> i64 { let mut s=0; for a in args.iter() { s = s + a; } s }
fn main() { let mut v: Vec<i64> = Vec::new(); v.push(10); v.push(20); println!("{}", go(&v)); }
