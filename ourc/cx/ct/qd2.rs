#[derive(Debug, Clone, Copy, PartialEq)] enum Q { A, B, C }
fn main() { let x = Q::C; println!("{}", x as u8); let r = &x; println!("{}", *r as u8); let v = vec![(Q::B, 2u32)]; for (q, k) in v.iter() { println!("{} {}", *q as u8, k); } let t: Vec<(u8, u32)> = v.iter().map(|(q, k)| (*q as u8, *k)).collect(); println!("{:?}", t); }
