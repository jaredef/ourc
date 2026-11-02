fn p(args: &[&str]) -> Result<Vec<String>, String> { let mut out = Vec::new(); let mut it = args.iter(); while let Some(&a) = it.next() { match a { "-n" => out.push(it.next().ok_or("needs value")?.to_string()), o => return Err(format!("bad `{}`", o)) } } Ok(out) }
fn main() { println!("{:?} {:?} {:?}", p(&["-n", "x"]), p(&["-n"]), p(&["-z"])); }
