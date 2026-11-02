fn find(v: &[i64], t: i64) -> Option<usize> { for (i, &x) in v.iter().enumerate() { if x == t { return Some(i); } } None } fn main() { println!("{:?} {:?}", find(&[4, 5, 6], 6), find(&[1], 9)); }
