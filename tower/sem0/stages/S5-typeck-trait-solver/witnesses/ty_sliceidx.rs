fn f(s: &str, a: usize, b: usize) -> String { let x = &s[a..b]; x.to_string() }
fn g(v: &[u8], a: usize) -> usize { let y = &v[a..]; y.len() }
fn main() {}
