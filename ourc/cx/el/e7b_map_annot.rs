use std::collections::HashMap;
#[derive(Clone, Copy, PartialEq, Debug)] struct Sym(u32);
#[derive(Clone, Copy, PartialEq, Debug)] struct P { x: i64 }
fn main() { let mut m: HashMap<String, Sym> = HashMap::new(); m.insert("a".to_string(), Sym(3)); let g: Option<&Sym> = m.get("a"); println!("{:?}", g); let s: Sym = *m.get("a").unwrap(); println!("{:?} {}", s, s.0);
  let mut q: HashMap<i64, P> = HashMap::new(); q.insert(1, P { x: 9 }); println!("{:?} {:?}", q.get(&1), q[&1]); let v: Vec<Sym> = m.values().cloned().collect(); println!("{:?}", v); }
