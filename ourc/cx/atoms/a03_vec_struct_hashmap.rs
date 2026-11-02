use std::collections::HashMap;
struct Module { parent: Option<usize>, items: HashMap<String, u32> }
fn main() { let mut mods: Vec<Module> = Vec::new(); mods.push(Module { parent: None, items: HashMap::new() }); mods.push(Module { parent: Some(0), items: HashMap::new() });
  mods[0].items.insert("a".to_string(), 7); mods[1].items.insert("b".to_string(), 9);
  println!("{:?} {:?} {:?}", mods[1].parent, mods[0].items.get("a"), mods[1].items.get("a")); }
