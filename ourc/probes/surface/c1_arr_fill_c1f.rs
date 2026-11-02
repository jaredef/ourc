fn main() { let a = [7u8; 1_000_000]; let s: u64 = a.iter().map(|&x| x as u64).sum(); println!("{}", s); }
