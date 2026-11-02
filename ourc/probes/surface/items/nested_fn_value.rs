fn ap(f: fn(i64) -> i64, v: i64) -> i64 { f(v) } fn main() { fn dbl(x: i64) -> i64 { x * 2 } let g = dbl; println!("{} {} {}", dbl(3), ap(dbl, 4), g(5)); }
