#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)] enum Level { Low, Mid(u8), High { v: i32 } }
fn main() { let ls = vec![Level::Mid(5), Level::Low]; println!("{:?}", ls.iter().max()); }
