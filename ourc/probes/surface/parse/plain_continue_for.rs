fn main() { let mut s = 0; for i in 0..10 { if i % 3 == 0 { continue; } s += i; } println!("{}", s); }
