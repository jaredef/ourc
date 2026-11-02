fn main(){ let m=vec![vec![1,2],vec![3,4,5]]; let mut t=0; for row in &m { for x in row { t+=x; } } println!("{}",t); }
