use std::collections::{HashSet, VecDeque};
#[derive(Clone, Debug, PartialEq, Eq, Hash)] struct Inst { def: String, args: Vec<String> }
fn callees(i: &Inst) -> Vec<Inst> { if i.def == "main" { vec![Inst { def: "a".into(), args: vec![] }, Inst { def: "b".into(), args: vec!["T".into()] }] } else if i.def == "a" { vec![Inst { def: "b".into(), args: vec!["T".into()] }] } else { vec![] } }
fn main() { let mut seen: HashSet<Inst> = HashSet::new(); let mut order = Vec::new(); let mut wl = VecDeque::new(); wl.push_back(Inst { def: "main".into(), args: vec![] });
  while let Some(i) = wl.pop_front() { if seen.insert(i.clone()) { for c in callees(&i) { wl.push_back(c); } order.push(i); } }
  for i in &order { println!("{}<{}>", i.def, i.args.join(",")); } println!("{}", seen.len()); }
