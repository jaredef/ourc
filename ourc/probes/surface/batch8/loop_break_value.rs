fn main() { let mut i = 0; let r = loop { i += 3; if i > 10 { break i * 2; } }; let found = (1..20).find(|x| x * x > 50); println!("{} {:?}", r, found); }
