struct Stack<T> { items: Vec<T> }
impl<T: Clone> Stack<T> { fn top(&self) -> Option<T> { self.items.last().cloned() } }
fn main() { let s: Stack<String> = Stack { items: vec!["a".to_string(), "b".to_string()] }; println!("{:?}", s.top()); let n: Stack<u32> = Stack { items: vec![1, 2] }; println!("{:?}", n.top()); }
