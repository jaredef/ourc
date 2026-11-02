struct M { items: Vec<(String, u32)>, globs: Vec<usize> }
fn look(ms: &Vec<M>, m: usize, n: &str, seen: &mut Vec<usize>) -> Option<u32> { if seen.contains(&m) { return None; } seen.push(m); for (k, v) in ms[m].items.iter() { if k == n { return Some(*v); } } for &g in &ms[m].globs { if let Some(r) = look(ms, g, n, seen) { return Some(r); } } None }
fn main() { let ms = vec![M { items: vec![("a".into(), 1)], globs: vec![1] }, M { items: vec![("b".into(), 2)], globs: vec![0] }]; println!("{:?} {:?}", look(&ms, 0, "b", &mut vec![]), look(&ms, 0, "z", &mut vec![])); }
