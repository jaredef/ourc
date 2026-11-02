struct Stack<T> { items: Vec<T> }
impl<T: Clone + std::fmt::Debug> Stack<T> { fn new() -> Self { Stack { items: Vec::new() } } fn push(&mut self, t: T) -> &mut Self { self.items.push(t); self } fn top(&self) -> Option<T> { self.items.last().cloned() } }
fn main() { let mut s: Stack<String> = Stack::new(); s.push("a".into()); s.push("b".into()); println!("{:?} {}", s.top(), s.items.len()); s.push("c".into()).push("d".into()); println!("{}", s.items.len()); }
