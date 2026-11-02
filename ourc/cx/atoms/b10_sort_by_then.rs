fn main() { let mut v = vec![("b", 3), ("a", 9), ("c", 1), ("a", 2)]; v.sort_by(|x, y| x.0.cmp(y.0)); println!("{:?}", v);
  let mut w = vec![("b", 3), ("a", 9), ("a", 2)]; w.sort_by(|x, y| x.0.cmp(y.0).then(y.1.cmp(&x.1))); println!("{:?}", w);
  let mut u = vec![3, 1, 2]; u.sort_by(|a, b| b.cmp(a)); println!("{:?}", u); }
