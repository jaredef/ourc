fn evens()->impl Iterator<Item=i32>{ (0..10).filter(|x| x%2==0) }
fn main(){ let s:i32=evens().sum(); println!("{}", s); }
