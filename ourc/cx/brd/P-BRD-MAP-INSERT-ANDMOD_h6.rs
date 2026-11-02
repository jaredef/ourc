use std::collections::HashMap;
fn main() { let mut h: HashMap<String, i32> = HashMap::new(); h.insert("a".into(), 1); let old = h.insert("a".into(), 10); let o2: Option<i32> = h.insert("a".into(), 11); let r: Option<i32> = h.remove("zz"); println!("{} {:?} {:?} {}", old.unwrap(), o2, r, h.insert("q".into(), 1).is_none()); }
