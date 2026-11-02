trait Shape { fn area(&self) -> i64; }
struct Sq(i64);
impl Shape for Sq { fn area(&self) -> i64 { self.0 * self.0 } }
fn total<S: Shape>(xs: &[S]) -> i64 { xs.iter().map(|s| s.area()).sum() }
fn total2<S: Shape>(xs: &[S]) -> i64 { let mut t = 0; for s in xs { t += s.area(); } t }
fn main() { let v = vec![Sq(2), Sq(3)]; println!("{} {} {}", total(&v), total2(&v), total2(&[Sq(4)])); }
