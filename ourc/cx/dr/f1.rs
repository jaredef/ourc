struct Cx { log: Vec<String> }
fn a(c: &mut Cx) -> Result<(), String> { c.log.push("a".into()); Ok(()) }
fn b(c: &mut Cx) -> Result<(), String> { Err(format!("b failed after {}", c.log.len())) }
fn main() { let mut cx = Cx { log: vec![] }; let ps: Vec<(&str, fn(&mut Cx) -> Result<(), String>)> = vec![("a", a), ("b", b)];
  for (n, p) in &ps { let r = p(&mut cx); println!("{} {:?}", n, r); } println!("{:?}", cx.log); }
