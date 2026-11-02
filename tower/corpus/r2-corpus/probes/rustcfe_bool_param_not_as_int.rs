fn t(b: bool) -> i32 { let c = !b; c as i32 }
fn main() { println!("{}", t(true)); println!("{}", t(false)); }
