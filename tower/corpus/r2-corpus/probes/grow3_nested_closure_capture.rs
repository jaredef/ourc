fn main(){ let base=100i64; let mut acc=0i64; let add=|x:i64|{ let f=|y:i64| base+x+y; f(1) }; for i in 0..4i64 { acc+=add(i); } println!("{}", acc); }
