fn main() { let bb = Box::new(vec![1, 2]); let n = bb.len(); println!("{}", n); let v: &Vec<i32> = &bb; println!("{}", v.len()); }
