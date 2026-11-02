fn f(n: i32) -> i32 { match n { 0 => 100, 1..=3 => n*10, _ => -1 } }
fn main() { println!("{} {} {}", f(0), f(2), f(9)); }
