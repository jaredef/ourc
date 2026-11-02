fn main(){ let r='outer: loop { let mut i=0i64; loop { i+=1; if i*i>20 { break 'outer i; } } }; println!("{}", r); }
