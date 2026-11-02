use std::collections::HashMap;
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)] struct Symbol(u32);
struct I { map: HashMap<String, Symbol>, strs: Vec<String> }
impl I { fn intern(&mut self, s: &str) -> Symbol { if let Some(&sym) = self.map.get(s) { return sym; } let sym = Symbol(self.strs.len() as u32); self.strs.push(s.to_string()); self.map.insert(s.to_string(), sym); sym }
  fn get(&self, s: Symbol) -> &str { &self.strs[s.0 as usize] }
  fn prefill(init: &[&str]) -> Self { let mut i = I { map: HashMap::new(), strs: Vec::new() }; for s in init { i.intern(s); } i } }
fn main() { let mut i = I::prefill(&["", "fn"]); let syms: Vec<Symbol> = ["foo", "fn", "foo"].iter().map(|s| i.intern(s)).collect(); println!("{:?} {}", syms, i.strs.len());
  let mut j = I::prefill(&["a"]); let mut v = Vec::new(); for s in ["b", "a"].iter() { v.push(j.intern(s)); } println!("{:?}", v); }
