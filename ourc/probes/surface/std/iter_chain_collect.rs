fn main() { let a = vec![1, 2]; let b = vec![3]; let c: Vec<i64> = a.iter().chain(b.iter()).map(|x| x * 2).collect(); println!("{:?}", c); }
