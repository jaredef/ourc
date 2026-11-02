fn main(){ let mut v=vec![1i64,2,3,4,5]; v.swap(0,4); v.swap(1,3); let mut s=0i64; for x in &v { s=s*10+x; } println!("{}", s); }
