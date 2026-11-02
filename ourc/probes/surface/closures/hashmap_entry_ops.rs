use std::collections::HashMap; fn main() { let mut m = HashMap::new(); m.insert(1, 5); *m.entry(1).or_insert(100) *= 2; *m.entry(2).or_insert(100) -= 1; println!("{} {}", m[&1], m[&2]); }
