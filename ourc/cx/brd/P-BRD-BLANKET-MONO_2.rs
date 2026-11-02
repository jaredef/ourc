trait Area { fn area(&self) -> u32; }
trait Report { fn report(&self) -> String; }
impl<T: Area> Report for T { fn report(&self) -> String { format!("area {}", self.area()) } }
struct Sq(u32); impl Area for Sq { fn area(&self) -> u32 { self.0 * self.0 } }
struct R(u32, u32); impl Area for R { fn area(&self) -> u32 { self.0 * self.1 } }
fn main() { println!("{} {}", Sq(4).report(), R(2, 9).report()); }
