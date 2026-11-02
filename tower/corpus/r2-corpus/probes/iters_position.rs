fn main(){let v=[5,6,7,8];let p=v.iter().position(|&x|x==7).unwrap();println!("{}",p);}
