use std::cmp::Reverse;
fn main() { let mut v = vec![3, 1, 2]; v.sort_by_key(|x| Reverse(*x)); println!("{:?}", v); let mut w = vec!["b", "c", "a"]; w.sort_by_key(|x| *x); println!("{:?}", w); let mut p = vec![(1, 5), (2, 3)]; p.sort_by_key(|t| Reverse(t.0)); println!("{:?}", p); }
