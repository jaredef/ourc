fn main() { let s = "Hello, World"; println!("{} {} {:?} {:?}", s.to_lowercase(), s.len(), s.find("World"), s.char_indices().nth(4)); }
