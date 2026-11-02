fn classify(n: i64) -> i64 {
    match n {
        0 => return 100,
        x if x < 0 => return -1,
        _ => {}
    }
    n * 2
}
fn main() {
    println!("{}", classify(0) + classify(-5) + classify(7));
}
