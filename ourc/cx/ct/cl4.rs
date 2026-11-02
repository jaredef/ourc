fn main() { let a: Vec<String> = vec!["T".into()]; let o = a.get(3).cloned(); println!("{:?}", o); let y = a.get(3).cloned().unwrap_or_default(); println!("[{}] {}", y, y.len()); }
