fn main() { let mut v = vec![1, 2, 2, 3, 4, 4, 5]; v.dedup(); v.retain(|x| *x % 2 == 1); println!("{:?}", v); }
