// R28b: `s.chars().map(|c| transform).collect::<String>()` -- transform each char, build a String. Was
// unhandled (chars() supported count/rev/nth/filter-count only) -> the collect returned a count/garbage.
fn main() {
    let a: String = "hello".chars().map(|c| c.to_ascii_uppercase()).collect();
    let b: String = "WORLD".chars().map(|c| c.to_ascii_lowercase()).collect();
    let c: String = "abc".chars().map(|c| c).collect();
    println!("{} {} {}", a, b, c);
}
