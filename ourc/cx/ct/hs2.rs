use std::collections::HashSet;
#[derive(Clone, Debug, PartialEq, Eq, Hash)] struct Inst { def: String, args: Vec<String> }
fn main() { let mut s: HashSet<Inst> = HashSet::new(); for k in 0..12 { let f = s.insert(Inst { def: format!("d{}", k), args: vec![] }); println!("{} {}", k, f); } println!("{}", s.len()); }
