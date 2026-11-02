use std::collections::BTreeSet;
fn main() { let set: BTreeSet<char> = "mississippi".chars().collect(); println!("{:?} first={:?} n={}", set, set.iter().next(), set.len()); }
