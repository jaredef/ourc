fn main(){let a=[1,2,3];let b=[10,20,30];let s:i64=a.iter().zip(b.iter()).map(|(x,y)|x*y).sum();println!("{}",s);}
