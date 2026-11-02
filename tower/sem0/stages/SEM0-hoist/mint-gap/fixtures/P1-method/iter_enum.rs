fn main(){ let v=vec![5,6,7]; let mut t=0; for (i,x) in v.iter().enumerate(){ t+=i as i32 * x; } println!("{}",t); }
