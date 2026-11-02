use std::collections::HashMap;
fn main() { let mut h: HashMap<String, i32> = HashMap::new(); h.insert("a".into(), 1); h.entry("a".into()).and_modify(|v| *v += 5).or_insert(0); println!("{:?}", h.get("a")); }
