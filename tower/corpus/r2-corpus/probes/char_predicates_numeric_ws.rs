// LAYER2: char::is_numeric / is_ascii_digit / is_whitespace predicates (ASCII), mirroring the existing
// is_alphabetic/is_alphanumeric. Enables the canonical chars().filter(|c| c.is_numeric()).collect() idiom.
fn main() {
    let digits: String = "a1b2c3".chars().filter(|c| c.is_numeric()).collect();
    println!("{}", digits);
    let words: String = "hi there world".chars().filter(|c| !c.is_whitespace()).collect();
    println!("{}", words);
    let n5 = (53 as u8) as char;
    println!("{} {} {}", n5.is_numeric(), n5.is_ascii_digit(), n5.is_whitespace());
}
