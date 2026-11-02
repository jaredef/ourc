use std::fmt;
#[derive(Debug)] struct E(i32);
impl fmt::Display for E { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "e{}", self.0) } }
impl std::error::Error for E {}
fn mk() -> Result<i32, Box<dyn std::error::Error>> { Err(Box::new(E(7))) }
fn main() { let b: Box<dyn std::error::Error> = Box::new(E(3)); println!("{}", b); println!("{}", mk().unwrap_err()); }
