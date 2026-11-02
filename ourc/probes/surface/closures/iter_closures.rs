fn main() { let v = vec![1, 2, 3, 4, 5]; let s: i64 = v.iter().filter(|x| **x % 2 == 1).map(|x| x * 10).sum(); println!("{}", s); }
