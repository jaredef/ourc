fn id<T>(x: T) -> T { x }
fn f() -> i32 { let a = id(5); a }
fn main() {}
