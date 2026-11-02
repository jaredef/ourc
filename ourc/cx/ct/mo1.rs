trait Shape { fn area(&self) -> i64; }
struct Sq(i64);
impl Shape for Sq { fn area(&self) -> i64 { self.0 * self.0 } }
fn total<S: Shape>(xs: &[S]) -> i64 { xs.iter().map(|s| s.area()).sum() }
fn main() { println!("{}", total(&[Sq(2), Sq(3)])); let v = [Sq(1), Sq(4)]; println!("{}", total(&v)); }
