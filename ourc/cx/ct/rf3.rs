fn h(e: &Vec<(u32, u32)>, node: u32, d: u32) -> bool { if d > 3 { return false; } println!("h{} {}", d, node); e.iter().filter(|(p, _)| *p == node).all(|(_, c)| h(e, *c, d + 1)) }
fn h2(e: &Vec<(u32, u32)>, node: u32, d: u32) -> bool { println!("x{}", d); if d > 3 { return false; } let r = e.iter().filter(|(p, _)| *p == node).all(|(_, c)| h2(e, *c, d + 1)); r }
fn main() { let f = vec![(1u32, 2u32), (2, 3)]; println!("{}", h(&f, 1, 0)); println!("{}", h2(&f, 1, 0)); }
