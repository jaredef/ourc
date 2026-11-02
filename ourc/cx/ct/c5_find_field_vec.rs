struct M { items: Vec<(String, u32)> }
fn look(ms: &[M], n: &str) -> Option<u32> { if let Some((_, r)) = ms[0].items.iter().find(|(k, _)| k == n) { return Some(*r); } None }
fn main() { let ms = vec![M { items: vec![("a".into(), 1), ("b".into(), 2)] }]; println!("{:?} {:?}", look(&ms, "b"), look(&ms, "z")); }
