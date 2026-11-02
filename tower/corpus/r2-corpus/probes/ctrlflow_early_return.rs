fn classify(n: i32) -> i32 {
    if n < 0 { return -1; }
    if n == 0 { return 0; }
    1
}
fn main() { println!("{}", classify(-5) + classify(0) + classify(9)); }
