fn f(s: &str) -> String { let mut i = 0; while i < s.len() { i += 1; } let sub = &s[i..]; sub.to_string() }
fn main() {}
