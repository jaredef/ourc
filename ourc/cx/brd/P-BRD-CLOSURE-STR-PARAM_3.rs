fn with<F: Fn(&str) -> String>(s: &str, f: F) -> String { f(s) }
fn main() { println!("{} {}", with("dog", |n| n.repeat(2)), with("ab", |n| n.to_uppercase())); }
