fn age(n: &str) -> Result<u32, String> { if n.len() == 3 { Ok(n.len() as u32 * 10) } else { Err(format!("bad {}", n)) } }
fn main() { let bad: Result<Vec<u32>, String> = ["ann", "ze", "bob"].iter().map(|n| age(n)).collect(); println!("{:?}", bad); }
