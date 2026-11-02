trait Shape {
    type Part<T>
    where
        Self: Sized;
    fn area(&self) -> i32;
}
struct Sq(i32);
impl Shape for Sq {
    type Part<T> = T where Self: Sized;
    fn area(&self) -> i32 {
        self.0 * self.0
    }
}
fn total(s: &dyn Shape) -> i32 {
    s.area()
}
fn main() {
    println!("{}", total(&Sq(3)));
}
