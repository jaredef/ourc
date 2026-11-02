fn main() { let s = String::from("hello world"); let w: Vec<&str> = s.split_whitespace().collect(); println!("{} {}", w.len(), w[1]); }
