fn p(args: &[&str]) -> Result<String, String> { let mut it = args.iter(); let v = it.next().ok_or("none")?; Ok(v.to_string()) }
fn main() { println!("{:?} {:?}", p(&["a"]), p(&[])); }
