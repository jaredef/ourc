fn fib(n: u64) -> u64 { fn go(a: u64, b: u64, k: u64) -> u64 { if k == 0 { a } else { go(b, a + b, k - 1) } } go(0, 1, n) } fn main() { println!("{} {}", fib(10), fib(50)); }
