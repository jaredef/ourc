fn main(){ let m = vec![vec![1, 2], vec![3]]; let s: i64 = m.iter().map(|r| r.iter().sum::<i64>()).sum(); println!("{} {:?}", s, m); }
