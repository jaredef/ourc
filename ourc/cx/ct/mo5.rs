struct Stack { items: Vec<String> }
impl Stack { fn push(&mut self, t: String) -> &mut Self { self.items.push(t); self } }
fn main() { let mut s = Stack { items: Vec::new() }; let r = s.push("c".into()); println!("{}", r.items.len()); r.push("d".into()); println!("{}", s.items.len()); }
