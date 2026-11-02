// P-STR: the str surface our C-string model expresses EXACTLY. `chars()` is absent on purpose -- a byte
// walk diverges on non-ASCII -- and `&s[a..b]` is a SUBSTRING (it used to reach the vector slicer and
// segfault). Strings order lexicographically.
fn main() {
    let s = "hello world";
    println!("{} {} {}", s.len(), s.is_empty(), "".is_empty());
    println!("{} {:?} {:?}", s.contains("lo w"), s.find("world"), s.find("zz"));
    println!("{} {} {} {}", s.starts_with("he"), s.starts_with("x"), s.ends_with("rld"), s.ends_with("x"));
    println!("{} {}", "Hello".to_uppercase(), "Hello".to_lowercase());
    println!("[{}] [{}] [{}]", "  hi  ".trim(), "  hi  ".trim_start(), "  hi  ".trim_end());
    println!("{} {}", "ab".repeat(3), "aXa".replace("X", "-"));
    println!("{} {}", &s[1..3], &s[..5]);
    println!("{} {} {}", "abc" == "abd", "abc" < "abd", "abc".to_string() == "abc");
}
