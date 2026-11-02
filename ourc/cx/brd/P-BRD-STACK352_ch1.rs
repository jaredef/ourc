fn main() { let v = vec![1, 2]; let w: Vec<i32> = v.iter().cloned().chain(std::iter::once(9)).collect(); println!("{:?}", w); }
