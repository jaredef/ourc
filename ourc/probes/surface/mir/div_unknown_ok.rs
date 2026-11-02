fn main() { let n = std::env::args().count() as i64; let b = if n > 5 { 0 } else { 2 }; println!("{}", 10 / b); }
