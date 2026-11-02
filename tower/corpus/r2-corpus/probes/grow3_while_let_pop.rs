fn main(){ let mut v=vec![1i64,2,3,4,5]; let mut s=0i64; while let Some(x)=v.pop() { s=s*10+x; } println!("{}", s); }
