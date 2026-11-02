fn main() { let mut v = vec![1, 2]; { let r = &mut v; r.push(3); } println!("{}", v.len()); }
