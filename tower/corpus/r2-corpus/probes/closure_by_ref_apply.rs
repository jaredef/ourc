fn apply<F: Fn(i64) -> i64>(v: i64, f: &F) -> i64 { f(v) }
fn main() { println!("{}", apply(5, &|x| x * 10)); }
