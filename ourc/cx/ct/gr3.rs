#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] enum Q { T, S, L }
fn cnt(e: &Vec<((Q, u32), (Q, u32))>, node: (Q, u32)) -> usize { e.iter().filter(|(p, _)| *p == node).count() }
fn al(e: &Vec<((Q, u32), (Q, u32))>, node: (Q, u32)) -> bool { e.iter().filter(|(p, _)| *p == node).all(|(_, c)| c.1 > 0) }
fn main() { let e = vec![((Q::L, 1), (Q::S, 1)), ((Q::S, 1), (Q::T, 1))]; println!("{} {} {}", cnt(&e, (Q::L, 1)), cnt(&e, (Q::T, 9)), al(&e, (Q::T, 9))); }
