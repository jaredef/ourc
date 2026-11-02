trait T { fn m(&self) -> i32; }
struct A;
impl T for A { fn m(&self) -> i32 { 1 } }
fn call<X: T>(x: X) -> i32 { x.m() }
fn f() -> i32 { call(A) }
fn main() {}
