struct Cx { log: Vec<String> }
fn a(c: &mut Cx) -> Result<(), String> { c.log.push("a".into()); Ok(()) }
fn b(c: &mut Cx) -> Result<(), String> { Err(format!("b failed after {}", c.log.len())) }
fn main() { let mut cx = Cx { log: vec![] }; let ps: Vec<(&str, fn(&mut Cx) -> Result<(), String>)> = vec![("a", a), ("b", b)];
  let r: Result<(), String> = (|| { for (n, p) in &ps { p(&mut cx).map_err(|e| format!("[{}] {}", n, e))?; } Ok(()) })(); println!("{:?} {:?}", r, cx.log); }
