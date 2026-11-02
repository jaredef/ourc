fn age(n: &str) -> Result<u32, String> { Ok(n.len() as u32) }
fn main() { let v: Vec<Result<u32, String>> = ["ann", "bo"].iter().map(|n| age(n)).collect(); println!("{:?}", v); let w = ["ann", "bo"].iter().map(|n| age(n)).collect::<Vec<Result<u32, String>>>(); println!("{:?}", w); }
