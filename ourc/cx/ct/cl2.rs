fn main() { let a: Vec<String> = vec!["T".into()]; let x = a.get(0).cloned().unwrap_or_default(); let y = a.get(3).cloned().unwrap_or_default(); println!("[{}] [{}]", x, y); }
