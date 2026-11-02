use std::collections::HashMap;
fn main() { let mut m = HashMap::new(); m.insert(1, 10); m.insert(2, 20); m.insert(3, 30);
  let r2 = m.remove(&2); let r9 = m.remove(&9);
  let mut s: HashMap<&str, i64> = HashMap::new(); s.insert("a", 1); s.insert("b", 2); let ra = s.remove("a");
  println!("{:?} {:?} {} {:?} {:?} {:?} {:?} {} {:?}", r2, r9, m.len(), m.get(&1), m.get(&3), m.get(&2), ra, s.len(), s.get("b")); }
