fn main() { let s = format!("{:?} {:?} {:?} {:?} {:?}", "a\"b\\c", x, ', 3, 2.0); println!("{} {}", s, s.len()); }
