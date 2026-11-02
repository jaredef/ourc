use std::ops::Add; #[derive(Clone, Copy)] struct V(i64); impl Add for V { type Output = V; fn add(self, o: V) -> V { V(self.0 + o.0) } } fn main() { let c = V(40) + V(2); println!("{}", c.0); }
