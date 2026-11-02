#[repr(u8)]
enum K { A = 1, B = 2, C = 3 }
fn name(k: &K) -> i64 { match k { K::A => 10, K::B => 20, K::C => 30 } }
fn main() { println!("{}", name(&K::B)); }
