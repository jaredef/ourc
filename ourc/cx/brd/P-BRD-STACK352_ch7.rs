fn main() { let v = vec![1, 2]; let s: i32 = v.iter().cloned().chain(std::iter::once(9)).sum(); println!("{}", s); }
