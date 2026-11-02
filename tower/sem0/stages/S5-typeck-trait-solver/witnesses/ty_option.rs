fn f() -> Option<i32> { Some(5) }
fn g() -> Option<i32> { let x: Option<i32> = None; x }
fn h(n: i32) -> Option<i32> { let y = Some(n); y }
fn main() {}
