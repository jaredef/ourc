fn main() { let x: Option<i32> = None; println!("before"); let v = x.unwrap(); println!("{}", v); }
