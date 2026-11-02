fn main(){ let mut n=0i64; let mut inc=|| { n+=1; n }; let a=inc(); let b=inc(); let c=inc(); println!("{}", a+b+c); }
