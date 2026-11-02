use std::collections::HashMap;
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)] struct Symbol(u32);
fn main() { let mut m: HashMap<String, Symbol> = HashMap::new(); m.insert("a".to_string(), Symbol(3));
  let x = if let Some(&s) = m.get("a") { s } else { Symbol(99) }; let y = m.get("a").copied(); let z = *m.get("a").unwrap(); println!("{:?} {:?} {:?}", x, y, z); }
