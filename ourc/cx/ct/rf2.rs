fn a(e: &Vec<(u32, u32)>, node: u32) -> bool { e.iter().filter(|(p, _)| *p == node).all(|(_, c)| *c > 0) }
fn k(x: u32) -> bool { x > 0 }
fn b(e: &Vec<(u32, u32)>, node: u32) -> bool { e.iter().filter(|(p, _)| *p == node).all(|(_, c)| k(*c)) }
fn h(e: &Vec<(u32, u32)>, node: u32, d: u32) -> bool { if d > 3 { return false; } e.iter().filter(|(p, _)| *p == node).all(|(_, c)| h(e, *c, d + 1)) }
fn main() { let f = vec![(1u32, 2u32), (2, 3)]; println!("{}", a(&f, 1)); println!("{}", b(&f, 1)); println!("{}", h(&f, 1, 0)); }
