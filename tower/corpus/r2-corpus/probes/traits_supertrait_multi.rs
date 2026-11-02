trait X { fn x(&self) -> i64; }
trait Y { fn y(&self) -> i64; }
trait Z: X + Y { fn z(&self) -> i64 { self.x() + self.y() } }
struct S;
impl X for S { fn x(&self) -> i64 { 20 } }
impl Y for S { fn y(&self) -> i64 { 5 } }
impl Z for S {}
fn main() { println!("{}", S.z()); }
