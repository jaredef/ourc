use std::collections::HashMap;
#[derive(Debug, Clone, PartialEq, Eq, Hash)] struct K { a: u32, b: u32 }
fn main() { let mut m: HashMap<K, i64> = HashMap::new(); m.insert(K { a: 1, b: 2 }, 10); m.insert(K { a: 1, b: 3 }, 20); m.insert(K { a: 1, b: 2 }, 30); println!("{} {:?}", m.len(), m.get(&K { a: 1, b: 3 })); }
