use std::collections::HashMap;
#[derive(Debug, Clone, Copy, PartialEq)] enum Res { Def(u32), Err }
fn main() { let mut m: HashMap<String, Res> = HashMap::new(); m.insert("b".to_string(), Res::Def(1)); if let Some(r) = m.get("b") { println!("{:?}", *r); } let o: Option<Res> = m.get("b").map(|r| *r); println!("{:?}", o); }
