fn apply<F: Fn(i64) -> i64>(f: F, x: i64) -> i64 { f(x) } fn main() { let k = 3; println!("{}", apply(|x| x * k, 5)); }
