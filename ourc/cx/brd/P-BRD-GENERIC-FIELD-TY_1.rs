use std::fmt::Debug;
struct Stack<T> { v: Vec<T> }
impl<T: Clone + Debug> Stack<T> { fn new() -> Self { Stack { v: Vec::new() } } fn push2(&mut self, t: T) { self.v.push(t.clone()); self.v.push(t); } }
fn main() { let mut s1: Stack<char> = Stack::new(); s1.push2('x'); let mut s2: Stack<Vec<u64>> = Stack::new(); s2.push2(vec![1u64]); println!("{:?} {:?}", s1.v, s2.v); }
