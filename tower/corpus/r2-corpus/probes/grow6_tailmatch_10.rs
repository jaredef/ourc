fn sign(n: i64) -> i64 {
    match n {
        0 => 0,
        x if x > 0 => 1,
        _ => -1,
    }
}
fn main() {
    println!("{}", sign(5) * 100 + sign(0) * 10 + sign(-3));
}
