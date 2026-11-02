fn main(){ let s="a,b,c"; let mut n=0; for p in s.split(','){ n+=p.len(); } println!("{}",n); }
