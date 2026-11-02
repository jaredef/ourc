fn fib(n: i32) -> i32 { if n < 2 { n } else { fib(n-1) + fib(n-2) } }
fn main() { let mut i = 0; while i < 10 { print!("{} ", fib(i)); i = i + 1; } println!(""); }
