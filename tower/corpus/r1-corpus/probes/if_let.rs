fn main(){ let o:Option<i32>=Some(7); if let Some(x)=o { println!("{}",x*2); } else { println!("0"); } }
