fn main() { let v: Vec<(String, u32)> = vec![("a".into(), 1), ("b".into(), 2)];
  let r = v.iter().find(|t| t.1 == 2); println!("{}", r.is_some());
  if let Some(t) = r { println!("{}", t.1); }
  let r2: Option<&(String, u32)> = v.iter().find(|t| t.1 == 2); if let Some(t) = r2 { println!("{} {}", t.0, t.1); }
  let w: Vec<(u32, u32)> = vec![(1, 2), (3, 4)]; let q = w.iter().find(|p| p.0 == 3); if let Some(p) = q { println!("{} {}", p.0, p.1); } }
