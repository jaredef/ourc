use std::ops::*;
#[derive(Debug, Clone, Copy, PartialEq)] struct V(i32, i32);
impl Add for V { type Output = V; fn add(self, o: V) -> V { V(self.0 + o.0, self.1 + o.1) } }
impl Mul<i32> for V { type Output = V; fn mul(self, k: i32) -> V { V(self.0 * k, self.1 * k) } }
impl Neg for V { type Output = V; fn neg(self) -> V { V(-self.0, -self.1) } }
impl AddAssign for V { fn add_assign(&mut self, o: V) { self.0 += o.0; self.1 += o.1; } }
fn main() { let a = V(7, 9); let b = V(2, 3); println!("{:?}", a + b); }
