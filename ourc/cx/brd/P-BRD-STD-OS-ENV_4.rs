use std::env;
fn main() { let d = env::current_dir().unwrap(); println!("[{}] {}", d.display(), d.is_absolute()); }
