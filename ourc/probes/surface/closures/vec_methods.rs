fn main() { let mut v = vec![5, 1, 4]; v.sort(); v.reverse(); v.dedup(); v.insert(1, 9); let last = v.pop(); println!("{:?} {:?} {}", v, last, v.iter().max().unwrap()); }
