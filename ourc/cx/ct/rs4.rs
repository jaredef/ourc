#[derive(Debug, Clone, Copy, PartialEq)] enum Res { Local(u32), Err }
fn main() { let ribs: Vec<Vec<(String, Res)>> = vec![vec![("x".into(), Res::Local(1))], vec![("x".into(), Res::Local(3))]]; let n = "x";
  for rib in ribs.iter().rev() { if let Some((_, r)) = rib.iter().rev().find(|(k, _)| k == n) { println!("{:?}", *r); break; } } }
