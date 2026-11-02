use std::collections::HashSet;
#[derive(Clone, Debug, PartialEq, Eq, Hash)] struct Inst { def: String, args: Vec<String> }
fn mk() -> Vec<Inst> { vec![Inst { def: "a".into(), args: vec![] }] }
fn main() { let v = mk(); let c = v[0].clone(); println!("{:?}", c); let mut s: HashSet<Inst> = HashSet::new(); s.insert(Inst { def: "x".into(), args: vec!["i".into()] }); println!("{}", s.insert(c.clone())); println!("{}", s.insert(c)); }
