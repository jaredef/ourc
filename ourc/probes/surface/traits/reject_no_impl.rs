trait T { fn m(&self) -> i64; } struct A; fn g<X: T>(x: X) -> i64 { x.m() } fn main() { g(A); }
