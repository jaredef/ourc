fn main(){ let s:i32=(1..=5).map(|x| x*x).filter(|x| x%2==1).sum(); println!("{}", s); }
