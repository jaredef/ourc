struct M { items: Vec<(String, u32)> }
fn a(m: &M, n: &str) -> Option<u32> { m.items.iter().find(|(k, _)| k == n).map(|(_, r)| *r) }
fn b(ms: &[M], n: &str) -> bool { ms[0].items.iter().any(|(k, _)| k == n) }
fn c(ms: &[M], n: &str) -> Option<u32> { let it = &ms[0].items; it.iter().find(|(k, _)| k == n).map(|(_, r)| *r) }
fn main() { let ms = vec![M { items: vec![("a".into(), 1), ("b".into(), 2)] }]; println!("{:?} {} {:?}", a(&ms[0], "b"), b(&ms, "b"), c(&ms, "b")); }
