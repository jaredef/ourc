fn apply<F: Fn(i64) -> i64>(f: F, x: i64) -> i64 { f(x) } fn main() { let k = 5; println!("{}", apply(|x| x * k, 8) + 2); }
