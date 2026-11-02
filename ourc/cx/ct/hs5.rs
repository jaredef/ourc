use std::collections::HashSet;
#[derive(Clone, Debug, PartialEq, Eq, Hash)] struct Inst { def: String, args: Vec<String> }
fn main() { let mut s: HashSet<Inst> = HashSet::new(); println!("{}", s.insert(Inst { def: "x".into(), args: vec!["i".into()] })); println!("{}", s.insert(Inst { def: "a".into(), args: vec![] })); }
