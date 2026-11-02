trait P { fn p(&self) -> i32; }
trait Q { fn q(&self) -> i32; }
struct A;
impl P for A { fn p(&self) -> i32 { 1 } }
impl Q for A { fn q(&self) -> i32 { 2 } }
fn both<X: P + Q>(x: X) -> i32 { x.p() }
fn whr<Y>(y: Y) -> i32 where Y: P { y.p() }
fn main() {}
