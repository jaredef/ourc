trait Shape { fn area(&self) -> i32; fn scale<T>(&self, t: T) -> i32 where Self: Sized { 0 } }
struct Sq(i32);
impl Shape for Sq { fn area(&self) -> i32 { self.0 * self.0 } }
fn main() {
    let s: Box<dyn Shape> = Box::new(Sq(3));
    println!("{}", s.area());
}
