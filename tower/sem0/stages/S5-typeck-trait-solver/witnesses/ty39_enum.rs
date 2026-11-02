enum E { A, B(u8), C { x: u8 } }
enum D { P = 1, Q = 2 }
fn use_e(e: E) -> u8 { match e { E::A => 1, E::B(_) => 2, E::C { .. } => 3 } }
fn main() {}
