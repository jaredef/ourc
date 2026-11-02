trait Area { fn side(&self) -> i64; fn area(&self) -> i64 { self.side() * self.side() } }
struct Sq(i64);
impl Area for Sq { fn side(&self) -> i64 { self.0 } }
fn main() { println!("{}", Sq(6).area()); }
