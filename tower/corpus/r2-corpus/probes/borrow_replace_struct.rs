struct P{x:i64}
fn main(){let mut p=P{x:5};let o=std::mem::replace(&mut p,P{x:9});println!("{}",o.x*10+p.x);}
