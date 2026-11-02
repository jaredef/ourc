use std::env;
fn main() { println!("{} {}", env::args().count() >= 1, env::current_dir().unwrap().is_absolute()); }
