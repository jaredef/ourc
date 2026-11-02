// three places of three different types, carried back together
fn tri(a:&mut i64,s:&mut String,b:&mut bool){ *a+=5; s.push_str("z"); *b=true; }
fn main(){ let mut n=1; let mut t=String::from("t"); let mut f=false; tri(&mut n,&mut t,&mut f); println!("{} {} {}",n,t,f); }
