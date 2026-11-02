fn main() { let c = 'a'; let d = (c as u8 + 2) as char; let up = c.to_ascii_uppercase(); println!("{} {} {} {}", d, up, 'z' as u32 - 'a' as u32, c.is_alphabetic()); }
