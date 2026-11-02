use std::collections::HashMap;
fn main() { let mut h: HashMap<String, i32> = HashMap::new(); h.insert("a".into(), 1); let old = h.insert("a".into(), 10); println!("{:?} {:?}", old, h.get("a")); }
