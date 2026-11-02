fn main() { let mut a = 1; let mut b = 2; let t = (b, a); a = t.0; b = t.1; println!("{}", a * 10 + b); }
