fn main(){ let mut c=0; 'o: for i in 0..5 { for j in 0..5 { if i+j==4 { c=i*10+j; break 'o; } } } println!("{}",c); }
