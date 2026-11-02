fn p(args: &[&str]) -> Result<Vec<String>, String> { let mut out = Vec::new(); let mut it = args.iter(); while let Some(&a) = it.next() { if a == "-n" { out.push(it.next().ok_or("needs value")?.to_string()); } } Ok(out) }
fn main() { println!("{:?} {:?}", p(&["-n", "x"]), p(&["-n"])); }
