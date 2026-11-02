// trait-core P-TRAIT-OPERATOR: a parenthesized operator-overload result followed by .field/.method must
// route through struct dispatch (opt_postfix does not handle a struct .field -> the raw pointer leaked).
use std::ops::{Add, Neg};
#[derive(Clone, Copy)] struct V { x: i32, y: i32 }
impl Add for V { type Output = V; fn add(self, o: V) -> V { V { x: self.x + o.x, y: self.y + o.y } } }
impl Neg for V { type Output = V; fn neg(self) -> V { V { x: -self.x, y: -self.y } } }
fn main() {
    let a = V { x: 2, y: 10 };
    let b = V { x: 3, y: 20 };
    println!("{} {} {}", (a + b).x, (a + b).y, (-a).x);
}
