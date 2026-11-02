fn twice(f: impl Fn(i64) -> i64, x: i64) -> i64 { f(f(x)) }
fn main() {
    println!("{}", twice(|n| n + 1, 40));
}
