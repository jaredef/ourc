fn main() { let s = "a1 B"; let d: Vec<bool> = s.chars().map(|c| c.is_ascii_digit()).collect(); println!("{:?} {} {}", d, 'x'.is_lowercase(), ('a' as u8 + 2) as char); }
