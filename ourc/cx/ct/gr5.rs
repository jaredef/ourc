#[derive(Debug, Clone, Copy, PartialEq, Eq)] enum Q { T, S, L }
fn g(e: &Vec<((Q, u32), (Q, u32))>, node: (Q, u32), d: u32) -> bool { println!("g{} {:?}", d, node); if d > 3 { return false; } e.iter().filter(|(p, _)| *p == node).all(|(_, c)| { println!(" c{} {:?} node={:?}", d, c, node); g(e, *c, d + 1) }) }
fn main() { let e = vec![((Q::L, 1), (Q::S, 1)), ((Q::S, 1), (Q::T, 1))]; println!("{}", g(&e, (Q::L, 1), 0)); }
