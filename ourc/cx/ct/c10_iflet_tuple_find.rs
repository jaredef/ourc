fn main() { let v: Vec<(String, u32)> = vec![("a".into(), 1), ("b".into(), 2)]; let n = "b";
  if let Some((_, r)) = v.iter().find(|(k, _)| k == n) { println!("r={}", r); } else { println!("none"); }
  match v.iter().find(|(k, _)| k == n) { Some((k, r)) => println!("{} {}", k, r), None => println!("none") } }
