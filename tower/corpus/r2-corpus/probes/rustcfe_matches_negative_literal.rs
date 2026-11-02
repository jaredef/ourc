fn is_op(t: i32) -> bool { matches!(t, -1 | -2 | -5) }
fn main() { println!("{} {} {}", is_op(-1) as i32, is_op(-2) as i32, is_op(3) as i32); }
