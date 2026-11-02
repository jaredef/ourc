fn with<F: Fn(&str) -> String>(s: &str, f: F) -> String { f(s) }
fn main() { println!("{} {}", with("dog", |n: &str| n.repeat(2)), with("ab", |n: &str| n.to_uppercase())); }
