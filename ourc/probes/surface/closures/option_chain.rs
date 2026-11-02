fn main() { let v = vec![1, 2, 3]; let r = v.iter().position(|&x| x == 2).map(|i| i * 10).unwrap_or(99); println!("{}", r); }
