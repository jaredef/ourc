#[derive(Debug, Clone, Copy, PartialEq, Eq)] enum Q { T, S, L }
fn g(e: &Vec<((Q, u32), (Q, u32))>, node: (Q, u32), d: u32) -> bool { if d > 3 { return false; } let v: Vec<&((Q, u32), (Q, u32))> = e.iter().filter(|(p, _)| *p == node).collect(); println!("g{} {:?} n={}", d, node, v.len()); v.iter().all(|(_, c)| g(e, *c, d + 1)) }
fn h(e: &Vec<(u32, u32)>, node: u32, d: u32) -> bool { if d > 3 { return false; } println!("h{} {}", d, node); e.iter().filter(|(p, _)| *p == node).all(|(_, c)| h(e, *c, d + 1)) }
fn main() { let e = vec![((Q::L, 1), (Q::S, 1)), ((Q::S, 1), (Q::T, 1))]; println!("{}", g(&e, (Q::L, 1), 0)); let f = vec![(1u32, 2u32), (2, 3)]; println!("{}", h(&f, 1, 0)); }
