fn main() { let s = String::from("a"); let r; { let t = String::from("b"); r = &t; } println!("{} {}", s, r); }
