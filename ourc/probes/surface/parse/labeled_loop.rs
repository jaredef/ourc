fn main() { let mut n = 0; 'o: loop { loop { n += 1; if n > 3 { break 'o; } } } println!("{}", n); }
