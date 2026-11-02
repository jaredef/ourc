fn main(){ let v=vec![1,2,3,4]; let s=v.iter().fold(0, |a,x| a+x); println!("{}",s); }
