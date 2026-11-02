use std::collections::HashMap;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] enum Q { A, B }
fn main() { let mut m: HashMap<(Q, u32), i64> = HashMap::new(); m.insert((Q::A, 1), 10); m.insert((Q::B, 1), 20); m.insert((Q::A, 1), 30);
  println!("{:?} {:?} {:?} {}", m.get(&(Q::A, 1)), m.get(&(Q::B, 1)), m.get(&(Q::B, 2)), m.len()); }
