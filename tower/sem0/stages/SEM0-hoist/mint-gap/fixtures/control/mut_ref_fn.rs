fn bump(x:&mut i32){ *x += 10; }
fn main(){ let mut a=5; bump(&mut a); println!("{}",a); }
