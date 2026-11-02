use std::collections::HashMap;
fn main() { let mut m = HashMap::new(); let e = m.is_empty(); m.insert(5, 50); m.insert(1, 10); m.insert(3, 30);
  let mut k: Vec<i64> = m.keys().cloned().collect(); k.sort();
  let mut s: HashMap<&str, i64> = HashMap::new(); s.insert("b", 1); s.insert("a", 2); let mut sk: Vec<&str> = s.keys().cloned().collect(); sk.sort();
  println!("{} {} {:?} {:?} {:?} {:?}", e, m.is_empty(), k, m.get(&5), sk, s.get("b")); }
