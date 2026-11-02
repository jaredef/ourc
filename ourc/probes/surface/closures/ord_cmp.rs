use std::cmp::Ordering; fn main() { let o = 3.cmp(&5); let p = 7.cmp(&7); println!("{} {}", o == Ordering::Less, p == Ordering::Equal); }
