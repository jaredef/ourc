struct Stack { items: Vec<String> }
impl Stack { fn push(&mut self, t: String) -> &mut Self { self.items.push(t); self } }
fn main() { let mut s = Stack { items: Vec::new() }; s.push("c".into()).push("d".into()); println!("{}", s.items.len()); }
