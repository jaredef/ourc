fn aa(s: &str, t: &str) -> bool { true }
fn bb(s: &str, n: usize, t: &str) -> bool { true }
fn cc(b: &[u8], i: usize) -> i32 { 0 }
fn use_all() -> i32 { if aa("x","y") { bb("x",0,"y") } else { false }; cc(b"z", 0) }
fn main() {}
