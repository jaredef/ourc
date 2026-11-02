struct Point { x: i32, y: i32 }
impl Point {
    fn new(x: i32, y: i32) -> Point { Point { x: x, y: y } }
    fn norm2(&self) -> i32 { self.x * self.x + self.y * self.y }
}
fn main() {
    let p = Point::new(3, 4);
    println!("norm2 = {}", p.norm2());
    println!("x = {}", p.x);
}
