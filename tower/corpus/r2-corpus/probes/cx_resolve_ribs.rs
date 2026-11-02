// P-CX-RESOLVE witness (ourc construct-generation, 2026-10-24): resolver-shaped lookups over enum-valued maps and
// rib stacks -- `if let Some(r) = map.get(k) { *r }` / `.map(|r| *r)` over Option<&Res>, `Some((_, r))` from find
// over Vec<(String, Res)> (direct, through a local, and on a loop binder over Vec<Vec<..>>).
use std::collections::HashMap;
#[derive(Debug, Clone, Copy, PartialEq)] enum Res { Local(u32), Def(u32), Err }
fn rib_lookup(ribs: &Vec<Vec<(String, Res)>>, n: &str) -> Res { for rib in ribs.iter().rev() { if let Some((_, r)) = rib.iter().rev().find(|(k, _)| k == n) { return *r; } } Res::Err }
fn main() {
    let mut m: HashMap<String, Res> = HashMap::new(); m.insert("b".to_string(), Res::Def(1));
    if let Some(r) = m.get("b") { println!("{:?}", *r); } println!("{:?} {:?}", m.get("b").map(|r| *r), m.get("z").map(|r| *r));
    let rib: Vec<(String, Res)> = vec![("x".into(), Res::Local(1)), ("y".into(), Res::Local(3))];
    let f = rib.iter().find(|(k, _)| k == "y"); if let Some((k, r)) = f { println!("{} {:?}", k, r); }
    let ribs = vec![vec![("x".to_string(), Res::Local(1)), ("main".to_string(), Res::Local(2))], vec![("x".to_string(), Res::Local(3))]];
    println!("{:?} {:?} {:?}", rib_lookup(&ribs, "x"), rib_lookup(&ribs, "main"), rib_lookup(&ribs, "q"));
}
