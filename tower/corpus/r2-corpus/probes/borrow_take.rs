fn main(){let mut a=8;let old=std::mem::take(&mut a);println!("{}",old*10+a);}
