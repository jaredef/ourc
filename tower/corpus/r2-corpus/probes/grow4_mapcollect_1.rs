fn main(){ let v:Vec<i64>=(1..=5).map(|x|x*x).collect::<Vec<_>>(); let s:i64=v.iter().sum(); println!("{}",s); }
