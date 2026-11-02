fn main() { let v: Vec<(String, u32)> = vec![("a".into(), 1), ("b".into(), 2)];
  if let Some((_, r)) = v.iter().find(|t| t.1 == 2) { println!("r={}", r); } else { println!("none"); } }
