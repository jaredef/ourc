fn lookup(id: u32) -> Option<&'static str> { match id { 1 => Some("ann"), 3 => Some(""), _ => None } }
fn main() { let x = lookup(1).filter(|s| !s.is_empty()).unwrap(); println!("{}", x); }
