fn main(){ let o:Option<i32>=Some(41); let mut acc=0; if let Some(x)=&o { acc=acc+*x; } println!("{}",acc); }
