fn main() { let a: Vec<String> = vec!["T".into()]; let o = a.get(0).cloned(); println!("{:?}", o); let x = a.get(0).cloned().unwrap_or_default(); println!("[{}]", x); }
