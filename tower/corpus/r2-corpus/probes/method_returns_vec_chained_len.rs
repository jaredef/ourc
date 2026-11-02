struct C { d: i64 }
impl C { fn mk(&self) -> Vec<i64> { let mut v: Vec<i64> = Vec::new(); v.push(1); v.push(2); v } }
fn main() { let c = C { d: 0 }; println!("{}", c.mk().len()); }
