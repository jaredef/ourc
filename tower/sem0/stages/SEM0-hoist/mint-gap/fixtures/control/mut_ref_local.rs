fn main(){ let mut a=5; { let r=&mut a; *r+=10; } println!("{}",a); }
