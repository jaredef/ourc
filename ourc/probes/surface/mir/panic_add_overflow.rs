fn main() { let a: i32 = 2147483647; let b = a + std::env::args().count() as i32; println!("{}", b); }
