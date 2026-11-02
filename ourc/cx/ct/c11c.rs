struct M { items: Vec<(String, u32)> }
fn a(ms: &[M], n: &str) -> Option<u32> { ms[0].items.iter().find(|(k, _)| k == n).map(|(_, r)| *r) }
fn b(ms: &[M], n: &str) -> Option<u32> { if let Some((_, r)) = ms[0].items.iter().find(|(k, _)| k == n) { return Some(*r); } None }
fn c(ms: &Vec<M>, n: &str) -> Option<u32> { if let Some((_, r)) = ms[0].items.iter().find(|(k, _)| k == n) { return Some(*r); } None }
fn main() { let ms = vec![M { items: vec![("a".into(), 1), ("b".into(), 2)] }]; println!("{:?}", c(&ms, "b")); }
