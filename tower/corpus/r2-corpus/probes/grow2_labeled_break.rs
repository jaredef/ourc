fn main(){ let mut c=0i64; 'outer: for i in 0..5i64 { for j in 0..5i64 { if i*j>6 { break 'outer; } c+=1; } } println!("{}", c); }
