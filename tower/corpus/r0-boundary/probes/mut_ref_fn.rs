fn inc(x:&mut i32){ *x += 5; }
fn main(){ let mut a=10; inc(&mut a); println!("{}", a); }
