use std::collections::HashMap;
#[derive(Clone, Copy, PartialEq, Debug)] struct Sym(u32);
fn main() { let mut m: HashMap<String, Sym> = HashMap::new(); m.insert("a".to_string(), Sym(3)); let g = m.get("a"); println!("{:?}", g); if let Some(&s) = m.get("a") { println!("{:?}", s); } let c = m.get("a").copied(); println!("{:?}", c); }
