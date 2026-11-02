trait Shape { fn area(&self) -> f64; fn name(&self) -> String { String::from("shape") } }
struct Circle { r: f64 } struct Rect { w: f64, h: f64 }
impl Shape for Circle { fn area(&self) -> f64 { 3.0 * self.r * self.r } fn name(&self) -> String { String::from("circle") } }
impl Shape for Rect { fn area(&self) -> f64 { self.w * self.h } }
fn main() { let v: Vec<Box<dyn Shape>> = vec![Box::new(Circle { r: 2.0 }), Box::new(Rect { w: 3.0, h: 4.0 })]; let mut t = 0.0; for s in v.iter() { println!("{} {}", s.name(), s.area()); t += s.area(); } println!("{}", t); }
