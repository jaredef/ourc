fn main() { let v: Vec<(String, usize)> = vec![("ab".to_string(), 1), ("x".to_string(), 2)]; for (p, b) in v.iter() { println!("{}={} {}", p, b, p == "x"); } }
