fn main() { let mut s = String::new(); let mut add = |x: &str| s.push_str(x); add("ab"); add("cd"); println!("{}", s); }
