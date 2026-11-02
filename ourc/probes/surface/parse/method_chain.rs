fn main() { let v = vec![3, 1, 2]; let s: i64 = v.iter().map(|x| x * 2).sum(); println!("{}", s); }
