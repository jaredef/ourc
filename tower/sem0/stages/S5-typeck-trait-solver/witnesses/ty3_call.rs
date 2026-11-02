fn add(a: i32, b: i32) -> i32 { a + b }
fn g() -> i32 { 5 }
fn f() -> i32 { let x = add(1, 2); let y = g(); x + y }
fn main() {}
