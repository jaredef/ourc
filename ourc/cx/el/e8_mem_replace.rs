use std::mem;
fn main() { let mut a = vec![1, 2]; let old = mem::replace(&mut a, vec![7]); println!("{:?} {:?}", a, old); let t = mem::take(&mut a); println!("{:?} {:?}", a, t); }
