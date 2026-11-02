fn classify(n:i32)->i32{ match n { x if x<0 => -1, 0..=9 => 0, _ => 1 } }
fn main(){ println!("{} {} {}", classify(-3), classify(5), classify(50)); }
