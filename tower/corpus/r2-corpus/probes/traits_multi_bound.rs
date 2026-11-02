trait A { fn a(&self) -> i64; }
trait B { fn b(&self) -> i64; }
struct S;
impl A for S { fn a(&self) -> i64 { 3 } }
impl B for S { fn b(&self) -> i64 { 4 } }
fn f<T: A + B>(t: &T) -> i64 { t.a() + t.b() }
fn main() { println!("{}", f(&S)); }
