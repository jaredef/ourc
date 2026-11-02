use std::collections::HashSet;
fn main() { let a: HashSet<i32> = [1, 2, 3].into_iter().collect(); let b: HashSet<i32> = [2, 3, 5].iter().copied().collect(); let i: HashSet<i32> = a.intersection(&b).copied().collect(); let mut v: Vec<i32> = i.into_iter().collect(); v.sort(); println!("{:?}", v); }
