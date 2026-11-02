fn f(v: &Vec<(String, u32)>, n: &str) -> Option<u32> { if let Some((_, r)) = v.iter().find(|(k, _)| k == n) { return Some(*r); } None }
fn g(v: &Vec<(String, u32)>, n: &str) -> u32 { if let Some((_, r)) = v.iter().find(|(k, _)| k == n) { let x = *r; return x; } 0 }
fn h(v: &Vec<(String, u32)>, n: &str) -> u32 { if let Some((_, r)) = v.iter().find(|(k, _)| k == n) { println!("h {}", r); } 0 }
fn main() { let v: Vec<(String, u32)> = vec![("a".into(), 1), ("b".into(), 2)]; h(&v, "b"); println!("{}", g(&v, "b")); println!("{:?}", f(&v, "b")); }
