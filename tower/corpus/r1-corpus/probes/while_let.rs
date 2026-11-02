fn main(){ let mut v=vec![1,2,3]; let mut t=0; while let Some(x)=v.pop() { t+=x; } println!("{}",t); }
