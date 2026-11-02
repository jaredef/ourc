// R1 keystone: dispatch a trait method through a generic param bound to a PRIMITIVE type-arg (by value).
trait Sh { fn n(&self) -> i32; }
impl Sh for i32 { fn n(&self) -> i32 { *self } }
fn f<T: Sh>(x: T) -> i32 { x.n() }
fn main() { println!("{}", f(42) + f(8)); }
