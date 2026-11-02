use std::collections::HashSet;
fn main() { let a: HashSet<i32> = [1, 2].into_iter().collect(); let s: HashSet<i32> = [2].into_iter().collect(); println!("{} {}", s.is_subset(&a), a.contains(&2)); }
