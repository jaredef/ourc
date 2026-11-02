enum E { A, B(u8), C { x: u8 } }
fn use_e(e: E) -> u8 { match e { E::A => 1, E::B(v) => v, E::C { x } => x } }
fn main() {}
