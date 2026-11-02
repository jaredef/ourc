fn main() { let s = "aZ"; let v: Vec<u32> = s.chars().map(|c| c as u32).collect(); println!("{:?}", v); }
