fn main() { let v: Vec<(String, u32)> = vec![("a".into(), 1), ("b".into(), 2)]; let n = "b"; let ns = String::from("b");
  println!("{:?} {:?} {:?} {:?}", v.iter().position(|(k, _)| k == "b"), v.iter().position(|(k, _)| *k == ns), v.iter().position(|(k, _)| k == n), v.iter().position(|t| t.0 == n));
  let w: Vec<String> = vec!["a".into(), "b".into()]; println!("{:?} {:?}", w.iter().position(|k| k == n), w.iter().any(|k| k == n)); }
