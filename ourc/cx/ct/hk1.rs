use std::collections::HashMap;
fn main() { let mut m: HashMap<(u32, u32), i64> = HashMap::new(); m.insert((1, 2), 10); println!("a {}", m.len()); m.insert((1, 3), 20); println!("b {}", m.len()); m.insert((1, 2), 30); println!("c {}", m.len()); println!("{:?}", m.get(&(1, 3))); }
