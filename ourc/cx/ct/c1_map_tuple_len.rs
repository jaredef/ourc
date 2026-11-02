fn main() { let v: Vec<(String, usize)> = vec![("ab".to_string(), 1), ("xyz".to_string(), 2)]; let s: usize = v.iter().map(|(a, b)| a.len() + b).sum(); println!("{}", s); }
