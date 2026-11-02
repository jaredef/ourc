fn main() { let v = vec![1, 5, 9]; println!("{} {} {}", v.contains(&5), v.iter().any(|x| *x > 8), v.iter().all(|x| *x > 0)); }
