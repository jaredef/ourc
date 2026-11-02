fn main() { let mut u = vec![3, 1, 2]; u.sort_by(|a, b| b.cmp(a)); println!("{:?}", u);
  let mut v = vec![("b", 3), ("a", 9), ("c", 1)]; v.sort_by(|x, y| x.0.cmp(y.0)); println!("{:?}", v);
  let mut w = vec![(2, 3), (1, 9), (2, 1)]; w.sort_by(|x, y| x.0.cmp(&y.0).then(y.1.cmp(&x.1))); println!("{:?}", w);
  let mut s = vec!["bb".to_string(), "a".to_string()]; s.sort_by(|a, b| a.len().cmp(&b.len())); println!("{:?}", s); }
