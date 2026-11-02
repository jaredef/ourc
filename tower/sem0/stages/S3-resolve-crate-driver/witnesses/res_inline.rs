mod a { pub fn f() -> i32 { 1 } }
use a::f;
fn main() { let x = f(); }
