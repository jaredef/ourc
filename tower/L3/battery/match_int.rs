fn describe(n: i32) -> i32 { match n { 0 => 100, 1 => 200, 2 => 300, _ => n * 1000 } }
fn main() { std::process::exit(describe(2) + describe(0)); }   // 300 + 100 = 400
