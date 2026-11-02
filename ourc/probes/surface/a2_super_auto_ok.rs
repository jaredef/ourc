trait Tr: Send { fn v(&self) -> i32; }
struct S(i32);
impl Tr for S { fn v(&self) -> i32 { self.0 } }
trait All: Send {}
impl<T: Send> All for T {}
fn main() { println!("{}", S(4).v()); }
