use std::collections::HashMap;
fn main() { let mut m: HashMap<String, i64> = HashMap::new(); for w in "x y x z x".split(' ') { *m.entry(w.to_string()).or_insert(0) += 1; }
  let mut n: HashMap<i64, i64> = HashMap::new(); let v = vec![3, 1, 3]; for i in 0..v.len() { *n.entry(v[i] * 10).or_insert(5) += 2; }
  println!("{:?} {:?} {:?} {} {:?} {:?}", m.get("x"), m.get("y"), m.get("q"), m.len(), n.get(&30), n.get(&10)); }
