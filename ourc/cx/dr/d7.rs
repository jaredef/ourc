fn p(args: &[&str]) -> Result<Vec<String>, String> { let mut out = Vec::new(); for a in args.iter() { match *a { "-n" => out.push("n".to_string()), o => return Err(format!("bad `{}`", o)) } } Ok(out) }
fn main() { println!("{:?} {:?}", p(&["-n"]), p(&["-z"])); }
