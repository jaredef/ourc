fn main(){ let o:Option<i32>=Some(5); let r=o.map(|x| x+1).unwrap_or(0); println!("{}",r); }
