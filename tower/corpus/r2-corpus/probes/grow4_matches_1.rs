fn main(){ let mut c=0; for n in 0..10 { if matches!(n, 2|4|6) { c+=1; } } println!("{}",c); }
