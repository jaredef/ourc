fn half(n:i32)->Option<i32>{ if n%2==0 { Some(n/2) } else { None } }
fn go()->Option<i32>{ let a=half(8)?; let b=half(a)?; Some(a+b) }
fn main(){ println!("{}", go().unwrap()); }
