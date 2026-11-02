struct C { d: i64 }
impl C { fn mk(&self) -> Vec<i64> { let mut v: Vec<i64> = Vec::new(); v.push(3); v.push(4); v } }
fn main() { let c = C { d: 0 }; let inner = c.mk(); let mut s = 0; for x in inner.iter() { s = s + x; } println!("{}", s); }
