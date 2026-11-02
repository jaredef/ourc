fn main(){let mut a=5;let old=std::mem::replace(&mut a,9);println!("{}",old*10+a);}
