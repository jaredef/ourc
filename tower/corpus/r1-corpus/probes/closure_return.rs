fn adder(n:i32)->impl Fn(i32)->i32{ move |x| x+n }
fn main(){ let a=adder(10); println!("{}",a(5)+a(7)); }
