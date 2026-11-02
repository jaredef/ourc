fn p(args: &[&str]) -> Vec<String> { let mut out = Vec::new(); let mut it = args.iter(); while let Some(&a) = it.next() { out.push(a.to_string()); } out }
fn main() { println!("{:?}", p(&["-n", "x"])); }
