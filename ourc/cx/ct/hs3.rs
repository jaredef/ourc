use std::collections::HashSet;
fn main() { let mut s: HashSet<String> = HashSet::new(); for k in 0..12 { let f = s.insert(format!("d{}", k)); println!("{} {}", k, f); } println!("{}", s.len()); }
