fn f(n:i32)->i32{ match n { x @ 1..=5 => x*10, _ => 0 } }
fn main(){ println!("{}", f(3)); }
