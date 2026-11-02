fn sum(s:&[i32])->i32{ let mut t=0; for x in s { t+=x; } t }
fn main(){ let v=vec![1,2,3,4,5]; println!("{}",sum(&v[1..4])); }
