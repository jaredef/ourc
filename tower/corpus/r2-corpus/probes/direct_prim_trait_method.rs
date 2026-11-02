// R1 keystone: a trait method implemented on a primitive, called directly on a scalar receiver.
trait Sh { fn n(&self) -> i32; }
impl Sh for i32 { fn n(&self) -> i32 { *self * 2 } }
fn main() { let x = 21; println!("{}", x.n()); }
