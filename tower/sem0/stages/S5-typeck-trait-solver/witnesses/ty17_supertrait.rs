trait A { fn a(&self) -> i32; }
trait B: A { fn b(&self) -> i32; }
struct S;
impl A for S { fn a(&self) -> i32 { 1 } }
impl B for S { fn b(&self) -> i32 { 2 } }
fn f<T: B>(t: T) -> i32 { t.a() + t.b() }
fn main() {}
