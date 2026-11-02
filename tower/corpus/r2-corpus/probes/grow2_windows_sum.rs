fn main(){ let v=vec![1i64,2,3,4]; let mut s=0i64; for w in v.windows(2) { s+=w[0]*w[1]; } println!("{}", s); }
