use std::collections::HashSet;
fn main() { let mut hs = HashSet::new(); let a = hs.insert("x"); let b = hs.insert("x"); println!("{} {} {}", a, b, hs.len()); }
