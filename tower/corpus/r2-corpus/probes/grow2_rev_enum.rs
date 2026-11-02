fn main(){ let v=vec![10i64,20,30]; let mut s=0i64; for (i,x) in v.iter().rev().enumerate() { s+= (i as i64)*x; } println!("{}", s); }
