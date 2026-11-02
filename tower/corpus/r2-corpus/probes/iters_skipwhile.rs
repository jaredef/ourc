fn main(){let s:i64=(1..10).skip_while(|&x|x<7).sum();println!("{}",s);}
