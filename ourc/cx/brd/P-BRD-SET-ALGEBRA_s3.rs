use std::collections::HashSet;
fn main() { let a: HashSet<i32> = [1, 2].into_iter().collect(); let b: HashSet<i32> = [2, 3].into_iter().collect(); let u: HashSet<i32> = a.union(&b).copied().collect(); let d: HashSet<i32> = a.difference(&b).copied().collect(); println!("{} {} {}", u.len(), d.len(), d.contains(&1)); }
