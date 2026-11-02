fn gcd(a: i32, b: i32) -> i32 { if b == 0 { a } else { gcd(b, a % b) } }
fn main() { std::process::exit(gcd(48, 36)); }