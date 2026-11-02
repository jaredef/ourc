fn main() { let mut v: Vec<i64> = Vec::new(); { let mut add = |x: i64| { v.push(x); }; add(1); add(2); } println!("{} {}", v.len(), v[1]); }
