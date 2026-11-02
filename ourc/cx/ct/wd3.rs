fn f(w: &Vec<((u32, u32), (u32, u32))>, node: (u32, u32), k: u32) -> bool { w.iter().filter(|(p, _)| *p == node).all(|(_, c)| { println!("c={:?} k={}", c, k); c.0 == 3 }) }
fn main() { let w: Vec<((u32, u32), (u32, u32))> = vec![((1, 2), (3, 4))]; println!("{}", f(&w, (1, 2), 7)); }
