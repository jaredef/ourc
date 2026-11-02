fn id<T>(x: T) -> T { x }
fn f() { let m: u8 = 1; id(m); id(m); id(5); }
fn g() { let m: u8 = 1; id(m); }
fn main() { f(); g(); }
