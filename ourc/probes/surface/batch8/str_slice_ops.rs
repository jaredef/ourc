fn main() { let s = "hello, world"; let a = &s[0..5]; let b = &s[7..]; let c = s.split(", ").collect::<Vec<_>>(); println!("{} {} {:?} {} {}", a, b, c, s.starts_with("hell"), s.replace("l", "L")); }
