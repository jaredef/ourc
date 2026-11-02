trait Base { fn id(&self) -> i64; }
trait Ext: Base { fn plus(&self) -> i64 { self.id() + 100 } }
struct S;
impl Base for S { fn id(&self) -> i64 { 5 } }
impl Ext for S {}
fn main() { println!("{}", S.plus()); }
