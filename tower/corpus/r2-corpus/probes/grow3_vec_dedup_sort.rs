fn main(){ let mut v=vec![3i64,1,2,3,1,4,2]; v.sort(); v.dedup(); let mut s=0i64; for x in &v { s=s*10+x; } println!("{}", s); }
