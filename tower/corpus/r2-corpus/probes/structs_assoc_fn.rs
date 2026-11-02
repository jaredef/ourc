struct P { x: i64, y: i64 }
impl P { fn origin() -> P { P { x: 0, y: 0 } } fn sum(&self) -> i64 { self.x + self.y } }
fn main() {
    let p = P::origin();
    println!("{}", p.sum() + 42);
}
