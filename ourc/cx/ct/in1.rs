use std::collections::HashMap;
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)] struct Symbol(u32);
struct I { map: HashMap<String, Symbol>, strs: Vec<String> }
impl I { fn intern(&mut self, s: &str) -> Symbol { if let Some(&sym) = self.map.get(s) { return sym; } let sym = Symbol(self.strs.len() as u32); self.strs.push(s.to_string()); self.map.insert(s.to_string(), sym); sym } }
fn main() { let mut i = I { map: HashMap::new(), strs: Vec::new() }; let a = i.intern("x"); let b = i.intern("y"); let c = i.intern("x"); println!("{:?} {:?} {:?} {}", a, b, c, i.strs.len()); }
