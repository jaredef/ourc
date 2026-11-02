#[derive(Debug, Clone, Copy, PartialEq)] enum Res { Local(u32), Err }
fn main() { let ribs: Vec<Vec<(String, Res)>> = vec![vec![("x".into(), Res::Local(3))]]; for rib in ribs.iter() { println!("{}", rib.len()); let f = rib.iter().find(|(k, _)| k == "x"); println!("{}", f.is_some()); if let Some((k, r)) = f { println!("{} {:?}", k, r); } } }
