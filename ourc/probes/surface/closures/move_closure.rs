fn main() { let v = vec![1, 2, 3]; let f = move || v.len(); println!("{}", f()); }
