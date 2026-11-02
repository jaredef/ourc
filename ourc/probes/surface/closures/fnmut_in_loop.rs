fn main() { let mut total = 0; let v = vec![1, 2, 3]; let mut add = |x: i64| total += x; for x in v.iter() { add(*x); } println!("{}", total); }
