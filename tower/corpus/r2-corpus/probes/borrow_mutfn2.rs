fn addto(x:&mut i64,y:i64){*x+=y;}
fn main(){let mut a=10;addto(&mut a,3);addto(&mut a,4);println!("{}",a);}
