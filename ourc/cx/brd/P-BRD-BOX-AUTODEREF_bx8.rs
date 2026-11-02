fn main() { let b = Box::new(vec![1, 2]); let s: i32 = b.iter().sum(); println!("{}", s); }
