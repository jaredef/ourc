fn dbl(x: i32) -> i32 { x * 2 }
fn main() { let a = 10; let b = dbl(a); std::process::exit(a + b); }   // 10 + 20 = 30
