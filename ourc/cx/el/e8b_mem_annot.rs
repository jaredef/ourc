use std::mem;
fn main() { let mut a = vec![1, 2]; let old: Vec<i32> = mem::replace(&mut a, vec![7]); println!("{:?} {:?}", a, old); let t: Vec<i32> = std::mem::take(&mut a); println!("{:?}", t); let mut s = String::from("x"); let o = mem::take(&mut s); println!("[{}] [{}]", s, o); }
