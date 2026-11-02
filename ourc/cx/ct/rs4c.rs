#[derive(Debug, Clone, Copy, PartialEq)] enum Res { Local(u32), Err }
fn main() { let rib: Vec<(String, Res)> = vec![("x".into(), Res::Local(1)), ("y".into(), Res::Local(3))]; if let Some((_, r)) = rib.iter().find(|(k, _)| k == "y") { println!("{:?}", r); println!("{:?}", *r); } }
