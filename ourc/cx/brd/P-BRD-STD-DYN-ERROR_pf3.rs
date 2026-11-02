use std::error::Error;
fn d() -> Result<i64, Box<dyn Error>> { let f = "1.5".parse::<f64>()?; Ok(f as i64 + 1) }
fn main() { match d() { Ok(n) => println!("ok {}", n), Err(_) => println!("err") } }
