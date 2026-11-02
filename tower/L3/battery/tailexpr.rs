fn sq(n: i32) -> i32 { n * n }             // tail expression (no return)
fn main() { std::process::exit(sq(6)); }   // 36
