fn main() { let v = vec![("ab".to_string(), 1usize), ("x".to_string(), 2)]; let s: Vec<String> = v.iter().map(|(p, b)| format!("{}={}", p, b)).collect(); println!("{:?}", s); }
