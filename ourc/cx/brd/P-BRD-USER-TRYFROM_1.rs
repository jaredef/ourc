use std::convert::{TryFrom, TryInto};
#[derive(Debug)] struct Even(u32);
impl TryFrom<u32> for Even { type Error = String; fn try_from(x: u32) -> Result<Self, String> { if x % 2 == 0 { Ok(Even(x)) } else { Err(format!("{} odd", x)) } } }
fn main() { println!("{:?} {:?}", Even::try_from(4), Even::try_from(5)); let r: Result<Even, _> = 10u32.try_into(); println!("{:?}", r); }
