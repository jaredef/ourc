use std::collections::BTreeSet;
fn main() { let mut a: BTreeSet<char> = BTreeSet::new(); for c in "mississippi".chars() { a.insert(c); } println!("{:?} {}", a, a.len());
  let b: BTreeSet<u32> = vec![3, 1, 3, 2].into_iter().collect(); println!("{:?}", b); let c: BTreeSet<char> = "abca".chars().collect(); println!("{:?} {:?}", c, c.iter().next()); }
