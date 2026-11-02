trait Shape { fn area(&self) -> i32; }
struct Sq { s: i32 }
impl Shape for Sq { fn area(&self) -> i32 { self.s * self.s } }
fn g<T: Shape>(s: &T) -> i32 { s.area() }
fn main() {
    let sq = Sq { s: 6 };
    println!("{}", g(&sq));
}
