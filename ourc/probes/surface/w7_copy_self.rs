#[derive(Copy, Clone)]
struct Value { n: isize }
impl Value {
    fn squared(mut self) -> Value { self.n *= self.n; self }
    fn peek(&self) -> isize { self.n }
    fn bump(&mut self) { self.n += 1; }
}
fn main() {
    let x = Value { n: 3 };
    let y = x.squared();
    let mut z = x;
    z.bump();
    println!("{} {} {} {}", x.n, y.n, z.peek(), x.peek());
}
