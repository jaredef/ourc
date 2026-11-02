use std::collections::{HashSet, VecDeque};
#[derive(Clone, Debug, PartialEq, Eq, Hash)] struct Inst { def: String, args: Vec<String> }
fn main() { let mut seen: HashSet<Inst> = HashSet::new(); let a = Inst { def: "f".into(), args: vec!["T".into()] };
  println!("{}", seen.insert(a.clone())); println!("{}", seen.insert(a.clone())); println!("{}", seen.insert(Inst { def: "f".into(), args: vec!["U".into()] })); println!("{}", seen.len());
  let mut wl: VecDeque<Inst> = VecDeque::new(); wl.push_back(a.clone()); while let Some(i) = wl.pop_front() { println!("{} {}", i.def, i.args.join(",")); } }
