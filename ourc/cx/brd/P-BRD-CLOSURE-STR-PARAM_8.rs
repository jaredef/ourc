fn ap<F: Fn(&str) -> String>(s: &str, f: F) -> String { f(s) }
fn main() { let r = ap("dog", |n| format!("{}!", n)); println!("{}", r); }
