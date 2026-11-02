use std::ops::Add;
#[derive(Clone, Copy)]
struct V { x: i64 }
impl Add for V { type Output = V; fn add(self, o: V) -> V { V { x: self.x + o.x } } }
fn main(){ let a = V { x: 2 }; let b = V { x: 5 }; let c = a + b; println!("{}", c.x + 1); }
