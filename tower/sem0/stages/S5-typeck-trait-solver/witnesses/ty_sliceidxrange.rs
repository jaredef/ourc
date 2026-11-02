fn f(t: &str) -> bool { let x = &t[0..12]; true }
fn g(b: &[u8]) -> u8 { let s = &b[2..5]; s[0] }
fn h(v: &[u8], a: usize, b: usize) -> u8 { let s = &v[a..b]; s[0] }
fn main(){}
