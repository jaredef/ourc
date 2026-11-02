use std::collections::HashMap; fn main() { let mut m = HashMap::new(); m.insert(10, 100); m.insert(20, 200); println!("{} {}", m[&20], m.get(&30).is_none()); }
