fn main() { let mut v = vec![1, 2]; let first = &v[0]; println!("{}", first); v.push(3); let r = &v; let n = r.len(); v.push(n as i64); println!("{:?}", v); }
