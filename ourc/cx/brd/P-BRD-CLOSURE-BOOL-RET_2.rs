fn lookup(id: u32) -> Option<u32> { if id < 3 { Some(id * 10) } else { None } }
fn main() { let f = |i: u32| i > 3; println!("{} {}", f(1), f(5)); }
