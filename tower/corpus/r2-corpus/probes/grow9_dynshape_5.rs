trait Shape { fn area(&self) -> i64; }
struct Sq { s: i64 }
struct Rect { w: i64, h: i64 }
impl Shape for Sq { fn area(&self) -> i64 { self.s * self.s } }
impl Shape for Rect { fn area(&self) -> i64 { self.w * self.h } }
fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Sq{s:3}), Box::new(Rect{w:2,h:5}), Box::new(Sq{s:4})];
    let mut total = 0;
    for s in &shapes { total += s.area(); }
    println!("{}", total);
}
