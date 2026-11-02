trait Ct { type Item; fn first(&self) -> i32; }
struct A;
impl Ct for A { type Item = i32; fn first(&self) -> i32 { 0 } }
fn use_it(a: A) -> i32 { a.first() }
fn main() {}
