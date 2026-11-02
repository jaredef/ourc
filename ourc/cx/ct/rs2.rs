use std::collections::HashMap;
#[derive(Debug, Clone, Copy, PartialEq)] enum Res { Def(u32), Err }
struct Module { items: HashMap<String, Res> }
fn main() { let mut mods = vec![Module { items: HashMap::new() }]; mods[0].items.insert("b".to_string(), Res::Def(1)); println!("{}", mods[0].items.len());
  let g = mods[0].items.get("b"); println!("{:?}", g); if let Some(r) = mods[0].items.get("b") { println!("{:?}", *r); } }
