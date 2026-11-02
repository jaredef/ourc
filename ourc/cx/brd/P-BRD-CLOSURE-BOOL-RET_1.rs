fn lookup(id: u32) -> Option<u32> { if id < 3 { Some(id * 10) } else { None } }
fn main() { let f = |i| lookup(i).map(|a| a >= 10).unwrap_or(false); println!("{} {}", f(1), f(2)); }
