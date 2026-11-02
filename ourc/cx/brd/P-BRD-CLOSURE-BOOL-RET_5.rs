fn age(n: u32) -> Result<u32, String> { if n > 0 { Ok(n * 10) } else { Err(String::from("x")) } }
fn main() { let adult = |i| age(i).map(|a| a >= 18).unwrap_or(false); println!("{} {} {}", adult(1), adult(2), adult(0)); }
