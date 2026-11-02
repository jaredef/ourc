use std::collections::HashMap;
fn main() { let mut h: HashMap<String, i32> = HashMap::new(); h.insert("b".into(), 2); let rem = h.remove("b"); println!("{:?} {}", rem, h.len()); }
