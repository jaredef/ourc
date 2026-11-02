fn adder(x:i64)->impl Fn(i64)->i64{ move |y| x+y }
fn main(){ let a5=adder(5); let a10=adder(10); println!("{}", a5(3)+a10(3)); }
