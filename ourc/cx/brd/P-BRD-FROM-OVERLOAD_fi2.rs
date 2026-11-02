#[derive(Debug)] struct Meters(f64);
impl From<u32> for Meters { fn from(x: u32) -> Self { Meters(x as f64) } }
impl From<&str> for Meters { fn from(s: &str) -> Self { Meters(s.len() as f64) } }
fn to_meters<T: Into<Meters>>(x: T) -> f64 { x.into().0 }
fn main() { println!("{:?}", Meters::from("abcd")); }
