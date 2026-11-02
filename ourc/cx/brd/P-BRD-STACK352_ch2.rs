fn main() { let v = vec![1, 2]; let w: Vec<i32> = v.iter().copied().chain(vec![7, 8].into_iter()).collect(); println!("{:?}", w); }
