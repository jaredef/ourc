fn inc(x:&mut i64){*x+=5;}
fn main(){let mut a=4;inc(&mut a);println!("{}",a);}
