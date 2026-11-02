fn main(){ let mut i=0; let x=loop { i+=1; if i*i>20 { break i; } }; println!("{}",x); }
