// R2a P5 guard: a value that is ALREADY a trait object (a `-> Box<dyn Sh>` result) used where a `Box<dyn Sh>` is expected
// -- a let, a vec! mixed with a fresh coercion, a push -- is not re-wrapped (it found no `impl Sh for dyn Sh` and refused)
trait Sh {
    fn area(&self) -> i32;
}
struct Sq(i32);
impl Sh for Sq {
    fn area(&self) -> i32 {
        self.0 * self.0
    }
}
fn mk() -> Box<dyn Sh> {
    Box::new(Sq(3))
}
fn main() {
    let b: Box<dyn Sh> = mk();
    let v: Vec<Box<dyn Sh>> = vec![mk(), Box::new(Sq(2))];
    let mut w: Vec<Box<dyn Sh>> = Vec::new();
    w.push(mk());
    for s in v {
        print!("{} ", s.area());
    }
    println!("{} {}", b.area(), w[0].area());
}
