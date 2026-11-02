fn id<T>(x: T) -> T { x }
fn pair<A, B>(a: A, b: B) -> A { a }
fn f() -> i32 { let m: u8 = 3; let a = id(5); let b = id(m); let c = pair(m, 7); a }
fn main() { f(); }
