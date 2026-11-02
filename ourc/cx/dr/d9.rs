fn p() -> Result<Vec<String>, String> { let mut out = Vec::new(); out.push("n".to_string()); Ok(out) }
fn q() -> Result<Vec<String>, String> { let mut out: Vec<String> = Vec::new(); out.push("n".to_string()); Ok(out) }
fn r(b: bool) -> Result<u32, String> { if b { return Err(format!("bad `{}`", 1)); } Ok(2) }
fn main() { println!("{:?}", q()); println!("{:?}", p()); println!("{:?} {:?}", r(false), r(true)); }
