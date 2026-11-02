fn main() { let mut i = 0; let v = loop { i += 1; if i == 5 { break i * 2; } }; println!("{}", v); }
