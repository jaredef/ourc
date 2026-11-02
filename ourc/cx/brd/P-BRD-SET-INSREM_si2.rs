use std::collections::HashSet;
fn main() { let mut hs: HashSet<&str> = HashSet::new(); hs.insert("x"); let r = hs.remove("x"); let r2 = hs.remove("x"); println!("{} {} {}", r, r2, hs.len()); }
