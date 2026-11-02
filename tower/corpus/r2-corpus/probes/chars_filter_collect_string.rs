// R29b: `s.chars().filter(|c| pred).collect::<String>()` -- keep chars where pred holds, build a String. A
// collect-terminated lookahead distinguishes it from `filter(..).count()` (which falls through unchanged).
fn main() {
    let a: String = "a1b2c3".chars().filter(|c| c.is_alphabetic()).collect();
    let b: String = "a b c d".chars().filter(|c| c.is_alphanumeric()).collect();
    println!("{} {}", a, b);
}
