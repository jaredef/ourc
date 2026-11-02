fn ap<F: Fn(&str) -> String>(s: &str, f: F) -> String { let r = f(s); r }
fn main() { let r = ap("dog", |n| n.to_string()); println!("{}", r.len()); }
