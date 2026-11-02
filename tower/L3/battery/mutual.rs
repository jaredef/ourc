fn is_even(n: i32) -> i32 { if n == 0 { return 1; } return is_odd(n - 1); }
fn is_odd(n: i32) -> i32 { if n == 0 { return 0; } return is_even(n - 1); }
fn main() { std::process::exit(is_even(10) + is_odd(7)); }   // 1 + 1 = 2
