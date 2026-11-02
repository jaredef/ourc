fn a() -> Result<i64, std::num::ParseFloatError> { let f: f64 = "1.5".parse()?; Ok(f as i64 + 1) }
fn main() { let g: f64 = "2.5".parse().unwrap(); println!("{} {:?}", g, a()); }
