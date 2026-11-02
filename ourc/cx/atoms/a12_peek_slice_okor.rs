fn p(args: &[&str]) -> Result<Vec<String>, String> { let mut out = Vec::new(); let mut it = args.iter().peekable();
  while let Some(&a) = it.next() { match a { "-n" => out.push(it.next().ok_or("needs value")?.to_string()), s if s.starts_with("--e=") => out.extend(s[4..].split(',').map(String::from)), o => return Err(format!("bad `{}`", o)) } } Ok(out) }
fn main() { println!("{:?} {:?} {:?}", p(&["-n", "x", "--e=a,b"]), p(&["-n"]), p(&["-z"])); }
