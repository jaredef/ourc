fn main() { let v = vec!["aaa", "b", "cc"]; println!("{:?} {:?}", v.iter().max_by_key(|s| s.len()), v.iter().min_by_key(|s| s.len())); }
