trait Show { fn val(&self) -> i32; }
struct Wrap { n: i32 }
impl Show for Wrap { fn val(&self) -> i32 { self.n } }
impl Wrap { fn bump(&self, k: i32) -> i32 { k } }
fn id<T>(x: T) -> T { x }
fn showit<X: Show>(x: X) -> i32 { x.val() }
fn compute(w: Wrap, a: i32) -> i32 {
    let base: i32 = 10;
    let g = id(base);
    let v = w.val();
    let b = w.bump(a + g);
    v + b + base
}
fn main() {}
