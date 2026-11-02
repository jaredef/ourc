trait Shape { fn area(&self) -> i64; fn name(&self) -> String { "shape".to_string() } }
struct Sq(i64); struct Rect { w: i64, h: i64 }
impl Shape for Sq { fn area(&self) -> i64 { self.0 * self.0 } fn name(&self) -> String { format!("sq{}", self.0) } }
impl Shape for Rect { fn area(&self) -> i64 { self.w * self.h } }
fn total<S: Shape>(xs: &[S]) -> i64 { xs.iter().map(|s| s.area()).sum() }
fn main() { println!("{}", total(&[Sq(2), Sq(3)])); }
