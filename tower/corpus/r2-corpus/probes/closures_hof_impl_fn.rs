fn apply(f: impl Fn(i64) -> i64, x: i64) -> i64 { f(x) }
fn main() {
    println!("{}", apply(|n| n * 2, 21));
}
