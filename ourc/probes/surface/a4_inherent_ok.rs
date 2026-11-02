struct P { x: i32 }
impl P { fn d(&self) -> i32 { self.x * 2 } }
trait Tr { fn t(&self) -> i32; }
impl Tr for P { fn t(&self) -> i32 { self.x + 1 } }
fn main() { let p = P { x: 3 }; println!("{} {}", p.d(), p.t()); }
