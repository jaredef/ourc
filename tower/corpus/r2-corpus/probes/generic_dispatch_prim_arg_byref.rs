// R1 keystone: dispatch a trait method through a generic param bound to a PRIMITIVE type-arg (by ref).
trait Sh { fn n(&self) -> i32; }
impl Sh for i32 { fn n(&self) -> i32 { *self + 100 } }
fn f<T: Sh>(x: &T) -> i32 { x.n() }
fn main() { println!("{}", f(&42)); }
