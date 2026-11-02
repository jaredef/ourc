trait T { fn m(&self) -> i32; }
struct A;
impl T for A { fn m(&self) -> i32 { 1 } }
fn keep<X: T>(x: X) -> X { x }
fn mk() -> A { A }
fn main() {}
