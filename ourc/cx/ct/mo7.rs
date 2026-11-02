trait Shape { fn area(&self) -> i64; fn name(&self) -> String { "shape".to_string() } }
struct Sq(i64); struct Rect { w: i64, h: i64 }
impl Shape for Sq { fn area(&self) -> i64 { self.0 * self.0 } fn name(&self) -> String { format!("sq{}", self.0) } }
impl Shape for Rect { fn area(&self) -> i64 { self.w * self.h } }
fn total<S: Shape>(xs: &[S]) -> i64 { xs.iter().map(|s| s.area()).sum() }
fn main() { let a = [Sq(2), Sq(3)]; let b = [Rect { w: 2, h: 5 }]; println!("{} {}", total(&a), total(&b)); let c = vec![Sq(1)]; let d = vec![Rect { w: 1, h: 1 }]; println!("{} {}", total(&c), total(&d)); }
