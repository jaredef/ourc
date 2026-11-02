fn main(){ let x=3i64; let f=|a:i64| { let g=|b:i64| a*b+x; g(4) }; println!("{}", f(5)); }
