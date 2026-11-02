use std::collections::HashMap;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] enum Q { A, B }
fn main() { let mut m: HashMap<(String, u32), i64> = HashMap::new(); m.insert(("a".to_string(), 2), 10); m.insert(("a".to_string(), 3), 20); m.insert(("a".to_string(), 2), 30); println!("{:?} {:?} {}", m.get(&("a".to_string(), 2)), m.get(&("a".to_string(), 3)), m.len()); }
