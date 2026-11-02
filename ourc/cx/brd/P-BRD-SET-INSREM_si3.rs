use std::collections::HashSet;
fn main() { let mut hs: HashSet<&str> = HashSet::new(); let t = (hs.insert("x"), hs.insert("x")); println!("{:?}", t); }
