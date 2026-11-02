fn main() { let mut s=String::from("a"); s.push(98 as u8 as char); s.push_str("cd"); println!("{} {}", s, s.len()); }
