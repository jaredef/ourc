fn main(){ let v=vec![1i64,2,3,4,5,6,7]; let mut s=0i64; for c in v.chunks(3) { s=s*100 + c.iter().sum::<i64>(); } println!("{}", s); }
