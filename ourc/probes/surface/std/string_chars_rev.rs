fn main() { let s = "hello"; let r: String = s.chars().rev().collect(); println!("{} {}", r, s.starts_with("he")); }
