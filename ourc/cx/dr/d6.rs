fn p(args: &[&str]) -> Result<Vec<String>, String> { let mut out = Vec::new(); let mut it = args.iter(); while let Some(&a) = it.next() { match a { "-n" => { let v = it.next().ok_or("needs value")?; out.push(v.to_string()); } _ => {} } } Ok(out) }
fn main() { println!("{:?} {:?}", p(&["-n", "x"]), p(&["-n"])); }
