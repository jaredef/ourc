fn main() { let v = vec![1, 2, 3, 4, 5]; let w: Vec<i64> = v.windows(2).map(|p| p[0] + p[1]).collect(); let c: Vec<i64> = v.chunks(2).map(|p| p.iter().sum()).collect(); println!("{:?} {:?}", w, c); }
