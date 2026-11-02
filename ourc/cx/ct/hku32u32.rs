use std::collections::HashMap;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] enum Q { A, B }
fn main() { let mut m: HashMap<(u32, u32), i64> = HashMap::new(); m.insert((1, 2), 10); m.insert((1, 3), 20); m.insert((1, 2), 30); println!("{:?} {:?} {}", m.get(&(1, 2)), m.get(&(1, 3)), m.len()); }
