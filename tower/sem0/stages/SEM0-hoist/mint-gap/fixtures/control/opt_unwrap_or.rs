fn main(){ let a:Option<i32>=Some(3); let b:Option<i32>=None; println!("{} {}",a.unwrap_or(9),b.unwrap_or(9)); }
