#[derive(Debug, Clone, Copy, PartialEq)] enum Q { A, B }
fn main() { let v: Vec<(Q, u32)> = vec![(Q::B, 2), (Q::A, 1)]; let mut keys: Vec<&(Q, u32)> = v.iter().collect(); keys.sort_by_key(|(q, k)| (*q as u8, *k)); println!("{:?} {:?}", keys[0], keys[keys.len() - 1]); let t = &v[0]; println!("{:?}", t); }
