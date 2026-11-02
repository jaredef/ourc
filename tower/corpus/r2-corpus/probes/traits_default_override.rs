trait T { fn v(&self) -> i64 { 1 } }
struct A; struct B;
impl T for A {}
impl T for B { fn v(&self) -> i64 { 9 } }
fn main() { println!("{}", A.v() + B.v()); }
