fn ap(f: fn(i64) -> i64, v: i64) -> i64 { f(v) } fn twice(f: fn(i64) -> i64, v: i64) -> i64 { f(f(v)) } fn main() { println!("{} {}", ap(|v| v + 1, 4), twice(|v| v * 3, 2)); }
