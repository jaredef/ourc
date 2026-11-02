fn main() { let s = "abc"; let u = s.to_uppercase(); let r: String = s.chars().rev().collect(); println!("{} {} {}", u, r, s.contains("b")); }
