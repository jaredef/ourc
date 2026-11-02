fn main(){ let a:Option<i32>=Some(3); let c=a.map(|x|x+1); println!("{}",c.unwrap_or(0)); }
