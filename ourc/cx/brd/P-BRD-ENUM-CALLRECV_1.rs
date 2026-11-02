fn r() -> Vec<String> { vec!["x".to_string()] }
fn main() { for (i, s) in r().iter().enumerate() { println!("{}: {}", i, s); } }
