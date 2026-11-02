use std::collections::HashMap;
#[derive(Debug, Clone, Copy, PartialEq)] enum DefKind { Fn, Mod }
#[derive(Debug, Clone, Copy, PartialEq)] enum Res { Def(DefKind, u32), Err }
struct Module { parent: Option<usize>, name: String, items: HashMap<String, Res>, globs: Vec<usize> }
struct R { mods: Vec<Module>, next: u32 }
impl R { fn add(&mut self, parent: Option<usize>, name: &str) -> usize { let id = self.mods.len(); self.mods.push(Module { parent, name: name.to_string(), items: HashMap::new(), globs: vec![] }); if let Some(p) = parent { self.mods[p].items.insert(name.to_string(), Res::Def(DefKind::Mod, id as u32)); } id }
  fn look(&self, m: usize, n: &str) -> Option<Res> { if let Some(r) = self.mods[m].items.get(n) { return Some(*r); } None } }
fn main() { let mut r = R { mods: vec![], next: 0 }; let a = r.add(None, "crate"); let b = r.add(Some(a), "b"); println!("{} {} {:?} {:?} {}", a, b, r.look(a, "b"), r.look(a, "z"), r.mods[b].name); }
