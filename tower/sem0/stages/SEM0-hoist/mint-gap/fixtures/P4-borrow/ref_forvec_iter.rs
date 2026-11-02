fn main(){ let v:Vec<i64>=vec![1,2,3]; let mut s=0i64; for x in v.iter() { s+=*x; } println!("{}",s); }
