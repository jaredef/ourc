extern "C" fn f(x: usize) -> usize { x * 2 } pub extern "C" fn g(a: i64, b: i64) -> i64 { a - b } fn main() { println!("{} {}", f(22), g(9, 4)); }
