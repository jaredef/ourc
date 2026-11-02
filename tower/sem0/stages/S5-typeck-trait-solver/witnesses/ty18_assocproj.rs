trait Ct { type Item; fn get(&self) -> Self::Item; }
struct A;
impl Ct for A { type Item = i32; fn get(&self) -> Self::Item { 0 } }
fn f(a: A) -> i32 { a.get() }
fn main() {}
