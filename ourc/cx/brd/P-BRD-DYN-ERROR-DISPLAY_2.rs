use std::fmt;
#[derive(Debug)] struct E(i32);
impl fmt::Display for E { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "e{}", self.0) } }
impl std::error::Error for E {}
fn inner() -> Result<i32, E> { Err(E(5)) }
fn outer() -> Result<i32, Box<dyn std::error::Error>> { let x = inner()?; Ok(x) }
fn main() { println!("{}", outer().unwrap_err()); }
