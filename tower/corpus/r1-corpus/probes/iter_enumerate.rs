fn main(){ let v=vec![10,20,30]; let mut t=0; for (i,x) in v.iter().enumerate() { t+=i as i32*x; } println!("{}",t); }
