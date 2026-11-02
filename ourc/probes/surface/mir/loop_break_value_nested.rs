fn main() { let mut i = 0; let v = loop { i += 1; let w = loop { break i * 10; }; if w > 25 { break w; } }; println!("{}", v); }
