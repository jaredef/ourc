fn f(b: &[u8]) -> u8 { let mut i = 0; while i < b.len() { if b[i] == 65 { return b[i]; } i += 1; } 0 }
fn main() {}
