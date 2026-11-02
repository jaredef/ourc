fn id<T>(x: T) -> T { x }
fn g<T>(x: T) -> T { id(x) }
fn f() { let m: u8 = 3; g(m); g(5); }
fn main() { f(); }
