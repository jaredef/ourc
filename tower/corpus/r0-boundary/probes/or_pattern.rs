fn f(n:i32)->i32{ match n { 1|2|3 => 10, 4|5 => 20, _ => 30 } }
fn main(){ println!("{} {} {}", f(2), f(5), f(9)); }
