fn f(x: i64) -> i64 { if x > 0 { x } else { unreachable!() } } fn main() { println!("{}", f(3)); }
