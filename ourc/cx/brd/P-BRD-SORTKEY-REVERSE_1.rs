use std::cmp::Reverse;
fn main() { let mut rev = vec!["pear", "fig", "banana"]; rev.sort_by_key(|w| Reverse(*w)); println!("{:?}", rev); }
