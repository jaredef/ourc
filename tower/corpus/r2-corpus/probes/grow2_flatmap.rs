fn main(){ let v=vec![1i64,2,3]; let s:i64=v.iter().flat_map(|&x| vec![x,x*10]).sum(); println!("{}", s); }
