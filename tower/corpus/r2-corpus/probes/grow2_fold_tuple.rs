fn main(){ let v=vec![1i64,2,3,4]; let (s,p)=v.iter().fold((0i64,1i64),|(s,p),&x| (s+x,p*x)); println!("{}", s+p); }
