fn adder(x:i64)->impl Fn(i64)->i64{ move |y| x+y }
fn main(){ let a=adder(10); let b=adder(100); println!("{}", a(5)+b(7)); }
