fn main() { let v: i64 = vec![1, 2, 3].iter().map(|x| x * 2).fold(0, |acc, x| acc + x); println!("{}", v); }
